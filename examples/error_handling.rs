//! Advanced example showing error handling with ValidationError

use validator_rs::{ValidationError, ValidationResult};
use validator_rs::*;
use validator_rs::url::is_valid_https_url;

/// Example struct for user registration
#[derive(Debug)]
struct UserRegistration {
    email: String,
    phone: String,
    website: String,
    credit_card: String,
}

impl UserRegistration {
    /// Validates all fields and returns a list of errors
    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Validate email
        if !is_valid_email(&self.email) {
            errors.push(ValidationError::new("email", "Invalid email format"));
        }

        // Validate phone
        if !is_valid_phone(&self.phone) {
            errors.push(ValidationError::new("phone", "Invalid phone number"));
        }

        // Validate website
        if !is_valid_https_url(&self.website) {
            errors.push(ValidationError::new("website", "Must be a valid HTTPS URL"));
        }

        // Validate credit card
        if !credit_card::is_valid_credit_card(&self.credit_card) {
            errors.push(ValidationError::new("credit_card", "Invalid credit card number"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Validates a password with multiple rules
fn validate_password(password: &str) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if !string::has_min_length(password, 8) {
        errors.push("Password must be at least 8 characters long".to_string());
    }

    if !string::has_max_length(password, 128) {
        errors.push("Password must not exceed 128 characters".to_string());
    }

    if !password.chars().any(|c| c.is_uppercase()) {
        errors.push("Password must contain at least one uppercase letter".to_string());
    }

    if !password.chars().any(|c| c.is_lowercase()) {
        errors.push("Password must contain at least one lowercase letter".to_string());
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        errors.push("Password must contain at least one digit".to_string());
    }

    if !password.chars().any(|c| !c.is_alphanumeric()) {
        errors.push("Password must contain at least one special character".to_string());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates age with range checking
fn validate_age(age: i32) -> ValidationResult {
    if !numeric::is_in_range(age, 18, 120) {
        return Err(ValidationError::new("age", "Age must be between 18 and 120"));
    }
    Ok(())
}

fn main() {
    println!("=== Advanced Validation Examples ===\n");

    // Example 1: Valid user registration
    println!("--- Valid User Registration ---");
    let valid_user = UserRegistration {
        email: "user@example.com".to_string(),
        phone: "+12345678901".to_string(),
        website: "https://example.com".to_string(),
        credit_card: "4532015112830366".to_string(),
    };

    match valid_user.validate() {
        Ok(()) => println!("✓ User registration is valid!"),
        Err(errors) => {
            println!("✗ Validation errors:");
            for error in errors {
                println!("  - {}", error);
            }
        }
    }
    println!();

    // Example 2: Invalid user registration
    println!("--- Invalid User Registration ---");
    let invalid_user = UserRegistration {
        email: "invalid-email".to_string(),
        phone: "123".to_string(),
        website: "http://example.com".to_string(), // HTTP not HTTPS
        credit_card: "1234567890123456".to_string(),
    };

    match invalid_user.validate() {
        Ok(()) => println!("✓ User registration is valid!"),
        Err(errors) => {
            println!("✗ Validation errors:");
            for error in errors {
                println!("  - {}", error);
            }
        }
    }
    println!();

    // Example 3: Password validation
    println!("--- Password Validation ---");
    let passwords = vec![
        ("weak", "password"),
        ("better", "Password123!"),
        ("short", "Pass1!"),
    ];

    for (label, password) in passwords {
        print!("Password '{}' ({}): ", password, label);
        match validate_password(password) {
            Ok(()) => println!("✓ Valid"),
            Err(errors) => {
                println!("✗ Invalid");
                for error in errors {
                    println!("    - {}", error);
                }
            }
        }
    }
    println!();

    // Example 4: Age validation
    println!("--- Age Validation ---");
    let ages = vec![17, 25, 65, 121];
    for age in ages {
        print!("Age {}: ", age);
        match validate_age(age) {
            Ok(()) => println!("✓ Valid"),
            Err(error) => println!("✗ {}", error),
        }
    }
    println!();

    println!("=== End of Advanced Examples ===");
}

