use crate::emca335::streams::blobs::BlobId;

use super::row;

row! {
  pub struct TypeSpecRow : 0x1b {
    signature: BlobId
  }
}
