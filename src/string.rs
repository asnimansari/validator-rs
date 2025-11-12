//! String validation functions

/// Validates if a string contains only alphanumeric characters
///
/// # Examples
///
/// ```
/// use validator_rs::string::is_alphanumeric;
///
/// assert!(is_alphanumeric("abc123"));
/// assert!(!is_alphanumeric("abc-123"));
/// ```
pub fn is_alphanumeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric())
}

/// Validates if a string contains only alphabetic characters
///
/// # Examples
///
/// ```
/// use validator_rs::string::is_alpha;
///
/// assert!(is_alpha("abcXYZ"));
/// assert!(!is_alpha("abc123"));
/// ```
pub fn is_alpha(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphabetic())
}

/// Validates if a string contains only numeric characters
///
/// # Examples
///
/// ```
/// use validator_rs::string::is_numeric;
///
/// assert!(is_numeric("12345"));
/// assert!(!is_numeric("123.45"));
/// ```
pub fn is_numeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

/// Validates if a string has a minimum length
pub fn has_min_length(s: &str, min: usize) -> bool {
    s.len() >= min
}

/// Validates if a string has a maximum length
pub fn has_max_length(s: &str, max: usize) -> bool {
    s.len() <= max
}

/// Validates if a string length is within a range
pub fn has_length_between(s: &str, min: usize, max: usize) -> bool {
    let len = s.len();
    len >= min && len <= max
}

/// Validates if a string contains a substring
pub fn contains(s: &str, pattern: &str) -> bool {
    s.contains(pattern)
}

/// Validates if a string matches a specific pattern (case-insensitive)
pub fn contains_case_insensitive(s: &str, pattern: &str) -> bool {
    s.to_lowercase().contains(&pattern.to_lowercase())
}

/// Validates if a string is uppercase
pub fn is_uppercase(s: &str) -> bool {
    !s.is_empty() && s.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase())
}

/// Validates if a string is lowercase
pub fn is_lowercase(s: &str) -> bool {
    !s.is_empty() && s.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphanumeric() {
        assert!(is_alphanumeric("abc123"));
        assert!(is_alphanumeric("ABC"));
        assert!(!is_alphanumeric("abc-123"));
        assert!(!is_alphanumeric(""));
    }

    #[test]
    fn test_alpha() {
        assert!(is_alpha("abcXYZ"));
        assert!(!is_alpha("abc123"));
        assert!(!is_alpha(""));
    }

    #[test]
    fn test_numeric() {
        assert!(is_numeric("12345"));
        assert!(!is_numeric("123.45"));
        assert!(!is_numeric("abc"));
        assert!(!is_numeric(""));
    }

    #[test]
    fn test_length_validations() {
        assert!(has_min_length("hello", 3));
        assert!(!has_min_length("hi", 3));
        
        assert!(has_max_length("hi", 5));
        assert!(!has_max_length("hello world", 5));
        
        assert!(has_length_between("hello", 3, 10));
        assert!(!has_length_between("hi", 3, 10));
    }

    #[test]
    fn test_contains() {
        assert!(contains("hello world", "world"));
        assert!(!contains("hello world", "foo"));
        
        assert!(contains_case_insensitive("Hello World", "world"));
        assert!(contains_case_insensitive("HELLO", "hello"));
    }

    #[test]
    fn test_case() {
        assert!(is_uppercase("HELLO"));
        assert!(is_uppercase("HELLO123"));
        assert!(!is_uppercase("Hello"));
        
        assert!(is_lowercase("hello"));
        assert!(is_lowercase("hello123"));
        assert!(!is_lowercase("Hello"));
    }
}

