//! Mobile phone number validation with locale support
//!
//! This module provides comprehensive mobile phone validation for different countries
//! and locales. It supports over 150 country/locale combinations.

use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;

static PHONE_PATTERNS: OnceLock<HashMap<&'static str, Regex>> = OnceLock::new();

fn get_phone_patterns() -> &'static HashMap<&'static str, Regex> {
    PHONE_PATTERNS.get_or_init(|| {
        let mut map = HashMap::new();
        
        // Helper macro to insert patterns
        macro_rules! add_pattern {
            ($locale:expr, $pattern:expr) => {
                map.insert($locale, Regex::new($pattern).expect("Invalid regex pattern"));
            };
        }

        add_pattern!("am-AM", r"^(\+?374|0)(33|4[134]|55|77|88|9[13-689])\d{6}$");
        add_pattern!("ar-AE", r"^((\+?971)|0)?5[024568]\d{7}$");
        add_pattern!("ar-BH", r"^(\+?973)?(3|6)\d{7}$");
        add_pattern!("ar-DZ", r"^(\+?213|0)(5|6|7)\d{8}$");
        add_pattern!("ar-LB", r"^(\+?961)?((3|81)\d{6}|7\d{7})$");
        add_pattern!("ar-EG", r"^((\+?20)|0)?1[0125]\d{8}$");
        add_pattern!("ar-IQ", r"^(\+?964|0)?7[0-9]\d{8}$");
        add_pattern!("ar-JO", r"^(\+?962|0)?7[789]\d{7}$");
        add_pattern!("ar-KW", r"^(\+?965)([569]\d{7}|41\d{6})$");
        add_pattern!("ar-LY", r"^((\+?218)|0)?(9[1-6]\d{7}|[1-8]\d{7,9})$");
        add_pattern!("ar-MA", r"^(?:(?:\+|00)212|0)[5-7]\d{8}$");
        add_pattern!("ar-OM", r"^((\+|00)968)?([79][1-9])\d{6}$");
        add_pattern!("ar-PS", r"^(\+?970|0)5[6|9](\d{7})$");
        add_pattern!("ar-SA", r"^(!?(\+?966)|0)?5\d{8}$");
        add_pattern!("ar-SD", r"^((\+?249)|0)?(9[012369]|1[012])\d{7}$");
        add_pattern!("ar-SY", r"^(!?(\+?963)|0)?9\d{8}$");
        add_pattern!("ar-TN", r"^(\+?216)?[2459]\d{7}$");
        add_pattern!("az-AZ", r"^(\+994|0)(10|5[015]|7[07]|99)\d{7}$");
        add_pattern!("ar-QA", r"^(\+?974|0)?([3567]\d{7})$");
        add_pattern!("ar-YE", r"^(((\+|00)9677|0?7)[0137]\d{7}|((\+|00)967|0)[1-7]\d{6})$");
        add_pattern!("ar-EH", r"^(\+?212|0)[\s\-]?(5288|5289)[\s\-]?\d{5}$");
        add_pattern!("bs-BA", r"^((((\+|00)3876)|06))((([0-3]|[5-6])\d{6})|(4\d{7}))$");
        add_pattern!("be-BY", r"^(\+?375)?(24|25|29|33|44)\d{7}$");
        add_pattern!("bg-BG", r"^(\+?359|0)?8[789]\d{7}$");
        add_pattern!("bn-BD", r"^(\+?880|0)1[13456789][0-9]{8}$");
        add_pattern!("ca-AD", r"^(\+376)?[346]\d{5}$");
        add_pattern!("cs-CZ", r"^(\+?420)? ?[1-9][0-9]{2} ?[0-9]{3} ?[0-9]{3}$");
        add_pattern!("da-DK", r"^(\+?45)?\s?\d{2}\s?\d{2}\s?\d{2}\s?\d{2}$");
        add_pattern!("de-DE", r"^((\+49|0)1)(5[0-25-9]\d|6([23]|0\d?)|7([0-57-9]|6\d))\d{7,9}$");
        add_pattern!("de-AT", r"^(\+43|0)\d{1,4}\d{3,12}$");
        add_pattern!("de-CH", r"^(\+41|0)([1-9])\d{1,9}$");
        add_pattern!("de-LU", r"^(\+352)?((6\d1)\d{6})$");
        add_pattern!("dv-MV", r"^(\+?960)?(7[2-9]|9[1-9])\d{5}$");
        add_pattern!("el-GR", r"^(\+?30|0)?6(8[5-9]|9[013-57-9])\d{7}$");
        add_pattern!("el-CY", r"^(\+?357?)?(9(9|7|6|5|4)\d{6})$");
        add_pattern!("en-AI", r"^(\+?1|0)264(?:2(35|92)|4(?:6[1-2]|76|97)|5(?:3[6-9]|8[1-4])|7(?:2(4|9)|72))\d{4}$");
        add_pattern!("en-AU", r"^(\+?61|0)4\d{8}$");
        add_pattern!("en-AG", r"^(?:\+1|1)268(?:464|7(?:1[3-9]|[28]\d|3[0246]|64|7[0-689]))\d{4}$");
        add_pattern!("en-BM", r"^(\+?1)?441(((3|7)\d{6}$)|(5[0-3][0-9]\d{4}$)|(59\d{5}$))");
        add_pattern!("en-BS", r"^(\+?1[-\s]?|0)?\(?242\)?[-\s]?\d{3}[-\s]?\d{4}$");
        add_pattern!("en-GB", r"^(\+?44|0)7[1-9]\d{8}$");
        add_pattern!("en-GG", r"^(\+?44|0)1481\d{6}$");
        add_pattern!("en-GH", r"^(\+233|0)(20|50|24|54|27|57|26|56|23|53|28|55|59)\d{7}$");
        add_pattern!("en-GY", r"^(\+592|0)6\d{6}$");
        add_pattern!("en-HK", r"^(\+?852[-\s]?)?[456789]\d{3}[-\s]?\d{4}$");
        add_pattern!("en-MO", r"^(\+?853[-\s]?)?[6]\d{3}[-\s]?\d{4}$");
        add_pattern!("en-IE", r"^(\+?353|0)8[356789]\d{7}$");
        add_pattern!("en-IN", r"^(\+?91|0)?[6789]\d{9}$");
        add_pattern!("en-JM", r"^(\+?876)?\d{7}$");
        add_pattern!("en-KE", r"^(\+?254|0)(7|1)\d{8}$");
        add_pattern!("fr-CF", r"^(\+?236| ?)(70|75|77|72|21|22)\d{6}$");
        add_pattern!("en-SS", r"^(\+?211|0)(9[1257])\d{7}$");
        add_pattern!("en-KI", r"^((\+686|686)?)?( )?((6|7)(2|3|8)[0-9]{6})$");
        add_pattern!("en-KN", r"^(?:\+1|1)869(?:46\d|48[89]|55[6-8]|66\d|76[02-7])\d{4}$");
        add_pattern!("en-LS", r"^(\+?266)(22|28|57|58|59|27|52)\d{6}$");
        add_pattern!("en-MT", r"^(\+?356|0)?(99|79|77|21|27|22|25)[0-9]{6}$");
        add_pattern!("en-MU", r"^(\+?230|0)?\d{8}$");
        add_pattern!("en-MW", r"^(\+?265|0)(((77|88|31|99|98|21)\d{7})|(((111)|1)\d{6})|(32000\d{4}))$");
        add_pattern!("en-NA", r"^(\+?264|0)(6|8)\d{7}$");
        add_pattern!("en-NG", r"^(\+?234|0)?[789]\d{9}$");
        add_pattern!("en-NZ", r"^(\+?64|0)[28]\d{7,9}$");
        add_pattern!("en-PG", r"^(\+?675|0)?(7\d|8[18])\d{6}$");
        add_pattern!("en-PK", r"^((00|\+)?92|0)3[0-6]\d{8}$");
        add_pattern!("en-PH", r"^(09|\+639)\d{9}$");
        add_pattern!("en-RW", r"^(\+?250|0)?[7]\d{8}$");
        add_pattern!("en-SG", r"^(\+65)?[3689]\d{7}$");
        add_pattern!("en-SL", r"^(\+?232|0)\d{8}$");
        add_pattern!("en-TZ", r"^(\+?255|0)?[67]\d{8}$");
        add_pattern!("en-UG", r"^(\+?256|0)?[7]\d{8}$");
        add_pattern!("en-US", r"^((\+1|1)?( |-)?)?(\([2-9][0-9]{2}\)|[2-9][0-9]{2})( |-)?([2-9][0-9]{2}( |-)?[0-9]{4})$");
        add_pattern!("en-ZA", r"^(\+?27|0)\d{9}$");
        add_pattern!("en-ZM", r"^(\+?26)?0[79][567]\d{7}$");
        add_pattern!("en-ZW", r"^(\+263)[0-9]{9}$");
        add_pattern!("en-BW", r"^(\+?267)?(7[1-8]{1})\d{6}$");
        add_pattern!("es-AR", r"^\+?549(11|[2368]\d)\d{8}$");
        add_pattern!("es-BO", r"^(\+?591)?(6|7)\d{7}$");
        add_pattern!("es-CO", r"^(\+?57)?3(0(0|1|2|4|5)|1\d|2[0-4]|5(0|1))\d{7}$");
        add_pattern!("es-CL", r"^(\+?56|0)[2-9]\d{1}\d{7}$");
        add_pattern!("es-CR", r"^(\+506)?[2-8]\d{7}$");
        add_pattern!("es-CU", r"^(\+53|0053)?5\d{7}$");
        add_pattern!("es-DO", r"^(\+?1)?8[024]9\d{7}$");
        add_pattern!("es-HN", r"^(\+?504)?[9|8|3|2]\d{7}$");
        add_pattern!("es-EC", r"^(\+?593|0)([2-7]|9[2-9])\d{7}$");
        add_pattern!("es-ES", r"^(\+?34)?[6|7]\d{8}$");
        add_pattern!("es-GT", r"^(\+?502)?[2|6|7]\d{7}$");
        add_pattern!("es-PE", r"^(\+?51)?9\d{8}$");
        add_pattern!("es-MX", r"^(\+?52)?(1|01)?\d{10,11}$");
        add_pattern!("es-NI", r"^(\+?505)\d{7,8}$");
        add_pattern!("es-PA", r"^(\+?507)\d{7,8}$");
        add_pattern!("es-PY", r"^(\+?595|0)9[9876]\d{7}$");
        add_pattern!("es-SV", r"^(\+?503)?[67]\d{7}$");
        add_pattern!("es-UY", r"^(\+598|0)9[1-9][\d]{6}$");
        add_pattern!("es-VE", r"^(\+?58)?(2|4)\d{9}$");
        add_pattern!("et-EE", r"^(\+?372)?\s?(5|8[1-4])\s?([0-9]\s?){6,7}$");
        add_pattern!("fa-IR", r"^(\+?98[\-\s]?|0)9[0-39]\d[\-\s]?\d{3}[\-\s]?\d{4}$");
        add_pattern!("fa-AF", r"^(\+93|0)?(2{1}[0-8]{1}|[3-5]{1}[0-4]{1})(\d{7})$");
        add_pattern!("fi-FI", r"^(\+?358|0)\s?(4[0-6]|50)\s?(\d\s?){4,8}$");
        add_pattern!("fj-FJ", r"^(\+?679)?\s?\d{3}\s?\d{4}$");
        add_pattern!("fo-FO", r"^(\+?298)?\s?\d{2}\s?\d{2}\s?\d{2}$");
        add_pattern!("fr-BF", r"^(\+226|0)[67]\d{7}$");
        add_pattern!("fr-BJ", r"^(\+229)\d{8}$");
        add_pattern!("fr-CD", r"^(\+?243|0)?(8|9)\d{8}$");
        add_pattern!("fr-CM", r"^(\+?237)6[0-9]{8}$");
        add_pattern!("fr-FR", r"^(\+?33|0)[67]\d{8}$");
        add_pattern!("fr-GF", r"^(\+?594|0|00594)[67]\d{8}$");
        add_pattern!("fr-GP", r"^(\+?590|0|00590)[67]\d{8}$");
        add_pattern!("fr-MQ", r"^(\+?596|0|00596)[67]\d{8}$");
        add_pattern!("fr-PF", r"^(\+?689)?8[789]\d{6}$");
        add_pattern!("fr-RE", r"^(\+?262|0|00262)[67]\d{8}$");
        add_pattern!("fr-WF", r"^(\+681)?\d{6}$");
        add_pattern!("he-IL", r"^(\+972|0)([23489]|5[012345689]|77)[1-9]\d{6}$");
        add_pattern!("hu-HU", r"^(\+?36|06)(20|30|31|50|70)\d{7}$");
        add_pattern!("id-ID", r"^(\+?62|0)8(1[123456789]|2[1238]|3[1238]|5[12356789]|7[78]|9[56789]|8[123456789])([\s?|\d]{5,11})$");
        add_pattern!("ir-IR", r"^(\+98|0)?9\d{9}$");
        add_pattern!("it-IT", r"^(\+?39)?\s?3\d{2} ?\d{6,7}$");
        add_pattern!("it-SM", r"^((\+378)|(0549)|(\+390549)|(\+3780549))?6\d{5,9}$");
        add_pattern!("ja-JP", r"^(\+81[ \-]?(\(0\))?|0)[6789]0[ \-]?\d{4}[ \-]?\d{4}$");
        add_pattern!("ka-GE", r"^(\+?995)?(79\d{7}|5\d{8})$");
        add_pattern!("kk-KZ", r"^(\+?7|8)?7\d{9}$");
        add_pattern!("kl-GL", r"^(\+?299)?\s?\d{2}\s?\d{2}\s?\d{2}$");
        add_pattern!("ko-KR", r"^((\+?82)[ \-]?)?0?1([0|1|6|7|8|9]{1})[ \-]?\d{3,4}[ \-]?\d{4}$");
        add_pattern!("ky-KG", r"^(\+996\s?)?(22[0-9]|50[0-9]|55[0-9]|70[0-9]|75[0-9]|77[0-9]|880|990|995|996|997|998)\s?\d{3}\s?\d{3}$");
        add_pattern!("lt-LT", r"^(\+370|8)\d{8}$");
        add_pattern!("lv-LV", r"^(\+?371)2\d{7}$");
        add_pattern!("mg-MG", r"^((\+?261|0)(2|3)\d)?\d{7}$");
        add_pattern!("mk-MK", r"^(\+?389|0)?((?:2[2-9]\d{6}|(?:3[1-4]|4[2-8])\d{6}|500\d{5}|5[2-9]\d{6}|7[0-9][2-9]\d{5}|8[1-9]\d{6}|800\d{5}|8009\d{4}))$");
        add_pattern!("mn-MN", r"^(\+|00|011)?976(77|81|88|91|94|95|96|99)\d{6}$");
        add_pattern!("my-MM", r"^(\+?959|09|9)(2[5-7]|3[1-2]|4[0-5]|6[6-9]|7[5-9]|9[6-9])[0-9]{7}$");
        add_pattern!("ms-MY", r"^(\+?60|0)1(([0145](-|\s)?\d{7,8})|([236-9](-|\s)?\d{7}))$");
        add_pattern!("mz-MZ", r"^(\+?258)?8[234567]\d{7}$");
        add_pattern!("nb-NO", r"^(\+?47)?[49]\d{7}$");
        add_pattern!("ne-NP", r"^(\+?977)?9[78]\d{8}$");
        add_pattern!("nl-BE", r"^(\+?32|0)4\d{8}$");
        add_pattern!("nl-NL", r"^(((\+|00)?31\(0\))|((\+|00)?31)|0)6{1}\d{8}$");
        add_pattern!("nl-AW", r"^(\+)?297(56|59|64|73|74|99)\d{5}$");
        add_pattern!("nn-NO", r"^(\+?47)?[49]\d{7}$");
        add_pattern!("pl-PL", r"^(\+?48)? ?([5-8]\d|45) ?\d{3} ?\d{2} ?\d{2}$");
        add_pattern!("pt-BR", r"^((\+?55\ ?[1-9]{2}\ ?)|(\+?55\ ?\([1-9]{2}\)\ ?)|(0[1-9]{2}\ ?)|(\([1-9]{2}\)\ ?)|([1-9]{2}\ ?))((\d{4}\-?\d{4})|(9[1-9]{1}\d{3}\-?\d{4}))$");
        add_pattern!("pt-PT", r"^(\+?351)?9[1236]\d{7}$");
        add_pattern!("pt-AO", r"^(\+?244)?9\d{8}$");
        add_pattern!("ro-MD", r"^(\+?373|0)((6(0|1|2|6|7|8|9))|(7(6|7|8|9)))\d{6}$");
        add_pattern!("ro-RO", r"^(\+?40|0)\s?7\d{2}(\/|\s|\.|-)?\d{3}(\s|\.|-)?\d{3}$");
        add_pattern!("ru-RU", r"^(\+?7|8)?9\d{9}$");
        add_pattern!("si-LK", r"^(?:0|94|\+94)?(7(0|1|2|4|5|6|7|8)( |-)?)\d{7}$");
        add_pattern!("sl-SI", r"^(\+386\s?|0)(\d{1}\s?\d{3}\s?\d{2}\s?\d{2}|\d{2}\s?\d{3}\s?\d{3})$");
        add_pattern!("sk-SK", r"^(\+?421)? ?[1-9][0-9]{2} ?[0-9]{3} ?[0-9]{3}$");
        add_pattern!("so-SO", r"^(\+?252|0)((6[0-9])\d{7}|(7[1-9])\d{7})$");
        add_pattern!("sq-AL", r"^(\+355|0)6[2-9]\d{7}$");
        add_pattern!("sr-RS", r"^(\+3816|06)[- \d]{5,9}$");
        add_pattern!("sv-SE", r"^(\+?46|0)[\s\-]?7[\s\-]?[02369]([\s\-]?\d){7}$");
        add_pattern!("tg-TJ", r"^(\+?992)?[5][5]\d{7}$");
        add_pattern!("th-TH", r"^(\+66|66|0)\d{9}$");
        add_pattern!("tr-TR", r"^(\+?90|0)?5\d{9}$");
        add_pattern!("tk-TM", r"^(\+993|993|8)\d{8}$");
        add_pattern!("uk-UA", r"^(\+?38)?0(50|6[36-8]|7[357]|9[1-9])\d{7}$");
        add_pattern!("uz-UZ", r"^(\+?998)?(6[125-79]|7[1-69]|88|9\d)\d{7}$");
        add_pattern!("vi-VN", r"^((\+?84)|0)((3([2-9]))|(5([25689]))|(7([0|6-9]))|(8([1-9]))|(9([0-9])))([0-9]{7})$");
        add_pattern!("zh-CN", r"^((\+|00)86)?(1[3-9]|9[28])\d{9}$");
        add_pattern!("zh-TW", r"^(\+?886\-?|0)?9\d{8}$");
        add_pattern!("dz-BT", r"^(\+?975|0)?(17|16|77|02)\d{6}$");

        // Add aliases
        let en_us = map.get("en-US").expect("en-US pattern must exist").clone();
        map.insert("en-CA", en_us.clone());
        map.insert("fr-CA", en_us);
        
        let nl_be = map.get("nl-BE").expect("nl-BE pattern must exist").clone();
        map.insert("fr-BE", nl_be);
        
        let en_hk = map.get("en-HK").expect("en-HK pattern must exist").clone();
        map.insert("zh-HK", en_hk);
        
        let en_mo = map.get("en-MO").expect("en-MO pattern must exist").clone();
        map.insert("zh-MO", en_mo);
        
        let en_ie = map.get("en-IE").expect("en-IE pattern must exist").clone();
        map.insert("ga-IE", en_ie);
        
        let de_ch = map.get("de-CH").expect("de-CH pattern must exist").clone();
        map.insert("fr-CH", de_ch.clone());
        map.insert("it-CH", de_ch);

        map
    })
}

