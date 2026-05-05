use super::FieldValue;

/// Split a comma-separated Football Manager position string into a
/// `FieldValue::Position` containing a list of individual position codes.
///
/// Whitespace around each segment is trimmed. Empty segments (e.g. from
/// leading/trailing commas or consecutive commas) are skipped.
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::FieldValue;
/// use fm_valuescout_lib::parsers::positions::parse_positions;
///
/// // Single position
/// assert!(matches!(
///     parse_positions("ST"),
///     FieldValue::Position(v) if v == vec!["ST"]
/// ));
///
/// // Multiple positions
/// assert!(matches!(
///     parse_positions("ST, AML, AMR"),
///     FieldValue::Position(v) if v == vec!["ST", "AML", "AMR"]
/// ));
///
/// // Empty input
/// assert!(matches!(parse_positions(""), FieldValue::Null));
///
/// // Whitespace only
/// assert!(matches!(parse_positions("   "), FieldValue::Null));
///
/// // Double commas produce no empty entries
/// assert!(matches!(
///     parse_positions("ST,,AML"),
///     FieldValue::Position(v) if v == vec!["ST", "AML"]
/// ));
/// ```
pub fn parse_positions(input: &str) -> FieldValue {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return FieldValue::Null;
    }

    let positions: Vec<String> = trimmed
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if positions.is_empty() {
        return FieldValue::Null;
    }

    FieldValue::Position(positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Happy path ----

    #[test]
    fn test_single_position() {
        let result = parse_positions("ST");
        assert!(matches!(result, FieldValue::Position(v) if v == vec!["ST"]));
    }

    #[test]
    fn test_multiple_positions() {
        let result = parse_positions("ST, AML, AMR");
        assert!(matches!(result, FieldValue::Position(v) if v == vec!["ST", "AML", "AMR"]));
    }

    #[test]
    fn test_two_positions() {
        let result = parse_positions("CM, DM");
        assert!(matches!(result, FieldValue::Position(v) if v == vec!["CM", "DM"]));
    }

    // ---- Whitespace handling ----

    #[test]
    fn test_whitespace_around_commas() {
        let result = parse_positions("  ST  ,  AML  ,  AMR  ");
        assert!(matches!(result, FieldValue::Position(v) if v == vec!["ST", "AML", "AMR"]));
    }

    #[test]
    fn test_leading_whitespace() {
        let result = parse_positions("  ST, AML");
        assert!(matches!(result, FieldValue::Position(v) if v == vec!["ST", "AML"]));
    }

    #[test]
    fn test_trailing_whitespace() {
        let result = parse_positions("ST, AML  ");
        assert!(matches!(result, FieldValue::Position(v) if v == vec!["ST", "AML"]));
    }

    // ---- Empty / edge cases ----

    #[test]
    fn test_empty_string() {
        assert!(matches!(parse_positions(""), FieldValue::Null));
    }

    #[test]
    fn test_whitespace_only() {
        assert!(matches!(parse_positions("   "), FieldValue::Null));
    }

    #[test]
    fn test_double_comma_skips_empty() {
        let result = parse_positions("ST,,AML");
        assert!(matches!(result, FieldValue::Position(v) if v == vec!["ST", "AML"]));
    }

    #[test]
    fn test_leading_comma_skips_empty() {
        let result = parse_positions(",ST, AML");
        assert!(matches!(result, FieldValue::Position(v) if v == vec!["ST", "AML"]));
    }

    #[test]
    fn test_trailing_comma_skips_empty() {
        let result = parse_positions("ST, AML,");
        assert!(matches!(result, FieldValue::Position(v) if v == vec!["ST", "AML"]));
    }

    #[test]
    fn test_only_commas_returns_null() {
        assert!(matches!(parse_positions(",,,"), FieldValue::Null));
    }

    #[test]
    fn test_only_commas_with_spaces_returns_null() {
        assert!(matches!(parse_positions("  ,  ,  "), FieldValue::Null));
    }

    #[test]
    fn test_single_position_with_underscore() {
        let result = parse_positions("AM_C");
        assert!(matches!(result, FieldValue::Position(v) if v == vec!["AM_C"]));
    }

    #[test]
    fn test_in_game_position_format() {
        let result = parse_positions("D (C), DM, M (C)");
        assert!(matches!(result, FieldValue::Position(v) if v == vec!["D (C)", "DM", "M (C)"]));
    }
}
