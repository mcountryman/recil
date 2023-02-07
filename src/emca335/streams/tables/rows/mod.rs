#[doc(hidden)]
pub mod assembly;
#[doc(hidden)]
pub mod assembly_os;
#[doc(hidden)]
pub mod assembly_processor;
#[doc(hidden)]
pub mod assembly_ref;
#[doc(hidden)]
pub mod assembly_ref_os;
#[doc(hidden)]
pub mod assembly_ref_processor;
#[doc(hidden)]
pub mod class_layout;
#[doc(hidden)]
pub mod constant;
#[doc(hidden)]
pub mod custom_attribute;
#[doc(hidden)]
pub mod decl_security;
#[doc(hidden)]
pub mod event;
#[doc(hidden)]
pub mod event_map;
#[doc(hidden)]
pub mod exported_type;
#[doc(hidden)]
pub mod field;
#[doc(hidden)]
pub mod field_layout;
#[doc(hidden)]
pub mod field_marshal;
#[doc(hidden)]
pub mod field_rva;
#[doc(hidden)]
pub mod file;
#[doc(hidden)]
pub mod generic_param;
#[doc(hidden)]
pub mod generic_param_constraint;
#[doc(hidden)]
pub mod impl_map;
#[doc(hidden)]
pub mod interface_impl;
#[doc(hidden)]
pub mod manifest_resource;
#[doc(hidden)]
pub mod member_ref;
#[doc(hidden)]
pub mod method_def;
#[doc(hidden)]
pub mod method_impl;
#[doc(hidden)]
pub mod method_semantics;
#[doc(hidden)]
pub mod method_spec;
#[doc(hidden)]
pub mod module;
#[doc(hidden)]
pub mod module_ref;
#[doc(hidden)]
pub mod nested_class;
#[doc(hidden)]
pub mod param;
#[doc(hidden)]
pub mod property;
#[doc(hidden)]
pub mod property_map;
#[doc(hidden)]
pub mod stand_alone_sig;
#[doc(hidden)]
pub mod type_def;
#[doc(hidden)]
pub mod type_ref;
#[doc(hidden)]
pub mod type_spec;

#[doc(inline)]
pub use assembly::*;
#[doc(inline)]
pub use assembly_os::*;
#[doc(inline)]
pub use assembly_processor::*;
#[doc(inline)]
pub use assembly_ref::*;
#[doc(inline)]
pub use assembly_ref_os::*;
#[doc(inline)]
pub use assembly_ref_processor::*;
#[doc(inline)]
pub use class_layout::*;
#[doc(inline)]
pub use constant::*;
#[doc(inline)]
pub use custom_attribute::*;
#[doc(inline)]
pub use decl_security::*;
#[doc(inline)]
pub use event::*;
#[doc(inline)]
pub use event_map::*;
#[doc(inline)]
pub use exported_type::*;
#[doc(inline)]
pub use field::*;
#[doc(inline)]
pub use field_layout::*;
#[doc(inline)]
pub use field_marshal::*;
#[doc(inline)]
pub use field_rva::*;
#[doc(inline)]
pub use file::*;
#[doc(inline)]
pub use generic_param::*;
#[doc(inline)]
pub use generic_param_constraint::*;
#[doc(inline)]
pub use impl_map::*;
#[doc(inline)]
pub use interface_impl::*;
#[doc(inline)]
pub use manifest_resource::*;
#[doc(inline)]
pub use member_ref::*;
#[doc(inline)]
pub use method_def::*;
#[doc(inline)]
pub use method_impl::*;
#[doc(inline)]
pub use method_semantics::*;
#[doc(inline)]
pub use method_spec::*;
#[doc(inline)]
pub use module::*;
#[doc(inline)]
pub use module_ref::*;
#[doc(inline)]
pub use nested_class::*;
#[doc(inline)]
pub use param::*;
#[doc(inline)]
pub use property::*;
#[doc(inline)]
pub use property_map::*;
#[doc(inline)]
pub use stand_alone_sig::*;
#[doc(inline)]
pub use type_def::*;
#[doc(inline)]
pub use type_ref::*;
#[doc(inline)]
pub use type_spec::*;

use crate::emca335::header::TablesStreamHeader;

