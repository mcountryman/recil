//! ECMA-335 Metadata `#~` stream data.
//!
//! The `#~` stream contains the metadata tables, which are the main source of information about the
//! types and methods in the assembly.

pub mod flags;
pub mod index;
#[doc(hidden)]
pub mod rows;
#[doc(inline)]
pub use index::*;
#[doc(inline)]
pub use rows::*;

use super::StreamHeader;
use anyhow::{Context, Error, Result};
use core::marker::PhantomData;
use scroll::{ctx::TryFromCtx, Pread, LE};

/// The `#~` stream data.
#[derive(Default)]
pub struct Tables<'a> {
  header: TablesHeader,
  assemblies: Table<'a, AssemblyRow>,
  assembly_oses: Table<'a, AssemblyOsRow>,
  assembly_processors: Table<'a, AssemblyProcessorRow>,
  assembly_refs: Table<'a, AssemblyRefRow>,
  assembly_ref_oses: Table<'a, AssemblyRefOsRow>,
  assembly_ref_processors: Table<'a, AssemblyRefProcessorRow>,
  class_layouts: Table<'a, ClassLayoutRow>,
  constants: Table<'a, ConstantRow>,
  custom_attributes: Table<'a, CustomAttributeRow>,
  decl_securities: Table<'a, DeclSecurityRow>,
  events: Table<'a, EventRow>,
  event_maps: Table<'a, EventMapRow>,
  exported_types: Table<'a, ExportedTypeRow>,
  fields: Table<'a, FieldRow>,
  field_layouts: Table<'a, FieldLayoutRow>,
  field_marshals: Table<'a, FieldMarshalRow>,
  field_rvas: Table<'a, FieldRvaRow>,
  files: Table<'a, FileRow>,
  generic_params: Table<'a, GenericParamRow>,
  generic_param_constraints: Table<'a, GenericParamConstraintRow>,
  impl_maps: Table<'a, ImplMapRow>,
  interface_impls: Table<'a, InterfaceImplRow>,
  manifest_resources: Table<'a, ManifestResourceRow>,
  member_refs: Table<'a, MemberRefRow>,
  method_defs: Table<'a, MethodDefRow>,
  method_impls: Table<'a, MethodImplRow>,
  method_semantics: Table<'a, MethodSemanticsRow>,
  method_specs: Table<'a, MethodSpecRow>,
  modules: Table<'a, ModuleRow>,
  module_refs: Table<'a, ModuleRefRow>,
  nested_classes: Table<'a, NestedClassRow>,
  params: Table<'a, ParamRow>,
  properties: Table<'a, PropertyRow>,
  property_maps: Table<'a, PropertyMapRow>,
  stand_alone_sigs: Table<'a, StandAloneSigRow>,
  type_defs: Table<'a, TypeDefRow>,
  type_refs: Table<'a, TypeRefRow>,
  type_specs: Table<'a, TypeSpecRow>,
}

