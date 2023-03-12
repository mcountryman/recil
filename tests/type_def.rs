mod common;

use recil::ecma335::Md;

#[test]
fn type_defs() {
  let pe = include_bytes!("./inputs/Newtonsoft.Json.dll");
  let md = Md::parse_from_pe(pe).unwrap().unwrap();

  for t in md.tables().type_defs() {
    t.unwrap();
  }
}
