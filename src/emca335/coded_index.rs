macro_rules! coded_index {
  (
    $(#[$attr:meta])*
    pub enum $name:ident : $bits:literal {
      $(
        $(#[$variant_attr:meta])*
        $variant:ident = $tag:literal
      ),*
    }
  ) => {
    paste::paste! {
      $(#[$attr])*
      #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
      pub enum $name {
        $(
          $(#[$variant_attr])*
          $variant(crate::emca335::streams::tables::rows::[< $variant RowId >])
        ),*
      }

      impl scroll::ctx::TryFromCtx<'_, crate::emca335::header::TablesStreamHeader> for $name {
        type Error = crate::error::Error;

        fn try_from_ctx(from: &[u8], ctx: crate::emca335::header::TablesStreamHeader) -> Result<(Self, usize), Self::Error> {
          use scroll::{Pread, ctx::SizeWith};

          let offset = &mut 0;
          let size = Self::size_with(&ctx);
          let val = if size == 4 {
            from.gread_with::<u32>(offset, scroll::LE)?
          } else {
            from.gread_with::<u16>(offset, scroll::LE)?.into()
          };

          Self::try_from(val)
            .map(|val| (val, size))
        }
      }

      impl scroll::ctx::SizeWith<crate::emca335::header::TablesStreamHeader> for $name {
        fn size_with(header: &crate::emca335::header::TablesStreamHeader) -> usize {
          let max_row_size = 0;

          $(
            let row_count = header.rows[crate::emca335::streams::rows::[< $variant:snake:upper _ROW_ID >]];
            let max_row_size = max_row_size.max(row_count);
          )*

          if max_row_size > u16::MAX as _ {
            4
          } else {
            2
          }
        }
      }

      impl TryFrom<u32> for $name {
        type Error = crate::error::Error;

        fn try_from(val: u32) -> Result<Self, Self::Error> {
          let tag = val & ((1 << $bits) - 1);
          let index = val >> $bits;

          match tag {
            $(
              $tag => Ok(Self::$variant(crate::emca335::streams::rows::[<$variant RowId>]::new(index))),
            )*
            _ => Err(crate::error::Error::Malformed(stringify!($name)))
          }
        }
      }
    }
  };
}

coded_index! {
  pub enum TypeDefOrRef : 2 {
    TypeDef = 0,
    TypeRef = 1,
    TypeSpec = 2
  }
}

coded_index! {
  pub enum HasConstant : 2 {
    Field = 0,
    Param = 1,
    Property = 2
  }
}

coded_index! {
  pub enum HasCustomAttribute : 5 {
    MethodDef = 0,
    Field = 1,
    TypeRef = 2,
    TypeDef = 3,
    Param = 4,
    InterfaceImpl = 5,
    MemberRef = 6,
    Module = 7,
    DeclSecurity = 8,
    Property = 9,
    Event = 10,
    StandAloneSig = 11,
    ModuleRef = 12,
    TypeSpec = 13,
    Assembly = 14,
    AssemblyRef = 15,
    File = 16,
    ExportedType = 17,
    ManifestResource = 18,
    GenericParam = 19,
    GenericParamConstraint = 20,
    MethodSpec = 21
  }
}

coded_index! {
  pub enum HasFieldMarshal : 1 {
    Field = 0,
    Param = 1
  }
}

coded_index! {
  pub enum HasDeclSecurity : 2 {
    TypeDef = 0,
    MethodDef = 1,
    Assembly = 2
  }
}

coded_index! {
  pub enum MemberRefParent : 3 {
    TypeDef = 0,
    TypeRef = 1,
    ModuleRef = 2,
    MethodDef = 3,
    TypeSpec = 4
  }
}

coded_index! {
  pub enum HasSemantics : 1 {
    Event = 0,
    Property = 1
  }
}

coded_index! {
  pub enum MethodDefOrRef : 1 {
    MethodDef = 0,
    MemberRef = 1
  }
}

coded_index! {
  pub enum MemberForwarded : 1 {
    Field = 0,
    MethodDef = 1
  }
}

coded_index! {
  pub enum Implementation : 2 {
    File = 0,
    AssemblyRef = 1,
    ExportedType = 2
  }
}

coded_index! {
  pub enum CustomAttributeType : 3 {
    MethodDef = 2,
    MemberRef = 3
  }
}

coded_index! {
  pub enum ResolutionScope : 2 {
    Module = 0,
    ModuleRef = 1,
    AssemblyRef = 2,
    TypeRef = 3
  }
}

coded_index! {
  pub enum TypeOrMethodDef : 1 {
    TypeDef = 0,
    MethodDef = 1
  }
}
