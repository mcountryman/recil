//! ECMA-335 Metadata `#GUID` stream data.

use super::{
  tables::{HeapSizes, TablesHeader},
  StreamHeader,
};
use anyhow::Result;
use scroll::{
  ctx::{SizeWith, TryFromCtx},
  Pread, LE,
};

/// Contains the GUIDs in the `#GUID` stream.
#[derive(Default)]
pub struct Guids<'a>(&'a [u8]);

impl<'a> Guids<'a> {
  /// Creates an instance of [Guids] from the given [StreamHeader].
  ///
  /// # Note
  /// The stream header must be for the `#GUID` stream and isn't verified here in release builds.
  pub fn parse_from_header(header: StreamHeader<'a>) -> Result<Self> {
    debug_assert!(matches!(header.name.to_bytes(), b"#GUID"));

    Ok(Self(header.data()?))
  }

  /// Gets the guid at the given [GuidIndex].
  pub fn get(&self, index: GuidIndex) -> Result<[u8; 16]> {
    let offset = index.0;
    let mut guid = [0u8; 16];

    (0..15).for_each(|i| {
      if offset + i > self.0.len() {
        return;
      }

      guid[i] = self.0[offset + i];
    });

    Ok(guid)
  }
}

// An index into the [Guids] stream.
#[derive(Debug, Clone, Copy)]
pub struct GuidIndex(usize);

impl<'a> TryFromCtx<'a, TablesHeader> for GuidIndex {
  type Error = anyhow::Error;

  fn try_from_ctx(from: &'a [u8], header: TablesHeader) -> Result<(Self, usize)> {
    let offset = &mut 0;
    let index = match header.heap_sizes.contains(HeapSizes::WIDE_GUID_HEAP) {
      true => from.gread_with::<u32>(offset, LE)? as usize,
      false => from.gread_with::<u16>(offset, LE)? as usize,
    };

    Ok((Self(index), *offset))
  }
}

impl SizeWith<TablesHeader> for GuidIndex {
  fn size_with(header: &TablesHeader) -> usize {
    match header.heap_sizes.contains(HeapSizes::WIDE_GUID_HEAP) {
      true => 4,
      false => 2,
    }
  }
}
