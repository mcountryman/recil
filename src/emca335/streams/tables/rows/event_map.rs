use super::{row, type_def::TypeDefRowId, EventRowId};

row! {
  pub struct EventMapRow : 0x12 {
    parent: TypeDefRowId,
    event_list: EventRowId // EventRowIdList
  }
}
