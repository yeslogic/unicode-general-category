//! Look up the general category for a character.
//!
//! ### Example
//!
//! ```
//! use unicode_general_category::{get_general_category, GeneralCategory};
//!
//! assert_eq!(get_general_category('A'), GeneralCategory::UppercaseLetter);
//! ```

#![no_std]

mod category;
mod tables;
pub use category::get_general_category;
pub use tables::GeneralCategory;

/// The version of [Unicode](http://www.unicode.org/)
/// that this version of unicode-general-category was generated from.
pub const UNICODE_VERSION: (u64, u64, u64) = (13, 0, 0);

#[cfg(test)]
mod test {
    use super::{get_general_category, GeneralCategory};

    #[test]
    fn test_get_category() {
        assert_eq!(get_general_category('a'), GeneralCategory::LowercaseLetter);
        assert_eq!(get_general_category('.'), GeneralCategory::OtherPunctuation);
        assert_eq!(get_general_category('カ'), GeneralCategory::OtherLetter);
        assert_eq!(get_general_category('🦳'), GeneralCategory::OtherSymbol);
    }
}
