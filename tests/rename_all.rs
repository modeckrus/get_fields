#![allow(dead_code)]

use get_fields::GetFields;

#[derive(GetFields)]
#[get_fields(rename_all = "lowercase")]
struct RenameLowercase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_lowercase() {
  assert_eq!(
    RenameLowercase::get_fields,
    ["field_one", "field_two", "field_three"],
  );
}

#[derive(GetFields)]
#[get_fields(rename_all = "UPPERCASE")]
struct RenameUppercase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_uppercase() {
  assert_eq!(
    RenameUppercase::get_fields,
    ["FIELD_ONE", "FIELD_TWO", "FIELD_THREE"],
  );
}

#[derive(GetFields)]
#[get_fields(rename_all = "PascalCase")]
struct RenamePascalCase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_pascal_case() {
  assert_eq!(
    RenamePascalCase::get_fields,
    ["FieldOne", "FieldTwo", "FieldThree"],
  );
}

#[derive(GetFields)]
#[get_fields(rename_all = "camelCase")]
struct RenameCamelCase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_camel_case() {
  assert_eq!(
    RenameCamelCase::get_fields,
    ["fieldOne", "fieldTwo", "fieldThree"],
  );
}

#[derive(GetFields)]
#[get_fields(rename_all = "snake_case")]
struct RenameSnakeCase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_snake_case() {
  assert_eq!(
    RenameSnakeCase::get_fields,
    ["field_one", "field_two", "field_three"],
  );
}

#[derive(GetFields)]
#[get_fields(rename_all = "SCREAMING_SNAKE_CASE")]
struct RenameScreamingSnakeCase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_screaming_snake_case() {
  assert_eq!(
    RenameScreamingSnakeCase::get_fields,
    ["FIELD_ONE", "FIELD_TWO", "FIELD_THREE"],
  );
}

#[derive(GetFields)]
#[get_fields(rename_all = "kebab-case")]
struct RenameKebabCase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_kebab_case() {
  assert_eq!(
    RenameKebabCase::get_fields,
    ["field-one", "field-two", "field-three"],
  );
}

#[derive(GetFields)]
#[get_fields(rename_all = "SCREAMING-KEBAB-CASE")]
struct RenameScreamingKebabCase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_screaming_kebab_case() {
  assert_eq!(
    RenameScreamingKebabCase::get_fields,
    ["FIELD-ONE", "FIELD-TWO", "FIELD-THREE"],
  );
}