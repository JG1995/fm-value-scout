use super::FieldValue;

/// Parse an FM appearances string into a `FieldValue::Appearances`.
///
/// Supports two input formats:
///
/// | Format        | Example    | Result                     |
/// |---------------|------------|----------------------------|
/// | "starts (subs)" | `"43 (3)"` | starts=43, subs=3          |
/// | Bare number   | `"43"`     | starts=43, subs=0          |
///
/// Leading/trailing whitespace is stripped before parsing.
/// Empty or whitespace-only input returns `FieldValue::Null`.
/// Unparseable input returns `FieldValue::Null`.
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::appearances::parse_appearances;
/// use fm_valuescout_lib::parsers::FieldValue;
///
/// let result = parse_appearances("43 (3)");
/// assert!(matches!(result, FieldValue::Appearances { starts: 43, subs: 3 }));
///
/// let result = parse_appearances("43");
/// assert!(matches!(result, FieldValue::Appearances { starts: 43, subs: 0 }));
///
/// assert!(matches!(parse_appearances(""), FieldValue::Null));
/// ```
pub fn parse_appearances(input: &str) -> FieldValue {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return FieldValue::Null;
    }

    if let Some(paren_idx) = trimmed.find('(') {
        // Verify closing paren exists; if not, treat as unparseable.
        if !trimmed[paren_idx..].contains(')') {
            return FieldValue::Null;
        }

        // Format: "starts (subs)"
        let before = trimmed[..paren_idx].trim();
        let after = trimmed[paren_idx + 1..]
            .trim_end()
            .trim_end_matches(')')
            .trim();

        let starts: i64 = match before.parse() {
            Ok(n) => n,
            Err(_) => return FieldValue::Null,
        };
        let subs: i64 = match after.parse() {
            Ok(n) => n,
            Err(_) => return FieldValue::Null,
        };

        FieldValue::Appearances { starts, subs }
    } else {
        // Bare number: "starts"
        match trimmed.parse::<i64>() {
            Ok(starts) => FieldValue::Appearances { starts, subs: 0 },
            Err(_) => FieldValue::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to assert an Appearances value
    fn assert_appearances(result: FieldValue, expected_starts: i64, expected_subs: i64) {
        match result {
            FieldValue::Appearances { starts, subs } => {
                assert_eq!(
                    starts, expected_starts,
                    "expected starts {expected_starts}, got {starts}"
                );
                assert_eq!(
                    subs, expected_subs,
                    "expected subs {expected_subs}, got {subs}"
                );
            }
            other => panic!(
                "expected FieldValue::Appearances {{ starts: {expected_starts}, subs: {expected_subs} }}, \
                 got {other:?}"
            ),
        }
    }

    // --- Standard "starts (subs)" format ---

    #[test]
    fn test_standard_format() {
        assert_appearances(parse_appearances("43 (3)"), 43, 3);
    }

    #[test]
    fn test_large_numbers() {
        assert_appearances(parse_appearances("999 (99)"), 999, 99);
    }

    #[test]
    fn test_zero_subs() {
        assert_appearances(parse_appearances("43 (0)"), 43, 0);
    }

    #[test]
    fn test_zero_starts() {
        assert_appearances(parse_appearances("0 (5)"), 0, 5);
    }

    // --- Bare number format ---

    #[test]
    fn test_bare_number() {
        assert_appearances(parse_appearances("43"), 43, 0);
    }

    #[test]
    fn test_bare_zero() {
        assert_appearances(parse_appearances("0"), 0, 0);
    }

    #[test]
    fn test_bare_large_number() {
        assert_appearances(parse_appearances("999"), 999, 0);
    }

    // --- Whitespace handling ---

    #[test]
    fn test_leading_whitespace() {
        assert_appearances(parse_appearances("  43 (3)"), 43, 3);
    }

    #[test]
    fn test_trailing_whitespace() {
        assert_appearances(parse_appearances("43 (3)  "), 43, 3);
    }

    #[test]
    fn test_both_sides_whitespace() {
        assert_appearances(parse_appearances("  43 (3)  "), 43, 3);
    }

    #[test]
    fn test_whitespace_around_parens() {
        assert_appearances(parse_appearances("43(3)"), 43, 3);
    }

    #[test]
    fn test_extra_whitespace_inside_parens() {
        assert_appearances(parse_appearances("43 ( 3 )"), 43, 3);
    }

    #[test]
    fn test_bare_number_with_whitespace() {
        assert_appearances(parse_appearances("  43  "), 43, 0);
    }

    // --- Empty / null cases ---

    #[test]
    fn test_empty_string() {
        assert!(matches!(parse_appearances(""), FieldValue::Null));
    }

    #[test]
    fn test_whitespace_only() {
        assert!(matches!(parse_appearances("   "), FieldValue::Null));
    }

    // --- Unparseable input ---

    #[test]
    fn test_gibberish() {
        assert!(matches!(parse_appearances("hello"), FieldValue::Null));
    }

    #[test]
    fn test_non_numeric_starts() {
        assert!(matches!(parse_appearances("abc (3)"), FieldValue::Null));
    }

    #[test]
    fn test_non_numeric_subs() {
        assert!(matches!(parse_appearances("43 (abc)"), FieldValue::Null));
    }

    #[test]
    fn test_missing_parens_with_text() {
        assert!(matches!(parse_appearances("43 abc"), FieldValue::Null));
    }

    #[test]
    fn test_unclosed_paren() {
        assert!(matches!(parse_appearances("43 (3"), FieldValue::Null));
    }
}
