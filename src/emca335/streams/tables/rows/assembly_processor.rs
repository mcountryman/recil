use super::row;

row! {
  /// This row should not be emitted into any PE file. However, if present in a PE file, it shall be
  /// treated as if all it's fields were zero.  It shall be ignored by the CLI.
  pub struct AssemblyProcessorRow : 0x21 {
    processor: u32
  }
}
