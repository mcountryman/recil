use super::row;
use crate::emca335::{coded_index::HasFieldMarshal, streams::blobs::BlobId};

row! {
  pub struct FieldMarshalRow : 0x0d {
    parent: HasFieldMarshal,
    native_type: BlobId
  }
}
