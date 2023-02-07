use super::row;
use crate::emca335::{streams::blobs::BlobId, streams::strings::StringId};
use scroll::{Pread, SizeWith};

row! {
  pub struct FieldRow : 0x04 {
    flags: FieldAttributes,
    name: StringId,
    signature: BlobId
  }
}

bitflags::bitflags! {
  #[derive(Pread, SizeWith)]
  pub struct FieldAttributes : u16 {
    // These 3 bits contain one of the following values:
    const FIELD_ACCESS_MASK = 0x0007;
    // Member not referenceable
    const COMPILER_CONTROLLED = 0x0000;
    // Accessible only by the parent type
    const PRIVATE = 0x0001;
    // Accessible by sub-types only in this Assembly
    const FAM_AND_ASSEM = 0x0002;
    // Accessibly by anyone in the Assembly
    const ASSEMBLY = 0x0003;
    // Accessible only by type and sub-types
    const FAMILY = 0x0004;
    // Accessibly by sub-types anywhere, plus anyone in assembly
    const FAM_OR_ASSEM = 0x0005;
    // Accessibly by anyone who has visibility to this scope field contract attributes
    const PUBLIC = 0x0006;
    // Defined on type, else per instance
    const STATIC = 0x0010;
    // Field can only be initialized, not written to after init
    const INIT_ONLY = 0x0020;
    // Value is compile time constant
    const LITERAL = 0x0040;
    // Reserved (to indicate this field should not be serialized when type is remoted)
    const NOT_SERIALIZED = 0x0080;

    // Field is special
    const SPECIAL_NAME = 0x0200;
    // Implementation is forwarded through PInvoke.
    const P_INVOKE_IMPL = 0x2000;

    // CLI provides 'special' behavior, depending upon the name of the field
    const RT_SPECIAL_NAME = 0x0400;
    // Field has marshalling information
    const HAS_FIELD_MARSHAL = 0x1000;
    // Field has default
    const HAS_DEFAULT = 0x8000;
    // Field has RVA
    const HAS_FIELD_RVA = 0x0100;
  }

}
