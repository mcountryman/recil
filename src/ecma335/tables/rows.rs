use super::{flags::*, index::*, TablesHeader};
use crate::ecma335::{blobs::BlobIndex, guids::GuidIndex, strings::StringIndex};
use anyhow::{Context, Result};
use scroll::{ctx::SizeWith, Endian, Pread};

/// A row in a metadata table.
pub trait Row<'a>: 'a + Sized + SizeWith<TablesHeader> {
  /// The id of the table the row belongs to.
  const ID: usize;
  /// The index type of the row.
  type Index: RowIndex + Copy;

  /// Parses the row from the given buffer containing the table data at the given row index.
  fn parse(table_buf: &'a [u8], index: Self::Index, header: &TablesHeader) -> Result<Self>;
}

/// Defines a metadata table row as well as it's index type.
macro_rules! row {
  (
    $(#[$attr:meta])*
    pub struct $name:ident, $index:ident : $id:literal {
      $(
        $(#[$field_attr:meta])*
        $field:ident: $ty:ty
      ),* $(,)?
    }
  ) => {
    $(#[$attr])*
    #[derive(Debug)]
    pub struct $name {
      $(
        $(#[$field_attr])*
        pub $field: $ty
      ),*
    }

    impl<'a> Row<'a> for $name {
      const ID: usize = $id;
      type Index = $index;

      fn parse(table_buf: &'a [u8], index: Self::Index, header: &TablesHeader) -> Result<Self> {
        let row = index.row as usize;
        let row_size = Self::size_with(header);
        let offset = &mut (row * row_size);

        Ok((
          Self {
            $(
              $field: table_buf
                .gread_with(offset, header.into())
                .with_context(|| format!("`{}.{}`", stringify!($name), stringify!($field)))?,
            )*
          }
        ))
      }
    }

    impl SizeWith<TablesHeader> for $name {
      fn size_with(ctx: &TablesHeader) -> usize {
        let size = 0;

        $(
          let size = size + <$ty>::size_with(&ctx.into());
        )*

        size
      }
    }
  };
}

row! {
  pub struct ModuleRow, ModuleRowId : 0x00 {
    /// Reserved, shall be 0.
    generation: u16,
    name: StringIndex,
    /// The module identity.
    mvid: GuidIndex,
    enc_id: GuidIndex,
    enc_base_id: GuidIndex
  }
}

row! {
  pub struct TypeRefRow, TypeRefRowId : 0x01 {
    resolution_scope: ResolutionScope,
    name: StringIndex,
    namespace: StringIndex
  }
}

row! {
  pub struct TypeDefRow, TypeDefRowId : 0x02 {
    flags: TypeAttributes,
    name: StringIndex,
    namespace: StringIndex,
    extends: TypeDefOrRef,
    field_list: FieldRowId, // FieldRowIdList,
    method_list: MethodDefRowId // MethodRowIdList
  }
}

row! {
  pub struct FieldRow, FieldRowId : 0x04 {
    flags: FieldAttributes,
    name: StringIndex,
    signature: BlobIndex
  }
}

row! {
  pub struct MethodDefRow, MethodDefRowId : 0x06 {
    rva: u32,
    impl_flags: MethodImplAttributes,
    flags: MethodAttributes,
    name: StringIndex,
    signature: BlobIndex,
    param_list: ParamRowId // ParamRowIdList
  }
}

row! {
  pub struct AssemblyRow, AssemblyRowId : 0x20 {
    hash_alg: AssemblyHashAlgorithm,
    major_version: u16,
    minor_version: u16,
    build_number: u16,
    revision_number: u16,
    flags: AssemblyFlags,
    public_key: BlobIndex,
    name: StringIndex,
    culture: StringIndex
  }
}

row! {
  /// This row should not be emitted into any PE file. However, if present in a PE file, it shall be
  /// treated as if all it's fields were zero.  It shall be ignored by the CLI.
  pub struct AssemblyOsRow, AssemblyOsRowId : 0x22 {
    os_platform_id: u32,
    os_major_version: u32,
    os_minor_version: u32
  }
}

row! {
  /// This row should not be emitted into any PE file. However, if present in a PE file, it shall be
  /// treated as if all it's fields were zero.  It shall be ignored by the CLI.
  pub struct AssemblyProcessorRow, AssemblyProcessorRowId : 0x21 {
    processor: u32
  }
}

row! {
  pub struct AssemblyRefRow, AssemblyRefRowId : 0x23 {
    major_version: u16,
    minor_version: u16,
    build_number: u16,
    revision_number: u16,
    flags: AssemblyFlags,
    public_key_or_token: BlobIndex,
    name: StringIndex,
    culture: StringIndex,
    hash_value: BlobIndex
  }
}

row! {
  /// This row should not be emitted into any PE file. However, if present in a PE file, it shall be
  /// treated as if all it's fields were zero.  It shall be ignored by the CLI.
  pub struct AssemblyRefOsRow, AssemblyRefOsRowId : 0x25 {
    os_platform_id: u32,
    os_major_version: u32,
    os_minor_version: u32,
    assembly_ref: AssemblyRefRowId
  }
}

row! {
  /// This row should not be emitted into any PE file. However, if present in a PE file, it shall be
  /// treated as if all it's fields were zero.  It shall be ignored by the CLI.
  pub struct AssemblyRefProcessorRow, AssemblyRefProcessorRowId : 0x24 {
    processor: u32,
    assembly_ref: AssemblyRefRowId
  }
}

row! {
  /// Defines how the fields of a class or value type are laid out in memory.
  pub struct ClassLayoutRow, ClassLayoutRowId : 0x0f {
    packing_size: u16,
    class_size: u32,
    parent: TypeDefRowId
  }
}

row! {
  pub struct ConstantRow, ConstantRowId : 0x0B {
    kind: ElementType,
    _padding: u8,
    parent: HasConstant,
    value: BlobIndex
  }
}

row! {
  pub struct CustomAttributeRow, CustomAttributeRowId : 0x0C {
    parent: HasCustomAttribute,
    attribute_type: CustomAttributeType,
    value: BlobIndex
  }
}

row! {
  pub struct DeclSecurityRow, DeclSecurityRowId : 0x0e {
    action: u16,
    parent: HasDeclSecurity,
    permission_set: BlobIndex
  }
}

row! {
  pub struct EventRow, EventRowId : 0x14 {
    flags: EventAttributes,
    name: StringIndex,
    event_type: TypeDefOrRef
  }
}

row! {
  pub struct EventMapRow, EventMapRowId : 0x12 {
    parent: TypeDefRowId,
    event_list: EventRowId // EventRowIdList
  }
}

row! {
  pub struct ExportedTypeRow, ExportedTypeRowId : 0x27 {
    flags: TypeAttributes,
    type_def_id: TypeDefRowId,
    type_name: StringIndex,
    type_namespace: StringIndex,
    implementation: Implementation
  }
}

row! {
  pub struct FieldLayoutRow, FieldLayoutRowId : 0x10 {
     offset: u32,
     field: FieldRowId
  }
}

row! {
  pub struct FieldMarshalRow, FieldMarshalRowId : 0x0d {
    parent: HasFieldMarshal,
    native_type: BlobIndex
  }
}

row! {
  pub struct FieldRvaRow, FieldRvaRowId : 0x1d {
    rva: u32,
    field: FieldRowId
  }
}

row! {
  pub struct FileRow, FileRowId : 0x26 {
    flags: FileAttributes,
    name: StringIndex,
    hash_value: BlobIndex
  }
}

row! {
  pub struct GenericParamRow, GenericParamRowId : 0x2a {
    number: u16,
    flags: GenericParamAttributes,
    owner: TypeOrMethodDef,
    name: StringIndex
  }
}

row! {
  pub struct GenericParamConstraintRow, GenericParamConstraintRowId : 0x2c {
    owner: GenericParamRowId,
    constraint: TypeDefOrRef
  }
}

row! {
  /// Holds information about un-managed methods that can be reached from managed code, using
  /// PInvoke dispatch.
  pub struct ImplMapRow, ImplMapRowId : 0x1c {
    mapping_flags: PInvokeAttributes,
    member_forwarded: MemberForwarded,
    import_name: StringIndex,
    import_scope: ModuleRefRowId
  }
}

row! {
  /// Contains interface implementation information.
  pub struct InterfaceImplRow, InterfaceImplRowId : 0x09 {
    class: TypeDefRowId,
    interface: TypeDefOrRef
  }
}

row! {
  pub struct ManifestResourceRow, ManifestResourceRowId : 0x28 {
    offset: u32,
    flags: ManifestResourceAttributes,
    name: StringIndex,
    implementation: Implementation
  }
}

row! {
  /// Contains a reference to a member of a type.
  pub struct MemberRefRow, MemberRefRowId : 0x0a {
    class: MemberRefParent,
    name: StringIndex,
    signature: BlobIndex
  }
}

row! {
  pub struct MethodImplRow, MethodImplRowId : 0x19 {
    class: TypeDefRowId,
    method_body: MethodDefOrRef,
    method_declaration: MethodDefOrRef
  }
}

row! {
  pub struct MethodSemanticsRow, MethodSemanticsRowId : 0x18 {
    semantics: MethodSemanticsAttributes,
    method: MethodDefRowId,
    association: HasSemantics
  }
}

row! {
  pub struct MethodSpecRow, MethodSpecRowId : 0x2b {
    method: MethodDefOrRef,
    instantiation: BlobIndex
  }
}

row! {
  pub struct ModuleRefRow, ModuleRefRowId : 0x1a {
    name: StringIndex
  }
}

row! {
  pub struct NestedClassRow, NestedClassRowId : 0x29 {
    nested_class: TypeDefRowId,
    enclosing_class: TypeDefRowId
  }
}

row! {
  pub struct ParamRow, ParamRowId : 0x08 {
    flags: ParamAttributes,
    sequence: u16,
    name: StringIndex
  }
}

row! {
  pub struct PropertyRow, PropertyRowId : 0x17 {
    flags: PropertyAttributes,
    name: StringIndex,
    signature: BlobIndex
  }
}

row! {
  pub struct PropertyMapRow, PropertyMapRowId : 0x15 {
    parent: TypeDefRowId,
    property_list: PropertyRowId // PropertyRowIdList
  }
}

row! {
  pub struct StandAloneSigRow, StandAloneSigRowId : 0x11 {
    signature: BlobIndex
  }
}

row! {
  pub struct TypeSpecRow, TypeSpecRowId : 0x1b {
    signature: BlobIndex
  }
}

impl From<&TablesHeader> for () {
  fn from(_: &TablesHeader) -> Self {}
}

impl From<&TablesHeader> for TablesHeader {
  fn from(header: &TablesHeader) -> Self {
    *header
  }
}

impl From<&TablesHeader> for Endian {
  fn from(_: &TablesHeader) -> Self {
    Endian::Little
  }
}
