use super::FieldValue;

/// Parse a currency/market-value string into a `FieldValue::Currency`.
///
/// Handles:
/// - Currency symbol stripping (€, £, $)
/// - Range extraction (takes upper bound)
/// - Magnitude multipliers (K=1_000, M=1_000_000, B=1_000_000_000, case-insensitive)
/// - Decimal before magnitude (€19.25K → 19_250)
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::currency::parse_currency;
/// use fm_valuescout_lib::parsers::FieldValue;
///
/// assert!(matches!(parse_currency("€55M"), FieldValue::Currency(v) if (v - 55_000_000.0).abs() < 1.0));
/// assert!(matches!(parse_currency("€160M - €210M"), FieldValue::Currency(v) if (v - 210_000_000.0).abs() < 1.0));
/// assert!(matches!(parse_currency("€19.25K"), FieldValue::Currency(v) if (v - 19_250.0).abs() < 0.01));
/// assert!(matches!(parse_currency("£0"), FieldValue::Currency(v) if v == 0.0));
/// assert!(matches!(parse_currency(""), FieldValue::Null));
/// ```
pub fn parse_currency(input: &str) -> FieldValue {
    let mut s = input.trim();

    if s.is_empty() {
        return FieldValue::Null;
    }

    // Strip currency symbol prefix (€, £, $) before range detection
    // so we can distinguish negative numbers from ranges.
    s = s
        .strip_prefix('€')
        .or_else(|| s.strip_prefix('£'))
        .or_else(|| s.strip_prefix('$'))
        .unwrap_or(s);

    s = s.trim();
    if s.is_empty() {
        return FieldValue::Null;
    }

    // Handle ranges: extract the upper bound from " - " (space-dash-space) separator.
    // A leading dash after symbol stripping means negative number, not a range.
    if !s.starts_with('-') {
        if let Some(dash_pos) = s.rfind(" - ") {
            s = s[dash_pos + 3..].trim();
            // Strip currency symbol from the right side of the range
            s = s
                .strip_prefix('€')
                .or_else(|| s.strip_prefix('£'))
                .or_else(|| s.strip_prefix('$'))
                .unwrap_or(s);
            s = s.trim();
            if s.is_empty() {
                return FieldValue::Null;
            }
        }
    }

    // Check for magnitude suffix (case-insensitive K/M/B)
    let last_char = s.chars().last().unwrap();
    let (numeric_str, multiplier) = match last_char.to_ascii_uppercase() {
        'K' => (&s[..s.len() - 1], 1_000.0),
        'M' => (&s[..s.len() - 1], 1_000_000.0),
        'B' => (&s[..s.len() - 1], 1_000_000_000.0),
        _ => (s, 1.0),
    };

    let numeric_str = numeric_str.trim();
    if numeric_str.is_empty() {
        return FieldValue::Null;
    }

    let value: f64 = match numeric_str.parse() {
        Ok(v) => v,
        Err(_) => return FieldValue::Null,
    };

    FieldValue::Currency(value * multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to assert a Currency value is approximately expected
    fn assert_currency(result: FieldValue, expected: f64) {
        match result {
            FieldValue::Currency(v) => {
                let diff = (v - expected).abs();
                assert!(diff < 0.001, "expected {expected}, got {v}");
            }
            other => panic!("expected FieldValue::Currency({expected}), got {other:?}"),
        }
    }

    #[test]
    fn test_basic_magnitude_m() {
        assert_currency(parse_currency("€55M"), 55_000_000.0);
    }

    #[test]
    fn test_range_upper_bound() {
        assert_currency(parse_currency("€160M - €210M"), 210_000_000.0);
    }

    #[test]
    fn test_decimal_before_magnitude() {
        assert_currency(parse_currency("€19.25K"), 19_250.0);
    }

    #[test]
    fn test_zero_value() {
        assert_currency(parse_currency("£0"), 0.0);
        assert_currency(parse_currency("€0"), 0.0);
        assert_currency(parse_currency("$0"), 0.0);
        assert_currency(parse_currency("0"), 0.0);
    }

    #[test]
    fn test_empty_string() {
        assert!(matches!(parse_currency(""), FieldValue::Null));
    }

    #[test]
    fn test_whitespace_only() {
        assert!(matches!(parse_currency("   "), FieldValue::Null));
    }

    #[test]
    fn test_dollar_symbol() {
        assert_currency(parse_currency("$100M"), 100_000_000.0);
    }

    #[test]
    fn test_pound_symbol_range() {
        assert_currency(parse_currency("£5M - £10M"), 10_000_000.0);
    }

    #[test]
    fn test_billions() {
        assert_currency(parse_currency("€1.5B"), 1_500_000_000.0);
    }

    #[test]
    fn test_lowercase_magnitude() {
        assert_currency(parse_currency("€55m"), 55_000_000.0);
        assert_currency(parse_currency("€19.25k"), 19_250.0);
        assert_currency(parse_currency("€1.5b"), 1_500_000_000.0);
    }

    #[test]
    fn test_no_magnitude_suffix() {
        assert_currency(parse_currency("55000000"), 55_000_000.0);
    }

    #[test]
    fn test_extra_whitespace() {
        assert_currency(parse_currency("  €12.5B  "), 12_500_000_000.0);
    }

    #[test]
    fn test_symbol_only_is_null() {
        assert!(matches!(parse_currency("€"), FieldValue::Null));
    }

    #[test]
    fn test_magnitude_only_is_null() {
        assert!(matches!(parse_currency("M"), FieldValue::Null));
    }

    #[test]
    fn test_trailing_whitespace_after_range() {
        assert_currency(parse_currency("€10M - €20M  "), 20_000_000.0);
    }

    #[test]
    fn test_space_after_currency_symbol() {
        assert_currency(parse_currency("€ 55M"), 55_000_000.0);
    }

    #[test]
    fn test_k_multiplier() {
        assert_currency(parse_currency("€500K"), 500_000.0);
    }

    #[test]
    fn test_negative_value() {
        assert_currency(parse_currency("€-5M"), -5_000_000.0);
    }

    #[test]
    fn test_negative_value_with_space() {
        assert_currency(parse_currency("€ -5M"), -5_000_000.0);
    }

    #[test]
    fn test_negative_decimal() {
        assert_currency(parse_currency("€-19.25K"), -19_250.0);
    }

    #[test]
    fn test_scientific_notation_not_mangled() {
        // The old rfind('-') consumed the exponent dash, producing 1.0 instead of 0.1
        assert_currency(parse_currency("1e-1"), 0.1);
    }
}
