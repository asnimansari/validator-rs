//! Basic usage examples for validator-rs library

use validator_rs::*;

fn main() {
    println!("=== Validator-rs Examples ===\n");

    // Email validation
    println!("--- Email Validation ---");
    let emails = vec!["user@example.com", "invalid.email", "test+tag@domain.co.uk"];
    for email in emails {
        println!("{}: {}", email, is_valid_email(email));
    }
    println!();

    // URL validation
    println!("--- URL Validation ---");
    let urls = vec![
        "https://www.example.com",
        "http://example.com/path?query=1",
        "not a url",
    ];
    for url in urls {
        println!("{}: {}", url, is_valid_url(url));
    }
    println!();

    // Phone validation (Mobile)
    println!("--- Mobile Phone Validation ---");
    let phones = vec!["+14155552671", "+447911123456", "+33612345678", "123"];
    for phone in phones {
        println!("{}: {}", phone, is_valid_phone(phone));
    }
    println!();

    // Credit card validation
    println!("--- Credit Card Validation ---");
    let cards = vec![
        "4532015112830366", // Valid Visa
        "1234567890123456", // Invalid
        "5425233430109903", // Valid MasterCard
    ];
    for card in cards {
        let valid = credit_card::is_valid_credit_card(card);
        let card_type = credit_card::get_card_type(card);
        println!("{}: valid={}, type={:?}", card, valid, card_type);
    }
    println!();

    // String validation
    println!("--- String Validation ---");
    let test_strings = vec!["abc123", "abcXYZ", "12345", "Hello World"];
    for s in test_strings {
        println!(
            "{}: alphanumeric={}, alpha={}, numeric={}",
            s,
            is_alphanumeric(s),
            is_alpha(s),
            is_numeric(s)
        );
    }
    println!();

    // Numeric validation
    println!("--- Numeric Validation ---");
    let numbers = vec![5, -3, 0, 10];
    for num in numbers {
        println!(
            "{}: positive={}, negative={}, in_range(0,10)={}, even={}",
            num,
            is_positive(num),
            is_negative(num),
            is_in_range(num, 0, 10),
            numeric::is_even(num as i64)
        );
    }
    println!();

    // Date validation
    println!("--- Date Validation ---");
    let dates = vec![
        "2023-12-31",
        "2024-02-29", // Leap year
        "2023-02-29", // Not a leap year
        "31-12-2023", // Wrong format
    ];
    for date in dates {
        println!("{}: {}", date, is_valid_date(date));
    }
    println!();

    // DateTime validation
    println!("--- DateTime Validation ---");
    let datetimes = vec![
        "2023-12-31T23:59:59Z",
        "2023-12-31T23:59:59+05:30",
        "2023-12-31 23:59:59", // Wrong format
    ];
    for datetime in datetimes {
        println!("{}: {}", datetime, date::is_valid_datetime(datetime));
    }
    println!();

    // Time validation
    println!("--- Time Validation ---");
    let times = vec!["12:30:45", "23:59:59", "25:00:00", "12:60:00"];
    for time in times {
        println!("{}: {}", time, date::is_valid_time(time));
    }
    println!();

    println!("=== End of Examples ===");
}
