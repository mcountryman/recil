//! `#Blob` stream data.

use crate::{
  emca335::header::{HeapSizes, StreamHeader, TablesStreamHeader},
  error::Error,
};
use scroll::{
  ctx::{SizeWith, TryFromCtx},
  Pread,
};

/// An index into the [BlobsHeap].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct BlobId(u32);

/// Contains blobs of bytes prefixed by a variable sized length, indexable by a [BlobId].  
///
/// Can contain garbage data, as long as any part that is reachable from any of the tables contains
/// a valid 'blob'.
///
/// The first entry in both these heaps is the empty 'blob' that consists of the single byte `0x00`.
#[derive(Debug, Default)]
pub struct BlobsHeap<'a>(pub(crate) &'a [u8]);

impl<'a> BlobsHeap<'a> {
  /// Parses the `#Blob` stream from the given buffer.
  pub fn parse(from: &'a [u8], header: &StreamHeader<'a>) -> Result<Self, Error> {
    Ok(Self(header.data(from)?))
  }
}

impl<'a> TryFromCtx<'a, &TablesStreamHeader> for BlobId {
  type Error = Error;

  fn try_from_ctx(
    from: &'a [u8],
    header: &TablesStreamHeader,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let id = Self(
      match header.heap_sizes.contains(HeapSizes::WIDE_BLOB_HEAP) {
        true => from.gread_with::<u32>(&mut 0, scroll::LE)?,
        false => from.gread_with::<u16>(&mut 0, scroll::LE)?.into(),
      },
    );

    Ok((id, *offset))
  }
}

impl SizeWith<TablesStreamHeader> for BlobId {
  fn size_with(ctx: &TablesStreamHeader) -> usize {
    match ctx.heap_sizes.contains(HeapSizes::WIDE_BLOB_HEAP) {
      true => 4,
      false => 2,
    }
  }
}
