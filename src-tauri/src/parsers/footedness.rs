use super::FieldValue;

/// Parse a footedness description string into a `FieldValue::FootScore`.
///
/// Maps FM footedness text descriptions to numeric scores on a 20-point scale:
///
/// | Description    | Score |
/// |----------------|-------|
/// | Very Strong    | 20    |
/// | Fairly Strong  | 15    |
/// | Reasonable     | 10    |
/// | Weak           | 5     |
///
/// The original trimmed input is always stored in the `raw` field.
/// Unrecognized text results in `score: None`. Empty input returns `FieldValue::Null`.
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::footedness::parse_footedness;
/// use fm_valuescout_lib::parsers::FieldValue;
///
/// let result = parse_footedness("Very Strong");
/// assert!(matches!(result, FieldValue::FootScore { score: Some(20), .. }));
///
/// let result = parse_footedness("fairly strong");
/// assert!(matches!(result, FieldValue::FootScore { score: Some(15), .. }));
///
/// let result = parse_footedness("Reasonable");
/// assert!(matches!(result, FieldValue::FootScore { score: Some(10), .. }));
///
/// let result = parse_footedness("weak");
/// assert!(matches!(result, FieldValue::FootScore { score: Some(5), .. }));
///
/// // Unrecognized
/// let result = parse_footedness("Unknown");
/// assert!(matches!(result, FieldValue::FootScore { score: None, .. }));
///
/// // Empty
/// assert!(matches!(parse_footedness(""), FieldValue::Null));
/// ```
pub fn parse_footedness(input: &str) -> FieldValue {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return FieldValue::Null;
    }

    let score = match trimmed.to_lowercase().as_str() {
        "very strong" => Some(20),
        "fairly strong" => Some(15),
        "reasonable" => Some(10),
        "weak" => Some(5),
        _ => None,
    };

    FieldValue::FootScore {
        score,
        raw: trimmed.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to assert a FootScore value
    fn assert_footscore(result: FieldValue, expected_score: Option<i64>, expected_raw: &str) {
        match result {
            FieldValue::FootScore { score, raw } => {
                assert_eq!(
                    score, expected_score,
                    "expected score {expected_score:?}, got {score:?}"
                );
                assert_eq!(
                    raw, expected_raw,
                    "expected raw {expected_raw:?}, got {raw:?}"
                );
            }
            other => panic!(
                "expected FieldValue::FootScore {{ score: {expected_score:?}, .. }}, got {other:?}"
            ),
        }
    }

    // --- Exact case matches ---

    #[test]
    fn test_very_strong() {
        assert_footscore(parse_footedness("Very Strong"), Some(20), "Very Strong");
    }

    #[test]
    fn test_fairly_strong() {
        assert_footscore(parse_footedness("Fairly Strong"), Some(15), "Fairly Strong");
    }

    #[test]
    fn test_reasonable() {
        assert_footscore(parse_footedness("Reasonable"), Some(10), "Reasonable");
    }

    #[test]
    fn test_weak() {
        assert_footscore(parse_footedness("Weak"), Some(5), "Weak");
    }

    // --- Case-insensitivity ---

    #[test]
    fn test_lowercase() {
        assert_footscore(parse_footedness("very strong"), Some(20), "very strong");
        assert_footscore(parse_footedness("fairly strong"), Some(15), "fairly strong");
        assert_footscore(parse_footedness("reasonable"), Some(10), "reasonable");
        assert_footscore(parse_footedness("weak"), Some(5), "weak");
    }

    #[test]
    fn test_uppercase() {
        assert_footscore(parse_footedness("VERY STRONG"), Some(20), "VERY STRONG");
        assert_footscore(parse_footedness("FAIRLY STRONG"), Some(15), "FAIRLY STRONG");
        assert_footscore(parse_footedness("REASONABLE"), Some(10), "REASONABLE");
        assert_footscore(parse_footedness("WEAK"), Some(5), "WEAK");
    }

    #[test]
    fn test_mixed_case() {
        assert_footscore(parse_footedness("Very strong"), Some(20), "Very strong");
        assert_footscore(parse_footedness("fairly Strong"), Some(15), "fairly Strong");
    }

    // --- Whitespace handling ---

    #[test]
    fn test_leading_whitespace() {
        assert_footscore(parse_footedness("  Very Strong"), Some(20), "Very Strong");
    }

    #[test]
    fn test_trailing_whitespace() {
        assert_footscore(parse_footedness("Weak  "), Some(5), "Weak");
    }

    #[test]
    fn test_both_sides_whitespace() {
        assert_footscore(parse_footedness("  Reasonable  "), Some(10), "Reasonable");
    }

    // --- Unrecognized input ---

    #[test]
    fn test_unrecognized_text() {
        let result = parse_footedness("Unknown");
        assert_footscore(result, None, "Unknown");
    }

    #[test]
    fn test_unrecognized_gibberish() {
        let result = parse_footedness("Extremely Strong");
        assert_footscore(result, None, "Extremely Strong");
    }

    // --- Edge cases ---

    #[test]
    fn test_empty_string() {
        assert!(matches!(parse_footedness(""), FieldValue::Null));
    }

    #[test]
    fn test_whitespace_only() {
        assert!(matches!(parse_footedness("   "), FieldValue::Null));
    }

    #[test]
    fn test_numeric_input_is_unrecognized() {
        let result = parse_footedness("20");
        assert_footscore(result, None, "20");
    }
}
