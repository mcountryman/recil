use goblin::pe::data_directories::DataDirectory;
use scroll::{Pread, Pwrite};

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
  #[derive(Pread, Pwrite)]
  pub struct CorMetaFlags: u32 {
    const COMIMAGE_FLAGS_ILONLY = 0x00000001;
    const COMIMAGE_FLAGS_32BITREQUIRED = 0x00000002;
    const COMIMAGE_FLAGS_STRONGNAMESIGNED = 0x00000008;
    const COMIMAGE_FLAGS_NATIVE_ENTRYPOINT = 0x00000010;
    const COMIMAGE_FLAGS_TRACKDEBUGDATA = 0x00010000;
  }
}
