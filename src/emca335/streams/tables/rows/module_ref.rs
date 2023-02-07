use super::row;
use crate::emca335::streams::strings::StringId;

row! {
  pub struct ModuleRefRow : 0x1a {
    name: StringId
  }
}
