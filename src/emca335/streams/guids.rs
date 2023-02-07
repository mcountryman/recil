//! `#GUID` stream data.

use crate::{
  emca335::header::{HeapSizes, StreamHeader, TablesStreamHeader},
  error::Error,
};
use scroll::{
  ctx::{SizeWith, TryFromCtx},
  Pread, LE,
};

/// An index into the [GuidsHeap].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct GuidId(u32);

/// Contains a globally unique identifier, indexable by a [GuidId].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pread)]
#[repr(transparent)]
pub struct Guid(pub [u8; 16]);

/// Contains a sequence of 128-bit GUIDs, indexable by a [GuidId].  May contain unreachable GUIDs.
#[derive(Debug, Default)]
pub struct GuidsHeap<'a>(pub(crate) &'a [u8]);

impl<'a> GuidsHeap<'a> {
  /// Parses the `#GUID` stream from the given buffer.
  pub fn parse(from: &'a [u8], header: &StreamHeader<'a>) -> Result<Self, Error> {
    Ok(Self(header.data(from)?))
  }

  pub fn get(&self, id: GuidId) -> Result<Option<Guid>, Error> {
    if id.0 == 0 {
      return Ok(None);
    }

    let offset = id.0 + 1;
    let offset = usize::try_from(offset).map_err(|_| Error::BadGuidId(id.0))?;
    let offset = offset * 16;

    self
      .0
      .pread_with::<Guid>(offset, LE)
      .map(Some)
      .map_err(Error::from)
  }
}

impl<'a> TryFromCtx<'a, &TablesStreamHeader> for GuidId {
  type Error = Error;

  fn try_from_ctx(
    from: &'a [u8],
    header: &TablesStreamHeader,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let id = Self(
      match header.heap_sizes.contains(HeapSizes::WIDE_GUID_HEAP) {
        true => from.gread_with::<u32>(offset, scroll::LE)?,
        false => from.gread_with::<u16>(offset, scroll::LE)?.into(),
      },
    );

    Ok((id, *offset))
  }
}

impl SizeWith<TablesStreamHeader> for GuidId {
  fn size_with(ctx: &TablesStreamHeader) -> usize {
    match ctx.heap_sizes.contains(HeapSizes::WIDE_GUID_HEAP) {
      true => 4,
      false => 2,
    }
  }
}
