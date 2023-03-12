mod common;

use common::{assert_guid_eq, assert_string_eq, first_row};
use recil::ecma335::Md;

#[test]
fn modules() {
  let pe = include_bytes!("./inputs/Newtonsoft.Json.dll");
  let md = Md::parse_from_pe(pe).unwrap().unwrap();

  assert_eq!(md.tables().modules().len(), 1);

  let m = first_row(md.tables().modules());

  assert_eq!(m.generation, 0);
  assert_string_eq(&md, m.name, "Newtonsoft.Json.dll");
  assert_guid_eq(&md, m.mvid, "4A5B913F-2570-4581-8A19-DC2CAD19C900");
  assert_guid_eq(&md, m.enc_id, "234A5B91-3F25-7045-818A-19DC2CAD1900");
  assert_guid_eq(&md, m.enc_base_id, "234A5B91-3F25-7045-818A-19DC2CAD1900");
}
