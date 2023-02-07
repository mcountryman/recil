use super::row;
use crate::emca335::{
  coded_index::{CustomAttributeType, HasCustomAttribute},
  streams::blobs::BlobId,
};

row! {
  pub struct CustomAttributeRow : 0x0C {
    parent: HasCustomAttribute,
    attribute_type: CustomAttributeType,
    value: BlobId
  }
}
