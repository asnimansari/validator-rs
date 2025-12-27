use validator_rs::currency::{is_currency, CurrencyOptions};

fn main() {
    println!("=== Currency Validator Examples ===\n");

    // Example 1: Default USD format
    println!("1. Default USD format:");
    let usd_values = vec!["$10,123.45", "10,123.45", "-$99.99", "$.99"];
    for val in usd_values {
        println!("   '{}' -> {}", val, is_currency(val, None));
    }

    // Example 2: Euro (Italian format)
    println!("\n2. Euro (Italian format - €1.234,56):");
    let euro_options = CurrencyOptions::new()
        .symbol("€")
        .thousands_separator('.')
        .decimal_separator(',')
        .allow_space_after_symbol(true);

    let euro_values = vec!["€1.234,56", "€ 1.234,56", "-€10,50"];
    for val in euro_values {
        println!(
            "   '{}' -> {}",
            val,
            is_currency(val, Some(euro_options.clone()))
        );
    }

    // Example 3: Chinese Yuan
    println!("\n3. Chinese Yuan (¥):");
    let yuan_options = CurrencyOptions::new()
        .symbol("¥")
        .negative_sign_before_digits(true);

    let yuan_values = vec!["¥1,234.56", "¥-999.99", "123,456.78"];
    for val in yuan_values {
        println!(
            "   '{}' -> {}",
            val,
            is_currency(val, Some(yuan_options.clone()))
        );
    }

    // Example 4: South African Rand
    println!("\n4. South African Rand (R 123 or R-123):");
    let rand_options = CurrencyOptions::new()
        .symbol("R")
        .thousands_separator(' ')
        .decimal_separator(',')
        .negative_sign_before_digits(true)
        .allow_negative_sign_placeholder(true);

    let rand_values = vec!["R 10 123,45", "R-10 123,45", "R 123,45"];
    for val in rand_values {
        println!(
            "   '{}' -> {}",
            val,
            is_currency(val, Some(rand_options.clone()))
        );
    }

    // Example 5: Brazilian Real
    println!("\n5. Brazilian Real (R$ 1.234,56):");
    let real_options = CurrencyOptions::new()
        .symbol("R$")
        .require_symbol(true)
        .allow_space_after_symbol(true)
        .thousands_separator('.')
        .decimal_separator(',');

    let real_values = vec!["R$ 1.400,00", "R$ 400,00", "$ 1.400,00"];
    for val in real_values {
        println!(
            "   '{}' -> {}",
            val,
            is_currency(val, Some(real_options.clone()))
        );
    }

    // Example 6: Parentheses for negatives
    println!("\n6. Parentheses for negatives:");
    let parens_options = CurrencyOptions::new().parens_for_negatives(true);

    let parens_values = vec!["($1,234.56)", "$1,234.56", "(1,234.56)", "-$1,234.56"];
    for val in parens_values {
        println!(
            "   '{}' -> {}",
            val,
            is_currency(val, Some(parens_options.clone()))
        );
    }

    // Example 7: Custom decimal digits
    println!("\n7. Custom decimal digits (1 or 3 digits):");
    let custom_decimal_options = CurrencyOptions::new().digits_after_decimal(vec![1, 3]);

    let custom_values = vec!["$10.5", "$10.123", "$10.12", "$10.1234"];
    for val in custom_values {
        println!(
            "   '{}' -> {}",
            val,
            is_currency(val, Some(custom_decimal_options.clone()))
        );
    }

    println!("\n=== All examples completed ===");
}
