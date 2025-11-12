//! Numeric validation functions

/// Validates if a number is within a specified range (inclusive)
///
/// # Examples
///
/// ```
/// use validator_rs::numeric::is_in_range;
///
/// assert!(is_in_range(5, 1, 10));
/// assert!(!is_in_range(15, 1, 10));
/// ```
pub fn is_in_range<T: PartialOrd>(value: T, min: T, max: T) -> bool {
    value >= min && value <= max
}

/// Validates if a number is positive
///
/// # Examples
///
/// ```
/// use validator_rs::numeric::is_positive;
///
/// assert!(is_positive(5));
/// assert!(!is_positive(-5));
/// ```
pub fn is_positive<T: PartialOrd + Default>(value: T) -> bool {
    value > T::default()
}

/// Validates if a number is negative
///
/// # Examples
///
/// ```
/// use validator_rs::numeric::is_negative;
///
/// assert!(is_negative(-5));
/// assert!(!is_negative(5));
/// ```
pub fn is_negative<T: PartialOrd + Default>(value: T) -> bool {
    value < T::default()
}

/// Validates if a number is zero
pub fn is_zero<T: PartialEq + Default>(value: T) -> bool {
    value == T::default()
}

/// Validates if a number is even
pub fn is_even(value: i64) -> bool {
    value % 2 == 0
}

/// Validates if a number is odd
pub fn is_odd(value: i64) -> bool {
    value % 2 != 0
}

/// Validates if a number is a multiple of another number
pub fn is_multiple_of(value: i64, divisor: i64) -> bool {
    if divisor == 0 {
        return false;
    }
    value % divisor == 0
}

/// Validates if a floating point number is within a tolerance of a target
pub fn is_close_to(value: f64, target: f64, tolerance: f64) -> bool {
    (value - target).abs() <= tolerance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        assert!(is_in_range(5, 1, 10));
        assert!(is_in_range(1, 1, 10));
        assert!(is_in_range(10, 1, 10));
        assert!(!is_in_range(0, 1, 10));
        assert!(!is_in_range(11, 1, 10));
    }

    #[test]
    fn test_positive_negative() {
        assert!(is_positive(5));
        assert!(is_positive(0.1));
        assert!(!is_positive(0));
        assert!(!is_positive(-5));

        assert!(is_negative(-5));
        assert!(is_negative(-0.1));
        assert!(!is_negative(0));
        assert!(!is_negative(5));
    }

    #[test]
    fn test_zero() {
        assert!(is_zero(0));
        assert!(is_zero(0.0));
        assert!(!is_zero(1));
        assert!(!is_zero(-1));
    }

    #[test]
    fn test_even_odd() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(-4));
        assert!(!is_even(3));

        assert!(is_odd(3));
        assert!(is_odd(-5));
        assert!(!is_odd(2));
        assert!(!is_odd(0));
    }

    #[test]
    fn test_multiple_of() {
        assert!(is_multiple_of(10, 5));
        assert!(is_multiple_of(15, 3));
        assert!(is_multiple_of(0, 5));
        assert!(!is_multiple_of(10, 3));
        assert!(!is_multiple_of(10, 0));
    }

    #[test]
    fn test_close_to() {
        assert!(is_close_to(1.0, 1.01, 0.02));
        assert!(is_close_to(1.0, 1.0, 0.0));
        assert!(!is_close_to(1.0, 1.1, 0.05));
    }
}

