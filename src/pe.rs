//! Portable executable parsing.

use crate::ecma335::Md;
use anyhow::{anyhow, Result};
use goblin::pe::{
  data_directories::DataDirectory, optional_header::OptionalHeader, options::ParseOptions,
  utils::find_offset, PE,
};
use scroll::Pread;

/// A macro to return `Ok(None)` if the expression is `None`.
macro_rules! try_option {
  ($e:expr) => {
    match $e {
      Some(e) => e,
      None => return Ok(None),
    }
  };
}

impl<'a> Md<'a> {
  pub fn parse_from_pe(buf: &'a [u8]) -> Result<Option<Self>> {
    // todo: don't panic.  it'll be okay.
    let pe = PE::parse(buf).unwrap();
    let optional_header = try_option!(pe.header.optional_header);

    let cor20 = try_option!(optional_header.data_directories.get_clr_runtime_header());
    let cor20 = read_data_directory(&pe, &optional_header, cor20, buf)?;
    let cor20 = cor20.pread::<Cor20Header>(0)?;

    let metadata = read_data_directory(&pe, &optional_header, &cor20.metadata, buf)?;

    Ok(Some(Self::from_cli_data(metadata)?))
  }
}

fn read_data_directory<'a>(
  pe: &PE,
  optional_header: &OptionalHeader,
  dir: &DataDirectory,
  buf: &'a [u8],
) -> Result<&'a [u8]> {
  let size = dir.size as usize;
  let offset = find_offset(
    dir.virtual_address as usize,
    &pe.sections,
    optional_header.windows_fields.file_alignment,
    &ParseOptions::default(),
  )
  .ok_or_else(|| anyhow!("Cannot find data directory"))?;

  Ok(buf.pread_with(offset, size)?)
}

#[derive(Copy, Clone, Debug, Pread)]
pub struct Cor20Header {
  /// The size of the header, currently 72.
  pub cb: u32,
  /// The minimum version of the runtime required to run this program, currently 2.
  pub major_runtime_version: u16,
  /// The minor portion of the version, currently 0.
  pub minor_runtime_version: u16,
  /// RVA and size of the physical metadata.
  pub metadata: DataDirectory,
  /// Flags describing this runtime image.
  pub flags: CorMetaFlags,
  /// Token for the MethodDef or File of the entry point for the image.
  pub entry_point_token: u32,
  /// RVA and size of implementation-specific resources.
  pub resources: DataDirectory,
  /// RVA of the hash data for this PE file used by the CLI loader for binding and versioning.
  pub strong_name_signature: DataDirectory,
  /// Always 0.
  pub code_manager_table: DataDirectory,
  /// RVA of an array of locations in the file that contain an array of function pointers (e.g., vtable slots).
  pub vtable_fixups: DataDirectory,
  /// Always 0.
  pub export_address_table_jumps: DataDirectory,
  /// Always 0.
  pub managed_native_header: DataDirectory,
}

bitflags::bitflags! {
  #[derive(Pread)]
  pub struct CorMetaFlags: u32 {
    const COMIMAGE_FLAGS_ILONLY = 0x00000001;
    const COMIMAGE_FLAGS_32BITREQUIRED = 0x00000002;
    const COMIMAGE_FLAGS_STRONGNAMESIGNED = 0x00000008;
    const COMIMAGE_FLAGS_NATIVE_ENTRYPOINT = 0x00000010;
    const COMIMAGE_FLAGS_TRACKDEBUGDATA = 0x00010000;
  }
}
