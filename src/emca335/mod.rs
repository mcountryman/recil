//! EMCA-335 CLI metadata parsing.

pub mod coded_index;
pub mod element_type;
pub mod header;
pub mod streams;
pub use streams::*;

use crate::error::Error;
use core::ffi::CStr;
use scroll::Pread;
use streams::{
  blobs::BlobsHeap, guids::GuidsHeap, strings::StringsHeap, tables::Tables, us::UserStringsHeap,
};

use self::header::{MetadataHeader, METADATA_MAGIC};

/// Contains EMCA-335 CLI metadata.
pub struct Metadata<'a> {
  /// The version of the CLI metadata.
  pub version: &'a CStr,
  /// The `#Blob` stream.
  pub blobs: BlobsHeap<'a>,
  /// The `#GUID` stream.
  pub guids: GuidsHeap<'a>,
  /// The `#~` stream.
  pub tables: Tables<'a>,
  /// The `#Strings` stream.
  pub strings: StringsHeap<'a>,
  /// The `#US` stream.
  pub user_strings: UserStringsHeap<'a>,
}

impl<'a> Metadata<'a> {
  /// Parses the EMCA-335 CLI metadata from the given buffer.
  pub fn parse(buf: &'a [u8]) -> Result<Self, Error> {
    let offset = &mut 0;
    let header = buf.gread::<MetadataHeader>(offset)?;
    if header.magic != METADATA_MAGIC {
      return Err(Error::BadMagic("Metadata"));
    }

    let mut blobs = BlobsHeap::default();
    let mut guids = GuidsHeap::default();
    let mut tables = Tables::default();
    let mut strings = StringsHeap::default();
    let mut user_strings = UserStringsHeap::default();

    if let Some(header) = header.streams.blobs {
      blobs = BlobsHeap::parse(buf, &header)?;
    }

    if let Some(header) = header.streams.guids {
      guids = GuidsHeap::parse(buf, &header)?;
    }

    if let Some(header) = header.streams.tables {
      tables = Tables::parse(buf, &header)?;
    }

    if let Some(header) = header.streams.strings {
      strings = StringsHeap::parse(buf, &header)?;
    }

    if let Some(header) = header.streams.user_strings {
      user_strings = UserStringsHeap::parse(buf, &header)?;
    }

    Ok(Self {
      version: header.version,
      blobs,
      guids,
      tables,
      strings,
      user_strings,
    })
  }
}
