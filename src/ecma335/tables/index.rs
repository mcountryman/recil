use super::{rows::*, TablesHeader};
use anyhow::{bail, Error, Result};
use scroll::{
  ctx::{SizeWith, TryFromCtx},
  Pread, LE,
};

/// A row index.
pub trait RowIndex: Sized {
  /// Gets the next row id of the same type.
  fn next(self) -> Option<Self>;

  /// Gets the first index.
  fn first(header: &TablesHeader) -> Option<Self>;
}

macro_rules! simple_index {
  ($name:ident, $row:ident) => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct $name {
      /// The row index.
      pub(crate) row: u32,
      /// The total number of rows in the table.
      rows: u32,
    }

    impl $name {
      #[doc = concat!("Creates a new [", stringify!($name), "].")]
      pub fn new(row: u32, header: &TablesHeader) -> Result<Self> {
        let rows = header.rows[$row::ID];

        match row <= rows {
          true => Ok(Self { row, rows }),
          false => bail!(
            "`{}`({}) too large, expected less than {}",
            stringify!($name),
            row,
            rows
          ),
        }
      }

      #[allow(dead_code)]
      pub(crate) unsafe fn new_unchecked(row: u32) -> Self {
        Self { row, rows: 0 }
      }
    }

    impl RowIndex for $name {
      fn next(self) -> Option<$name> {
        let row = self.row + 1;
        let rows = self.rows;

        match row < rows {
          true => Some(Self { row, rows }),
          false => None,
        }
      }

      fn first(header: &TablesHeader) -> Option<Self> {
        let rows = header.rows[$row::ID];

        match rows > 0 {
          true => Some(Self { row: 0, rows }),
          false => None,
        }
      }
    }

    impl<'a> TryFromCtx<'a, TablesHeader> for $name {
      type Error = Error;

      fn try_from_ctx(from: &'a [u8], ctx: TablesHeader) -> Result<(Self, usize)> {
        let offset = &mut 0;
        let row = match Self::size_with(&ctx) {
          4 => from.gread_with::<u32>(offset, LE)?,
          2 => from.gread_with::<u16>(offset, LE)?.into(),
          _ => panic!("Invalid size"),
        };

        Ok(($name::new(row, &ctx)?, *offset))
      }
    }

    impl SizeWith<TablesHeader> for $name {
      fn size_with(header: &TablesHeader) -> usize {
        let rows = header.rows[$row::ID];

        match rows < 1 << 16 {
          true => 2,
          false => 4,
        }
      }
    }
  };
}