/// Options for mobile phone validation
#[derive(Debug, Clone, Default)]
pub struct MobileOptions {
    /// If true, the phone number must start with '+'
    pub strict_mode: bool,
}

/// Locale type for validation
#[derive(Debug, Clone)]
pub enum Locale {
    /// A specific locale (e.g., "en-US", "fr-FR")
    Specific(String),
    /// Multiple locales - validates if the number matches any of them
    Multiple(Vec<String>),
    /// Any locale - validates against all available patterns
    Any,
}

impl From<&str> for Locale {
    fn from(s: &str) -> Self {
        if s.is_empty() || s == "any" {
            Locale::Any
        } else {
            Locale::Specific(s.to_string())
        }
    }
}

impl From<String> for Locale {
    fn from(s: String) -> Self {
        if s.is_empty() || s == "any" {
            Locale::Any
        } else {
            Locale::Specific(s)
        }
    }
}

impl From<Vec<String>> for Locale {
    fn from(v: Vec<String>) -> Self {
        if v.is_empty() {
            Locale::Any
        } else {
            Locale::Multiple(v)
        }
    }
}

impl From<Vec<&str>> for Locale {
    fn from(v: Vec<&str>) -> Self {
        if v.is_empty() {
            Locale::Any
        } else {
            Locale::Multiple(v.iter().map(|s| s.to_string()).collect())
        }
    }
}

