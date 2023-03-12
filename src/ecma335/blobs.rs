//! ECMA-335 Metadata `#Blob` stream data.

use super::{
  tables::{HeapSizes, TablesHeader},
  StreamHeader,
};
use anyhow::{anyhow, Result};
use scroll::{
  ctx::{SizeWith, TryFromCtx},
  Pread, LE,
};

/// Contains the blobs in the `#Blob` stream.
#[derive(Default)]
pub struct Blobs<'a>(&'a [u8]);

impl<'a> Blobs<'a> {
  /// Creates an instance of [Blobs] from the given [StreamHeader].
  ///
  /// # Note
  /// The stream header must be for the `#Blob` stream and isn't verified here in release builds.
  pub fn parse_from_header(header: StreamHeader<'a>) -> Result<Self> {
    debug_assert!(matches!(header.name.to_bytes(), b"#Blob"));

    Ok(Self(header.data()?))
  }

  /// Gets the blob at the given [BlobIndex].
  pub fn get(&self, index: BlobIndex) -> Result<&'a str> {
    todo!()
  }
}

// An index into the [Blobs] stream.
#[derive(Debug, Clone, Copy)]
pub struct BlobIndex(usize);

impl<'a> TryFromCtx<'a, TablesHeader> for BlobIndex {
  type Error = anyhow::Error;

  fn try_from_ctx(from: &'a [u8], header: TablesHeader) -> Result<(Self, usize)> {
    let offset = &mut 0;
    let index = match header.heap_sizes.contains(HeapSizes::WIDE_BLOB_HEAP) {
      true => from.gread_with::<u32>(offset, LE)? as usize,
      false => from.gread_with::<u16>(offset, LE)? as usize,
    };

    Ok((Self(index), *offset))
  }
}

impl SizeWith<TablesHeader> for BlobIndex {
  fn size_with(header: &TablesHeader) -> usize {
    match header.heap_sizes.contains(HeapSizes::WIDE_BLOB_HEAP) {
      true => 4,
      false => 2,
    }
  }
}
