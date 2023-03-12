//! ECMA-335 Metadata Format.

pub mod blobs;
pub mod guids;
pub mod strings;
pub mod tables;

use self::{blobs::Blobs, guids::Guids, strings::Strings, tables::Tables};
use anyhow::{anyhow, bail, Error, Result};
use core::ffi::CStr;
use scroll::{ctx::TryFromCtx, Pread, LE};

/// Partially decoded ECMA-335 metadata.
pub struct Md<'a> {
  guids: Guids<'a>,
  blobs: Blobs<'a>,
  tables: Tables<'a>,
  strings: Strings<'a>,
}

impl<'a> Md<'a> {
  /// Parses the ECMA-335 metadata from the given buffer containing the metadata.
  pub fn from_cli_data(buf: &'a [u8]) -> Result<Self> {
    let offset = &mut 0;
    let header = buf.gread::<MdHeader>(offset)?;
    if header.magic != MD_MAGIC {
      bail!("Bad magic {:#x}", header.magic);
    }

    let guids = header
      .streams
      .guids
      .map(Guids::parse_from_header)
      .unwrap_or(Ok(Default::default()))?;

    let blobs = header
      .streams
      .blobs
      .map(Blobs::parse_from_header)
      .unwrap_or(Ok(Default::default()))?;

    let tables = header
      .streams
      .tables
      .map(Tables::parse_from_header)
      .unwrap_or(Ok(Default::default()))?;

    let strings = header
      .streams
      .strings
      .map(Strings::parse_from_header)
      .unwrap_or(Ok(Default::default()))?;

    Ok(Self {
      guids,
      blobs,
      tables,
      strings,
    })
  }

  /// Gets the guids metadata stream.
  pub fn guids(&self) -> &Guids {
    &self.guids
  }

  /// Gets the blobs metadata stream.
  pub fn blobs(&self) -> &Blobs {
    &self.blobs
  }

  /// Gets the tables metadata stream.
  pub fn tables(&self) -> &Tables {
    &self.tables
  }

  /// Gets the strings metadata stream.
  pub fn strings(&self) -> &Strings {
    &self.strings
  }
}

/// The magic number for the metadata header.
const MD_MAGIC: u32 = 0x424A5342;

// Header information for the metadata section.
pub struct MdHeader<'a> {
  /// The magic number for the metadata header. This should always be `0x424A5342`.
  pub magic: u32,
  /// The major version of the metadata header. This should always be `1`.
  pub major_version: u16,
  /// The minor version of the metadata header. This should always be `1`.
  pub minor_version: u16,
  /// The reserved bytes. This should always be `0`.
  _reserved_0: u32,
  /// The version string.
  pub version: &'a CStr,
  /// The reserved bytes. This should always be `0`.
  _reserved_1: u16,
  /// The stream headers.
  pub streams: StreamHeaders<'a>,
}

impl<'a> TryFromCtx<'a> for MdHeader<'a> {
  type Error = Error;

  fn try_from_ctx(from: &'a [u8], _: ()) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;

    Ok((
      Self {
        magic: from.gread_with(offset, LE)?,
        major_version: from.gread_with(offset, LE)?,
        minor_version: from.gread_with(offset, LE)?,
        _reserved_0: from.gread(offset)?,
        version: from.gread_with(offset, LengthPrefixed)?,
        _reserved_1: from.gread(offset)?,
        streams: from.gread_with(offset, from)?,
      },
      *offset,
    ))
  }
}

/// Contains the number of streams and the stream headers.
#[derive(Clone, Copy)]
pub struct StreamHeaders<'a> {
  /// The `#Blob` stream header.
  pub blobs: Option<StreamHeader<'a>>,
  /// The `#GUID` stream header.
  pub guids: Option<StreamHeader<'a>>,
  /// The `#~` stream header.
  pub tables: Option<StreamHeader<'a>>,
  /// The `#Strings` stream header.
  pub strings: Option<StreamHeader<'a>>,
  /// The `#US` stream header.
  pub user_strings: Option<StreamHeader<'a>>,
}

impl<'a> TryFromCtx<'a, &'a [u8]> for StreamHeaders<'a> {
  type Error = Error;

  fn try_from_ctx(from: &'a [u8], md_buf: &'a [u8]) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let count = from.gread_with::<u16>(offset, LE)?;

