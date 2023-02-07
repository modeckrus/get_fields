# get_fields


Provides the `GetFields` procedural macro.
The macro adds the `get_fields` constant to the struct the
macro is dervied on.
The `get_fields` contains the field names of the given 
struct.

**Note:** The macro can only be derived from named structs.

## Table of Contents

<!--ts-->
   * [Usage](#usage)
   * [Attributes](#attributes)
      * [Container Attributes](#container-attributes)
         * [Rename all](#rename-all)
      * [Field Attributes](#field-attributes)
         * [Skip](#skip)
   * [Visibility](#visibility)
<!--te-->

## Usage

You can derive the `GetFields` macro like this:

```rust
use get_fields::GetFields;

#[derive(GetFields)]
struct Foo {
  bar: String,
  baz: String,
  bat: String,
}

assert_eq!(Foo::get_fields, ["bar", "baz", "bat"]);
```

## Attributes

The `GetFields` macro supports the
`get_fields` attribute.
`get_fields` can be applied to the container or to a field
with different arguments listed below.

### Container Attributes

Container attributes are global attributes that change the behavior
of the whole field names array, rather than that of a single field.

#### Rename all

The `rename_all` attribute renames every field of the struct according
to the provided naming convention.
This attribute works exactly like the [serde][serde_rename_all]
equivalent.
Supported are these naming conventions:
  - `lowercase`
  - `UPPERCASE`
  - `PascalCase`
  - `camelCase`
  - `snake_case`
  - `SCREAMING_SNAKE_CASE`
  - `kebab-case`
  - `SCREAMING-KEBAB-CASE`

```rust
use get_fields::GetFields;

#[derive(GetFields)]
#[get_fields(rename_all = "SCREAMING-KEBAB-CASE")]
struct Foo {
  field_one: String,
  field_two: String,
  field_three: String,
}

assert_eq!(
  Foo::get_fields, 
  ["FIELD-ONE", "FIELD-TWO", "FIELD-THREE"],
);
```

**Note:** Same as serde's implementation of `rename_all`, it is
assumed that your field names follow the rust naming convention, that 
all field names must be given in `snake_case`.
If not, applying `rename_all` may result in unexpected field names.

### Field Attributes

Field attributes can be added to the fields of a named struct and 
change the behavior of a single field.

#### Skip

The `skip` attribute removes the field from `get_fields`.

```rust
use get_fields::GetFields;

#[derive(GetFields)]
struct Foo {
  bar: String,
  baz: String,
  #[get_fields(skip)]
  bat: String,
}

assert_eq!(Foo::get_fields, ["bar", "baz"]);
```

## Visibility

The visibility of the `get_fields` is the same as the
corresponding struct.
E.g. is it `pub struct Foo { ... }`, the `get_fields`
will be public as well.
This, for example, will work:

```rust
mod foo {
  use get_fields::GetFields;

  #[derive(GetFields)]
  pub(super) struct Foo {
    bar: String,
    baz: String,
    bat: String,
  }
}

assert_eq!(foo::Foo::get_fields, ["bar", "baz", "bat"]);
```

Whereas this will not, since `get_fields` is private:

```compile_fail
mod foo {
  use get_fields::GetFields;

  #[derive(GetFields)]
  struct Foo {
    bar: String,
    baz: String,
    bat: String,
  }
}

assert_eq!(foo::Foo::get_fields, ["bar", "baz", "bat"]);
```

[serde_rename_all]: https://serde.rs/container-attrs.html#rename_all
