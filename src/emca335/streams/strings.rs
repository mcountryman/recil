//! `#Strings` stream data.

use crate::{
  emca335::header::{HeapSizes, StreamHeader, TablesStreamHeader},
  error::Error,
};
use core::ffi::CStr;
use scroll::{
  ctx::{SizeWith, TryFromCtx},
  Pread,
};

/// An index into the [StringsHeap].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct StringId(u32);

/// Contains null terminated UTF8 strings, indexable by a [StringId].  
///
/// The heap can contain garbage, as long as any part that is reachable from any of the tables
/// contains a valid null-terminated UTF8 string.
///
/// The first entry is always an empty string.
#[derive(Debug, Default)]
pub struct StringsHeap<'a>(pub(crate) &'a [u8]);

impl<'a> StringsHeap<'a> {
  /// Parses the `#Strings` stream from the given buffer.
  pub fn parse(from: &'a [u8], header: &StreamHeader<'a>) -> Result<Self, Error> {
    Ok(Self(header.data(from)?))
  }

  pub fn get(&self, id: StringId) -> Result<&'a str, Error> {
    let offset = id.0;
    let offset = usize::try_from(offset).map_err(|_| Error::BadStringId(offset))?;
    let cstr = self.0.pread_with::<&CStr>(offset, ())?;

    core::str::from_utf8(cstr.to_bytes()).map_err(Error::Utf8)
  }
}

impl<'a> TryFromCtx<'a, &TablesStreamHeader> for StringId {
  type Error = Error;

  fn try_from_ctx(
    from: &'a [u8],
    header: &TablesStreamHeader,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let id = Self(
      match header.heap_sizes.contains(HeapSizes::WIDE_STRING_HEAP) {
        true => from.gread_with::<u32>(&mut 0, scroll::LE)?,
        false => from.gread_with::<u16>(&mut 0, scroll::LE)?.into(),
      },
    );

    Ok((id, *offset))
  }
}

impl SizeWith<TablesStreamHeader> for StringId {
  fn size_with(ctx: &TablesStreamHeader) -> usize {
    match ctx.heap_sizes.contains(HeapSizes::WIDE_STRING_HEAP) {
      true => 4,
      false => 2,
    }
  }
}
