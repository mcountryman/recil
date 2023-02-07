use scroll::{Pread, SizeWith};

bitflags::bitflags! {
  #[derive(Pread, SizeWith)]
  pub struct ElementType : u8 {
    const ELEMENT_TYPE_END =  0x00; //  Marks end of a list
    const ELEMENT_TYPE_VOID =  0x01; //
    const ELEMENT_TYPE_BOOLEAN =  0x02; //
    const ELEMENT_TYPE_CHAR =  0x03; //
    const ELEMENT_TYPE_I1 =  0x04; //
    const ELEMENT_TYPE_U1 =  0x05; //
    const ELEMENT_TYPE_I2 =  0x06; //
    const ELEMENT_TYPE_U2 =  0x07; //
    const ELEMENT_TYPE_I4 =  0x08; //
    const ELEMENT_TYPE_U4 =  0x09; //
    const ELEMENT_TYPE_I8 =  0x0a; //
    const ELEMENT_TYPE_U8 =  0x0b; //
    const ELEMENT_TYPE_R4 =  0x0c; //
    const ELEMENT_TYPE_R8 =  0x0d; //
    const ELEMENT_TYPE_STRING =  0x0e; //
    const ELEMENT_TYPE_PTR =  0x0f; //  Followed by type
    const ELEMENT_TYPE_BYREF =  0x10; //  Followed by type
    const ELEMENT_TYPE_VALUETYPE =  0x11; //  Followed by TypeDef or TypeRef token
    const ELEMENT_TYPE_CLASS =  0x12; //  Followed by TypeDef or TypeRef token
    const ELEMENT_TYPE_VAR =  0x13; //  Generic parameter in a generic type definition, represented as number (compressed unsigned integer)
    const ELEMENT_TYPE_ARRAY =  0x14; //  type rank boundsCount bound1 … loCount lo1 …
    const ELEMENT_TYPE_GENERICINST =  0x15; //  Generic type instantiation. Followed by type type-arg-count type-1 ... type-n
    const ELEMENT_TYPE_TYPEDBYREF =  0x16; //
    const ELEMENT_TYPE_I =  0x18; //  System.IntPtr
    const ELEMENT_TYPE_U =  0x19; //  System.UIntPtr
    const ELEMENT_TYPE_FNPTR =  0x1b; //  Followed by full method signature
    const ELEMENT_TYPE_OBJECT =  0x1c; //  System.Object
    const ELEMENT_TYPE_SZARRAY =  0x1d; //  Single-dim array with 0 lower bound
    const ELEMENT_TYPE_MVAR =  0x1e; //  Generic parameter in a generic method definition, represented as number (compressed unsigned integer)
    const ELEMENT_TYPE_CMOD_REQD =  0x1f; //  Required modifier : followed by a TypeDef or TypeRef token
    const ELEMENT_TYPE_CMOD_OPT =  0x20; //  Optional modifier : followed by a TypeDef or TypeRef token
    const ELEMENT_TYPE_INTERNAL =  0x21; //  Implemented within the CLI
    const ELEMENT_TYPE_MODIFIER =  0x40; //  Or’d with following element types
    const ELEMENT_TYPE_SENTINEL =  0x41; //  Sentinel for vararg method signature
    const ELEMENT_TYPE_PINNED =  0x45; //  Denotes a local variable that points at a pinned object
  }
}
