//! The [`array`][array] module contains validators which apply to the `[T; N]`
//! type.
//!
//! [vec]: self

use crate::validation::Validator;

/// Validates that a vector is not empty.
pub struct IsEmpty;

impl<T, const N: usize> Validator<[T; N]> for IsEmpty {
    fn validate(&self, value: &[T; N]) -> Option<&str> {
        value.is_empty().then_some("empty")
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
        array::IsEmpty,
    };

    // Is Empty

    #[test]
    fn is_empty_valid_with_integers() {
        let validator = IsEmpty;
        let value = [1, 2, 3];

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn is_empty_invalid() {
        let validator = IsEmpty;
        let value: [i32; 0] = [];

        assert_some_eq!(Some("empty"), validator.validate(&value));
    }

    #[test]
    fn is_empty_valid_with_strings() {
        let validator = IsEmpty;
        let value = [String::from("hello")];

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn is_empty_valid_single_element() {
        let validator = IsEmpty;
        let value = [42];

        assert_none!(validator.validate(&value));
    }
}
