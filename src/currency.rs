//! Currency validation functions
//!
//! This module provides validation for currency strings with extensive
//! customization options for different currency formats worldwide.

use regex::Regex;

/// Options for currency validation
///
/// This struct provides configuration for validating currency strings
/// with support for various formats, symbols, and regional conventions.
#[derive(Debug, Clone)]
pub struct CurrencyOptions {
    /// Currency symbol (e.g., "$", "€", "¥")
    pub symbol: String,
    /// Require the currency symbol to be present
    pub require_symbol: bool,
    /// Allow space after the symbol
    pub allow_space_after_symbol: bool,
    /// Symbol appears after the digits instead of before
    pub symbol_after_digits: bool,
    /// Allow negative values
    pub allow_negatives: bool,
    /// Use parentheses for negative values instead of minus sign
    pub parens_for_negatives: bool,
    /// Negative sign appears before the digits
    pub negative_sign_before_digits: bool,
    /// Negative sign appears after the digits
    pub negative_sign_after_digits: bool,
    /// Allow space placeholder for negative sign (e.g., "R 123" and "R-123")
    pub allow_negative_sign_placeholder: bool,
    /// Thousands separator character
    pub thousands_separator: char,
    /// Decimal separator character
    pub decimal_separator: char,
    /// Allow decimal portion
    pub allow_decimal: bool,
    /// Require decimal portion
    pub require_decimal: bool,
    /// Allowed number of digits after decimal (e.g., vec![2] for exactly 2 digits)
    pub digits_after_decimal: Vec<usize>,
    /// Allow space after digits
    pub allow_space_after_digits: bool,
}

impl Default for CurrencyOptions {
    fn default() -> Self {
        Self {
            symbol: "$".to_string(),
            require_symbol: false,
            allow_space_after_symbol: false,
            symbol_after_digits: false,
            allow_negatives: true,
            parens_for_negatives: false,
            negative_sign_before_digits: false,
            negative_sign_after_digits: false,
            allow_negative_sign_placeholder: false,
            thousands_separator: ',',
            decimal_separator: '.',
            allow_decimal: true,
            require_decimal: false,
            digits_after_decimal: vec![2],
            allow_space_after_digits: false,
        }
    }
}

impl CurrencyOptions {
    /// Create a new CurrencyOptions with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the currency symbol
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = symbol.into();
        self
    }

    /// Set whether symbol is required
    pub fn require_symbol(mut self, require: bool) -> Self {
        self.require_symbol = require;
        self
    }

    /// Set whether to allow space after symbol
    pub fn allow_space_after_symbol(mut self, allow: bool) -> Self {
        self.allow_space_after_symbol = allow;
        self
    }

    /// Set whether symbol appears after digits
    pub fn symbol_after_digits(mut self, after: bool) -> Self {
        self.symbol_after_digits = after;
        self
    }

    /// Set whether negatives are allowed
    pub fn allow_negatives(mut self, allow: bool) -> Self {
        self.allow_negatives = allow;
        self
    }

    /// Set whether to use parentheses for negatives
    pub fn parens_for_negatives(mut self, use_parens: bool) -> Self {
        self.parens_for_negatives = use_parens;
        self
    }

    /// Set thousands separator character
    pub fn thousands_separator(mut self, sep: char) -> Self {
        self.thousands_separator = sep;
        self
    }

    /// Set decimal separator character
    pub fn decimal_separator(mut self, sep: char) -> Self {
        self.decimal_separator = sep;
        self
    }

    /// Set whether decimals are allowed
    pub fn allow_decimal(mut self, allow: bool) -> Self {
        self.allow_decimal = allow;
        self
    }

    /// Set whether decimal is required
    pub fn require_decimal(mut self, require: bool) -> Self {
        self.require_decimal = require;
        self
    }

    /// Set allowed digits after decimal
    pub fn digits_after_decimal(mut self, digits: Vec<usize>) -> Self {
        self.digits_after_decimal = digits;
        self
    }

    /// Set whether negative sign appears before digits
    pub fn negative_sign_before_digits(mut self, before: bool) -> Self {
        self.negative_sign_before_digits = before;
        self
    }

    /// Set whether negative sign appears after digits
    pub fn negative_sign_after_digits(mut self, after: bool) -> Self {
        self.negative_sign_after_digits = after;
        self
    }

    /// Set whether to allow negative sign placeholder
    pub fn allow_negative_sign_placeholder(mut self, allow: bool) -> Self {
        self.allow_negative_sign_placeholder = allow;
        self
    }

    /// Set whether to allow space after digits
    pub fn allow_space_after_digits(mut self, allow: bool) -> Self {
        self.allow_space_after_digits = allow;
        self
    }
}

