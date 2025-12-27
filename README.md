# validator-rs

A comprehensive validation library for Rust that provides various validator functions for common data types and formats.

## Features

- **Email Validation**: Validate email addresses with domain filtering
- **URL Validation**: Validate URLs, HTTPS-only URLs, and domain-specific URLs
- **Phone Number Validation**: Validate international phone numbers, US phone numbers, and country-code specific validation
- **Credit Card Validation**: Luhn algorithm validation and card type detection
- **Currency Validation**: Validate currency strings with extensive customization for different formats worldwide
- **String Validation**: Alphanumeric, alphabetic, numeric, length, and case validation
- **Numeric Validation**: Range checking, positive/negative validation, even/odd, multiples
- **Date/Time Validation**: ISO 8601 date and datetime validation, time validation, leap year checking

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
validator-rs = "0.1.0"
```

## Usage

### Email Validation

```rust
use validator_rs::email::{is_valid_email, is_valid_email_with_domain};

// Basic email validation
assert!(is_valid_email("user@example.com"));
assert!(is_valid_email("test.email+tag@domain.co.uk"));
assert!(!is_valid_email("invalid.email"));

// Email validation with allowed domains
assert!(is_valid_email_with_domain(
    "user@example.com", 
    &["example.com", "test.com"]
));
```

### URL Validation

```rust
use validator_rs::url::{is_valid_url, is_valid_https_url, is_url_from_domain};

// Basic URL validation
assert!(is_valid_url("https://www.example.com"));
assert!(is_valid_url("http://example.com/path?query=1"));

// HTTPS-only validation
assert!(is_valid_https_url("https://example.com"));
assert!(!is_valid_https_url("http://example.com"));

// Domain-specific validation
assert!(is_url_from_domain("https://example.com/path", "example.com"));
```

### Mobile Phone Number Validation

```rust
use validator_rs::mobile::{is_valid_phone, is_mobile_phone, Locale, MobileOptions};

// Simple validation (any locale)
assert!(is_valid_phone("+14155552671"));
assert!(is_valid_phone("+447911123456"));
assert!(is_valid_phone("+33612345678"));

// Validate with specific locale
assert!(is_mobile_phone("+14155552671", Locale::from("en-US"), None).unwrap());
assert!(is_mobile_phone("+447911123456", Locale::from("en-GB"), None).unwrap());

// Validate with multiple locales
let locales = Locale::from(vec!["en-US", "en-GB"]);
assert!(is_mobile_phone("+14155552671", locales, None).unwrap());

// Strict mode (must start with +)
let options = MobileOptions { strict_mode: true };
assert!(is_mobile_phone("+14155552671", Locale::from("en-US"), Some(options)).unwrap());

// Supported locales: 150+ countries including
// US, UK, France, Germany, India, Australia, Brazil, Japan, China, and many more
```

### Credit Card Validation

```rust
use validator_rs::credit_card::{is_valid_credit_card, get_card_type, CardType};

// Luhn algorithm validation
assert!(is_valid_credit_card("4532015112830366")); // Valid Visa
assert!(!is_valid_credit_card("1234567890123456")); // Invalid

// Card type detection
assert_eq!(get_card_type("4532015112830366"), CardType::Visa);
assert_eq!(get_card_type("5425233430109903"), CardType::MasterCard);
assert_eq!(get_card_type("374245455400126"), CardType::Amex);
```

### Currency Validation

```rust
use validator_rs::currency::{is_currency, CurrencyOptions};

// Default USD format validation
assert!(is_currency("$10,123.45", None));
assert!(is_currency("10,123.45", None));
assert!(is_currency("-$99.99", None));

// Euro format (Italian style)
let euro_options = CurrencyOptions::new()
    .symbol("€")
    .thousands_separator('.')
    .decimal_separator(',')
    .allow_space_after_symbol(true);
assert!(is_currency("€ 1.234,56", Some(euro_options)));

