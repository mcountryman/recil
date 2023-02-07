use super::row;
use crate::emca335::streams::blobs::BlobId;

row! {
  pub struct StandAloneSigRow : 0x11 {
    signature: BlobId
  }
}