    // I would like to avoid allocating a vector here. According to the ECMA spec, the number of
    // possible streams is greater than the size of [StreamHeader; _] I'm willing to stack allocate.
    //
    // Certain obfuscators also, tend to append duplicate streams. To contend with these issues we
    // will just iterate over the streams taking the first one we see and also care about.
    //
    // Side node: if an obfuscator were to append a bunch of garbage streams before the ones the
    // runtime relies on we would be able to ignore them.  Although, I haven't seen anyone do this
    // yet.
    let mut blobs: Option<StreamHeader<'a>> = None;
    let mut guids: Option<StreamHeader<'a>> = None;
    let mut tables: Option<StreamHeader<'a>> = None;
    let mut strings: Option<StreamHeader<'a>> = None;
    let mut user_strings: Option<StreamHeader<'a>> = None;

    for _ in 0..count {
      let header = from.gread_with::<StreamHeader>(offset, md_buf)?;
      match header.name.to_bytes() {
        b"#~" | b"#-" => {
          tables.get_or_insert(header);
        }
        b"#Blob" => {
          blobs.get_or_insert(header);
        }
        b"#GUID" => {
          guids.get_or_insert(header);
        }
        b"#Strings" => {
          strings.get_or_insert(header);
        }
        b"#US" => {
          user_strings.get_or_insert(header);
        }
        _ => {}
      };
    }

    Ok((
      Self {
        blobs,
        guids,
        tables,
        strings,
        user_strings,
      },
      *offset,
    ))
  }
}

/// Contains the offset, size, and name of a stream in the metadata.
#[derive(Clone, Copy)]
pub struct StreamHeader<'a> {
  /// The slice containing the entire metadata section.
  md_buf: &'a [u8],
  /// The memory offset to the start of the stream from the start of the metadata root.
  pub offset: u32,
  /// The size of the stream in bytes.
  pub size: u32,
  /// The name of the stream.
  pub name: &'a CStr,
}

impl<'a> StreamHeader<'a> {
  /// Gets the data for the stream.
  pub fn data(&self) -> Result<&'a [u8], Error> {
    let offset = self.offset;
    let offset = &mut usize::try_from(offset).map_err(|_| anyhow!("Not enough bytes"))?;

    let size = self.size;
    let size = usize::try_from(size).map_err(|_| anyhow!("Not enough bytes"))?;

    if size == 0 {
      return Ok(&self.md_buf[0..0]);
    }

    Ok(self.md_buf.gread_with(offset, size)?)
  }
}

impl<'a> TryFromCtx<'a, &'a [u8]> for StreamHeader<'a> {
  type Error = Error;

  fn try_from_ctx(from: &'a [u8], md_buf: &'a [u8]) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;

    Ok((
      Self {
        offset: from.gread_with(offset, LE)?,
        size: from.gread_with(offset, LE)?,
        name: from.gread_with(offset, FourByteBoundaryPadded)?,
        md_buf,
      },
      *offset,
    ))
  }
}

/// A context for reading a `&CStr` that is prefixed with a `u32` length.
#[derive(Clone, Copy)]
struct LengthPrefixed;

impl<'a> TryFromCtx<'a, LengthPrefixed> for &'a CStr {
  type Error = Error;

  fn try_from_ctx(from: &'a [u8], _: LengthPrefixed) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0usize;
    let length = from.gread_with::<u32>(offset, LE)?;
    let cstr = from.gread_with::<&CStr>(offset, ())?;

    Ok((cstr, length as usize + 4))
  }
}

/// A context for reading a `&CStr` that is padded to a 4-byte boundary.
#[derive(Clone, Copy)]
struct FourByteBoundaryPadded;

impl<'a> TryFromCtx<'a, FourByteBoundaryPadded> for &'a CStr {
  type Error = Error;

  fn try_from_ctx(from: &'a [u8], _: FourByteBoundaryPadded) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0usize;
    let cstr = from.gread_with::<&CStr>(offset, ())?;

    // Padding (4-byte aligned)
    let cstr_len = cstr.to_bytes_with_nul().len();
    let cstr_pad = ((cstr_len + 3) & !3) - cstr_len;

    *offset += cstr_pad;

    Ok((cstr, *offset))
  }
}
