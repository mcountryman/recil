use super::{row, type_def::TypeDefRowId};

row! {
  /// Defines how the fields of a class or value type are laid out in memory.
  pub struct ClassLayoutRow : 0x0f {
    packing_size: u16,
    class_size: u32,
    parent: TypeDefRowId
  }
}