/// Validates a mobile phone number with locale and options
///
/// # Examples
///
/// ```
/// use validator_rs::mobile::{is_mobile_phone, Locale, MobileOptions};
///
/// // Validate US phone number
/// assert!(is_mobile_phone("4155552671", Locale::from("en-US"), None).unwrap());
///
/// // Validate with strict mode (must start with +)
/// let options = MobileOptions { strict_mode: true };
/// assert!(is_mobile_phone("+14155552671", Locale::from("en-US"), Some(options)).unwrap());
///
/// // Validate against any locale
/// assert!(is_mobile_phone("+447911123456", Locale::Any, None).unwrap());
/// ```
pub fn is_mobile_phone(
    phone: &str,
    locale: Locale,
    options: Option<MobileOptions>,
) -> Result<bool, String> {
    if phone.is_empty() {
        return Ok(false);
    }

    let opts = options.unwrap_or_default();

    // Check strict mode
    if opts.strict_mode && !phone.starts_with('+') {
        return Ok(false);
    }

    let patterns = get_phone_patterns();

    match locale {
        Locale::Specific(ref loc) => {
            if let Some(pattern) = patterns.get(loc.as_str()) {
                Ok(pattern.is_match(phone))
            } else {
                Err(format!("Invalid locale '{}'", loc))
            }
        }
        Locale::Multiple(ref locales) => {
            for loc in locales {
                if let Some(pattern) = patterns.get(loc.as_str()) {
                    if pattern.is_match(phone) {
                        return Ok(true);
                    }
                }
            }
            Ok(false)
        }
        Locale::Any => {
            for pattern in patterns.values() {
                if pattern.is_match(phone) {
                    return Ok(true);
                }
            }
            Ok(false)
        }
    }
}

