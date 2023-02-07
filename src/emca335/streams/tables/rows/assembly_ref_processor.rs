use super::{assembly_ref::AssemblyRefRowId, row};

row! {
  /// This row should not be emitted into any PE file. However, if present in a PE file, it shall be
  /// treated as if all it's fields were zero.  It shall be ignored by the CLI.
  pub struct AssemblyRefProcessorRow : 0x24 {
    processor: u32,
    assembly_ref: AssemblyRefRowId
  }
}