/// Defines a row struct and it's associated id type.  
///
/// The row type implements [TableRow], while both types implement [TryFromCtx] and [SizeWith] with
/// a [TableRowCtx] as the context.  This is needed in order to determine the size of an index type
/// (which is dependent on the size of the table), the size of a CodedIndex type (which is dependent
/// on the size of multiple tables), and the size of a stream index type which is based off the
/// [HeapSizes] flag.
macro_rules! row {
  (
    $(#[$attr:meta])*
    pub struct $name:ident : $id:literal {
      $(
        $(#[$field_attr:meta])*
         $field:ident: $ty:ty
      ),*
    }
  ) => {
    paste::paste! {
      #[doc(hidden)]
      #[doc = concat!("The table index for the [", stringify!($name), "] metadata table.")]
      pub const [<$name:snake:upper _ID>]: usize = $id;

      #[derive(Clone, Copy, Debug)]
      $(#[$attr])*
      pub struct $name {
        pub id: [< $name Id >],
        $(
          $(#[$field_attr])*
          pub $field: $ty
        ),*
      }

      impl<'a> scroll::ctx::TryFromCtx<'a, [<$name ReadContext>]<'_>> for $name {
        type Error = crate::error::Error;

        fn try_from_ctx(from: &'a [u8], context: [<$name ReadContext>]<'_>) -> Result<(Self, usize), Self::Error> {
          use scroll::Pread;

          let offset = &mut 0;

          Ok((
            Self {
              id: context.id,
              $($field: from.gread_with(offset, context.into())?),*
            },
            *offset
          ))
        }
      }

      impl scroll::ctx::SizeWith<crate::emca335::header::TablesStreamHeader> for $name {
        fn size_with(header: &crate::emca335::header::TablesStreamHeader) -> usize {
          let size = 0;

          $(
            let size = size + $ty::size_with(&(*header).into());
          )*

          size
        }
      }
    }

    row!(row_id($name));
    row!(row_read_context($name));
    row!(table($name));
    row!(table_reader($name));
    row!(table_reader_iter($name));
  };

  (row_id($name:ident)) => {
    paste::paste! {
      #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
      #[repr(transparent)]
      #[doc = concat!("The id of a [", stringify!($name), "] metadata table row.")]
      pub struct [< $name Id >](u32);

      impl [< $name Id >] {
        #[allow(dead_code)]
        pub(crate) fn new(val: u32) -> Self {
          Self(val)
        }
      }

      impl<'a> scroll::ctx::TryFromCtx<'a, &crate::emca335::header::TablesStreamHeader> for [< $name Id >] {
        type Error = crate::error::Error;

        fn try_from_ctx(
          from: &'a [u8],
          header: &crate::emca335::header::TablesStreamHeader,
        ) -> Result<(Self, usize), Self::Error> {
          use scroll::{ctx::SizeWith, Pread};

          let offset = &mut 0;
          let id = Self(
            match Self::size_with(header) {
              4 => from.gread_with::<u32>(offset, scroll::LE)?,
              2 => from.gread_with::<u16>(offset, scroll::LE)?.into(),
              _ => unreachable!(),
            }
          );

          Ok((id, *offset))
        }
      }

      impl scroll::ctx::SizeWith<crate::emca335::header::TablesStreamHeader> for [< $name Id >] {
        fn size_with(header: &crate::emca335::header::TablesStreamHeader) -> usize {
          if header.rows[[<$name:snake:upper _ ID>]] > u16::MAX.into() {
            4
          } else {
            2
          }
        }
      }
    }
  };

  (row_read_context($name:ident)) => {
    paste::paste! {
      #[derive(Clone, Copy, Debug)]
      pub(crate) struct [<$name ReadContext>]<'a> {
        id: [< $name Id >],
        header: &'a crate::emca335::header::TablesStreamHeader,
      }

      impl<'a> From<[<$name ReadContext>]<'a>> for &'a crate::emca335::header::TablesStreamHeader {
        fn from(context: [<$name ReadContext>]<'a>) -> Self {
          context.header
        }
      }

      impl From<[<$name ReadContext>]<'_>> for crate::emca335::header::TablesStreamHeader {
        fn from(context: [<$name ReadContext>]<'_>) -> Self {
          *context.header
        }
      }

      impl From<[<$name ReadContext>]<'_>> for scroll::Endian {
        fn from(_: [<$name ReadContext>]<'_>) -> Self {
          scroll::LE
        }
      }

      impl From<[<$name ReadContext>]<'_>> for () {
        fn from(_: [<$name ReadContext>]<'_>) -> Self {}
      }
    }
  };

  (table($name:ident)) => {
    paste::paste! {
      #[derive(Clone, Copy, Debug, Default)]
      #[doc = concat!("The [", stringify!($name), "] metadata table.")]
      pub struct [<$name Table>]<'a> {
        buf: &'a [u8],
      }

      impl<'a> scroll::ctx::TryFromCtx<'a, &crate::emca335::header::TablesStreamHeader> for [<$name Table>]<'a> {
        type Error = crate::error::Error;

        fn try_from_ctx(
          from: &'a [u8],
          header: &crate::emca335::header::TablesStreamHeader,
        ) -> Result<(Self, usize), Self::Error> {
          use scroll::{ctx::SizeWith, Pread};

          let offset = &mut 0;
          let size = $name::size_with(header);
          let buf = from.gread_with(offset, size)?;

          Ok((Self { buf }, *offset))
        }
      }
    }
  };

  (table_reader($name:ident)) => {
    paste::paste! {
      #[derive(Clone, Copy, Debug)]
      #[doc = concat!("The [", stringify!($name), "] metadata table reader.")]
      pub struct [<$name TableReader>]<'a, 'b> {
        table: [<$name Table>]<'a>,
        header: &'b crate::emca335::header::TablesStreamHeader,
      }

      impl<'a, 'b> [<$name TableReader>]<'a, 'b> {
        #[doc = concat!("Creates a new [", stringify!([<$name TableReader>]), "] metadata table reader.")]
        pub(crate) fn new(table: [<$name Table>]<'a>, header: &'b crate::emca335::header::TablesStreamHeader) -> Self {
          Self { table, header }
        }

        /// Gets the row at the specified index.
        pub fn get(&self, id: [< $name Id >]) -> Result<Option<$name>, crate::error::Error> {
          use scroll::{Pread, ctx::SizeWith};

          if id.0 >= self.header.rows[[<$name:snake:upper _ ID>]] {
            return Ok(None);
          }

          let offset = usize::try_from(id.0).map_err(|_| crate::error::Error::BadRowId(id.0))?;
          let offset = offset * $name::size_with(&(*self.header).into());
          let context = [<$name ReadContext>] { id, header: self.header };
          let value = self.table.buf.pread_with::<$name>(offset, context)?;

          Ok(Some(value))
        }
      }

      impl<'a, 'b> IntoIterator for [<$name TableReader>]<'a, 'b> {
        type Item = Result<$name, crate::error::Error>;
        type IntoIter = [<$name TableReaderIter>]<'a, 'b>;

        fn into_iter(self) -> Self::IntoIter {
          Self::IntoIter {
            index: 0,
            reader: self,
          }
        }
      }
    }
  };

  (table_reader_iter($name:ident)) => {
    paste::paste! {
      #[derive(Clone, Copy, Debug)]
      #[doc = concat!("The [", stringify!($name), "] metadata table reader.")]
      pub struct [<$name TableReaderIter>]<'a, 'b> {
        index: u32,
        reader: [<$name TableReader>]<'a, 'b>,
      }

      impl<'a, 'b> Iterator for [<$name TableReaderIter>]<'a, 'b> {
        type Item = Result<$name, crate::error::Error>;

        fn next(&mut self) -> Option<Self::Item> {
          let id = [< $name Id >](self.index);

          self.index += 1;
          self.reader.get(id).transpose()
        }
      }

      impl<'a, 'b> core::iter::FusedIterator for [<$name TableReaderIter>]<'a, 'b> {
      }

      impl<'a, 'b> ExactSizeIterator for [<$name TableReaderIter>]<'a, 'b> {
        fn len(&self) -> usize {
          usize::try_from(self.reader.header.rows[[<$name:snake:upper _ ID>]]).unwrap_or_default()
        }
      }
    }
  };
}

pub(crate) use row;

impl From<TablesStreamHeader> for scroll::Endian {
  fn from(_: TablesStreamHeader) -> Self {
    scroll::LE
  }
}

impl From<TablesStreamHeader> for () {
  fn from(_: TablesStreamHeader) -> Self {}
}
