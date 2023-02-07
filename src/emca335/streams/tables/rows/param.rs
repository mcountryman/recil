use scroll::{Pread, SizeWith};

use crate::emca335::streams::strings::StringId;

use super::row;

row! {
  pub struct ParamRow : 0x08 {
    flags: ParamAttributes,
    sequence: u16,
    name: StringId
  }
}

bitflags::bitflags! {
  #[derive(Pread, SizeWith)]
  pub struct ParamAttributes : u16 {
    /// Param is [In]
    const IN = 0x0001;
    /// Param is [out]
    const OUT = 0x0002;
    /// Param is optional
    const OPTIONAL = 0x0010;
    /// Param has default value
    const HAS_DEFAULT = 0x1000;
    /// Param has FieldMarshal
    const HAS_FIELD_MARSHAL = 0x2000;
    /// Reserved: shall be zero in a conforming implementation
    const UNUSED = 0xcfe0;
  }
}