// Chinese Yuan with custom negative sign placement
let yuan_options = CurrencyOptions::new()
    .symbol("¥")
    .negative_sign_before_digits(true);
assert!(is_currency("¥-1,234.56", Some(yuan_options)));

// Brazilian Real with required symbol
let real_options = CurrencyOptions::new()
    .symbol("R$")
    .require_symbol(true)
    .allow_space_after_symbol(true)
    .thousands_separator('.')
    .decimal_separator(',');
assert!(is_currency("R$ 1.400,00", Some(real_options)));

// Parentheses for negatives (accounting format)
let parens_options = CurrencyOptions::new()
    .parens_for_negatives(true);
assert!(is_currency("($1,234.56)", Some(parens_options)));
```

### String Validation

```rust
use validator_rs::string::{
    is_alphanumeric, is_alpha, is_numeric,
    has_min_length, has_max_length, has_length_between,
    contains, is_uppercase, is_lowercase
};

// Character type validation
assert!(is_alphanumeric("abc123"));
assert!(is_alpha("abcXYZ"));
assert!(is_numeric("12345"));

// Length validation
assert!(has_min_length("hello", 3));
assert!(has_max_length("hi", 5));
assert!(has_length_between("hello", 3, 10));

// Content validation
assert!(contains("hello world", "world"));
assert!(is_uppercase("HELLO"));
assert!(is_lowercase("hello"));
```

### Numeric Validation

```rust
use validator_rs::numeric::{
    is_in_range, is_positive, is_negative, is_zero,
    is_even, is_odd, is_multiple_of, is_close_to
};

// Range validation
assert!(is_in_range(5, 1, 10));
assert!(is_positive(5));
assert!(is_negative(-5));
assert!(is_zero(0));

// Mathematical properties
assert!(is_even(4));
assert!(is_odd(3));
assert!(is_multiple_of(10, 5));

// Floating point comparison
assert!(is_close_to(1.0, 1.01, 0.02));
```

### Date/Time Validation

```rust
use validator_rs::date::{is_valid_date, is_valid_datetime, is_valid_time};

// Date validation (ISO 8601: YYYY-MM-DD)
assert!(is_valid_date("2023-12-31"));
assert!(is_valid_date("2024-02-29")); // Leap year
assert!(!is_valid_date("2023-02-29")); // Not a leap year

// Datetime validation (ISO 8601)
assert!(is_valid_datetime("2023-12-31T23:59:59Z"));
assert!(is_valid_datetime("2023-12-31T23:59:59+05:30"));

// Time validation (HH:MM:SS)
assert!(is_valid_time("12:30:45"));
assert!(!is_valid_time("25:00:00"));
```

## Module Structure

The library is organized into the following modules:

- `email` - Email validation functions
- `url` - URL validation functions
- `mobile` - Mobile phone number validation with locale support (150+ countries)
- `credit_card` - Credit card validation functions
- `currency` - Currency string validation with extensive customization
- `string` - String content and format validation
- `numeric` - Numeric value validation
- `date` - Date and time validation functions

## Re-exports

Commonly used validators are re-exported at the crate root for convenience:

```rust
use validator_rs::{
    is_valid_email,
    is_valid_url,
    is_valid_phone,
    is_valid_credit_card,
    is_currency,
    is_alphanumeric, is_alpha, is_numeric,
    is_in_range, is_positive, is_negative,
    is_valid_date
};
```

## Error Handling

The library also provides a `ValidationError` type for more structured error handling:

```rust
use validator_rs::{ValidationError, ValidationResult};

fn validate_user_email(email: &str) -> ValidationResult {
    if !validator_rs::is_valid_email(email) {
        return Err(ValidationError::new("email", "Invalid email format"));
    }
    Ok(())
}
```

## Testing

Run the test suite:

```bash
cargo test
```

Run tests with documentation examples:

```bash
cargo test --doc
```

## License

This project is available under your choice of license.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