impl<'a> Tables<'a> {
  /// Parses the metadata tables stream from the given stream header.
  ///
  /// # Note
  /// The stream header must be for the `#~` stream and isn't verified here in release builds.  
  /// Don't be a dummy.
  pub fn parse_from_header(header: StreamHeader<'a>) -> Result<Self> {
    debug_assert!(matches!(header.name.to_bytes(), b"#~" | b"#-"));

    let buf = header.data()?;
    let offset = &mut 0;
    let header = buf.gread::<TablesHeader>(offset)?;
    let mut tables = Self {
      header,
      ..Default::default()
    };

    for i in 0..64 {
      if header.rows[i] == 0 {
        continue;
      }

      if !header.has_table(i) {
        continue;
      }

      match i {
        AssemblyRow::ID => tables.assemblies = buf.gread_with(offset, header)?,
        AssemblyOsRow::ID => tables.assembly_oses = buf.gread_with(offset, header)?,
        AssemblyProcessorRow::ID => tables.assembly_processors = buf.gread_with(offset, header)?,
        AssemblyRefRow::ID => tables.assembly_refs = buf.gread_with(offset, header)?,
        AssemblyRefOsRow::ID => tables.assembly_ref_oses = buf.gread_with(offset, header)?,
        AssemblyRefProcessorRow::ID => {
          tables.assembly_ref_processors = buf.gread_with(offset, header)?
        }
        ClassLayoutRow::ID => tables.class_layouts = buf.gread_with(offset, header)?,
        ConstantRow::ID => tables.constants = buf.gread_with(offset, header)?,
        CustomAttributeRow::ID => tables.custom_attributes = buf.gread_with(offset, header)?,
        DeclSecurityRow::ID => tables.decl_securities = buf.gread_with(offset, header)?,
        EventRow::ID => tables.events = buf.gread_with(offset, header)?,
        EventMapRow::ID => tables.event_maps = buf.gread_with(offset, header)?,
        ExportedTypeRow::ID => tables.exported_types = buf.gread_with(offset, header)?,
        FieldRow::ID => tables.fields = buf.gread_with(offset, header)?,
        FieldLayoutRow::ID => tables.field_layouts = buf.gread_with(offset, header)?,
        FieldMarshalRow::ID => tables.field_marshals = buf.gread_with(offset, header)?,
        FieldRvaRow::ID => tables.field_rvas = buf.gread_with(offset, header)?,
        FileRow::ID => tables.files = buf.gread_with(offset, header)?,
        GenericParamRow::ID => tables.generic_params = buf.gread_with(offset, header)?,
        GenericParamConstraintRow::ID => {
          tables.generic_param_constraints = buf.gread_with(offset, header)?
        }
        ImplMapRow::ID => tables.impl_maps = buf.gread_with(offset, header)?,
        InterfaceImplRow::ID => tables.interface_impls = buf.gread_with(offset, header)?,
        ManifestResourceRow::ID => tables.manifest_resources = buf.gread_with(offset, header)?,
        MemberRefRow::ID => tables.member_refs = buf.gread_with(offset, header)?,
        MethodDefRow::ID => tables.method_defs = buf.gread_with(offset, header)?,
        MethodImplRow::ID => tables.method_impls = buf.gread_with(offset, header)?,
        MethodSemanticsRow::ID => tables.method_semantics = buf.gread_with(offset, header)?,
        MethodSpecRow::ID => tables.method_specs = buf.gread_with(offset, header)?,
        ModuleRow::ID => tables.modules = buf.gread_with(offset, header)?,
        ModuleRefRow::ID => tables.module_refs = buf.gread_with(offset, header)?,
        NestedClassRow::ID => tables.nested_classes = buf.gread_with(offset, header)?,
        ParamRow::ID => tables.params = buf.gread_with(offset, header)?,
        PropertyRow::ID => tables.properties = buf.gread_with(offset, header)?,
        PropertyMapRow::ID => tables.property_maps = buf.gread_with(offset, header)?,
        StandAloneSigRow::ID => tables.stand_alone_sigs = buf.gread_with(offset, header)?,
        TypeDefRow::ID => tables.type_defs = buf.gread_with(offset, header)?,
        TypeRefRow::ID => tables.type_refs = buf.gread_with(offset, header)?,
        TypeSpecRow::ID => tables.type_specs = buf.gread_with(offset, header)?,
        _ => {}
      }
    }

    Ok(tables)
  }

  pub fn assemblies<'t: 'a>(&'t self) -> TableRowReader<'a, 't, AssemblyRow> {
    TableRowReader::new(&self.assemblies, &self.header)
  }

  pub fn assembly_oses<'t: 'a>(&'t self) -> TableRowReader<'a, 't, AssemblyOsRow> {
    TableRowReader::new(&self.assembly_oses, &self.header)
  }

  pub fn assembly_processors<'t: 'a>(&'t self) -> TableRowReader<'a, 't, AssemblyProcessorRow> {
    TableRowReader::new(&self.assembly_processors, &self.header)
  }

  pub fn assembly_refs<'t: 'a>(&'t self) -> TableRowReader<'a, 't, AssemblyRefRow> {
    TableRowReader::new(&self.assembly_refs, &self.header)
  }

