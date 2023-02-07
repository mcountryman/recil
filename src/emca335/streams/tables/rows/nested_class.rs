use super::{row, type_def::TypeDefRowId};

row! {
  pub struct NestedClassRow : 0x29 {
    nested_class: TypeDefRowId,
    enclosing_class: TypeDefRowId
  }
}
