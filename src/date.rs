//! Date and time validation functions

use regex::Regex;
use std::sync::OnceLock;

static DATE_REGEX: OnceLock<Regex> = OnceLock::new();
static DATETIME_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_date_regex() -> &'static Regex {
    DATE_REGEX.get_or_init(|| {
        Regex::new(r"^\d{4}-\d{2}-\d{2}$")
            .expect("Failed to compile date regex")
    })
}

fn get_datetime_regex() -> &'static Regex {
    DATETIME_REGEX.get_or_init(|| {
        Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})?$")
            .expect("Failed to compile datetime regex")
    })
}

/// Validates if a string is in ISO 8601 date format (YYYY-MM-DD)
///
/// # Examples
///
/// ```
/// use validator_rs::date::is_valid_date;
///
/// assert!(is_valid_date("2023-12-31"));
/// assert!(!is_valid_date("31-12-2023"));
/// ```
pub fn is_valid_date(date: &str) -> bool {
    if !get_date_regex().is_match(date) {
        return false;
    }

    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return false;
    }

    let year = parts[0].parse::<i32>().ok();
    let month = parts[1].parse::<u32>().ok();
    let day = parts[2].parse::<u32>().ok();

    match (year, month, day) {
        (Some(y), Some(m), Some(d)) => is_valid_date_parts(y, m, d),
        _ => false,
    }
}

/// Validates if year, month, and day form a valid date
fn is_valid_date_parts(year: i32, month: u32, day: u32) -> bool {
    if !(1..=12).contains(&month) || day < 1 {
        return false;
    }

    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => return false,
    };

    day <= max_day
}

/// Checks if a year is a leap year
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Validates if a string is in ISO 8601 datetime format
pub fn is_valid_datetime(datetime: &str) -> bool {
    get_datetime_regex().is_match(datetime)
}

/// Validates if a string is a valid time in HH:MM:SS format
pub fn is_valid_time(time: &str) -> bool {
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 3 {
        return false;
    }

    let hour = parts[0].parse::<u32>().ok();
    let minute = parts[1].parse::<u32>().ok();
    let second = parts[2].parse::<u32>().ok();

    match (hour, minute, second) {
        (Some(h), Some(m), Some(s)) => h < 24 && m < 60 && s < 60,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_dates() {
        assert!(is_valid_date("2023-12-31"));
        assert!(is_valid_date("2024-02-29")); // Leap year
        assert!(is_valid_date("2000-01-01"));
    }

    #[test]
    fn test_invalid_dates() {
        assert!(!is_valid_date(""));
        assert!(!is_valid_date("2023-13-01")); // Invalid month
        assert!(!is_valid_date("2023-02-30")); // Invalid day
        assert!(!is_valid_date("2023-02-29")); // Not a leap year
        assert!(!is_valid_date("31-12-2023")); // Wrong format
        assert!(!is_valid_date("2023/12/31")); // Wrong separator
    }

    #[test]
    fn test_leap_year() {
        assert!(is_leap_year(2024));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(2023));
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn test_valid_datetime() {
        assert!(is_valid_datetime("2023-12-31T23:59:59Z"));
        assert!(is_valid_datetime("2023-12-31T23:59:59.123Z"));
        assert!(is_valid_datetime("2023-12-31T23:59:59+05:30"));
    }

    #[test]
    fn test_invalid_datetime() {
        assert!(!is_valid_datetime("2023-12-31"));
        assert!(!is_valid_datetime("2023-12-31 23:59:59"));
    }

    #[test]
    fn test_valid_time() {
        assert!(is_valid_time("12:30:45"));
        assert!(is_valid_time("00:00:00"));
        assert!(is_valid_time("23:59:59"));
    }

    #[test]
    fn test_invalid_time() {
        assert!(!is_valid_time("24:00:00"));
        assert!(!is_valid_time("12:60:00"));
        assert!(!is_valid_time("12:30:60"));
        assert!(!is_valid_time("12:30"));
    }
}

