//! URL validation functions

use regex::Regex;
use std::sync::OnceLock;

static URL_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_url_regex() -> &'static Regex {
    URL_REGEX.get_or_init(|| {
        Regex::new(r"^https?://[^\s/$.?#].[^\s]*$")
            .expect("Failed to compile URL regex")
    })
}

/// Validates if a string is a valid URL
///
/// # Examples
///
/// ```
/// use validator_rs::url::is_valid_url;
///
/// assert!(is_valid_url("https://www.example.com"));
/// assert!(is_valid_url("http://example.com/path?query=1"));
/// assert!(!is_valid_url("not a url"));
/// assert!(!is_valid_url("ftp://example.com"));
/// ```
pub fn is_valid_url(url: &str) -> bool {
    if url.is_empty() {
        return false;
    }

    get_url_regex().is_match(url)
}

/// Validates if a string is a valid HTTPS URL only
pub fn is_valid_https_url(url: &str) -> bool {
    url.starts_with("https://") && is_valid_url(url)
}

/// Validates if a URL belongs to a specific domain
pub fn is_url_from_domain(url: &str, domain: &str) -> bool {
    if !is_valid_url(url) {
        return false;
    }

    url.contains(&format!("://{}", domain)) || url.contains(&format!("://www.{}", domain))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_urls() {
        assert!(is_valid_url("https://www.example.com"));
        assert!(is_valid_url("http://example.com"));
        assert!(is_valid_url("https://example.com/path/to/page"));
        assert!(is_valid_url("https://example.com?query=1&other=2"));
    }

    #[test]
    fn test_invalid_urls() {
        assert!(!is_valid_url(""));
        assert!(!is_valid_url("not a url"));
        assert!(!is_valid_url("ftp://example.com"));
        assert!(!is_valid_url("javascript:alert(1)"));
    }

    #[test]
    fn test_https_only() {
        assert!(is_valid_https_url("https://example.com"));
        assert!(!is_valid_https_url("http://example.com"));
    }

    #[test]
    fn test_url_domain() {
        assert!(is_url_from_domain("https://example.com/path", "example.com"));
        assert!(is_url_from_domain("https://www.example.com/path", "example.com"));
        assert!(!is_url_from_domain("https://other.com/path", "example.com"));
    }
}