  pub fn assembly_ref_oses<'t: 'a>(&'t self) -> TableRowReader<'a, 't, AssemblyRefOsRow> {
    TableRowReader::new(&self.assembly_ref_oses, &self.header)
  }

  pub fn assembly_ref_processors<'t: 'a>(
    &'t self,
  ) -> TableRowReader<'a, 't, AssemblyRefProcessorRow> {
    TableRowReader::new(&self.assembly_ref_processors, &self.header)
  }

  pub fn class_layouts<'t: 'a>(&'t self) -> TableRowReader<'a, 't, ClassLayoutRow> {
    TableRowReader::new(&self.class_layouts, &self.header)
  }

  pub fn constants<'t: 'a>(&'t self) -> TableRowReader<'a, 't, ConstantRow> {
    TableRowReader::new(&self.constants, &self.header)
  }

  pub fn custom_attributes<'t: 'a>(&'t self) -> TableRowReader<'a, 't, CustomAttributeRow> {
    TableRowReader::new(&self.custom_attributes, &self.header)
  }

  pub fn decl_securities<'t: 'a>(&'t self) -> TableRowReader<'a, 't, DeclSecurityRow> {
    TableRowReader::new(&self.decl_securities, &self.header)
  }

  pub fn events<'t: 'a>(&'t self) -> TableRowReader<'a, 't, EventRow> {
    TableRowReader::new(&self.events, &self.header)
  }

  pub fn event_maps<'t: 'a>(&'t self) -> TableRowReader<'a, 't, EventMapRow> {
    TableRowReader::new(&self.event_maps, &self.header)
  }

  pub fn exported_types<'t: 'a>(&'t self) -> TableRowReader<'a, 't, ExportedTypeRow> {
    TableRowReader::new(&self.exported_types, &self.header)
  }

