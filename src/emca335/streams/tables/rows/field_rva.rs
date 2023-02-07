use super::{field::FieldRowId, row};

row! {
  pub struct FieldRvaRow : 0x1d {
    rva: u32,
    field: FieldRowId
  }
}
