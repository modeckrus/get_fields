#![allow(dead_code)]

use get_fields::GetFields;

#[derive(GetFields)]
struct Test {
  f1: String,
  f2: i64,
  f3: String,
  f4: bool,
}

#[derive(GetFields)]
struct TestGenerics<A, B, C> {
  foo: A,
  bar: B,
  baz: C,
}

#[derive(GetFields)]
struct TestSkip {
  a: String,
  b: String,
  #[get_fields(skip)]
  c: String,
}

#[test]
fn test_struct() {
  assert_eq!(Test::get_fields, ["f1", "f2", "f3", "f4"]);
}

#[test]
fn test_generics_struct() {
  assert_eq!(
    TestGenerics::<u8, u8, u8>::get_fields,
    ["foo", "bar", "baz"],
  );
}

#[test]
fn test_skip() {
  assert_eq!(TestSkip::get_fields, ["a", "b"]);
}