//! `#~` stream data.

pub mod rows;
pub use rows::*;

use crate::{
  emca335::header::{StreamHeader, TablesStreamHeader},
  error::Error,
};
use scroll::Pread;

/// Contains the metadata tables representing the types, methods, fields, etc. in the assembly.
#[derive(Debug, Default)]
pub struct Tables<'a> {
  header: TablesStreamHeader,

  assembly: AssemblyRowTable<'a>,
  assembly_os: AssemblyOsRowTable<'a>,
  assembly_processor: AssemblyProcessorRowTable<'a>,
  assembly_ref: AssemblyRefRowTable<'a>,
  assembly_ref_os: AssemblyRefOsRowTable<'a>,
  assembly_ref_processor: AssemblyRefProcessorRowTable<'a>,
  class_layout: ClassLayoutRowTable<'a>,
  constant: ConstantRowTable<'a>,
  custom_attribute: CustomAttributeRowTable<'a>,
  decl_security: DeclSecurityRowTable<'a>,
  event: EventRowTable<'a>,
  event_map: EventMapRowTable<'a>,
  exported_type: ExportedTypeRowTable<'a>,
  field: FieldRowTable<'a>,
  field_layout: FieldLayoutRowTable<'a>,
  field_marshal: FieldMarshalRowTable<'a>,
  field_rva: FieldRvaRowTable<'a>,
  file: FileRowTable<'a>,
  generic_param: GenericParamRowTable<'a>,
  generic_param_constraint: GenericParamConstraintRowTable<'a>,
  impl_map: ImplMapRowTable<'a>,
  interface_impl: InterfaceImplRowTable<'a>,
  manifest_resource: ManifestResourceRowTable<'a>,
  member_ref: MemberRefRowTable<'a>,
  method_def: MethodDefRowTable<'a>,
  method_impl: MethodImplRowTable<'a>,
  method_semantics: MethodSemanticsRowTable<'a>,
  method_spec: MethodSpecRowTable<'a>,
  module: ModuleRowTable<'a>,
  module_ref: ModuleRefRowTable<'a>,
  nested_class: NestedClassRowTable<'a>,
  param: ParamRowTable<'a>,
  property: PropertyRowTable<'a>,
  property_map: PropertyMapRowTable<'a>,
  stand_alone_sig: StandAloneSigRowTable<'a>,
  type_def: TypeDefRowTable<'a>,
  type_ref: TypeRefRowTable<'a>,
  type_spec: TypeSpecRowTable<'a>,
}

