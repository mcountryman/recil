use recil::emca335::Metadata;

#[test]
fn test_read_net7() {
  let pe = include_bytes!("./inputs/Newtonsoft.Json.dll");
  let md = Metadata::from_pe(pe).expect("Invalid module").unwrap();

  for assembly_ref in md.tables.assembly_refs() {
    let assembly_ref = assembly_ref.expect("Failed to read AssemblyRefRow");
  }
}
