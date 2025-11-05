//! The [`validation`][validation] module contains validation traits and a
//! simple validation mechanism which can be straightforwardly extended. This is
//! not a complex or particularly powerful approach, but it is simple and free
//! of heavyweight dependencies like many validator implementations.
//!
//! [validation]: self

pub mod string;
pub mod vec;

use std::{
    error,
    fmt::Display,
};

use thiserror::Error;

// =================================================================================================
// Validation
// =================================================================================================

// Traits

/// Defines an implementation to be a validator of the given parameter `T`.
pub trait Validator<T> {
    /// Validates the given value, returning an optional error message if the
    /// validation criterion is not met.
    fn validate(&self, value: &T) -> Option<&str>;
}

/// Defines an implementation to be validatable, i.e. that it may or may not be
/// in a valid state.
pub trait Validate
where
    Self::Err: error::Error + From<Error>,
    Self: Sized,
{
    /// The error type to return from validation, which must be convertible from
    /// the standard validation [`Error`] type.
    type Err;

    /// Validate self, and return self if valid, or an error if not.
    ///
    /// # Errors
    ///
    /// Returns an error on validation fails, which should be the
    /// [`Error::Validation`] variant of the core error type.
    fn validate(self) -> Result<Self, Self::Err>;
}

// -------------------------------------------------------------------------------------------------

// Errors

/// The [`Error`] enumeration gives possible error cases when validation fails.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Error {
    /// The validation request failed with the supplied error message.
    #[error("Validation Error: {0}")]
    Invalid(String),
}

impl Error {
    /// Creates an [`Error::Invalid`] variant with the supplied error message.
    pub fn invalid<E>(error: E) -> Self
    where
        E: Into<String>,
    {
        Self::Invalid(error.into())
    }
}

// -------------------------------------------------------------------------------------------------

// Validate

/// Validates a given value, taking a provided name for any resulting error
/// value, and a collection of validators which can be applied to the given
/// instance.
///
/// # Errors
///
/// Returns an error when validation fails, produced by the first validator in
/// the given collection to produce an error result (the execution is
/// short-circuiting, subsequent validations will not be attempted after the
/// first failure).
pub fn validate<T, N>(value: &T, name: N, validators: &[&dyn Validator<T>]) -> Result<(), Error>
where
    N: Display,
{
    for validator in validators {
        if let Some(error) = validator.validate(value) {
            return Err(Error::invalid(format!("{name}: {error}")));
        }
    }

    Ok(())
}

// -------------------------------------------------------------------------------------------------

// Tests

#[cfg(test)]
mod tests {
    use assertables::assert_ok;

    use crate::validation::{
        Error,
        Validator,
        string::{
            ControlCharacters,
            IsEmpty as StringIsEmpty,
            PrecedingWhitespace,
        },
        validate,
        vec::IsEmpty as VecIsEmpty,
    };

    // Success Cases

    #[test]
    fn success_with_empty_validators() {
        let value = String::from("test");
        let validators: &[&dyn Validator<String>] = &[];

        assert_ok!(validate(&value, "field", validators));
    }

    #[test]
    fn success_with_single_validator() {
        let value = String::from("test");
        let validators: &[&dyn Validator<String>] = &[&StringIsEmpty];

        assert_ok!(validate(&value, "field", validators));
    }

    #[test]
    #[rustfmt::skip]
    fn success_with_multiple_validators() {
        let value = String::from("test");
        let validators: &[&dyn Validator<String>] = &[
            &StringIsEmpty,
            &ControlCharacters,
            &PrecedingWhitespace
        ];

        assert_ok!(validate(&value, "field", validators));
    }

    #[test]
    fn success_with_vec() {
        let value = vec![1, 2, 3];
        let validators: &[&dyn Validator<Vec<i32>>] = &[&VecIsEmpty];

        assert_ok!(validate(&value, "items", validators));
    }

    // Failure Cases

    #[test]
    fn failure_with_single_validator() {
        let value = String::new();
        let validators: &[&dyn Validator<String>] = &[&StringIsEmpty];

        assert_eq!(
            Error::invalid("username: empty"),
            validate(&value, "username", validators).unwrap_err(),
        );
    }

    #[test]
    fn failure_first_validator_fails() {
        let value = String::from(" test");
        let validators: &[&dyn Validator<String>] = &[&PrecedingWhitespace, &StringIsEmpty];

        assert_eq!(
            Error::invalid("name: preceding whitespace"),
            validate(&value, "name", validators).unwrap_err(),
        );
    }

    #[test]
    fn failure_second_validator_fails() {
        let value = String::from("test\n");
        let validators: &[&dyn Validator<String>] = &[&StringIsEmpty, &ControlCharacters];

        assert_eq!(
            Error::invalid("description: control characters"),
            validate(&value, "description", validators).unwrap_err(),
        );
    }

    #[test]
    fn failure_with_vec() {
        let value: Vec<i32> = Vec::new();
        let validators: &[&dyn Validator<Vec<i32>>] = &[&VecIsEmpty];

        assert_eq!(
            Error::invalid("tags: empty"),
            validate(&value, "tags", validators).unwrap_err(),
        );
    }

    // Error Message Formatting

    #[test]
    fn error_message_format_with_str_name() {
        let value = String::new();
        let validators: &[&dyn Validator<String>] = &[&StringIsEmpty];

        assert_eq!(
            Error::invalid("email: empty"),
            validate(&value, "email", validators).unwrap_err(),
        );
    }

    #[test]
    fn error_message_format_with_string_name() {
        let value = String::new();
        let validators: &[&dyn Validator<String>] = &[&StringIsEmpty];

        assert_eq!(
            Error::invalid("user_email: empty"),
            validate(&value, "user_email", validators).unwrap_err(),
        );
    }

    #[test]
    fn error_message_with_complex_name() {
        let value = String::new();
        let validators: &[&dyn Validator<String>] = &[&StringIsEmpty];

        assert_eq!(
            Error::invalid("user.profile.name: empty"),
            validate(&value, "user.profile.name", validators).unwrap_err(),
        );
    }

    // Short-Circuit Behavior

    #[test]
    fn short_circuits_on_first_failure() {
        let value = String::from("\n");
        let validators: &[&dyn Validator<String>] = &[&ControlCharacters, &StringIsEmpty];

        assert_eq!(
            Error::invalid("field: control characters"),
            validate(&value, "field", validators).unwrap_err(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn short_circuits_on_first_failure_with_multiple_potential_failures() {
        let value = String::from(" test\n");
        let validators: &[&dyn Validator<String>] = &[
            &PrecedingWhitespace,
            &ControlCharacters,
            &StringIsEmpty,
        ];

        assert_eq!(
            Error::invalid("input: preceding whitespace"),
            validate(&value, "input", validators).unwrap_err(),
        );
    }
}
