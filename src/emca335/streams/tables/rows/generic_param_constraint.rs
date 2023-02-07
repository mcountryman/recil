use super::{generic_param::GenericParamRowId, row};
use crate::emca335::coded_index::TypeDefOrRef;

row! {
  pub struct GenericParamConstraintRow : 0x2c {
    owner: GenericParamRowId,
    constraint: TypeDefOrRef
  }
}