/// Validates a mobile phone number (convenience function using 'any' locale)
///
/// # Examples
///
/// ```
/// use validator_rs::mobile::is_valid_phone;
///
/// assert!(is_valid_phone("+14155552671"));
/// assert!(is_valid_phone("+447911123456"));
/// assert!(!is_valid_phone("abc123"));
/// ```
pub fn is_valid_phone(phone: &str) -> bool {
    is_mobile_phone(phone, Locale::Any, None).unwrap_or(false)
}

/// Returns a list of all supported locales
pub fn get_supported_locales() -> Vec<&'static str> {
    let mut locales: Vec<&str> = get_phone_patterns().keys().copied().collect();
    locales.sort_unstable();
    locales
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_us_phones() {
        let locale = Locale::from("en-US");
        // US pattern requires specific formats
        assert!(is_mobile_phone("+14155552671", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("4155552671", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("415 555 2671", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("(415) 555-2671", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("123", locale, None).unwrap());
    }

    #[test]
    fn test_uk_phones() {
        let locale = Locale::from("en-GB");
        assert!(is_mobile_phone("+447911123456", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("07911123456", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("0123456789", locale, None).unwrap());
    }

    #[test]
    fn test_strict_mode() {
        let locale = Locale::from("en-US");
        let options = Some(MobileOptions { strict_mode: true });
        
        assert!(is_mobile_phone("+14155552671", locale.clone(), options.clone()).unwrap());
        assert!(!is_mobile_phone("4155552671", locale, options).unwrap());
    }

    #[test]
    fn test_multiple_locales() {
        let locale = Locale::from(vec!["en-US", "en-GB"]);
        
        assert!(is_mobile_phone("+14155552671", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+447911123456", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("+331234567", locale, None).unwrap()); // French number
    }

    #[test]
    fn test_any_locale() {
        assert!(is_mobile_phone("+14155552671", Locale::Any, None).unwrap()); // US
        assert!(is_mobile_phone("+447911123456", Locale::Any, None).unwrap()); // UK
        assert!(is_mobile_phone("+33612345678", Locale::Any, None).unwrap()); // France
        assert!(is_mobile_phone("+81 90 1234 5678", Locale::Any, None).unwrap()); // Japan
        assert!(!is_mobile_phone("abc123", Locale::Any, None).unwrap());
    }

    #[test]
    fn test_invalid_locale() {
        let result = is_mobile_phone("+14155552671", Locale::from("xx-XX"), None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid locale"));
    }

    #[test]
    fn test_convenience_function() {
        assert!(is_valid_phone("+14155552671"));
        assert!(is_valid_phone("+447911123456"));
        assert!(!is_valid_phone("abc123"));
        assert!(!is_valid_phone(""));
    }

    #[test]
    fn test_various_countries() {
        // France
        assert!(is_mobile_phone("+33612345678", Locale::from("fr-FR"), None).unwrap());
        
        // Germany - mobile numbers start with 015x, 016x, 017x
        assert!(is_mobile_phone("+4915123456789", Locale::from("de-DE"), None).unwrap());
        
        // India
        assert!(is_mobile_phone("+919876543210", Locale::from("en-IN"), None).unwrap());
        
        // Australia
        assert!(is_mobile_phone("+61412345678", Locale::from("en-AU"), None).unwrap());
        
        // Brazil - test simpler format
        assert!(is_mobile_phone("+5511912345678", Locale::from("pt-BR"), None).unwrap());
    }

    #[test]
    fn test_get_supported_locales() {
        let locales = get_supported_locales();
        assert!(!locales.is_empty());
        assert!(locales.contains(&"en-US"));
        assert!(locales.contains(&"en-GB"));
        assert!(locales.contains(&"fr-FR"));
    }

    #[test]
    fn test_aliases() {
        // Canada should use US pattern
        assert!(is_mobile_phone("+14155552671", Locale::from("en-CA"), None).unwrap());
        assert!(is_mobile_phone("+14155552671", Locale::from("fr-CA"), None).unwrap());
    }

    #[test]
    fn test_am_am() {
        let locale = Locale::from("am-AM");
        assert!(is_mobile_phone("+37433123456", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+37441123456", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("055123456", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("+37403498855", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("+37416498123", locale, None).unwrap());
    }

    #[test]
    fn test_ar_ae() {
        let locale = Locale::from("ar-AE");
        assert!(is_mobile_phone("+971502674453", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("0585215778", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("585215778", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("+971511498855", locale, None).unwrap());
    }

    #[test]
    fn test_ar_sa() {
        let locale = Locale::from("ar-SA");
        assert!(is_mobile_phone("0556578654", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+966556578654", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("596578654", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("+9665626626262", locale, None).unwrap());
    }

    #[test]
    fn test_zh_cn() {
        let locale = Locale::from("zh-CN");
        assert!(is_mobile_phone("13523333233", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+8616238234822", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("008616238234822", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("12345", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("010-38238383", locale, None).unwrap());
    }

    #[test]
    fn test_zh_tw() {
        let locale = Locale::from("zh-TW");
        assert!(is_mobile_phone("0987123456", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+886987123456", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("886-987123456", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("0-987123456", locale, None).unwrap());
    }

    #[test]
    fn test_ja_jp() {
        let locale = Locale::from("ja-JP");
        assert!(is_mobile_phone("09012345678", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("080 1234 5678", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+8190-1234-5678", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("0312345678", locale, None).unwrap());
    }

    #[test]
    fn test_ko_kr() {
        let locale = Locale::from("ko-KR");
        assert!(is_mobile_phone("+82-010-1234-5678", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("010-123-5678", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("01012345678", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("+82 10 1234 567", locale, None).unwrap());
    }

    #[test]
    fn test_es_es() {
        let locale = Locale::from("es-ES");
        assert!(is_mobile_phone("+34654789321", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("654789321", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+34714789321", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("65478932", locale, None).unwrap());
    }

    #[test]
    fn test_pt_br() {
        let locale = Locale::from("pt-BR");
        assert!(is_mobile_phone("+55 12 996551215", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("5511914314567", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("(11) 94123-4567", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("+55 11 90431-4567", locale, None).unwrap());
    }

    #[test]
    fn test_it_it() {
        let locale = Locale::from("it-IT");
        assert!(is_mobile_phone("370 3175423", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+39 310 7688449", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("011 7387545", locale, None).unwrap());
    }

    #[test]
    fn test_pl_pl() {
        let locale = Locale::from("pl-PL");
        assert!(is_mobile_phone("+48512689767", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("657562855", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+48 56 376 87 47", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("3454535", locale, None).unwrap());
    }

    #[test]
    fn test_tr_tr() {
        let locale = Locale::from("tr-TR");
        assert!(is_mobile_phone("+905321234567", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("05321234567", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("12345", locale, None).unwrap());
    }

    #[test]
    fn test_en_in() {
        let locale = Locale::from("en-IN");
        assert!(is_mobile_phone("+919876543210", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("919876543210", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("09876543210", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("12345", locale, None).unwrap());
    }

    #[test]
    fn test_vi_vn() {
        let locale = Locale::from("vi-VN");
        assert!(is_mobile_phone("0336012403", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+84586012403", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("84981577798", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("01678912345", locale, None).unwrap());
    }

    #[test]
    fn test_en_sg() {
        let locale = Locale::from("en-SG");
        assert!(is_mobile_phone("87654321", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+6587654321", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("12345678", locale, None).unwrap());
    }

    #[test]
    fn test_ar_eg() {
        let locale = Locale::from("ar-EG");
        assert!(is_mobile_phone("+201004513789", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("01090124576", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("1090124576", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("+201404513789", locale, None).unwrap());
    }

    #[test]
    fn test_id_id() {
        let locale = Locale::from("id-ID");
        assert!(is_mobile_phone("0811 778 998", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+62811778998", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("628993123618190", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("0217123456", locale, None).unwrap());
    }

    #[test]
    fn test_th_th() {
        let locale = Locale::from("th-TH");
        assert!(is_mobile_phone("0912345678", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+66912345678", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("66912345678", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("67812345623", locale, None).unwrap());
    }

    #[test]
    fn test_ru_ru() {
        let locale = Locale::from("ru-RU");
        assert!(is_mobile_phone("+79676338855", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("79676338855", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("89676338855", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("+9676338855", locale, None).unwrap());
    }

    #[test]
    fn test_nb_no() {
        let locale = Locale::from("nb-NO");
        assert!(is_mobile_phone("+4796338855", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("4796338855", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("46338855", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("+4676338855", locale, None).unwrap());
    }

    #[test]
    fn test_da_dk() {
        let locale = Locale::from("da-DK");
        assert!(is_mobile_phone("12345678", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+45 12 34 56 78", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("12 34 56", locale, None).unwrap());
    }

    #[test]
    fn test_sv_se() {
        let locale = Locale::from("sv-SE");
        assert!(is_mobile_phone("+46701234567", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("0721234567", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("+46301234567", locale, None).unwrap());
    }

    #[test]
    fn test_fi_fi() {
        let locale = Locale::from("fi-FI");
        assert!(is_mobile_phone("+358505557171", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("0455571", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("045557", locale, None).unwrap());
    }

    #[test]
    fn test_strict_mode_comprehensive() {
        let options = Some(MobileOptions { strict_mode: true });
        
        // Should pass - all start with +
        assert!(is_mobile_phone("+254728530234", Locale::Any, options.clone()).unwrap());
        assert!(is_mobile_phone("+299 12 34 56", Locale::Any, options.clone()).unwrap());
        assert!(is_mobile_phone("+94766660206", Locale::Any, options.clone()).unwrap());
        
        // Should fail - don't start with +
        assert!(!is_mobile_phone("254728530234", Locale::Any, options.clone()).unwrap());
        assert!(!is_mobile_phone("0728530234", Locale::Any, options.clone()).unwrap());
        assert!(!is_mobile_phone("766667206", Locale::Any, options).unwrap());
    }

    #[test]
    fn test_any_locale_comprehensive() {
        // Test a variety of valid numbers from different countries
        let valid_numbers = vec![
            "+14155552671",      // US
            "+447911123456",     // UK
            "+33612345678",      // France
            "+4915123456789",    // Germany
            "+919876543210",     // India
            "+61412345678",      // Australia
            "+5511912345678",    // Brazil
            "+81 90 1234 5678",  // Japan
            "+8613523333233",    // China
            "+82 10 1234 5678",  // South Korea
            "+34654789321",      // Spain
            "+39 310 7688449",   // Italy
            "+48512689767",      // Poland
            "+79676338855",      // Russia
            "+66912345678",      // Thailand
            "+84586012403",      // Vietnam
            "+6587654321",       // Singapore
        ];

        for number in valid_numbers {
            assert!(is_mobile_phone(number, Locale::Any, None).unwrap(), 
                    "Failed for: {}", number);
        }

        // Test invalid numbers
        let invalid_numbers = vec!["", "asdf", "1", "12345"];
        for number in invalid_numbers {
            assert!(!is_mobile_phone(number, Locale::Any, None).unwrap(),
                    "Should have failed for: {}", number);
        }
    }

    #[test]
    fn test_empty_locale_defaults_to_any() {
        // Empty locale should default to 'any'
        assert!(is_mobile_phone("+14155552671", Locale::from(""), None).unwrap());
        assert!(is_mobile_phone("+447911123456", Locale::from(""), None).unwrap());
    }

    #[test]
    fn test_ar_jo() {
        let locale = Locale::from("ar-JO");
        assert!(is_mobile_phone("0796477263", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+962796477263", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("00962786725261", locale, None).unwrap());
    }

    #[test]
    fn test_es_mx() {
        let locale = Locale::from("es-MX");
        assert!(is_mobile_phone("+52019654789321", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("5219654789321", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("65478932", locale, None).unwrap());
    }

    #[test]
    fn test_es_ar() {
        let locale = Locale::from("es-AR");
        assert!(is_mobile_phone("5491143214321", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+5491143214321", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("1143214321", locale, None).unwrap());
    }

    #[test]
    fn test_nl_nl() {
        let locale = Locale::from("nl-NL");
        assert!(is_mobile_phone("0670123456", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("+31612345678", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("012345678", locale, None).unwrap());
    }

    #[test]
    fn test_el_gr() {
        let locale = Locale::from("el-GR");
        assert!(is_mobile_phone("+306944848966", locale.clone(), None).unwrap());
        assert!(is_mobile_phone("6944848966", locale.clone(), None).unwrap());
        assert!(!is_mobile_phone("6924567890", locale, None).unwrap());
    }
}

