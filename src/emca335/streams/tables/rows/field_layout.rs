use super::{field::FieldRowId, row};

row! {
  pub struct FieldLayoutRow : 0x10 {
     offset: u32,
     field: FieldRowId
  }
}
