use super::FieldValue;

/// Controls how a parsed numeric value is rounded and which `FieldValue` variant is returned.
#[derive(Clone, Debug, PartialEq)]
pub enum UnitPrecision {
    /// Parse as an integer (no decimal), return `FieldValue::Int`.
    /// Used for height fields.
    Height,
    /// Parse as f64, round to 1 decimal place, return `FieldValue::Float`.
    /// Used for distance fields.
    Distance,
    /// Parse as f64, round to 2 decimal places, return `FieldValue::Float`.
    /// Used for xG fields.
    Xg,
}

/// Parse a string with an optional unit suffix (cm, km) into a `FieldValue`
/// with field-specific precision.
///
/// Supported units: `cm`, `km` (case-insensitive).
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::units::{parse_unit_field, UnitPrecision};
/// use fm_valuescout_lib::parsers::FieldValue;
///
/// // Height: integer, no decimal
/// let h = parse_unit_field("185cm", UnitPrecision::Height);
/// assert!(matches!(h, FieldValue::Int(185)));
///
/// // Distance: float, 1 decimal place
/// let d = parse_unit_field("12.5km", UnitPrecision::Distance);
/// assert!(matches!(d, FieldValue::Float(v) if (v - 12.5).abs() < 0.01));
///
/// // xG: float, 2 decimal places
/// let x = parse_unit_field("0.753", UnitPrecision::Xg);
/// assert!(matches!(x, FieldValue::Float(v) if (v - 0.75).abs() < 0.01));
///
/// // Empty → Null
/// assert!(matches!(parse_unit_field("", UnitPrecision::Height), FieldValue::Null));
/// ```
pub fn parse_unit_field(input: &str, precision: UnitPrecision) -> FieldValue {
    let s = input.trim();

    if s.is_empty() {
        return FieldValue::Null;
    }

    // Detect and strip trailing unit suffix (case-insensitive).
    // Work on a lowercased copy for matching, but slice the original string
    // so we preserve any mixed-case input for error messages.
    let lower = s.to_lowercase();
    let numeric_str = if lower.ends_with("cm") {
        s[..s.len() - 2].trim()
    } else if lower.ends_with("km") {
        s[..s.len() - 2].trim()
    } else {
        s
    };

    if numeric_str.is_empty() {
        return FieldValue::Null;
    }

    let value: f64 = match numeric_str.parse() {
        Ok(v) => v,
        Err(_) => return FieldValue::Null,
    };

    match precision {
        UnitPrecision::Height => {
            // Round to nearest integer and return Int
            FieldValue::Int(value.round() as i64)
        }
        UnitPrecision::Distance => {
            // Round to 1 decimal place
            let rounded = (value * 10.0).round() / 10.0;
            FieldValue::Float(rounded)
        }
        UnitPrecision::Xg => {
            // Round to 2 decimal places
            let rounded = (value * 100.0).round() / 100.0;
            FieldValue::Float(rounded)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to assert an Int value
    fn assert_int(result: FieldValue, expected: i64) {
        match result {
            FieldValue::Int(v) => assert_eq!(v, expected, "expected Int({expected}), got Int({v})"),
            other => panic!("expected FieldValue::Int({expected}), got {other:?}"),
        }
    }

    // Helper to assert a Float value within tolerance
    fn assert_float(result: FieldValue, expected: f64) {
        match result {
            FieldValue::Float(v) => {
                let diff = (v - expected).abs();
                assert!(diff < 0.001, "expected Float({expected}), got Float({v})");
            }
            other => panic!("expected FieldValue::Float({expected}), got {other:?}"),
        }
    }

    // --- Height (Int) tests ---

    #[test]
    fn test_height_cm() {
        assert_int(parse_unit_field("185cm", UnitPrecision::Height), 185);
    }

    #[test]
    fn test_height_cm_uppercase() {
        assert_int(parse_unit_field("185CM", UnitPrecision::Height), 185);
    }

    #[test]
    fn test_height_cm_mixed_case() {
        assert_int(parse_unit_field("185Cm", UnitPrecision::Height), 185);
    }

    #[test]
    fn test_height_no_unit() {
        assert_int(parse_unit_field("185", UnitPrecision::Height), 185);
    }

    #[test]
    fn test_height_negative() {
        assert_int(parse_unit_field("-185cm", UnitPrecision::Height), -185);
    }

    #[test]
    fn test_height_decimal_input_rounded() {
        // 185.7 rounds to 186
        assert_int(parse_unit_field("185.7cm", UnitPrecision::Height), 186);
    }

    #[test]
    fn test_height_decimal_input_rounded_down() {
        // 185.3 rounds to 185
        assert_int(parse_unit_field("185.3cm", UnitPrecision::Height), 185);
    }

    #[test]
    fn test_height_with_whitespace() {
        assert_int(parse_unit_field("  185cm  ", UnitPrecision::Height), 185);
    }

    #[test]
    fn test_height_zero() {
        assert_int(parse_unit_field("0cm", UnitPrecision::Height), 0);
    }

    // --- Distance (Float, 1dp) tests ---

    #[test]
    fn test_distance_km() {
        assert_float(parse_unit_field("12.5km", UnitPrecision::Distance), 12.5);
    }

    #[test]
    fn test_distance_km_uppercase() {
        assert_float(parse_unit_field("12.5KM", UnitPrecision::Distance), 12.5);
    }

    #[test]
    fn test_distance_no_unit() {
        assert_float(parse_unit_field("12.5", UnitPrecision::Distance), 12.5);
    }

    #[test]
    fn test_distance_negative() {
        assert_float(parse_unit_field("-5.3km", UnitPrecision::Distance), -5.3);
    }

    #[test]
    fn test_distance_rounds_to_1dp() {
        // 12.55 rounded to 1dp → 12.6
        assert_float(parse_unit_field("12.55km", UnitPrecision::Distance), 12.6);
    }

    #[test]
    fn test_distance_rounds_down_to_1dp() {
        // 12.54 rounded to 1dp → 12.5
        assert_float(parse_unit_field("12.543km", UnitPrecision::Distance), 12.5);
    }

    #[test]
    fn test_distance_whole_number() {
        assert_float(parse_unit_field("10km", UnitPrecision::Distance), 10.0);
    }

    #[test]
    fn test_distance_with_whitespace() {
        assert_float(
            parse_unit_field("  12.5km  ", UnitPrecision::Distance),
            12.5,
        );
    }

    #[test]
    fn test_distance_zero() {
        assert_float(parse_unit_field("0.0km", UnitPrecision::Distance), 0.0);
    }

    // --- xG (Float, 2dp) tests ---

    #[test]
    fn test_xg_no_unit() {
        assert_float(parse_unit_field("0.753", UnitPrecision::Xg), 0.75);
    }

    #[test]
    fn test_xg_rounds_to_2dp() {
        // 0.756 rounded to 2dp → 0.76
        assert_float(parse_unit_field("0.756", UnitPrecision::Xg), 0.76);
    }

    #[test]
    fn test_xg_rounds_down_to_2dp() {
        // 0.754 rounded to 2dp → 0.75
        assert_float(parse_unit_field("0.754", UnitPrecision::Xg), 0.75);
    }

    #[test]
    fn test_xg_negative() {
        assert_float(parse_unit_field("-0.5", UnitPrecision::Xg), -0.5);
    }

    #[test]
    fn test_xg_whole_number() {
        assert_float(parse_unit_field("1", UnitPrecision::Xg), 1.0);
    }

    #[test]
    fn test_xg_with_whitespace() {
        assert_float(parse_unit_field("  0.753  ", UnitPrecision::Xg), 0.75);
    }

    #[test]
    fn test_xg_zero() {
        assert_float(parse_unit_field("0", UnitPrecision::Xg), 0.0);
    }

    #[test]
    fn test_xg_small_value() {
        assert_float(parse_unit_field("0.0123", UnitPrecision::Xg), 0.01);
    }

    // --- Null / edge cases ---

    #[test]
    fn test_empty_string() {
        assert!(matches!(
            parse_unit_field("", UnitPrecision::Height),
            FieldValue::Null
        ));
    }

    #[test]
    fn test_whitespace_only() {
        assert!(matches!(
            parse_unit_field("   ", UnitPrecision::Height),
            FieldValue::Null
        ));
    }

    #[test]
    fn test_unit_only_is_null() {
        assert!(matches!(
            parse_unit_field("cm", UnitPrecision::Height),
            FieldValue::Null
        ));
        assert!(matches!(
            parse_unit_field("km", UnitPrecision::Distance),
            FieldValue::Null
        ));
    }

    #[test]
    fn test_unparseable_is_null() {
        assert!(matches!(
            parse_unit_field("abc", UnitPrecision::Height),
            FieldValue::Null
        ));
        assert!(matches!(
            parse_unit_field("abc123", UnitPrecision::Distance),
            FieldValue::Null
        ));
    }

    #[test]
    fn test_numeric_with_garbage_suffix() {
        // "185xyz" doesn't end with cm/km, parse "185xyz" as f64 → fails → Null
        assert!(matches!(
            parse_unit_field("185xyz", UnitPrecision::Height),
            FieldValue::Null
        ));
    }

    #[test]
    fn test_all_precisions_on_same_input() {
        let input = "185.7cm";
        assert_int(parse_unit_field(input, UnitPrecision::Height), 186);
        assert_float(parse_unit_field(input, UnitPrecision::Distance), 185.7);
        assert_float(parse_unit_field(input, UnitPrecision::Xg), 185.70);
    }

    #[test]
    fn test_negative_float_with_unit() {
        assert_float(
            parse_unit_field("-12.345km", UnitPrecision::Distance),
            -12.3,
        );
    }
}
