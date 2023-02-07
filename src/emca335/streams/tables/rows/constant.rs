use super::row;
use crate::emca335::{coded_index::HasConstant, element_type::ElementType, streams::blobs::BlobId};

row! {
  pub struct ConstantRow : 0x0B {
    kind: ElementType,
    _padding: u8,
    parent: HasConstant,
    value: BlobId
  }
}