/// Build a regex pattern for currency validation based on options
/// Note: Rust regex doesn't support lookahead/lookbehind, so we validate differently
fn build_currency_regex(options: &CurrencyOptions) -> Result<Regex, regex::Error> {
    // Build decimal digits pattern
    let mut decimal_digits = format!(r"\d{{{}}}", options.digits_after_decimal[0]);
    for digit in options.digits_after_decimal.iter().skip(1) {
        decimal_digits.push_str(&format!(r"|\d{{{}}}", digit));
    }

    // Escape non-word characters in symbol (matching JS /\W/ regex)
    // Use regex::escape which handles all special regex characters
    let escaped_symbol = regex::escape(&options.symbol);

    let symbol = format!(
        "({}){}",
        escaped_symbol,
        if options.require_symbol { "" } else { "?" }
    );

    let negative = r"-?";
    let whole_dollar_amount_without_sep = r"[1-9]\d*";

    // Escape thousands separator
    let escaped_thousands_sep =
        if options.thousands_separator.is_alphanumeric() || options.thousands_separator == '_' {
            options.thousands_separator.to_string()
        } else {
            format!(r"\{}", options.thousands_separator)
        };

    let whole_dollar_amount_with_sep =
        format!(r"[1-9]\d{{0,2}}({}\d{{3}})*", escaped_thousands_sep);

    let valid_whole_dollar_amounts = vec![
        "0",
        whole_dollar_amount_without_sep,
        &whole_dollar_amount_with_sep,
    ];

    let whole_dollar_amount = format!("({})?", valid_whole_dollar_amounts.join("|"));

    // Escape decimal separator
    let escaped_decimal_sep =
        if options.decimal_separator.is_alphanumeric() || options.decimal_separator == '_' {
            options.decimal_separator.to_string()
        } else {
            format!(r"\{}", options.decimal_separator)
        };

    let decimal_amount = format!(
        r"({}({})){}",
        escaped_decimal_sep,
        decimal_digits,
        if options.require_decimal { "" } else { "?" }
    );

    let mut pattern = whole_dollar_amount.clone();
    if options.allow_decimal || options.require_decimal {
        pattern.push_str(&decimal_amount);
    }

    // Handle negative sign placement (default is negative sign before symbol)
    if options.allow_negatives && !options.parens_for_negatives {
        if options.negative_sign_after_digits {
            pattern.push_str(negative);
        } else if options.negative_sign_before_digits {
            pattern = format!("{}{}", negative, pattern);
        }
    }

    // Handle spacing - simplified without lookahead
    if options.allow_negative_sign_placeholder {
        // South African Rand: allows "R 123" or "R-123"
        pattern = format!(r"( ?-?)?{}", pattern);
    } else if options.allow_space_after_symbol {
        pattern = format!(r" ?{}", pattern);
    } else if options.allow_space_after_digits {
        pattern.push_str(r" ?");
    }

    // Add symbol before or after
    if options.symbol_after_digits {
        pattern.push_str(&symbol);
    } else {
        pattern = format!("{}{}", symbol, pattern);
    }

    // Handle parentheses for negatives or default negative placement
    if options.allow_negatives {
        if options.parens_for_negatives {
            // Pattern: ( \( PATTERN \) | PATTERN )
            pattern = format!(r"(\({}\)|{})", pattern, pattern);
        } else if !options.negative_sign_before_digits && !options.negative_sign_after_digits {
            pattern = format!("{}{}", negative, pattern);
        }
    }

    // Build final pattern - simplified without lookahead
    let final_pattern = format!(r"^{}$", pattern);

    Regex::new(&final_pattern)
}

