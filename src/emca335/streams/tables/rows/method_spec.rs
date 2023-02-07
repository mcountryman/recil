use super::row;
use crate::emca335::{coded_index::MethodDefOrRef, streams::blobs::BlobId};

row! {
  pub struct MethodSpecRow : 0x2b {
    method: MethodDefOrRef,
    instantiation: BlobId
  }
}
