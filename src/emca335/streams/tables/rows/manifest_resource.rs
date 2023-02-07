use super::row;
use crate::emca335::{coded_index::Implementation, streams::StringId};
use scroll::{Pread, SizeWith};

row! {
  pub struct ManifestResourceRow : 0x28 {
    offset: u32,
    flags: ManifestResourceAttributes,
    name: StringId,
    implementation: Implementation
  }
}

bitflags::bitflags! {
  #[derive(Pread, SizeWith)]
  pub struct ManifestResourceAttributes : u32 {
    /// The resource is exported from the assembly.
    const PUBLIC = 0x0001;
    /// The resource is private to the assembly.
    const PRIVATE = 0x0002;
  }
}
