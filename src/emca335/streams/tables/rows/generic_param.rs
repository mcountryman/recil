use super::row;
use crate::emca335::{coded_index::TypeOrMethodDef, streams::strings::StringId};
use scroll::{Pread, SizeWith};

row! {
  pub struct GenericParamRow : 0x2a {
    number: u16,
    flags: GenericParamAttributes,
    owner: TypeOrMethodDef,
    name: StringId
  }
}

bitflags::bitflags! {
  #[derive(Pread, SizeWith)]
  pub struct GenericParamAttributes : u16 {
  // These 2 bits contain one of the following values:
    const VARIANCE_MASK = 0x0003;
    // The generic parameter is non-variant and has no special constraints
    const NONE = 0x0000;
    // The generic parameter is covariant
    const COVARIANT = 0x0001;
    // The generic parameter is contravariant
    const CONTRAVARIANT = 0x0002;
    // These 3 bits contain one of the following values:
    const SPECIAL_CONSTRAINT_MASK = 0x001C;
    // The GENERIC parameter has the class special constraint
    const REFERENCE_TYPE_CONSTRAINT = 0x0004;
    // The generic parameter has the valuetype special constraint
    const NOT_NULLABLE_VALUE_TYPE_CONSTRAINT = 0x0008;
    // The generic parameter has the .ctor special constraint
    const DEFAULT_CONSTRUCTOR_CONSTRAINT = 0x0010;
  }
}
