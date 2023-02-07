use super::{row, type_def::TypeDefRowId, PropertyRowId};

row! {
  pub struct PropertyMapRow : 0x15 {
    parent: TypeDefRowId,
    property_list: PropertyRowId // PropertyRowIdList
  }
}