  pub fn fields<'t>(&'t self) -> TableRowReader<'a, 't, FieldRow> {
    TableRowReader::new(&self.fields, &self.header)
  }

  pub fn field_layouts<'t: 'a>(&'t self) -> TableRowReader<'a, 't, FieldLayoutRow> {
    TableRowReader::new(&self.field_layouts, &self.header)
  }

  pub fn field_marshals<'t: 'a>(&'t self) -> TableRowReader<'a, 't, FieldMarshalRow> {
    TableRowReader::new(&self.field_marshals, &self.header)
  }

  pub fn field_rvas<'t: 'a>(&'t self) -> TableRowReader<'a, 't, FieldRvaRow> {
    TableRowReader::new(&self.field_rvas, &self.header)
  }

  pub fn files<'t>(&'t self) -> TableRowReader<'a, 't, FileRow> {
    TableRowReader::new(&self.files, &self.header)
  }

  pub fn generic_params<'t: 'a>(&'t self) -> TableRowReader<'a, 't, GenericParamRow> {
    TableRowReader::new(&self.generic_params, &self.header)
  }

  pub fn generic_param_constraints<'t>(
    &'t self,
  ) -> TableRowReader<'a, 't, GenericParamConstraintRow> {
    TableRowReader::new(&self.generic_param_constraints, &self.header)
  }

  pub fn impl_maps<'t>(&'t self) -> TableRowReader<'a, 't, ImplMapRow> {
    TableRowReader::new(&self.impl_maps, &self.header)
  }

  pub fn interface_impls<'t: 'a>(&'t self) -> TableRowReader<'a, 't, InterfaceImplRow> {
    TableRowReader::new(&self.interface_impls, &self.header)
  }

  pub fn manifest_resources<'t: 'a>(&'t self) -> TableRowReader<'a, 't, ManifestResourceRow> {
    TableRowReader::new(&self.manifest_resources, &self.header)
  }

  pub fn member_refs<'t: 'a>(&'t self) -> TableRowReader<'a, 't, MemberRefRow> {
    TableRowReader::new(&self.member_refs, &self.header)
  }

  pub fn method_defs<'t: 'a>(&'t self) -> TableRowReader<'a, 't, MethodDefRow> {
    TableRowReader::new(&self.method_defs, &self.header)
  }

  pub fn method_impls<'t: 'a>(&'t self) -> TableRowReader<'a, 't, MethodImplRow> {
    TableRowReader::new(&self.method_impls, &self.header)
  }

  pub fn method_semantics<'t: 'a>(&'t self) -> TableRowReader<'a, 't, MethodSemanticsRow> {
    TableRowReader::new(&self.method_semantics, &self.header)
  }

  pub fn method_specs<'t: 'a>(&'t self) -> TableRowReader<'a, 't, MethodSpecRow> {
    TableRowReader::new(&self.method_specs, &self.header)
  }

  pub fn modules<'t>(&'t self) -> TableRowReader<'a, 't, ModuleRow> {
    TableRowReader::new(&self.modules, &self.header)
  }

  pub fn module_refs<'t: 'a>(&'t self) -> TableRowReader<'a, 't, ModuleRefRow> {
    TableRowReader::new(&self.module_refs, &self.header)
  }

  pub fn nested_classes<'t: 'a>(&'t self) -> TableRowReader<'a, 't, NestedClassRow> {
    TableRowReader::new(&self.nested_classes, &self.header)
  }

  pub fn params<'t>(&'t self) -> TableRowReader<'a, 't, ParamRow> {
    TableRowReader::new(&self.params, &self.header)
  }

  pub fn properties<'t: 'a>(&'t self) -> TableRowReader<'a, 't, PropertyRow> {
    TableRowReader::new(&self.properties, &self.header)
  }

  pub fn property_maps<'t: 'a>(&'t self) -> TableRowReader<'a, 't, PropertyMapRow> {
    TableRowReader::new(&self.property_maps, &self.header)
  }

  pub fn stand_alone_sigs<'t: 'a>(&'t self) -> TableRowReader<'a, 't, StandAloneSigRow> {
    TableRowReader::new(&self.stand_alone_sigs, &self.header)
  }

  pub fn type_defs<'t>(&'t self) -> TableRowReader<'a, 't, TypeDefRow> {
    TableRowReader::new(&self.type_defs, &self.header)
  }

  pub fn type_refs<'t>(&'t self) -> TableRowReader<'a, 't, TypeRefRow> {
    TableRowReader::new(&self.type_refs, &self.header)
  }

  pub fn type_specs<'t: 'a>(&'t self) -> TableRowReader<'a, 't, TypeSpecRow> {
    TableRowReader::new(&self.type_specs, &self.header)
  }
}

/// Contains table data for a single metadata table.
#[derive(Clone, Copy)]
pub struct Table<'a, R> {
  len: usize,
  buf: &'a [u8],
  row: PhantomData<R>,
}

impl<'a, R: Row<'a>> Table<'a, R> {
  pub fn id() -> usize {
    R::ID
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn is_empty(&self) -> bool {
    self.len == 0
  }
}

impl<'a, R> Default for Table<'a, R> {
  fn default() -> Self {
    Self {
      len: 0,
      buf: &[],
      row: Default::default(),
    }
  }
}

impl<'a, R: Row<'a>> TryFromCtx<'a, TablesHeader> for Table<'a, R> {
  type Error = Error;

  fn try_from_ctx(from: &'a [u8], ctx: TablesHeader) -> Result<(Self, usize)> {
    let offset = &mut 0;

    let rows = ctx.rows[R::ID];
    let row_size = R::size_with(&ctx);
    let buf_len = rows as usize * row_size;
    let buf = from
      .gread_with(offset, buf_len)
      .context("Table buffer too small")?;

    Ok((
      Self {
        len: rows as usize,
        buf,
        row: Default::default(),
      },
      *offset,
    ))
  }
}

/// Provides read access to metadata table rows.
#[derive(Clone, Copy)]
pub struct TableRowReader<'a, 't, R> {
  table: &'t Table<'a, R>,
  header: &'t TablesHeader,
}