macro_rules! coded_index {
  (
      $(#[$attr:meta])*
      pub enum $name:ident : $bits:literal {
        $(
          $(#[$variant_attr:meta])*
          $variant:ident($table:ident::$index:ident) = $tag:literal
        ),* $(,)?
      }
    ) => {
      $(#[$attr])*
      #[derive(Debug, PartialEq, Eq)]
      pub enum $name {
        $(
          $(#[$variant_attr])*
          $variant($index)
        ),*
      }

      impl $name {
        pub fn new(val: u32, header: &TablesHeader) -> Result<Self> {
          let tag = val & ((1 << $bits) - 1);
          let index = val >> $bits;

          match tag {
            $(
              $tag => Ok(Self::$variant($index::new(index, header)?)),
            )*
            _ => bail!("Malformed {}, tag {tag}", stringify!($name), tag = tag)
          }
        }

        #[allow(dead_code)]
        pub(crate) unsafe fn new_unchecked(val: u32) -> Self {
          let tag = val & ((1 << $bits) - 1);
          let index = val >> $bits;

          match tag {
            $(
              $tag => Self::$variant($index::new_unchecked(index)),
            )*
            _ => panic!("Malformed {}, tag {tag}", stringify!($name), tag = tag)
          }
        }
      }

      impl<'a> TryFromCtx<'a, TablesHeader> for $name {
        type Error = Error;

        fn try_from_ctx(from: &'a [u8], ctx: TablesHeader) -> Result<(Self, usize)> {
          let offset = &mut 0;
          let size = Self::size_with(&ctx);
          let val = match Self::size_with(&ctx) {
            4 => from.gread_with::<u32>(offset, LE)?,
            2 => from.gread_with::<u16>(offset, LE)?.into(),
            _ => panic!("Invalid size"),
          };

          Self::new(val, &ctx).map(|val| (val, size))
        }
      }

      impl SizeWith<TablesHeader> for $name {
        fn size_with(header: &TablesHeader) -> usize {
          $(
            if header.rows[$table::ID] >= (1u32 << (16 - $bits)) {
              return 4;
            }
          )+

          2
        }
      }
    };
}

simple_index!(AssemblyRowId, AssemblyRow);
simple_index!(AssemblyOsRowId, AssemblyOsRow);
simple_index!(AssemblyProcessorRowId, AssemblyProcessorRow);
simple_index!(AssemblyRefRowId, AssemblyRefRow);
simple_index!(AssemblyRefOsRowId, AssemblyRefOsRow);
simple_index!(AssemblyRefProcessorRowId, AssemblyRefProcessorRow);
simple_index!(ClassLayoutRowId, ClassLayoutRow);
simple_index!(ConstantRowId, ConstantRow);
simple_index!(CustomAttributeRowId, CustomAttributeRow);
simple_index!(DeclSecurityRowId, DeclSecurityRow);
simple_index!(EventRowId, EventRow);
simple_index!(EventMapRowId, EventMapRow);
simple_index!(ExportedTypeRowId, ExportedTypeRow);
simple_index!(FieldRowId, FieldRow);
simple_index!(FieldLayoutRowId, FieldLayoutRow);
simple_index!(FieldMarshalRowId, FieldMarshalRow);
simple_index!(FieldRvaRowId, FieldRvaRow);
simple_index!(FileRowId, FileRow);
simple_index!(GenericParamRowId, GenericParamRow);
simple_index!(GenericParamConstraintRowId, GenericParamConstraintRow);
simple_index!(ImplMapRowId, ImplMapRow);
simple_index!(InterfaceImplRowId, InterfaceImplRow);
simple_index!(ManifestResourceRowId, ManifestResourceRow);
simple_index!(MemberRefRowId, MemberRefRow);
simple_index!(MethodDefRowId, MethodDefRow);
simple_index!(MethodImplRowId, MethodImplRow);
simple_index!(MethodSemanticsRowId, MethodSemanticsRow);
simple_index!(MethodSpecRowId, MethodSpecRow);
simple_index!(ModuleRowId, ModuleRow);
simple_index!(ModuleRefRowId, ModuleRefRow);
simple_index!(NestedClassRowId, NestedClassRow);
simple_index!(ParamRowId, ParamRow);
simple_index!(PropertyRowId, PropertyRow);
simple_index!(PropertyMapRowId, PropertyMapRow);
simple_index!(StandAloneSigRowId, StandAloneSigRow);
simple_index!(TypeDefRowId, TypeDefRow);
simple_index!(TypeRefRowId, TypeRefRow);
simple_index!(TypeSpecRowId, TypeSpecRow);

coded_index! {
  pub enum TypeDefOrRef : 2 {
    TypeDef(TypeDefRow::TypeDefRowId) = 0,
    TypeRef(TypeRefRow::TypeRefRowId) = 1,
    TypeSpec(TypeSpecRow::TypeSpecRowId) = 2
  }
}

coded_index! {
  pub enum HasConstant : 2 {
    Field(FieldRow::FieldRowId) = 0,
    Param(ParamRow::ParamRowId) = 1,
    Property(PropertyRow::PropertyRowId) = 2
  }
}

coded_index! {
  pub enum HasCustomAttribute : 5 {
    MethodDef(MethodDefRow::MethodDefRowId) = 0,
    Field(FieldRow::FieldRowId) = 1,
    TypeRef(TypeRefRow::TypeRefRowId) = 2,
    TypeDef(TypeDefRow::TypeDefRowId) = 3,
    Param(ParamRow::ParamRowId) = 4,
    InterfaceImpl(InterfaceImplRow::InterfaceImplRowId) = 5,
    MemberRef(MemberRefRow::MemberRefRowId) = 6,
    Module(ModuleRow::ModuleRowId) = 7,
    DeclSecurity(DeclSecurityRow::DeclSecurityRowId) = 8,
    Property(PropertyRow::PropertyRowId) = 9,
    Event(EventRow::EventRowId) = 10,
    StandAloneSig(StandAloneSigRow::StandAloneSigRowId) = 11,
    ModuleRef(ModuleRefRow::ModuleRefRowId) = 12,
    TypeSpec(TypeSpecRow::TypeSpecRowId) = 13,
    Assembly(AssemblyRow::AssemblyRowId) = 14,
    AssemblyRef(AssemblyRefRow::AssemblyRefRowId) = 15,
    File(FileRow::FileRowId) = 16,
    ExportedType(ExportedTypeRow::ExportedTypeRowId) = 17,
    ManifestResource(ManifestResourceRow::ManifestResourceRowId) = 18,
    GenericParam(GenericParamRow::GenericParamRowId) = 19,
    GenericParamConstraint(GenericParamConstraintRow::GenericParamConstraintRowId) = 20,
    MethodSpec(MethodSpecRow::MethodSpecRowId) = 21
  }
}

coded_index! {
  pub enum HasFieldMarshal : 1 {
    Field(FieldRow::FieldRowId) = 0,
    Param(ParamRow::ParamRowId) = 1
  }
}

coded_index! {
  pub enum HasDeclSecurity : 2 {
    TypeDef(TypeDefRow::TypeDefRowId) = 0,
    MethodDef(MethodDefRow::MethodDefRowId) = 1,
    Assembly(AssemblyRow::AssemblyRowId) = 2
  }
}

coded_index! {
  pub enum MemberRefParent : 3 {
    TypeDef(TypeDefRow::TypeDefRowId) = 0,
    TypeRef(TypeRefRow::TypeRefRowId) = 1,
    ModuleRef(ModuleRefRow::ModuleRefRowId) = 2,
    MethodDef(MethodDefRow::MethodDefRowId) = 3,
    TypeSpec(TypeSpecRow::TypeSpecRowId) = 4
  }
}

coded_index! {
  pub enum HasSemantics : 1 {
    Event(EventRow::EventRowId) = 0,
    Property(PropertyRow::PropertyRowId) = 1
  }
}

coded_index! {
  pub enum MethodDefOrRef : 1 {
    MethodDef(MethodDefRow::MethodDefRowId) = 0,
    MemberRef(MemberRefRow::MemberRefRowId) = 1
  }
}

coded_index! {
  pub enum MemberForwarded : 1 {
    Field(FieldRow::FieldRowId) = 0,
    MethodDef(MethodDefRow::MethodDefRowId) = 1
  }
}

coded_index! {
  pub enum Implementation : 2 {
    File(FileRow::FileRowId) = 0,
    AssemblyRef(AssemblyRefRow::AssemblyRefRowId) = 1,
    ExportedType(ExportedTypeRow::ExportedTypeRowId) = 2
  }
}

coded_index! {
  pub enum CustomAttributeType : 3 {
    MethodDef(MethodDefRow::MethodDefRowId) = 2,
    MemberRef(MemberRefRow::MemberRefRowId) = 3
  }
}

coded_index! {
  pub enum ResolutionScope : 2 {
    Module(ModuleRow::ModuleRowId) = 0,
    ModuleRef(ModuleRefRow::ModuleRefRowId) = 1,
    AssemblyRef(AssemblyRefRow::AssemblyRefRowId) = 2,
    TypeRef(TypeRefRow::TypeRefRowId) = 3
  }
}

coded_index! {
  pub enum TypeOrMethodDef : 1 {
    TypeDef(TypeDefRow::TypeDefRowId) = 0,
    MethodDef(MethodDefRow::MethodDefRowId) = 1
  }
}

#[cfg(test)]
mod tests {
  use super::ResolutionScope;
  use crate::ecma335::tables::{AssemblyRefRowId, ModuleRefRowId, ModuleRowId, TypeRefRowId};

  #[test]
  fn new_resolution_scope() {
    unsafe {
      assert_eq!(
        ResolutionScope::new_unchecked(0),
        ResolutionScope::Module(ModuleRowId::new_unchecked(0))
      );

      assert_eq!(
        ResolutionScope::new_unchecked((0xffffff << 2) | 3),
        ResolutionScope::TypeRef(TypeRefRowId::new_unchecked(0xffffff))
      );

      assert_eq!(
        ResolutionScope::new_unchecked(0x04),
        ResolutionScope::Module(ModuleRowId::new_unchecked(1))
      );

      assert_eq!(
        ResolutionScope::new_unchecked(0x04 | 1),
        ResolutionScope::ModuleRef(ModuleRefRowId::new_unchecked(1))
      );

      assert_eq!(
        ResolutionScope::new_unchecked(0x04 | 2),
        ResolutionScope::AssemblyRef(AssemblyRefRowId::new_unchecked(1))
      );

      assert_eq!(
        ResolutionScope::new_unchecked(0x04 | 3),
        ResolutionScope::TypeRef(TypeRefRowId::new_unchecked(1))
      );
    }
  }
}