impl<'a> Tables<'a> {
  pub fn parse(from: &'a [u8], header: &StreamHeader<'a>) -> Result<Self, Error> {
    let buf = header.data(from)?;
    let offset = &mut 0usize;
    let header = buf.gread::<TablesStreamHeader>(offset)?;

    let mut tables = Self {
      header,
      ..Default::default()
    };

    for id in 0..64 {
      if !header.is_valid(id) {
        continue;
      }

      match id {
        ASSEMBLY_ROW_ID => tables.assembly = buf.gread_with(offset, &header)?,
        ASSEMBLY_OS_ROW_ID => tables.assembly_os = buf.gread_with(offset, &header)?,
        ASSEMBLY_PROCESSOR_ROW_ID => tables.assembly_processor = buf.gread_with(offset, &header)?,
        ASSEMBLY_REF_ROW_ID => tables.assembly_ref = buf.gread_with(offset, &header)?,
        ASSEMBLY_REF_OS_ROW_ID => tables.assembly_ref_os = buf.gread_with(offset, &header)?,
        ASSEMBLY_REF_PROCESSOR_ROW_ID => {
          tables.assembly_ref_processor = buf.gread_with(offset, &header)?
        }
        CLASS_LAYOUT_ROW_ID => tables.class_layout = buf.gread_with(offset, &header)?,
        CONSTANT_ROW_ID => tables.constant = buf.gread_with(offset, &header)?,
        CUSTOM_ATTRIBUTE_ROW_ID => tables.custom_attribute = buf.gread_with(offset, &header)?,
        DECL_SECURITY_ROW_ID => tables.decl_security = buf.gread_with(offset, &header)?,
        EVENT_ROW_ID => tables.event = buf.gread_with(offset, &header)?,
        EVENT_MAP_ROW_ID => tables.event_map = buf.gread_with(offset, &header)?,
        EXPORTED_TYPE_ROW_ID => tables.exported_type = buf.gread_with(offset, &header)?,
        FIELD_ROW_ID => tables.field = buf.gread_with(offset, &header)?,
        FIELD_LAYOUT_ROW_ID => tables.field_layout = buf.gread_with(offset, &header)?,
        FIELD_MARSHAL_ROW_ID => tables.field_marshal = buf.gread_with(offset, &header)?,
        FIELD_RVA_ROW_ID => tables.field_rva = buf.gread_with(offset, &header)?,
        FILE_ROW_ID => tables.file = buf.gread_with(offset, &header)?,
        GENERIC_PARAM_ROW_ID => tables.generic_param = buf.gread_with(offset, &header)?,
        GENERIC_PARAM_CONSTRAINT_ROW_ID => {
          tables.generic_param_constraint = buf.gread_with(offset, &header)?
        }
        IMPL_MAP_ROW_ID => tables.impl_map = buf.gread_with(offset, &header)?,
        INTERFACE_IMPL_ROW_ID => tables.interface_impl = buf.gread_with(offset, &header)?,
        MANIFEST_RESOURCE_ROW_ID => tables.manifest_resource = buf.gread_with(offset, &header)?,
        MEMBER_REF_ROW_ID => tables.member_ref = buf.gread_with(offset, &header)?,
        METHOD_DEF_ROW_ID => tables.method_def = buf.gread_with(offset, &header)?,
        METHOD_IMPL_ROW_ID => tables.method_impl = buf.gread_with(offset, &header)?,
        METHOD_SEMANTICS_ROW_ID => tables.method_semantics = buf.gread_with(offset, &header)?,
        METHOD_SPEC_ROW_ID => tables.method_spec = buf.gread_with(offset, &header)?,
        MODULE_ROW_ID => tables.module = buf.gread_with(offset, &header)?,
        MODULE_REF_ROW_ID => tables.module_ref = buf.gread_with(offset, &header)?,
        NESTED_CLASS_ROW_ID => tables.nested_class = buf.gread_with(offset, &header)?,
        PARAM_ROW_ID => tables.param = buf.gread_with(offset, &header)?,
        PROPERTY_ROW_ID => tables.property = buf.gread_with(offset, &header)?,
        PROPERTY_MAP_ROW_ID => tables.property_map = buf.gread_with(offset, &header)?,
        STAND_ALONE_SIG_ROW_ID => tables.stand_alone_sig = buf.gread_with(offset, &header)?,
        TYPE_DEF_ROW_ID => tables.type_def = buf.gread_with(offset, &header)?,
        TYPE_REF_ROW_ID => tables.type_ref = buf.gread_with(offset, &header)?,
        TYPE_SPEC_ROW_ID => tables.type_spec = buf.gread_with(offset, &header)?,
        _ => unreachable!(),
      }
    }

    Ok(tables)
  }

