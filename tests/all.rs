use recil::ecma335::Md;

macro_rules! iter {
  ($md:ident.$table:ident()) => {
    for row in $md.tables().$table() {
      row.expect("Failed to read row");
    }
  };
  ($md:ident) => {
    iter!($md.assembly_oses());
    iter!($md.assembly_processors());
    iter!($md.assembly_refs());
    iter!($md.assembly_ref_oses());
    iter!($md.assembly_ref_processors());
    iter!($md.class_layouts());
    iter!($md.constants());
    iter!($md.custom_attributes());
    iter!($md.decl_securities());
    iter!($md.events());
    iter!($md.event_maps());
    iter!($md.exported_types());
    iter!($md.fields());
    iter!($md.field_layouts());
    iter!($md.field_marshals());
    iter!($md.field_rvas());
    iter!($md.files());
    iter!($md.generic_params());
    iter!($md.generic_param_constraints());
    iter!($md.impl_maps());
    iter!($md.interface_impls());
    iter!($md.manifest_resources());
    iter!($md.member_refs());
    iter!($md.method_defs());
    iter!($md.method_impls());
    iter!($md.method_semantics());
    iter!($md.method_specs());
    iter!($md.modules());
    iter!($md.module_refs());
    iter!($md.nested_classes());
    iter!($md.params());
    iter!($md.properties());
    iter!($md.property_maps());
    iter!($md.stand_alone_sigs());
    iter!($md.type_defs());
    iter!($md.type_refs());
    iter!($md.type_specs());
  };
}

#[test]
fn all() {
  let pe = include_bytes!("./inputs/Newtonsoft.Json.dll");
  let md = Md::parse_from_pe(pe).expect("Invalid module").unwrap();

  iter!(md);
}
