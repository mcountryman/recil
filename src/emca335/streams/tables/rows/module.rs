use super::row;
use crate::emca335::{streams::guids::GuidId, streams::strings::StringId};

row! {
  pub struct ModuleRow : 0x00 {
    /// Reserved, shall be 0.
    generation: u16,
    name: StringId,
    /// The module identity.
    mvid: GuidId,
    enc_id: GuidId,
    enc_base_id: GuidId
  }
}
