use super::{row, type_def::TypeDefRowId};
use crate::emca335::coded_index::TypeDefOrRef;

row! {
  /// Contains interface implementation information.
  pub struct InterfaceImplRow : 0x09 {
    class: TypeDefRowId,
    interface: TypeDefOrRef
  }
}
