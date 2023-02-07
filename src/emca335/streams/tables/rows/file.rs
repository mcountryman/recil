use scroll::{Pread, SizeWith};

use super::row;
use crate::emca335::{streams::blobs::BlobId, streams::strings::StringId};

row! {
  pub struct FileRow : 0x26 {
    flags: FileAttributes,
    name: StringId,
    hash_value: BlobId
  }
}

bitflags::bitflags! {
  #[derive(Pread, SizeWith)]
  pub struct FileAttributes: u32 {
    const CONTAINS_METADATA = 0x0000;
    const CONTAINS_NO_METADATA = 0x0001;
  }
}
