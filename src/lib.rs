//! # Validator-rs
//!
//! A comprehensive validation library for Rust that provides various validator functions
//! for common data types and formats.
//!
//! ## Usage
//!
//! ```rust
//! use validator_rs::email::is_valid_email;
//! use validator_rs::url::is_valid_url;
//!
//! assert!(is_valid_email("test@example.com"));
//! assert!(is_valid_url("https://www.example.com"));
//! ```

// Export all validator modules
pub mod credit_card;
pub mod currency;
pub mod date;
pub mod email;
pub mod mobile;
pub mod numeric;
pub mod string;
pub mod url;

// Re-export commonly used validators for convenience
pub use credit_card::is_valid_credit_card;
pub use currency::is_currency;
pub use date::is_valid_date;
pub use email::is_valid_email;
pub use mobile::is_valid_phone;
pub use numeric::{is_in_range, is_negative, is_positive};
pub use string::{is_alpha, is_alphanumeric, is_numeric};
pub use url::is_valid_url;

/// Common result type used across validators
pub type ValidationResult = Result<(), ValidationError>;

/// Error type for validation failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validation error for '{}': {}", self.field, self.message)
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error() {
        let error = ValidationError::new("email", "Invalid email format");
        assert_eq!(error.field, "email");
        assert_eq!(error.message, "Invalid email format");
    }
}