/// Additional validation without using lookahead (manual checks)
fn validate_currency_manual(value: &str, options: &CurrencyOptions) -> bool {
    // Empty string is invalid
    if value.is_empty() {
        return false;
    }

    // Don't allow leading or trailing whitespace
    if value.starts_with(' ') || value.ends_with(' ') {
        return false;
    }

    // Check for "- " pattern (negative sign followed by space)
    if value.starts_with("- ") {
        return false;
    }

    // Must contain at least one digit
    if !value.chars().any(|c| c.is_ascii_digit()) {
        return false;
    }

    // Check for invalid patterns with spaces
    // "$ " (symbol followed by space when not allowed)
    if !options.allow_space_after_symbol && !options.allow_negative_sign_placeholder {
        if value.contains(&format!("{} ", options.symbol)) {
            return false;
        }
    }

    // Check for "SYMBOL -" pattern (space between symbol and negative)
    // This is invalid with allow_negative_sign_placeholder but valid with allow_space_after_symbol
    if options.allow_negative_sign_placeholder && !options.allow_space_after_symbol {
        if value.contains(&format!("{} -", options.symbol)) {
            return false;
        }
    }

    // Check specific invalid patterns
    // Don't allow digit followed by space followed by end of string
    // unless allow_space_after_digits is true
    if !options.allow_space_after_digits && !options.allow_negative_sign_placeholder {
        let trimmed = value.trim_end_matches(&options.symbol);
        let trimmed = trimmed.trim_end_matches(')');
        if trimmed.ends_with(' ') {
            return false;
        }
    }

    true
}

