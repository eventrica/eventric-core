//! The [`vec`][vec] module contains validators which apply to the  [`Vec<T>`]
//! type.
//!
//! [vec]: self

use crate::validation::Validator;

/// Validates that a vector is not empty.
pub struct IsEmpty;

impl<T> Validator<Vec<T>> for IsEmpty {
    fn validate(&self, value: &Vec<T>) -> Option<&str> {
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
        vec::IsEmpty,
    };

    // Is Empty

    #[test]
    fn is_empty_valid_with_integers() {
        let validator = IsEmpty;
        let value = vec![1, 2, 3];

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn is_empty_invalid() {
        let validator = IsEmpty;
        let value: Vec<i32> = Vec::new();

        assert_some_eq!(Some("empty"), validator.validate(&value));
    }

    #[test]
    fn is_empty_valid_with_strings() {
        let validator = IsEmpty;
        let value = vec![String::from("hello")];

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn is_empty_valid_single_element() {
        let validator = IsEmpty;
        let value = vec![42];

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn is_empty_invalid_after_clear() {
        let validator = IsEmpty;
        let mut value = vec![1, 2, 3];

        value.clear();

        assert_some_eq!(Some("empty"), validator.validate(&value));
    }
}
