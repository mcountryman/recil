use super::{method_def::MethodDefRowId, row};
use crate::emca335::coded_index::HasSemantics;
use scroll::{Pread, SizeWith};

row! {
  pub struct MethodSemanticsRow : 0x18 {
    semantics: MethodSemanticsAttributes,
    method: MethodDefRowId,
    association: HasSemantics
  }
}

bitflags::bitflags! {
  #[derive(Pread, SizeWith)]
  pub struct MethodSemanticsAttributes : u16 {
    /// Setter for property
    const SETTER = 0x0001;
    /// Getter for property
    const GETTER = 0x0002;
    /// Other method for property or event
    const OTHER = 0x0004;
    /// AddOn method for event. This refers to the required add_ method for events.
    const ADD_ON = 0x0008;
    /// RemoveOn method for event. . This refers to the required remove_ method for events.
    const REMOVE_ON = 0x0010;
    /// Fire method for event. This refers to the optional raise_ method for events.
    const FIRE = 0x0020;
  }
}
