use super::{assembly::AssemblyFlags, row};
use crate::emca335::{streams::blobs::BlobId, streams::strings::StringId};

row! {
  pub struct AssemblyRefRow : 0x23 {
    major_version: u16,
    minor_version: u16,
    build_number: u16,
    revision_number: u16,
    flags: AssemblyFlags,
    public_key_or_token: BlobId,
    name: StringId,
    culture: StringId,
    hash_value: BlobId
  }
}
