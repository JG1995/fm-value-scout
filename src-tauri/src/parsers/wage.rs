use super::FieldValue;

/// Parse a wage string into a `FieldValue::Wage`.
///
/// Handles:
/// - Currency symbol stripping (€, £, $)
/// - Range extraction (takes upper bound from " - " separator)
/// - Denomination suffix stripping (p/w, p/m, p/a, case-insensitive)
/// - Magnitude multipliers (K=1_000, M=1_000_000, B=1_000_000_000, case-insensitive)
/// - Decimal before magnitude (€2.5M → 2_500_000)
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::wage::parse_wage;
/// use fm_valuescout_lib::parsers::FieldValue;
///
/// // Basic with denomination
/// let result = parse_wage("€100K p/w");
/// assert!(matches!(result, FieldValue::Wage { value, .. } if (value - 100_000.0).abs() < 1.0));
///
/// // Range with denomination
/// let result = parse_wage("€160K - €210K p/w");
/// assert!(matches!(result, FieldValue::Wage { value, .. } if (value - 210_000.0).abs() < 1.0));
///
/// // No denomination
/// assert!(matches!(parse_wage("€200K"), FieldValue::Wage { value, denomination: None } if (value - 200_000.0).abs() < 1.0));
///
/// // Empty
/// assert!(matches!(parse_wage(""), FieldValue::Null));
/// ```
pub fn parse_wage(input: &str) -> FieldValue {
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

    // Strip denomination suffix (p/w, p/m, p/a) before extracting magnitude
    let (remainder, denomination) = strip_denomination(s);
    s = remainder;

    if s.is_empty() {
        return FieldValue::Null;
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

    FieldValue::Wage {
        value: value * multiplier,
        denomination,
    }
}

/// Strip a trailing denomination suffix (p/w, p/m, p/a, case-insensitive)
/// from the input, returning the remainder and the lowercase denomination.
///
/// The suffix may optionally be preceded by whitespace.  Returns `None` for
/// the denomination when no recognised suffix is found.
fn strip_denomination(s: &str) -> (&str, Option<String>) {
    let trimmed = s.trim_end();
    let lower = trimmed.to_lowercase();

    for pattern in &["p/w", "p/m", "p/a"] {
        if lower.ends_with(pattern) {
            let rest = trimmed[..trimmed.len() - pattern.len()].trim_end();
            return (rest, Some(pattern.to_string()));
        }
    }

    (s, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to assert a Wage value is approximately expected with expected denomination
    fn assert_wage(result: FieldValue, expected_value: f64, expected_denom: Option<&str>) {
        match result {
            FieldValue::Wage { value, denomination } => {
                let diff = (value - expected_value).abs();
                assert!(diff < 0.001, "expected value {expected_value}, got {value}");
                assert_eq!(
                    denomination.as_deref(),
                    expected_denom,
                    "expected denomination {expected_denom:?}, got {denomination:?}"
                );
            }
            other => panic!(
                "expected FieldValue::Wage {{ value: {expected_value}, denomination: {expected_denom:?} }}, got {other:?}"
            ),
        }
    }

    // --- Task examples ---

    #[test]
    fn test_basic_wage_with_denomination_pw() {
        assert_wage(parse_wage("€100K p/w"), 100_000.0, Some("p/w"));
    }

    #[test]
    fn test_wage_with_denomination_pm() {
        assert_wage(parse_wage("£50K p/m"), 50_000.0, Some("p/m"));
    }

    #[test]
    fn test_wage_with_denomination_pa() {
        assert_wage(parse_wage("£2.5M p/a"), 2_500_000.0, Some("p/a"));
    }

    #[test]
    fn test_wage_no_denomination() {
        assert_wage(parse_wage("€200K"), 200_000.0, None);
    }

    #[test]
    fn test_wage_range_with_denomination() {
        assert_wage(parse_wage("€160K - €210K p/w"), 210_000.0, Some("p/w"));
    }

    #[test]
    fn test_empty_string() {
        assert!(matches!(parse_wage(""), FieldValue::Null));
    }

    #[test]
    fn test_symbol_only() {
        assert!(matches!(parse_wage("€"), FieldValue::Null));
    }

    #[test]
    fn test_denomination_only() {
        assert!(matches!(parse_wage("p/w"), FieldValue::Null));
    }

    // --- Case-insensitive denomination tests ---

    #[test]
    fn test_uppercase_denomination() {
        assert_wage(parse_wage("€100K P/W"), 100_000.0, Some("p/w"));
        assert_wage(parse_wage("€100K P/M"), 100_000.0, Some("p/m"));
        assert_wage(parse_wage("€100K P/A"), 100_000.0, Some("p/a"));
    }

    #[test]
    fn test_mixed_case_denomination() {
        assert_wage(parse_wage("€100K P/w"), 100_000.0, Some("p/w"));
        assert_wage(parse_wage("€100K p/W"), 100_000.0, Some("p/w"));
        assert_wage(parse_wage("€100K P/m"), 100_000.0, Some("p/m"));
        assert_wage(parse_wage("€100K p/M"), 100_000.0, Some("p/m"));
    }

    // --- Whitespace variants ---

    #[test]
    fn test_extra_whitespace_before_denomination() {
        assert_wage(parse_wage("€100K  p/w"), 100_000.0, Some("p/w"));
    }

    #[test]
    fn test_leading_and_trailing_whitespace() {
        assert_wage(parse_wage("  €100K  p/w  "), 100_000.0, Some("p/w"));
    }

    // --- Zero and edge cases ---

    #[test]
    fn test_zero_value_no_denomination() {
        assert_wage(parse_wage("€0"), 0.0, None);
    }

    #[test]
    fn test_zero_value_with_denomination() {
        assert_wage(parse_wage("€0K p/w"), 0.0, Some("p/w"));
    }

    #[test]
    fn test_no_denomination_no_magnitude() {
        assert_wage(parse_wage("€55000000"), 55_000_000.0, None);
    }

    // --- Magnitude case-insensitivity ---

    #[test]
    fn test_lowercase_magnitude() {
        assert_wage(parse_wage("€55m p/w"), 55_000_000.0, Some("p/w"));
        assert_wage(parse_wage("€19.25k p/m"), 19_250.0, Some("p/m"));
        assert_wage(parse_wage("€1.5b p/a"), 1_500_000_000.0, Some("p/a"));
    }

    // --- Negative values ---

    #[test]
    fn test_negative_value() {
        assert_wage(parse_wage("€-5M p/w"), -5_000_000.0, Some("p/w"));
    }

    #[test]
    fn test_negative_value_with_space() {
        assert_wage(parse_wage("€ -5M p/w"), -5_000_000.0, Some("p/w"));
    }

    #[test]
    fn test_negative_decimal() {
        assert_wage(parse_wage("€-19.25K p/w"), -19_250.0, Some("p/w"));
    }

    // --- Symbol and range coverage ---

    #[test]
    fn test_dollar_symbol() {
        assert_wage(parse_wage("$100M p/w"), 100_000_000.0, Some("p/w"));
    }

    #[test]
    fn test_range_no_denomination() {
        assert_wage(parse_wage("€5M - €10M"), 10_000_000.0, None);
    }

    #[test]
    fn test_billions() {
        assert_wage(parse_wage("€1.5B p/a"), 1_500_000_000.0, Some("p/a"));
    }

    #[test]
    fn test_space_after_currency_symbol() {
        assert_wage(parse_wage("€ 100K p/w"), 100_000.0, Some("p/w"));
    }

    // --- Null edge cases ---

    #[test]
    fn test_magnitude_only_is_null() {
        assert!(matches!(parse_wage("M"), FieldValue::Null));
    }

    #[test]
    fn test_whitespace_only() {
        assert!(matches!(parse_wage("   "), FieldValue::Null));
    }

    #[test]
    fn test_trailing_whitespace_after_range() {
        assert_wage(parse_wage("€10M - €20M  "), 20_000_000.0, None);
    }

    // --- Non-mangled scientific notation ---

    #[test]
    fn test_scientific_notation_not_mangled() {
        assert_wage(parse_wage("1e-1"), 0.1, None);
    }
}
