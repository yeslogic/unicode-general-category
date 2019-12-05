unicode-general-category
========================

<div align="center">
  <a href="https://travis-ci.com/yeslogic/unicode-general-category">
    <img src="https://travis-ci.com/yeslogic/unicode-general-category.svg?branch=master" alt="Build Status"></a>
  <a href="https://docs.rs/unicode-general-category">
    <img src="https://docs.rs/unicode-general-category/badge.svg" alt="Documentation">
  </a>
  <a href="https://crates.io/crates/unicode-general-category">
    <img src="https://img.shields.io/crates/v/unicode-general-category.svg" alt="Version">
  </a>
  <a href="https://github.com/yeslogic/unicode-general-category/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/unicode-general-category.svg" alt="License">
  </a>
</div>

<br>

Fast lookup of the Unicode General Category property for `char` in Rust.

Usage
-----

`Cargo.toml`:

```toml
[dependencies]
unicode-general-category = "0.1.0"
```

`main.rs`:

```rust
use unicode_general_category::{get_general_category, GeneralCategory};

fn main() {
    assert_eq!(get_general_category('A'), GeneralCategory::UppercaseLetter);
}
```

Implementation Notes
--------------------

[ucd-generate] is used to generate `tables.rs`. A build script (`build.rs`)
compiles this into a two level look up table. The look up time is constant as it
is just indexing into two arrays.

[ucd-generate]: https://github.com/BurntSushi/ucd-generate
