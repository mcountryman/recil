//! ECMA-335 Metadata `#Strings` stream data.

use super::{
  tables::{HeapSizes, TablesHeader},
  StreamHeader,
};
use anyhow::Result;
use core::ffi::CStr;
use scroll::{
  ctx::{SizeWith, TryFromCtx},
  Pread, LE,
};

/// Contains the strings in the `#Strings` stream.
///
/// Strings are indexable by their offset into the stream data.  The first string is at offset 0.
/// String lengths are not stored in the stream, so the length of a string is determined by a null
/// terminator.
#[derive(Default)]
pub struct Strings<'a>(&'a [u8]);

impl<'a> Strings<'a> {
  /// Creates an instance of [Strings] from the given [StreamHeader].
  ///
  /// # Note
  /// The stream header must be for the `#Strings` stream and isn't verified here in release builds.
  pub fn parse_from_header(header: StreamHeader<'a>) -> Result<Self> {
    debug_assert!(matches!(header.name.to_bytes(), b"#Strings"));

    Ok(Self(header.data()?))
  }

  /// Gets the string at the given [StringIndex].
  pub fn get(&self, index: StringIndex) -> Result<&'a str> {
    let offset = index.0;
    let cstr = self.0.pread_with::<&CStr>(offset, ())?;

    Ok(core::str::from_utf8(cstr.to_bytes())?)
  }
}

// An index into the [Strings] stream.
#[derive(Debug, Clone, Copy)]
pub struct StringIndex(usize);

impl<'a> TryFromCtx<'a, TablesHeader> for StringIndex {
  type Error = anyhow::Error;

  fn try_from_ctx(from: &'a [u8], header: TablesHeader) -> Result<(Self, usize)> {
    let offset = &mut 0;
    let index = match header.heap_sizes.contains(HeapSizes::WIDE_STRING_HEAP) {
      true => from.gread_with::<u32>(offset, LE)? as usize,
      false => from.gread_with::<u16>(offset, LE)? as usize,
    };

    Ok((Self(index), *offset))
  }
}

impl SizeWith<TablesHeader> for StringIndex {
  fn size_with(header: &TablesHeader) -> usize {
    match header.heap_sizes.contains(HeapSizes::WIDE_STRING_HEAP) {
      true => 4,
      false => 2,
    }
  }
}
