use super::row;
use crate::emca335::{coded_index::ResolutionScope, streams::strings::StringId};

row! {
  pub struct TypeRefRow : 0x01 {
    resolution_scope: ResolutionScope,
    name: StringId,
    namespace: StringId
  }
}
