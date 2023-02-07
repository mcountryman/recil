use super::row;
use crate::{
  emca335::{streams::blobs::BlobId, streams::strings::StringId},
  error::Error,
};
use scroll::{
  ctx::{SizeWith, TryFromCtx},
  Pread, SizeWith,
};

row! {
  pub struct AssemblyRow : 0x20 {
    hash_alg: AssemblyHashAlgorithm,
    major_version: u16,
    minor_version: u16,
    build_number: u16,
    revision_number: u16,
    flags: AssemblyFlags,
    public_key: BlobId,
    name: StringId,
    culture: StringId
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssemblyHashAlgorithm {
  None = 0x0000,
  MD5 = 0x8003,
  SHA1 = 0x8004,
  SHA256 = 0x800C,
  SHA384 = 0x800D,
  SHA512 = 0x800E,
}

bitflags::bitflags! {
  #[derive(Pread, SizeWith)]
  pub struct AssemblyFlags : u32 {
    /// The assembly reference holds the full (unhashed) public key.
    const PUBLIC_KEY = 0x0001;
    /// The implementation of this assembly used at runtime is not expected to match the version
    /// seen at compile time.
    const RETARGETABLE =  0x0100;
    /// Reserved (a conforming implementation of the CLI can ignore this setting on read; some
    /// implementations might use this bit to indicate that a CIL-to-native-code compiler should not
    /// should not generate optimized code)
    const DISABLE_JIT_COMPILE_OPTIMIZER = 0x4000;
    /// Reserved (a conforming implementation of the CLI can ignore this setting on read; some
    /// implementations might use this bit to indicate that a CIL-to-native-code compiler should
    /// generate CIL-to-native code map)
    const ENABLE_JIT_COMPILE_TRACKING = 0x8000;
  }
}

impl From<u32> for AssemblyHashAlgorithm {
  fn from(value: u32) -> Self {
    match value {
      0x0000 => Self::None,
      0x8003 => Self::MD5,
      0x8004 => Self::SHA1,
      0x800C => Self::SHA256,
      0x800D => Self::SHA384,
      0x800E => Self::SHA512,
      _ => Self::None,
    }
  }
}

impl Default for AssemblyHashAlgorithm {
  fn default() -> Self {
    Self::None
  }
}

impl TryFromCtx<'_> for AssemblyHashAlgorithm {
  type Error = Error;

  fn try_from_ctx(value: &[u8], _: ()) -> Result<(Self, usize), Self::Error> {
    Ok((Self::from(value.pread::<u32>(0)?), 4))
  }
}

impl SizeWith for AssemblyHashAlgorithm {
  fn size_with(_: &()) -> usize {
    4
  }
}
