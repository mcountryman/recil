use super::row;
use crate::emca335::{streams::blobs::BlobId, streams::strings::StringId};
use scroll::{Pread, SizeWith};

row! {
  pub struct PropertyRow : 0x17 {
    flags: PropertyAttributes,
    name: StringId,
    signature: BlobId
  }
}

bitflags::bitflags! {
  #[derive(Pread, SizeWith)]
  pub struct PropertyAttributes : u16 {
    /// Property is special
    const SPECIAL_NAME = 0x0200;
    /// Runtime(metadata internal APIs) should check name encoding
    const RT_SPECIAL_NAME = 0x0400;
    /// Property has default
    const HAS_DEFAULT = 0x1000;
    /// Reserved: shall be zero in a conforming implementation
    const UNUSED = 0xe9ff;
  }
}
