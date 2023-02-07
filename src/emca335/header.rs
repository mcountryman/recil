//! EMCA-335 CLI metadata headers.

use crate::error::Error;
use core::ffi::CStr;
use scroll::{ctx::TryFromCtx, Pread, LE};

/// The magic number for the metadata header.
pub const METADATA_MAGIC: u32 = 0x424A5342;

/// Contains the root metadata header information.
#[derive(Debug, Clone, Copy)]
pub struct MetadataHeader<'a> {
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

/// Contains the number of streams and the stream headers.
#[derive(Debug, Clone, Copy)]
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

/// Contains the offset, size, and name of a stream in the metadata.
#[derive(Debug, Clone, Copy)]
pub struct StreamHeader<'a> {
  /// The memory offset to the start of the stream from the start of the metadata root.
  pub offset: u32,
  /// The size of the stream in bytes.
  pub size: u32,
  /// The name of the stream.
  pub name: &'a CStr,
}

/// Contains the table stream header information.
#[derive(Debug, Clone, Copy)]
pub struct TablesStreamHeader {
  // Reserved, always 0.
  _reserved_0: u32,
  /// Major version of table schemata; shall be 2.
  pub major_version: u8,
  /// Minor version of table schemata; shall be 0.
  pub minor_version: u8,
  /// Bit vector for heap sizes.
  pub heap_sizes: HeapSizes,
  /// Reserved, always 1.
  _reserved_1: u8,
  /// Bit vector of present tables, let n be the number of bits that are 1.
  pub valid: u64,
  /// Bit vector of sorted tables.
  pub sorted: u64,
  /// The number of rows for each present table.
  pub rows: [u32; 64],
}

impl TablesStreamHeader {
  /// Determines if the table with the given index is present.
  pub fn is_valid(&self, i: usize) -> bool {
    self.valid & (1u64 << i as u64) != 0
  }
}

bitflags::bitflags! {
  /// The bit flags indicating which heaps should have 4 bit wide indexes or 2 bit wide indexes.
  #[derive(Default, Pread)]
  pub struct HeapSizes : u8 {
    /// If set indicates the `#Strings` heap index should be `4` bytes wide, otherwise `2`.
    const WIDE_STRING_HEAP = 0x01;
    /// If set indicates the `#GUID` heap index should be `4` bytes wide, otherwise `2`.
    const WIDE_GUID_HEAP = 0x02;
    /// If set indicates the `#Blob` heap index should be `4` bytes wide, otherwise `2`.
    const WIDE_BLOB_HEAP = 0x04;
  }
}

impl<'a> TryFromCtx<'a> for MetadataHeader<'a> {
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
        streams: from.gread(offset)?,
      },
      *offset,
    ))
  }
}

impl<'a> TryFromCtx<'a> for StreamHeaders<'a> {
  type Error = Error;

  fn try_from_ctx(from: &'a [u8], _: ()) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let count = from.gread_with::<u16>(offset, LE)?;

    // I would like to avoid allocating a vector here. According to the ECMA spec, the number of
    // possible streams is greater than the size of [StreamHeader] I'm willing to stack allocate.
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
      let header = from.gread::<StreamHeader>(offset)?;
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

impl<'a> StreamHeader<'a> {
  /// Gets the data for the stream from the given buffer where the buffer is the entire metadata
  /// section.
  pub fn data(&self, buf: &'a [u8]) -> Result<&'a [u8], Error> {
    let offset = self.offset;
    let offset = &mut usize::try_from(offset).map_err(|_| Error::BadOffset(offset))?;

    let size = self.size;
    let size = usize::try_from(size).map_err(|_| Error::BadLength(size))?;

    if size == 0 {
      return Ok(&buf[0..0]);
    }

    Ok(buf.gread_with(offset, size)?)
  }
}

impl<'a> TryFromCtx<'a> for StreamHeader<'a> {
  type Error = Error;

  fn try_from_ctx(from: &'a [u8], _: ()) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;

    Ok((
      Self {
        offset: from.gread_with(offset, LE)?,
        size: from.gread_with(offset, LE)?,
        name: from.gread_with(offset, FourByteBoundaryPadded)?,
      },
      *offset,
    ))
  }
}

impl<'a> TryFromCtx<'a> for TablesStreamHeader {
  type Error = Error;

  fn try_from_ctx(from: &'a [u8], _: ()) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;

    *offset += 4;
    let major_version = from.gread_with(offset, LE)?;
    let minor_version = from.gread_with(offset, LE)?;
    let heap_sizes = from.gread_with(offset, LE)?;
    *offset += 1;
    let valid = from.gread_with(offset, LE)?;
    let sorted = from.gread_with(offset, LE)?;
    let mut rows = [0; 64];

    for (i, row) in rows.iter_mut().enumerate() {
      if valid & (1 << i) != 0 {
        *row = from.gread_with(offset, LE)?;
      }
    }

    Ok((
      Self {
        _reserved_0: 0,
        major_version,
        minor_version,
        heap_sizes,
        _reserved_1: 1,
        valid,
        sorted,
        rows,
      },
      *offset,
    ))
  }
}

impl Default for TablesStreamHeader {
  fn default() -> Self {
    Self {
      _reserved_0: 0,
      major_version: 0,
      minor_version: 0,
      heap_sizes: HeapSizes::default(),
      _reserved_1: 1,
      valid: 0,
      sorted: 0,
      rows: [0; 64],
    }
  }
}

/// A context for reading a `&CStr` that is prefixed with a `u32` length.
#[derive(Clone, Copy)]
struct LengthPrefixed;

/// A context for reading a `&CStr` that is padded to a 4-byte boundary.
#[derive(Clone, Copy)]
struct FourByteBoundaryPadded;

impl<'a> TryFromCtx<'a, LengthPrefixed> for &'a CStr {
  type Error = Error;

  fn try_from_ctx(from: &'a [u8], _: LengthPrefixed) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0usize;
    let length = from.gread_with::<u32>(offset, LE)?;
    let cstr = from.gread_with::<&CStr>(offset, ())?;

    Ok((cstr, length as usize + 4))
  }
}

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
