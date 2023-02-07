use super::{assembly_ref::AssemblyRefRowId, row};

row! {
  /// This row should not be emitted into any PE file. However, if present in a PE file, it shall be
  /// treated as if all it's fields were zero.  It shall be ignored by the CLI.
  pub struct AssemblyRefOsRow : 0x25 {
    os_platform_id: u32,
    os_major_version: u32,
    os_minor_version: u32,
    assembly_ref: AssemblyRefRowId
  }
}
