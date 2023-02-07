use super::{
  row,
  type_def::{TypeAttributes, TypeDefRowId},
};
use crate::emca335::{coded_index::Implementation, streams::strings::StringId};

row! {
  pub struct ExportedTypeRow : 0x27 {
    flags: TypeAttributes,
    type_def_id: TypeDefRowId,
    type_name: StringId,
    type_namespace: StringId,
    implementation: Implementation
  }
}
