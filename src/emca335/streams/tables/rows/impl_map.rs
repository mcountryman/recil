use scroll::{Pread, SizeWith};

use super::{module_ref::ModuleRefRowId, row};
use crate::emca335::{coded_index::MemberForwarded, streams::strings::StringId};

row! {
  /// Holds information about un-managed methods that can be reached from managed code, using
  /// PInvoke dispatch.
  pub struct ImplMapRow : 0x1c {
    mapping_flags: PInvokeAttributes,
    member_forwarded: MemberForwarded,
    import_name: StringId,
    import_scope: ModuleRefRowId
  }
}

bitflags::bitflags! {
  #[derive(Pread, SizeWith)]
  pub struct PInvokeAttributes : u16 {
    /// PInvoke is to use the member name as specified
    const NO_MANGLE = 0x0001;
    /// This is a resource file or other non-metadata-containing file.
    const CHAR_SET_MASK = 0x0006;
    const CHAR_SET_NOT_SPEC = 0x0000;
    const CHAR_SET_ANSI = 0x0002;
    const CHAR_SET_UNICODE = 0x0004;
    const CHAR_SET_AUTO = 0x0006;
  }
}
