use scroll::{Pread, SizeWith};

use crate::emca335::{coded_index::TypeDefOrRef, streams::strings::StringId};

use super::row;

row! {
  pub struct EventRow : 0x14 {
    flags: EventAttributes,
    name: StringId,
    event_type: TypeDefOrRef
  }
}

bitflags::bitflags! {
  #[derive(Pread, SizeWith)]
  pub struct EventAttributes : u16 {
    /// The event is special.
    const SPECIAL_NAME = 0x0200;
    /// CLI provides 'special' behavior, depending upon the name of the event.
    const RTSPECIAL_NAME = 0x0400;
  }
}
