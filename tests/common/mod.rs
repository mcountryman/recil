use guid_create::GUID;
use recil::ecma335::{
  guids::GuidIndex,
  strings::StringIndex,
  tables::{Row, TableRowReader},
  Md,
};

/// Gets the first row from the given [TableRowReader].
pub fn first_row<'a, R: Row<'a>>(table: TableRowReader<'a, '_, R>) -> R {
  table.into_iter().next().unwrap().unwrap()
}

/// Asserts that the given [actual] guid is equal to the given [expected] guid.
pub fn assert_guid_eq(md: &Md<'_>, actual: GuidIndex, expected: &str) {
  let actual = md.guids().get(actual).expect("Failed to read guid");

  assert_eq!(GUID::build_from_slice(&actual).to_string(), expected);
}

/// Asserts that the given [actual] str is equal to the given [expected] str.
pub fn assert_string_eq(md: &Md<'_>, actual: StringIndex, expected: &str) {
  let actual = md.strings().get(actual).expect("Failed to read string");

  assert_eq!(actual, expected);
}