  /// Gets the [AssemblyRowTableReader].
  pub fn assemblies<'b>(&'b self) -> AssemblyRowTableReader<'a, 'b> {
    AssemblyRowTableReader::new(self.assembly, &self.header)
  }

  /// Gets the [AssemblyOsRowTableReader].
  pub fn assembly_oses<'b>(&'b self) -> AssemblyOsRowTableReader<'a, 'b> {
    AssemblyOsRowTableReader::new(self.assembly_os, &self.header)
  }

  /// Gets the [AssemblyProcessorRowTableReader].
  pub fn assembly_processors<'b>(&'b self) -> AssemblyProcessorRowTableReader<'a, 'b> {
    AssemblyProcessorRowTableReader::new(self.assembly_processor, &self.header)
  }

  /// Gets the [AssemblyRefRowTableReader].
  pub fn assembly_refs<'b>(&'b self) -> AssemblyRefRowTableReader<'a, 'b> {
    AssemblyRefRowTableReader::new(self.assembly_ref, &self.header)
  }

  /// Gets the [AssemblyRefOsRowTableReader].
  pub fn assembly_ref_oses<'b>(&'b self) -> AssemblyRefOsRowTableReader<'a, 'b> {
    AssemblyRefOsRowTableReader::new(self.assembly_ref_os, &self.header)
  }

  /// Gets the [AssemblyRefProcessorRowTableReader].
  pub fn assembly_ref_processors<'b>(&'b self) -> AssemblyRefProcessorRowTableReader<'a, 'b> {
    AssemblyRefProcessorRowTableReader::new(self.assembly_ref_processor, &self.header)
  }

  /// Gets the [ClassLayoutRowTableReader].
  pub fn class_layouts<'b>(&'b self) -> ClassLayoutRowTableReader<'a, 'b> {
    ClassLayoutRowTableReader::new(self.class_layout, &self.header)
  }

  /// Gets the [ConstantRowTableReader].
  pub fn constants<'b>(&'b self) -> ConstantRowTableReader<'a, 'b> {
    ConstantRowTableReader::new(self.constant, &self.header)
  }

  /// Gets the [CustomAttributeRowTableReader].
  pub fn custom_attributes<'b>(&'b self) -> CustomAttributeRowTableReader<'a, 'b> {
    CustomAttributeRowTableReader::new(self.custom_attribute, &self.header)
  }

  /// Gets the [DeclSecurityRowTableReader].
  pub fn decl_securities<'b>(&'b self) -> DeclSecurityRowTableReader<'a, 'b> {
    DeclSecurityRowTableReader::new(self.decl_security, &self.header)
  }

  /// Gets the [EventRowTableReader].
  pub fn events<'b>(&'b self) -> EventRowTableReader<'a, 'b> {
    EventRowTableReader::new(self.event, &self.header)
  }

  /// Gets the [EventMapRowTableReader].
  pub fn event_maps<'b>(&'b self) -> EventMapRowTableReader<'a, 'b> {
    EventMapRowTableReader::new(self.event_map, &self.header)
  }

  /// Gets the [ExportedTypeRowTableReader].
  pub fn exported_types<'b>(&'b self) -> ExportedTypeRowTableReader<'a, 'b> {
    ExportedTypeRowTableReader::new(self.exported_type, &self.header)
  }

  /// Gets the [FieldRowTableReader].
  pub fn fields<'b>(&'b self) -> FieldRowTableReader<'a, 'b> {
    FieldRowTableReader::new(self.field, &self.header)
  }

  /// Gets the [FieldLayoutRowTableReader].
  pub fn field_layouts<'b>(&'b self) -> FieldLayoutRowTableReader<'a, 'b> {
    FieldLayoutRowTableReader::new(self.field_layout, &self.header)
  }

  /// Gets the [FieldMarshalRowTableReader].
  pub fn field_marshals<'b>(&'b self) -> FieldMarshalRowTableReader<'a, 'b> {
    FieldMarshalRowTableReader::new(self.field_marshal, &self.header)
  }

  /// Gets the [FieldRvaRowTableReader].
  pub fn field_rvas<'b>(&'b self) -> FieldRvaRowTableReader<'a, 'b> {
    FieldRvaRowTableReader::new(self.field_rva, &self.header)
  }

  /// Gets the [FileRowTableReader].
  pub fn files<'b>(&'b self) -> FileRowTableReader<'a, 'b> {
    FileRowTableReader::new(self.file, &self.header)
  }

  /// Gets the [GenericParamRowTableReader].
  pub fn generic_params<'b>(&'b self) -> GenericParamRowTableReader<'a, 'b> {
    GenericParamRowTableReader::new(self.generic_param, &self.header)
  }

  /// Gets the [GenericParamConstraintRowTableReader].
  pub fn generic_param_constraints<'b>(&'b self) -> GenericParamConstraintRowTableReader<'a, 'b> {
    GenericParamConstraintRowTableReader::new(self.generic_param_constraint, &self.header)
  }

  /// Gets the [ImplMapRowTableReader].
  pub fn impl_maps<'b>(&'b self) -> ImplMapRowTableReader<'a, 'b> {
    ImplMapRowTableReader::new(self.impl_map, &self.header)
  }

  /// Gets the [InterfaceImplRowTableReader].
  pub fn interface_impls<'b>(&'b self) -> InterfaceImplRowTableReader<'a, 'b> {
    InterfaceImplRowTableReader::new(self.interface_impl, &self.header)
  }

  /// Gets the [ManifestResourceRowTableReader].
  pub fn manifest_resources<'b>(&'b self) -> ManifestResourceRowTableReader<'a, 'b> {
    ManifestResourceRowTableReader::new(self.manifest_resource, &self.header)
  }

  /// Gets the [MemberRefRowTableReader].
  pub fn member_refs<'b>(&'b self) -> MemberRefRowTableReader<'a, 'b> {
    MemberRefRowTableReader::new(self.member_ref, &self.header)
  }

  /// Gets the [MethodDefRowTableReader].
  pub fn method_defs<'b>(&'b self) -> MethodDefRowTableReader<'a, 'b> {
    MethodDefRowTableReader::new(self.method_def, &self.header)
  }

  /// Gets the [MethodImplRowTableReader].
  pub fn method_impls<'b>(&'b self) -> MethodImplRowTableReader<'a, 'b> {
    MethodImplRowTableReader::new(self.method_impl, &self.header)
  }

  /// Gets the [MethodSemanticsRowTableReader].
  pub fn method_semantics<'b>(&'b self) -> MethodSemanticsRowTableReader<'a, 'b> {
    MethodSemanticsRowTableReader::new(self.method_semantics, &self.header)
  }

  /// Gets the [MethodSpecRowTableReader].
  pub fn method_specs<'b>(&'b self) -> MethodSpecRowTableReader<'a, 'b> {
    MethodSpecRowTableReader::new(self.method_spec, &self.header)
  }

  /// Gets the [ModuleRowTableReader].
  pub fn modules<'b>(&'b self) -> ModuleRowTableReader<'a, 'b> {
    ModuleRowTableReader::new(self.module, &self.header)
  }

  /// Gets the [ModuleRefRowTableReader].
  pub fn module_refs<'b>(&'b self) -> ModuleRefRowTableReader<'a, 'b> {
    ModuleRefRowTableReader::new(self.module_ref, &self.header)
  }

  /// Gets the [NestedClassRowTableReader].
  pub fn nested_classes<'b>(&'b self) -> NestedClassRowTableReader<'a, 'b> {
    NestedClassRowTableReader::new(self.nested_class, &self.header)
  }

  /// Gets the [ParamRowTableReader].
  pub fn params<'b>(&'b self) -> ParamRowTableReader<'a, 'b> {
    ParamRowTableReader::new(self.param, &self.header)
  }

  /// Gets the [PropertyRowTableReader].
  pub fn properties<'b>(&'b self) -> PropertyRowTableReader<'a, 'b> {
    PropertyRowTableReader::new(self.property, &self.header)
  }

  /// Gets the [PropertyMapRowTableReader].
  pub fn property_maps<'b>(&'b self) -> PropertyMapRowTableReader<'a, 'b> {
    PropertyMapRowTableReader::new(self.property_map, &self.header)
  }

  /// Gets the [StandAloneSigRowTableReader].
  pub fn stand_alone_sigs<'b>(&'b self) -> StandAloneSigRowTableReader<'a, 'b> {
    StandAloneSigRowTableReader::new(self.stand_alone_sig, &self.header)
  }

  /// Gets the [TypeDefRowTableReader].
  pub fn type_defs<'b>(&'b self) -> TypeDefRowTableReader<'a, 'b> {
    TypeDefRowTableReader::new(self.type_def, &self.header)
  }

  /// Gets the [TypeRefRowTableReader].
  pub fn type_refs<'b>(&'b self) -> TypeRefRowTableReader<'a, 'b> {
    TypeRefRowTableReader::new(self.type_ref, &self.header)
  }

  /// Gets the [TypeSpecRowTableReader].
  pub fn type_specs<'b>(&'b self) -> TypeSpecRowTableReader<'a, 'b> {
    TypeSpecRowTableReader::new(self.type_spec, &self.header)
  }
}
