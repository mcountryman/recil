use super::row;
use crate::emca335::{coded_index::HasDeclSecurity, streams::blobs::BlobId};

row! {
  pub struct DeclSecurityRow : 0x0e {
    action: u16,
    parent: HasDeclSecurity,
    permission_set: BlobId
  }
}
