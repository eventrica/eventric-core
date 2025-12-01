//! The [`string`][string] module contains validators which apply to the
//! [`String`] type.
//!
//! [string]: self

use crate::validation::Validator;

// =================================================================================================
// String
// =================================================================================================

/// Validates that a string does not contain control characters.
pub struct ControlCharacters;

impl Validator<String> for ControlCharacters {
    fn validate(&self, value: &String) -> Option<&str> {
        value
            .chars()
            .any(char::is_control)
            .then_some("control characters")
    }
}

/// Validates that a string is not empty.
pub struct IsEmpty;

impl Validator<String> for IsEmpty {
    fn validate(&self, value: &String) -> Option<&str> {
        value.is_empty().then_some("empty")
    }
}

/// Validates that a string does not contain preceding whitespace.
pub struct PrecedingWhitespace;

impl Validator<String> for PrecedingWhitespace {
    fn validate(&self, value: &String) -> Option<&str> {
        value
            .starts_with(char::is_whitespace)
            .then_some("preceding whitespace")
    }
}

/// Validates that a string does not contain trailing whitespace.
pub struct TrailingWhitespace;

impl Validator<String> for TrailingWhitespace {
    fn validate(&self, value: &String) -> Option<&str> {
        value
            .ends_with(char::is_whitespace)
            .then_some("trailing whitespace")
    }
}

// -------------------------------------------------------------------------------------------------

// Tests

#[cfg(test)]
mod tests {
    use assertables::{
        assert_none,
        assert_some_eq,
    };

    use crate::validation::{
        Validator as _,
        string::{
            ControlCharacters,
            IsEmpty,
            PrecedingWhitespace,
            TrailingWhitespace,
        },
    };

    // Control Characters

    #[test]
    fn control_characters_valid() {
        let validator = ControlCharacters;
        let value = String::from("Hello World");

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn control_characters_with_newline() {
        let validator = ControlCharacters;
        let value = String::from("Hello\nWorld");

        assert_some_eq!(Some("control characters"), validator.validate(&value));
    }

    #[test]
    fn control_characters_with_tab() {
        let validator = ControlCharacters;
        let value = String::from("Hello\tWorld");

        assert_some_eq!(Some("control characters"), validator.validate(&value));
    }

    #[test]
    fn control_characters_with_carriage_return() {
        let validator = ControlCharacters;
        let value = String::from("Hello\rWorld");

        assert_some_eq!(Some("control characters"), validator.validate(&value));
    }

    #[test]
    fn control_characters_with_null() {
        let validator = ControlCharacters;
        let value = String::from("Hello\0World");

        assert_some_eq!(Some("control characters"), validator.validate(&value));
    }

    #[test]
    fn control_characters_empty_string() {
        let validator = ControlCharacters;
        let value = String::new();

        assert_none!(validator.validate(&value));
    }

    // Is Empty

    #[test]
    fn is_empty_valid() {
        let validator = IsEmpty;
        let value = String::from("Hello");

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn is_empty_invalid() {
        let validator = IsEmpty;
        let value = String::new();

        assert_some_eq!(Some("empty"), validator.validate(&value));
    }

    #[test]
    fn is_empty_whitespace_only() {
        let validator = IsEmpty;
        let value = String::from("   ");

        assert_none!(validator.validate(&value));
    }

    // Preceding Whitespace

    #[test]
    fn preceding_whitespace_valid() {
        let validator = PrecedingWhitespace;
        let value = String::from("Hello World");

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn preceding_whitespace_with_space() {
        let validator = PrecedingWhitespace;
        let value = String::from(" Hello");

        assert_some_eq!(Some("preceding whitespace"), validator.validate(&value));
    }

    #[test]
    fn preceding_whitespace_with_tab() {
        let validator = PrecedingWhitespace;
        let value = String::from("\tHello");

        assert_some_eq!(Some("preceding whitespace"), validator.validate(&value));
    }

    #[test]
    fn preceding_whitespace_with_newline() {
        let validator = PrecedingWhitespace;
        let value = String::from("\nHello");

        assert_some_eq!(Some("preceding whitespace"), validator.validate(&value));
    }

    #[test]
    fn preceding_whitespace_empty_string() {
        let validator = PrecedingWhitespace;
        let value = String::new();

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn preceding_whitespace_trailing_only() {
        let validator = PrecedingWhitespace;
        let value = String::from("Hello ");

        assert_none!(validator.validate(&value));
    }

    // Trailing Whitespace

    #[test]
    fn trailing_whitespace_valid() {
        let validator = TrailingWhitespace;
        let value = String::from("Hello World");

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn trailing_whitespace_with_space() {
        let validator = TrailingWhitespace;
        let value = String::from("Hello ");

        assert_some_eq!(Some("trailing whitespace"), validator.validate(&value));
    }

    #[test]
    fn trailing_whitespace_with_tab() {
        let validator = TrailingWhitespace;
        let value = String::from("Hello\t");

        assert_some_eq!(Some("trailing whitespace"), validator.validate(&value));
    }

    #[test]
    fn trailing_whitespace_with_newline() {
        let validator = TrailingWhitespace;
        let value = String::from("Hello\n");

        assert_some_eq!(Some("trailing whitespace"), validator.validate(&value));
    }

    #[test]
    fn trailing_whitespace_empty_string() {
        let validator = TrailingWhitespace;
        let value = String::new();

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn trailing_whitespace_preceding_only() {
        let validator = TrailingWhitespace;
        let value = String::from(" Hello");

        assert_none!(validator.validate(&value));
    }
}