impl<'a, 't, R: Row<'a>> TableRowReader<'a, 't, R> {
  pub fn new(table: &'t Table<'a, R>, header: &'t TablesHeader) -> Self {
    Self { table, header }
  }

  pub fn len(&self) -> usize {
    self.table.len()
  }

  pub fn is_empty(&self) -> bool {
    self.table.is_empty()
  }

  pub fn read(&self, index: R::Index) -> Result<R> {
    R::parse(self.table.buf, index, self.header)
  }
}

impl<'a, 't, R: Row<'a>> IntoIterator for TableRowReader<'a, 't, R> {
  type Item = Result<R>;
  type IntoIter = TableRowIterator<'a, 't, R>;

  fn into_iter(self) -> Self::IntoIter {
    let index = R::Index::first(self.header);

    TableRowIterator {
      reader: self,
      index,
    }
  }
}

/// Iterates over rows in a metadata table.
pub struct TableRowIterator<'a, 't, R: Row<'a>> {
  reader: TableRowReader<'a, 't, R>,
  index: Option<R::Index>,
}

impl<'a, 't, R: Row<'a>> Iterator for TableRowIterator<'a, 't, R> {
  type Item = Result<R>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.index {
      Some(index) => {
        let row = self.reader.read(index);
        self.index = index.next();
        Some(row)
      }
      None => None,
    }
  }
}

/// Contains the table stream header information.
#[derive(Clone, Copy)]
pub struct TablesHeader {
  // Reserved, always 0.
  _reserved_0: u32,
  /// Major version of table schemata; shall be 2.
  pub major_version: u8,
  /// Minor version of table schemata; shall be 0.
  pub minor_version: u8,
  /// Bit vector for heap sizes.
  pub heap_sizes: HeapSizes,
  /// Reserved, always 1.
  _reserved_1: u8,
  /// Bit vector of present tables, let n be the number of bits that are 1.
  pub valid: u64,
  /// Bit vector of sorted tables.
  pub sorted: u64,
  /// The array containing the values representing the number of rows in a table, indexed by the id
  /// of the table.
  pub rows: [u32; 64],
}

impl TablesHeader {
  /// Determines if the table data exists in the metadata or not.
  pub fn has_table(&self, id: usize) -> bool {
    self.valid & (1 << id) != 0
  }
}

impl<'a> TryFromCtx<'a> for TablesHeader {
  type Error = Error;

  fn try_from_ctx(from: &'a [u8], _: ()) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;

    *offset += 4;
    let major_version = from.gread_with(offset, LE)?;
    let minor_version = from.gread_with(offset, LE)?;
    let heap_sizes = from.gread_with(offset, LE)?;
    *offset += 1;
    let valid = from.gread_with(offset, LE)?;
    let sorted = from.gread_with(offset, LE)?;
    let mut rows = [0; 64];

    for (i, row) in rows.iter_mut().enumerate() {
      if valid & (1 << i) != 0 {
        *row = from.gread_with(offset, LE)?;
      }
    }

    Ok((
      Self {
        _reserved_0: 0,
        major_version,
        minor_version,
        heap_sizes,
        _reserved_1: 1,
        valid,
        sorted,
        rows,
      },
      *offset,
    ))
  }
}

impl Default for TablesHeader {
  fn default() -> Self {
    Self {
      _reserved_0: 0,
      major_version: 0,
      minor_version: 0,
      heap_sizes: HeapSizes::default(),
      _reserved_1: 1,
      valid: 0,
      sorted: 0,
      rows: [0; 64],
    }
  }
}

bitflags::bitflags! {
  /// The bit flags indicating which heaps should have 4 bit wide indexes or 2 bit wide indexes.
  #[derive(Default, Pread)]
  pub struct HeapSizes : u8 {
    /// If set indicates the `#Strings` heap index should be `4` bytes wide, otherwise `2`.
    const WIDE_STRING_HEAP = 0x01;
    /// If set indicates the `#GUID` heap index should be `4` bytes wide, otherwise `2`.
    const WIDE_GUID_HEAP = 0x02;
    /// If set indicates the `#Blob` heap index should be `4` bytes wide, otherwise `2`.
    const WIDE_BLOB_HEAP = 0x04;
  }
}
