use super::{row, type_def::TypeDefRowId};
use crate::emca335::coded_index::MethodDefOrRef;

row! {
  pub struct MethodImplRow : 0x19 {
    class: TypeDefRowId,
    method_body: MethodDefOrRef,
    method_declaration: MethodDefOrRef
  }
}
