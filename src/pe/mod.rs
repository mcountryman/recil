mod header;
mod util;

use self::{header::Cor20Header, util::try_option};
use crate::{emca335::Metadata, error::Error};
use goblin::pe::{
  data_directories::DataDirectory, optional_header::OptionalHeader, options::ParseOptions,
  utils::find_offset, PE,
};
use scroll::Pread;

impl<'a> Metadata<'a> {
  /// Parses the [Metadata] from the given buffer containing the bytes of a PE.
  pub fn from_pe(buf: &'a [u8]) -> Result<Option<Self>, Error> {
    let pe = PE::parse(buf)?;
    let optional_header = try_option!(pe.header.optional_header);

    let cor20 = try_option!(optional_header.data_directories.get_clr_runtime_header());
    let cor20 = read_data_directory(&pe, &optional_header, cor20, buf)?;
    let cor20 = cor20.pread::<Cor20Header>(0)?;

    let metadata = read_data_directory(&pe, &optional_header, &cor20.metadata, buf)?;

    Ok(Some(Self::parse(metadata)?))
  }
}

fn read_data_directory<'a>(
  pe: &PE,
  optional_header: &OptionalHeader,
  dir: &DataDirectory,
  buf: &'a [u8],
) -> Result<&'a [u8], Error> {
  let size = dir.size as usize;
  let offset = find_offset(
    dir.virtual_address as usize,
    &pe.sections,
    optional_header.windows_fields.file_alignment,
    &ParseOptions::default(),
  )
  .ok_or(Error::Malformed("Cannot find data directory"))?;

  Ok(buf.pread_with(offset, size)?)
}
