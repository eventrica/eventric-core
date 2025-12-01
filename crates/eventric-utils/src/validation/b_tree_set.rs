//! The [`vec`][vec] module contains validators which apply to the  [`Vec<T>`]
//! type.
//!
//! [vec]: self

use std::collections::BTreeSet;

use crate::validation::Validator;

/// Validates that a vector is not empty.
pub struct IsEmpty;

impl<T> Validator<BTreeSet<T>> for IsEmpty {
    fn validate(&self, value: &BTreeSet<T>) -> Option<&str> {
        value.is_empty().then_some("empty")
    }
}

// -------------------------------------------------------------------------------------------------

// Tests

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use assertables::{
        assert_none,
        assert_some_eq,
    };

    use crate::validation::{
        Validator as _,
        b_tree_set::IsEmpty,
    };

    // Is Empty

    #[test]
    fn is_empty_valid_with_integers() {
        let validator = IsEmpty;
        let value = BTreeSet::from_iter([1, 2, 3]);

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn is_empty_invalid() {
        let validator = IsEmpty;
        let value: BTreeSet<i32> = BTreeSet::new();

        assert_some_eq!(Some("empty"), validator.validate(&value));
    }

    #[test]
    fn is_empty_valid_with_strings() {
        let validator = IsEmpty;
        let value = BTreeSet::from_iter([String::from("hello")]);

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn is_empty_valid_single_element() {
        let validator = IsEmpty;
        let value = BTreeSet::from_iter([42]);

        assert_none!(validator.validate(&value));
    }

    #[test]
    fn is_empty_invalid_after_clear() {
        let validator = IsEmpty;
        let mut value = BTreeSet::from_iter([1, 2, 3]);

        value.clear();

        assert_some_eq!(Some("empty"), validator.validate(&value));
    }
}
