//! The [`vec`][vec] module contains validators which apply to the  [`Vec<T>`]
//! type.
//!
//! [vec]: self

use std::collections::HashSet;

use crate::validation::Validator;

/// Validates that a vector is not empty.
pub struct IsEmpty;

impl<T> Validator<HashSet<T>> for IsEmpty {
    fn validate(&self, value: &HashSet<T>) -> Option<&str> {
        value.is_empty().then_some("empty")
    }
}

// -------------------------------------------------------------------------------------------------

// Tests

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use assertables::{
        assert_none,
        assert_some_eq,
    };

    use crate::validation::{
        Validator as _,
        hashset::IsEmpty,
    };

    // Is Empty

    #[test]
    fn is_empty_valid_with_integers() {
        let validator = IsEmpty;
        let value = HashSet::from_iter([1, 2, 3]);

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn is_empty_invalid() {
        let validator = IsEmpty;
        let value: HashSet<i32> = HashSet::new();

        assert_some_eq!(Some("empty"), validator.validate(&value));
    }

    #[test]
    fn is_empty_valid_with_strings() {
        let validator = IsEmpty;
        let value = HashSet::from_iter([String::from("hello")]);

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn is_empty_valid_single_element() {
        let validator = IsEmpty;
        let value = HashSet::from_iter([42]);

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn is_empty_invalid_after_clear() {
        let validator = IsEmpty;
        let mut value = HashSet::from_iter([1, 2, 3]);

        value.clear();

        assert_some_eq!(Some("empty"), validator.validate(&value));
    }
}
