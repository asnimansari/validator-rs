//! Credit card validation functions

/// Validates a credit card number using the Luhn algorithm
///
/// # Examples
///
/// ```
/// use validator_rs::credit_card::is_valid_credit_card;
///
/// assert!(is_valid_credit_card("4532015112830366")); // Valid Visa
/// assert!(!is_valid_credit_card("1234567890123456")); // Invalid
/// ```
pub fn is_valid_credit_card(card_number: &str) -> bool {
    let cleaned = card_number.replace([' ', '-'], "");
    
    if cleaned.is_empty() || cleaned.len() < 13 || cleaned.len() > 19 {
        return false;
    }

    if !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    luhn_check(&cleaned)
}

/// Implements the Luhn algorithm for credit card validation
fn luhn_check(number: &str) -> bool {
    let digits: Vec<u32> = number
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.is_empty() {
        return false;
    }

    let mut sum = 0;
    let mut alternate = false;

    for &digit in digits.iter().rev() {
        let mut n = digit;
        if alternate {
            n *= 2;
            if n > 9 {
                n -= 9;
            }
        }
        sum += n;
        alternate = !alternate;
    }

    sum % 10 == 0
}

/// Identifies the credit card type based on the card number
#[derive(Debug, PartialEq, Eq)]
pub enum CardType {
    Visa,
    MasterCard,
    Amex,
    Discover,
    Unknown,
}

/// Determines the type of credit card
pub fn get_card_type(card_number: &str) -> CardType {
    let cleaned = card_number.replace([' ', '-'], "");
    
    if cleaned.starts_with('4') {
        CardType::Visa
    } else if cleaned.starts_with("51") || cleaned.starts_with("52") || 
              cleaned.starts_with("53") || cleaned.starts_with("54") || 
              cleaned.starts_with("55") {
        CardType::MasterCard
    } else if cleaned.starts_with("34") || cleaned.starts_with("37") {
        CardType::Amex
    } else if cleaned.starts_with("6011") || cleaned.starts_with("65") {
        CardType::Discover
    } else {
        CardType::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_credit_cards() {
        // Valid test card numbers
        assert!(is_valid_credit_card("4532015112830366")); // Visa
        assert!(is_valid_credit_card("5425233430109903")); // MasterCard
        assert!(is_valid_credit_card("374245455400126"));  // Amex
    }

    #[test]
    fn test_invalid_credit_cards() {
        assert!(!is_valid_credit_card(""));
        assert!(!is_valid_credit_card("1234567890123456"));
        assert!(!is_valid_credit_card("abc"));
        assert!(!is_valid_credit_card("4532015112830367")); // Wrong check digit
    }

    #[test]
    fn test_card_with_spaces() {
        assert!(is_valid_credit_card("4532 0151 1283 0366"));
        assert!(is_valid_credit_card("5425-2334-3010-9903"));
    }

    #[test]
    fn test_card_types() {
        assert_eq!(get_card_type("4532015112830366"), CardType::Visa);
        assert_eq!(get_card_type("5425233430109903"), CardType::MasterCard);
        assert_eq!(get_card_type("374245455400126"), CardType::Amex);
        assert_eq!(get_card_type("6011111111111117"), CardType::Discover);
        assert_eq!(get_card_type("9999999999999999"), CardType::Unknown);
    }

    #[test]
    fn test_luhn_algorithm() {
        assert!(luhn_check("79927398713"));
        assert!(!luhn_check("79927398714"));
    }
}

