use super::{row, ParamRowId};
use crate::emca335::{streams::blobs::BlobId, streams::strings::StringId};
use scroll::{Pread, SizeWith};

row! {
  pub struct MethodDefRow : 0x06 {
    rva: u32,
    impl_flags: MethodImplAttributes,
    flags: MethodAttributes,
    name: StringId,
    signature: BlobId,
    param_list: ParamRowId // ParamRowIdList
  }
}

bitflags::bitflags! {
  #[derive(Pread, SizeWith)]
  pub struct MethodImplAttributes : u16 {
    /// These 3 bits contain one of the following values:
    const MEMBER_ACCESS_MASK = 0x0007;
    /// Member not referenceable
    const COMPILER_CONTROLLED = 0x0000;
    /// Accessible only by the parent type
    const PRIVATE = 0x0001;
    /// Accessible by sub-types only in this Assembly
    const FAM_AND_ASSEM = 0x0002;
    /// Accessibly by anyone in the Assembly
    const ASSEM = 0x0003;
    /// Accessible only by type and sub-types
    const FAMILY = 0x0004;
    /// Accessibly by sub-types anywhere, plus anyone in assembly
    const FAM_OR_ASSEM = 0x0005;
    /// Accessibly by anyone who has visibility to this scope
    const PUBLIC = 0x0006;
    /// Defined on type, else per instance
    const STATIC = 0x0010;
    /// Method cannot be overridden
    const FINAL = 0x0020;
    /// Method is virtual
    const VIRTUAL = 0x0040;
    /// Method hides by name+sig, else just by name
    const HIDE_BY_SIG = 0x0080;
    /// Use this mask to retrieve vtable attributes. This bit contains one of the following values:
    const VTABLE_LAYOUT_MASK = 0x0100;
    /// Method reuses existing slot in vtable
    const REUSE_SLOT = 0x0000;
    /// Method always gets a new slot in the vtable
    const NEW_SLOT = 0x0100;
    /// Method can only be overriden if also accessible
    const STRICT = 0x0200;
    /// Method does not provide an implementation
    const ABSTRACT = 0x0400;
    /// Method is special
    const SPECIAL_NAME = 0x0800;

    // Interop attributes

    /// Implementation is forwarded through PInvoke
    const P_INVOKE_IMPL = 0x2000;
    /// Reserved: shall be zero for conforming implementations
    const UNMANAGED_EXPORT = 0x0008;

    // Additional flags

    /// CLI provides 'special' behavior, depending upon the name of the method
    const RT_SPECIAL_NAME = 0x1000;
    /// Method has security associate with it
    const HAS_SECURITY = 0x4000;
    /// Method calls another method containing security code.
    const REQUIRE_SEC_OBJECT = 0x8000;
  }
}

bitflags::bitflags! {
  #[derive(Pread, SizeWith)]
  pub struct MethodAttributes : u16 {
    /// These 3 bits contain one of the following values:
    const MEMBER_ACCESS_MASK = 0x0007;
    /// Member not referenceable
    const COMPILER_CONTROLLED = 0x0000;
    /// Accessible only by the parent type
    const PRIVATE = 0x0001;
    /// Accessible by sub-types only in this Assembly
    const FAM_AND_ASSEM = 0x0002;
    /// Accessibly by anyone in the Assembly
    const ASSEM = 0x0003;
    /// Accessible only by type and sub-types
    const FAMILY = 0x0004;
    /// Accessibly by sub-types anywhere, plus anyone in assembly
    const FAM_OR_ASSEM = 0x0005;
    /// Accessibly by anyone who has visibility to this scope
    const PUBLIC = 0x0006;
    /// Defined on type, else per instance
    const STATIC = 0x0010;
    /// Method cannot be overridden
    const FINAL = 0x0020;
    /// Method is virtual
    const VIRTUAL = 0x0040;
    /// Method hides by name+sig, else just by name
    const HIDE_BY_SIG = 0x0080;
    /// Use this mask to retrieve vtable attributes. This bit contains one of the following values:
    const VTABLE_LAYOUT_MASK = 0x0100;
    /// Method reuses existing slot in vtable
    const REUSE_SLOT = 0x0000;
    /// Method always gets a new slot in the vtable
    const NEW_SLOT = 0x0100;
    /// Method can only be overriden if also accessible
    const STRICT = 0x0200;
    /// Method does not provide an implementation
    const ABSTRACT = 0x0400;
    /// Method is special
    const SPECIAL_NAME = 0x0800;

    // Interop attributes

    /// Implementation is forwarded through PInvoke
    const P_INVOKE_IMPL = 0x2000;
    /// Reserved: shall be zero for conforming implementations
    const UNMANAGED_EXPORT = 0x0008;

    // Additional flags

    /// CLI provides 'special' behavior, depending upon the name of the method
    const RT_SPECIAL_NAME = 0x1000;
    /// Method has security associate with it
    const HAS_SECURITY = 0x4000;
    /// Method calls another method containing security code
    const REQUIRE_SEC_OBJECT = 0x8000;
  }
}
