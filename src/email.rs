//! Email validation functions

use regex::Regex;
use std::sync::OnceLock;

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_email_regex() -> &'static Regex {
    EMAIL_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$")
            .expect("Failed to compile email regex")
    })
}

/// Validates if a string is a valid email address
///
/// # Examples
///
/// ```
/// use validator_rs::email::is_valid_email;
///
/// assert!(is_valid_email("user@example.com"));
/// assert!(is_valid_email("test.email+tag@domain.co.uk"));
/// assert!(!is_valid_email("invalid.email"));
/// assert!(!is_valid_email("@example.com"));
/// ```
pub fn is_valid_email(email: &str) -> bool {
    if email.is_empty() || email.len() > 254 {
        return false;
    }

    get_email_regex().is_match(email)
}

/// Validates if a string is a valid email address and checks domain
pub fn is_valid_email_with_domain(email: &str, allowed_domains: &[&str]) -> bool {
    if !is_valid_email(email) {
        return false;
    }

    if let Some(domain) = email.split('@').nth(1) {
        allowed_domains.iter().any(|&d| domain == d)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_emails() {
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("test.email@domain.com"));
        assert!(is_valid_email("user+tag@example.co.uk"));
        assert!(is_valid_email("a@b.c"));
    }

    #[test]
    fn test_invalid_emails() {
        assert!(!is_valid_email(""));
        assert!(!is_valid_email("invalid"));
        assert!(!is_valid_email("@example.com"));
        assert!(!is_valid_email("user@"));
        assert!(!is_valid_email("user @example.com"));
        assert!(!is_valid_email("user@.com"));
    }

    #[test]
    fn test_email_with_domain() {
        assert!(is_valid_email_with_domain(
            "user@example.com",
            &["example.com", "test.com"]
        ));
        assert!(!is_valid_email_with_domain(
            "user@other.com",
            &["example.com", "test.com"]
        ));
    }
}
