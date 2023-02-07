use super::{row, FieldRowId, MethodDefRowId};
use crate::emca335::{coded_index::TypeDefOrRef, streams::strings::StringId};
use scroll::{Pread, SizeWith};

row! {
  pub struct TypeDefRow : 0x02 {
    flags: TypeAttributes,
    name: StringId,
    namespace: StringId,
    extends: TypeDefOrRef,
    field_list: FieldRowId, // FieldRowIdList,
    method_list: MethodDefRowId // MethodRowIdList
  }
}

bitflags::bitflags! {
  #[derive(Pread, SizeWith)]
  pub struct TypeAttributes : u32 {
    // Visibility attributes

    /// Use this mask to retrieve visibility information. These 3 bits contain one of the following values:
    const VISIBILITY_MASK = 0x00000007;
    /// Class has no public scope
    const NOT_PUBLIC = 0x00000000;
    /// Class has public scope
    const PUBLIC = 0x00000001;
    /// Class is nested with public visibility
    const NESTED_PUBLIC = 0x00000002;
    /// Class is nested with private visibility
    const NESTED_PRIVATE = 0x00000003;
    /// Class is nested with family visibility
    const NESTED_FAMILY = 0x00000004;
    /// Class is nested with assembly visibility
    const NESTED_ASSEMBLY = 0x00000005;
    /// Class is nested with family and assembly visibility
    const NESTED_FAM_AND_ASSEM = 0x00000006;
    /// Class is nested with family or assembly visibility
    const NESTED_FAM_OR_ASSEM = 0x00000007;

    // Class layout attributes

    /// Use this mask to retrieve class layout information. These 2 bits contain one of the following values:
    const LAYOUT_MASK = 0x00000018;
    /// Class fields are auto-laid out
    const AUTO_LAYOUT = 0x00000000;
    /// Class fields are laid out sequentially
    const SEQUENTIAL_LAYOUT = 0x00000008;
    /// Layout is supplied explicitly
    const EXPLICIT_LAYOUT = 0x00000010;

    // Class semantics attributes

    /// Use this mask to retrive class semantics information. This bit contains one of the following values:
    const CLASS_SEMANTICS_MASK = 0x00000020;
    /// Type is a class
    const CLASS = 0x00000000;
    /// Type is an interface
    const INTERFACE = 0x00000020;

    /// Class is abstract
    const ABSTRACT = 0x00000080;
    /// Class cannot be extended
    const SEALED = 0x00000100;
    /// Class name is special
    const SPECIAL_NAME = 0x00000400;

    // Implementation Attributes

    /// Class/Interface is imported
    const IMPORT = 0x00001000;
    /// Reserved (Class is serializable)
    const SERIALIZABLE = 0x00002000;

    /// Use this mask to retrieve string information for native interop. These 2 bits contain one of the following values:
    const STRING_FORMAT_MASK = 0x00030000;
    /// LPSTR is interpreted as ANSI
    const ANSI_CLASS = 0x00000000;
    /// LPSTR is interpreted as Unicode
    const UNICODE_CLASS = 0x00010000;
    /// LPSTR is interpreted automatically
    const AUTO_CLASS = 0x00020000;
    /// A non-standard encoding specified by
    const CUSTOM_FORMAT_CLASS = 0x00030000;

    // CustomStringFormatMask

    /// Use this mask to retrieve non-standard encoding information for native interop. The meaning of the values of these 2 bits is unspecified.
    const CUSTOM_STRING_FORMAT_MASK = 0x00C00000;

    // Class Initialization Attributes

    /// Initialize the class before first static field access
    const BEFORE_FIELD_INIT = 0x00100000;

    // Additional Flags

    /// CLI provides 'special' behavior, depending upon the name of the Type
    const RT_SPECIAL_NAME = 0x00000800;
    /// Type has security associate with it
    const HAS_SECURITY = 0x00040000;
    /// This ExportedType entry is a type forwarder
    const IS_TYPE_FORWARDER = 0x00200000;
  }
}
