use super::row;
use crate::emca335::{
  coded_index::MemberRefParent, streams::blobs::BlobId, streams::strings::StringId,
};

row! {
  /// Contains a reference to a member of a type.
  pub struct MemberRefRow : 0x0a {
    class: MemberRefParent,
    name: StringId,
    signature: BlobId
  }
}