/// Validates if a string is a valid currency format
///
/// # Examples
///
/// ```
/// use validator_rs::currency::{is_currency, CurrencyOptions};
///
/// // Default options (US format)
/// assert!(is_currency("$10,123.45", None));
/// assert!(is_currency("10123.45", None));
/// assert!(!is_currency("$ 32.50", None)); // no space after symbol
///
/// // Custom options
/// let euro_options = CurrencyOptions::new()
///     .symbol("€")
///     .thousands_separator('.')
///     .decimal_separator(',');
/// assert!(is_currency("€1.234,56", Some(euro_options)));
/// ```
pub fn is_currency(value: &str, options: Option<CurrencyOptions>) -> bool {
    let opts = options.unwrap_or_default();

    // Manual validation first (replaces lookahead assertions)
    if !validate_currency_manual(value, &opts) {
        return false;
    }

    match build_currency_regex(&opts) {
        Ok(regex) => regex.is_match(value),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: Default format -$##,###.## (en-US, en-CA, en-AU, en-NZ, en-HK)
    #[test]
    fn test_default_currency() {
        let valid = vec![
            "-$10,123.45",
            "$10,123.45",
            "$10123.45",
            "10,123.45",
            "10123.45",
            "10,123",
            "1,123,456",
            "1123456",
            "1.39",
            ".03",
            "0.10",
            "$0.10",
            "-$0.01",
            "-$.99",
            "$100,234,567.89",
            "$10,123",
            "10,123",
            "-10123",
        ];

        let invalid = vec![
            "1.234",
            "$1.1",
            "$ 32.50",
            "500$",
            ".0001",
            "$.001",
            "$0.001",
            "12,34.56",
            "123456,123,123456",
            "123,4",
            ",123",
            "$-,123",
            "$",
            ".",
            ",",
            "00",
            "$-",
            "$-,.",
            "-",
            "-$",
            "",
            "- $",
        ];

        for val in valid {
            assert!(is_currency(val, None), "Expected '{}' to be valid", val);
        }

        for val in invalid {
            assert!(!is_currency(val, None), "Expected '{}' to be invalid", val);
        }
    }

    // Test 2: No decimal allowed
    #[test]
    fn test_no_decimal() {
        let options = CurrencyOptions::new().allow_decimal(false);

        let valid = vec![
            "-$10,123",
            "$10,123",
            "$10123",
            "10,123",
            "10123",
            "1,123,456",
            "1123456",
            "1",
            "0",
            "$0",
            "-$0",
            "$100,234,567",
            "$10,123",
            "10,123",
            "-10123",
        ];

        let invalid = vec![
            "-$10,123.45",
            "$10,123.45",
            "$10123.45",
            "10,123.45",
            "10123.45",
            "1.39",
            ".03",
            "0.10",
            "$0.10",
            "-$0.01",
            "-$.99",
            "$100,234,567.89",
            "1.234",
            "$1.1",
            "$ 32.50",
            ".0001",
            "$.001",
            "$0.001",
            "12,34.56",
            "123,4",
            ",123",
            "$-,123",
            "$",
            ".",
            ",",
            "00",
            "$-",
            "$-,.",
            "-",
            "-$",
            "",
            "- $",
        ];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }

    // Test 3: Require decimal
    #[test]
    fn test_require_decimal() {
        let options = CurrencyOptions::new().require_decimal(true);

        let valid = vec![
            "-$10,123.45",
            "$10,123.45",
            "$10123.45",
            "10,123.45",
            "10123.45",
            "10,123.00",
            "1.39",
            ".03",
            "0.10",
            "$0.10",
            "-$0.01",
            "-$.99",
            "$100,234,567.89",
        ];

        let invalid = vec![
            "$10,123",
            "10,123",
            "-10123",
            "1,123,456",
            "1123456",
            "1.234",
            "$1.1",
            "$ 32.50",
            "500$",
            ".0001",
            "$.001",
            "$0.001",
            "12,34.56",
            "123456,123,123456",
            "123,4",
            ",123",
            "$-,123",
            "$",
            ".",
            ",",
            "00",
            "$-",
            "$-,.",
            "-",
            "-$",
            "",
            "- $",
        ];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }

    // Test 4: Custom digits after decimal
    #[test]
    fn test_digits_after_decimal() {
        let options = CurrencyOptions::new().digits_after_decimal(vec![1, 3]);

        let valid = vec![
            "-$10,123.4",
            "$10,123.454",
            "$10123.452",
            "10,123.453",
            "10123.450",
            "10,123",
            "1,123,456",
            "1123456",
            "1.3",
            ".030",
            "0.100",
            "$0.1",
            "-$0.0",
            "-$.9",
            "$100,234,567.893",
            "$10,123",
            "10,123.123",
            "-10123.1",
        ];

        let invalid = vec![
            "1.23",
            "$1.13322",
            "$ 32.50",
            "500$",
            ".0001",
            "$.01",
            "$0.01",
            "12,34.56",
            "123456,123,123456",
            "123,4",
            ",123",
            "$-,123",
            "$",
            ".",
            ",",
            "00",
            "$-",
            "$-,.",
            "-",
            "-$",
            "",
            "- $",
        ];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }

    // Test 5: Require symbol
    #[test]
    fn test_require_symbol() {
        let options = CurrencyOptions::new().require_symbol(true);

        let valid = vec![
            "-$10,123.45",
            "$10,123.45",
            "$10123.45",
            "$10,123.45",
            "$10,123",
            "$1,123,456",
            "$1123456",
            "$1.39",
            "$.03",
            "$0.10",
            "$0.10",
            "-$0.01",
            "-$.99",
            "$100,234,567.89",
            "$10,123",
            "-$10123",
        ];

        let invalid = vec![
            "1.234",
            "$1.234",
            "1.1",
            "$1.1",
            "$ 32.50",
            " 32.50",
            "500",
            "10,123,456",
            ".0001",
            "$.001",
            "$0.001",
            "1,234.56",
            "123456,123,123456",
            "$123456,123,123456",
            "123.4",
            "$123.4",
            ",123",
            "$,123",
            "$-,123",
            "$",
            ".",
            "$.",
            ",",
            "$,",
            "00",
            "$00",
            "$-",
            "$-,.",
            "-",
            "-$",
            "",
            "$ ",
            "- $",
        ];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }

    // Test 6: Chinese Yuan format with negative_sign_before_digits
    #[test]
    fn test_yuan_format() {
        let mut options = CurrencyOptions::new();
        options.symbol = "¥".to_string();
        options.negative_sign_before_digits = true;

        let valid = vec![
            "123,456.78",
            "-123,456.78",
            "¥6,954,231",
            "¥-6,954,231",
            "¥10.03",
            "¥-10.03",
            "10.03",
            "1.39",
            ".03",
            "0.10",
            "¥-10567.01",
            "¥0.01",
            "¥1,234,567.89",
            "¥10,123",
            "¥-10,123",
            "¥-10,123.45",
            "10,123",
            "10123",
            "¥-100",
        ];

        let invalid = vec![
            "1.234",
            "¥1.1",
            "5,00",
            ".0001",
            "¥.001",
            "¥0.001",
            "12,34.56",
            "123456,123,123456",
            "123 456",
            ",123",
            "¥-,123",
            "",
            " ",
            "¥",
            "¥-",
            "¥-,.",
            "-",
            "- ¥",
            "-¥",
        ];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }

    // Test 7: Negative sign after digits
    #[test]
    fn test_negative_sign_after_digits() {
        let mut options = CurrencyOptions::new();
        options.negative_sign_after_digits = true;

        let valid = vec![
            "$10,123.45-",
            "$10,123.45",
            "$10123.45",
            "10,123.45",
            "10123.45",
            "10,123",
            "1,123,456",
            "1123456",
            "1.39",
            ".03",
            "0.10",
            "$0.10",
            "$0.01-",
            "$.99-",
            "$100,234,567.89",
            "$10,123",
            "10,123",
            "10123-",
        ];

        let invalid = vec![
            "-123",
            "1.234",
            "$1.1",
            "$ 32.50",
            "500$",
            ".0001",
            "$.001",
            "$0.001",
            "12,34.56",
            "123456,123,123456",
            "123,4",
            ",123",
            "$-,123",
            "$",
            ".",
            ",",
            "00",
            "$-",
            "$-,.",
            "-",
            "-$",
            "",
            "- $",
        ];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }

    // Test 8: No negatives allowed (Chinese Yuan)
    #[test]
    fn test_no_negatives_yuan() {
        let mut options = CurrencyOptions::new();
        options.symbol = "¥".to_string();
        options.allow_negatives = false;

        let valid = vec![
            "123,456.78",
            "¥6,954,231",
            "¥10.03",
            "10.03",
            "1.39",
            ".03",
            "0.10",
            "¥0.01",
            "¥1,234,567.89",
            "¥10,123",
            "10,123",
            "10123",
            "¥100",
        ];

        let invalid = vec![
            "1.234",
            "-123,456.78",
            "¥-6,954,231",
            "¥-10.03",
            "¥-10567.01",
            "¥1.1",
            "¥-10,123",
            "¥-10,123.45",
            "5,00",
            "¥-100",
            ".0001",
            "¥.001",
            "¥-.001",
            "¥0.001",
            "12,34.56",
            "123456,123,123456",
            "123 456",
            ",123",
            "¥-,123",
            "",
            " ",
            "¥",
            "¥-",
            "¥-,.",
            "-",
            "- ¥",
            "-¥",
        ];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }

    // Test 9: South African Rand format with negative_sign_placeholder
    #[test]
    fn test_south_african_rand() {
        let mut options = CurrencyOptions::new();
        options.symbol = "R".to_string();
        options.negative_sign_before_digits = true;
        options.thousands_separator = ' ';
        options.decimal_separator = ',';
        options.allow_negative_sign_placeholder = true;

        let valid = vec![
            "123 456,78",
            "-10 123",
            "R-10 123",
            "R 6 954 231",
            "R10,03",
            "10,03",
            "1,39",
            ",03",
            "0,10",
            "R10567,01",
            "R0,01",
            "R1 234 567,89",
            "R10 123",
            "R 10 123",
            "R 10123",
            "R-10123",
            "10 123",
            "10123",
        ];

        let invalid = vec![
            "1,234",
            "R -10123",
            "R- 10123",
            "R,1",
            ",0001",
            "R,001",
            "R0,001",
            "12 34,56",
            "123456 123 123456",
            " 123",
            "- 123",
            "123 ",
            "",
            " ",
            "R",
            "R- .1",
            "R-",
            "-",
            "-R 10123",
            "R00",
            "R -",
            "-R",
        ];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }

    // Test 10: Italian Euro format with space after symbol
    #[test]
    fn test_euro_italian() {
        let mut options = CurrencyOptions::new();
        options.symbol = "€".to_string();
        options.thousands_separator = '.';
        options.decimal_separator = ',';
        options.allow_space_after_symbol = true;

        let valid = vec![
            "123.456,78",
            "-123.456,78",
            "€6.954.231",
            "-€6.954.231",
            "€ 896.954.231",
            "-€ 896.954.231",
            "16.954.231",
            "-16.954.231",
            "€10,03",
            "-€10,03",
            "10,03",
            "-10,03",
            "-1,39",
            ",03",
            "0,10",
            "-€10567,01",
            "-€ 10567,01",
            "€ 0,01",
            "€1.234.567,89",
            "€10.123",
            "10.123",
            "-€10.123",
            "€ 10.123",
            "€10.123",
            "€ 10123",
            "10.123",
            "-10123",
        ];

        let invalid = vec![
            "1,234",
            "€ 1,1",
            "50#,50",
            "123,@€ ",
            "€€500",
            ",0001",
            "€ ,001",
            "€0,001",
            "12.34,56",
            "123456.123.123456",
            "€123€",
            "",
            " ",
            "€",
            " €",
            "€ ",
            "€€",
            " 123",
            "- 123",
            ".123",
            "-€.123",
            "123 ",
            "€-",
            "- €",
            "€ - ",
            "-",
            "- ",
            "-€",
        ];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }

    // Test 11: Greek Euro format with symbol after digits
    #[test]
    fn test_euro_greek() {
        let mut options = CurrencyOptions::new();
        options.symbol = "€".to_string();
        options.thousands_separator = '.';
        options.symbol_after_digits = true;
        options.decimal_separator = ',';
        options.allow_space_after_digits = true;

        let valid = vec![
            "123.456,78",
            "-123.456,78",
            "6.954.231 €",
            "-6.954.231 €",
            "896.954.231",
            "-896.954.231",
            "16.954.231",
            "-16.954.231",
            "10,03€",
            "-10,03€",
            "10,03",
            "-10,03",
            "1,39",
            ",03",
            "-,03",
            "-,03 €",
            "-,03€",
            "0,10",
            "10567,01€",
            "0,01 €",
            "1.234.567,89€",
            "10.123€",
            "10.123",
            "10.123€",
            "10.123 €",
            "10123 €",
            "10.123",
            "10123",
        ];

        let invalid = vec![
            "1,234",
            "1,1 €",
            ",0001",
            ",001 €",
            "0,001€",
            "12.34,56",
            "123456.123.123456",
            "€123€",
            "",
            " ",
            "€",
            " €",
            "€ ",
            " 123",
            "- 123",
            ".123",
            "-.123€",
            "-.123 €",
            "123 ",
            "-€",
            "- €",
            "-",
            "- ",
        ];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }

    // Test 12: Danish Krone with space after symbol
    #[test]
    fn test_danish_krone() {
        let mut options = CurrencyOptions::new();
        options.symbol = "kr.".to_string();
        options.negative_sign_before_digits = true;
        options.thousands_separator = '.';
        options.decimal_separator = ',';
        options.allow_space_after_symbol = true;

        let valid = vec![
            "123.456,78",
            "-10.123",
            "kr. -10.123",
            "kr.-10.123",
            "kr. 6.954.231",
            "kr.10,03",
            "kr. -10,03",
            "10,03",
            "1,39",
            ",03",
            "0,10",
            "kr. 10567,01",
            "kr. 0,01",
            "kr. 1.234.567,89",
            "kr. -1.234.567,89",
            "10.123",
            "kr. 10.123",
            "kr.10.123",
            "10123",
            "10.123",
            "kr.-10123",
        ];

        let invalid = vec![
            "1,234",
            "kr.  -10123",
            "kr.,1",
            ",0001",
            "kr. ,001",
            "kr.0,001",
            "12.34,56",
            "123456.123.123456",
            ".123",
            "kr.-.123",
            "kr. -.123",
            "- 123",
            "123 ",
            "",
            " ",
            "kr.",
            " kr.",
            "kr. ",
            "kr.-",
            "kr. -",
            "kr. - ",
            " - ",
            "-",
            "- kr.",
            "-kr.",
        ];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }

    // Test 13: Danish Krone with no negatives
    #[test]
    fn test_danish_krone_no_negatives() {
        let mut options = CurrencyOptions::new();
        options.symbol = "kr.".to_string();
        options.allow_negatives = false;
        options.negative_sign_before_digits = true;
        options.thousands_separator = '.';
        options.decimal_separator = ',';
        options.allow_space_after_symbol = true;

        let valid = vec![
            "123.456,78",
            "10.123",
            "kr. 10.123",
            "kr.10.123",
            "kr. 6.954.231",
            "kr.10,03",
            "kr. 10,03",
            "10,03",
            "1,39",
            ",03",
            "0,10",
            "kr. 10567,01",
            "kr. 0,01",
            "kr. 1.234.567,89",
            "kr.1.234.567,89",
            "10.123",
            "kr. 10.123",
            "kr.10.123",
            "10123",
            "10.123",
            "kr.10123",
        ];

        let invalid = vec![
            "1,234",
            "-10.123",
            "kr. -10.123",
            "kr. -1.234.567,89",
            "kr.-10123",
            "kr.  -10123",
            "kr.-10.123",
            "kr. -10,03",
            "kr.,1",
            ",0001",
            "kr. ,001",
            "kr.0,001",
            "12.34,56",
            "123456.123.123456",
            ".123",
            "kr.-.123",
            "kr. -.123",
            "- 123",
            "123 ",
            "",
            " ",
            "kr.",
            " kr.",
            "kr. ",
            "kr.-",
            "kr. -",
            "kr. - ",
            " - ",
            "-",
            "- kr.",
            "-kr.",
        ];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }

    // Test 14: Parentheses for negatives
    #[test]
    fn test_parens_for_negatives() {
        let mut options = CurrencyOptions::new();
        options.parens_for_negatives = true;

        let valid = vec![
            "1,234",
            "(1,234)",
            "($6,954,231)",
            "$10.03",
            "(10.03)",
            "($10.03)",
            "1.39",
            ".03",
            "(.03)",
            "($.03)",
            "0.10",
            "$10567.01",
            "($0.01)",
            "$1,234,567.89",
            "$10,123",
            "(10,123)",
            "10123",
        ];

        let invalid = vec![
            "1.234",
            "($1.1)",
            "-$1.10",
            "$ 32.50",
            "500$",
            ".0001",
            "$.001",
            "($0.001)",
            "12,34.56",
            "123456,123,123456",
            "( 123)",
            ",123",
            "$-,123",
            "",
            " ",
            "  ",
            "   ",
            "$",
            "$ ",
            " $",
            " 123",
            "(123) ",
            ".",
            ",",
            "00",
            "$-",
            "$ - ",
            "$- ",
            " - ",
            "-",
            "- $",
            "-$",
            "()",
            "( )",
            "(  -)",
            "(  - )",
            "(  -  )",
            "(-)",
            "(-$)",
        ];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }

    // Test 15: No negatives allowed (standard USD)
    #[test]
    fn test_no_negatives_usd() {
        let mut options = CurrencyOptions::new();
        options.allow_negatives = false;

        let valid = vec![
            "$10,123.45",
            "$10123.45",
            "10,123.45",
            "10123.45",
            "10,123",
            "1,123,456",
            "1123456",
            "1.39",
            ".03",
            "0.10",
            "$0.10",
            "$100,234,567.89",
            "$10,123",
            "10,123",
        ];

        let invalid = vec![
            "1.234",
            "-1.234",
            "-10123",
            "-$0.01",
            "-$.99",
            "$1.1",
            "-$1.1",
            "$ 32.50",
            "500$",
            ".0001",
            "$.001",
            "$0.001",
            "12,34.56",
            "123456,123,123456",
            "-123456,123,123456",
            "123,4",
            ",123",
            "$-,123",
            "$",
            ".",
            ",",
            "00",
            "$-",
            "$-,.",
            "-",
            "-$",
            "",
            "- $",
            "-$10,123.45",
        ];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }

    // Test 16: Brazilian Real format
    #[test]
    fn test_brazilian_real() {
        let mut options = CurrencyOptions::new();
        options.symbol = "R$".to_string();
        options.require_symbol = true;
        options.allow_space_after_symbol = true;
        options.symbol_after_digits = false;
        options.thousands_separator = '.';
        options.decimal_separator = ',';

        let valid = vec!["R$ 1.400,00", "R$ 400,00"];

        let invalid = vec!["$ 1.400,00", "$R 1.400,00"];

        for val in valid {
            assert!(
                is_currency(val, Some(options.clone())),
                "Expected '{}' to be valid",
                val
            );
        }

        for val in invalid {
            assert!(
                !is_currency(val, Some(options.clone())),
                "Expected '{}' to be invalid",
                val
            );
        }
    }
}
