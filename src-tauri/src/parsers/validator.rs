/// CSV row validator for FM data import.
///
/// Performs pre-parse validation on raw CSV rows before they are handed off
/// to the field parsers. Validates UID presence/uniqueness, player name
/// presence, and basic numeric field sanity.
///
/// # Organization
///
/// * [`ValidationResult`] — validation outcome for a single row
/// * [`find_column_index`] — case-insensitive header lookup helper
/// * [`validate_row`] — main entry point, runs all validation checks
use std::collections::HashSet;

// ---------------------------------------------------------------------------
// ValidationResult
// ---------------------------------------------------------------------------

/// The outcome of validating a single CSV row.
///
/// When `is_valid` is `false`, `rejection_reasons` contains one or more
/// human-readable descriptions of what failed.
#[derive(Clone, Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub rejection_reasons: Vec<String>,
}

// ---------------------------------------------------------------------------
// Column lookup
// ---------------------------------------------------------------------------

/// Find the index of a column in the header by name (case-insensitive).
///
/// Returns `Some(index)` for the first header whose case-folded value exactly
/// matches the case-folded `name`, or `None` when no column matches.
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::validator::find_column_index;
///
/// let headers = vec![
///     "UID".to_string(),
///     "Name".to_string(),
///     "Age".to_string(),
/// ];
///
/// assert_eq!(find_column_index(&headers, "UID"), Some(0));
/// assert_eq!(find_column_index(&headers, "uid"), Some(0));
/// assert_eq!(find_column_index(&headers, "Name"), Some(1));
/// assert_eq!(find_column_index(&headers, "name"), Some(1));
/// assert_eq!(find_column_index(&headers, "AGE"), Some(2));
/// assert_eq!(find_column_index(&headers, "Club"), None);
/// ```
pub fn find_column_index(headers: &[String], name: &str) -> Option<usize> {
    let lower_name = name.to_lowercase();
    headers.iter().position(|h| h.to_lowercase() == lower_name)
}

// ---------------------------------------------------------------------------
// Non-numeric column set
// ---------------------------------------------------------------------------

/// Columns that are expected to contain string (non-numeric) values and should
/// be skipped during the numeric-format validation pass.
///
/// All comparisons are case-insensitive via [`is_non_numeric_column`].
const NON_NUMERIC_COLUMNS: &[&str] = &[
    "uid",
    "name",
    "player name",
    "player_name",
    "nation",
    "nationality",
    "second nationality",
    "second_nationality",
    "secondary nationality",
    "second nat",
    "2nd nation",
    "positions",
    "position",
    "club",
    "expires",
    "contract expiry",
    "contract expiration",
    "left foot",
    "left_foot",
    "right foot",
    "right_foot",
    "wage",
    "value",
    "transfer value",
    "transfer_value",
    "height",
    "weight",
    "distance",
    "dist",
    "starts",
    "apps",
    "subs",
];

/// Returns `true` when `col` is a column whose values are expected to be
/// non-numeric (free-form text, dates, currency strings, unit-suffixed
/// measurements, etc.).
fn is_non_numeric_column(col: &str) -> bool {
    let lower = col.to_lowercase();
    NON_NUMERIC_COLUMNS.contains(&lower.as_str())
}

// ---------------------------------------------------------------------------
// validate_row
// ---------------------------------------------------------------------------

/// Validate a single CSV row against the expected FM data schema.
///
/// Checks performed in order:
///
/// 1. **UID column exists** — the header must contain a `UID` column
///    (case-insensitive). If missing, the check is fatal and returns
///    immediately with a single rejection reason.
/// 2. **UID not empty** — the UID field must be non-blank.
/// 3. **UID not duplicate** — the UID must not have been seen in a prior
///    valid row (tracked via the `seen_uids` set).
/// 4. **Name column exists** — the header must contain a `Name` or
///    `Player Name` column (case-insensitive).
/// 5. **Name not empty** — the player name field must be non-blank.
/// 6. **Numeric field format** — every column that is *not* in the
///    non-numeric set must have a value that is empty (treated as
///    missing/NULL) or parses as a valid `f64`.
///
/// On success (all checks pass), the UID is inserted into `seen_uids` so
/// that subsequent rows can be checked for duplicates.
///
/// # Arguments
///
/// * `fields` — Field values of the current CSV row, in column order.
/// * `headers` — Column headers from the CSV (must be the same length as
///   or longer than `fields` for meaningful validation).
/// * `seen_uids` — Mutable set tracking UIDs already seen in the current
///   import session.
/// * `row_index` — 1-based row number used in error messages (e.g. for
///   display to the user or logging).
///
/// # Returns
///
/// A [`ValidationResult`] describing the outcome.
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::validator::validate_row;
/// use std::collections::HashSet;
///
/// let headers = vec![
///     "UID".to_string(),
///     "Name".to_string(),
///     "Age".to_string(),
///     "Goals".to_string(),
/// ];
///
/// let fields = vec![
///     "1001".to_string(),
///     "Lionel Messi".to_string(),
///     "37".to_string(),
///     "".to_string(),       // empty → OK (treated as NULL)
/// ];
///
/// let mut seen = HashSet::new();
/// let result = validate_row(&fields, &headers, &mut seen, 2);
/// assert!(result.is_valid);
/// assert!(result.rejection_reasons.is_empty());
/// assert!(seen.contains("1001"));
/// ```
pub fn validate_row(
    fields: &[String],
    headers: &[String],
    seen_uids: &mut HashSet<String>,
    row_index: usize,
) -> ValidationResult {
    let mut rejection_reasons: Vec<String> = Vec::new();

    // ---- 1. UID column must exist ----
    let uid_col = match find_column_index(headers, "UID") {
        Some(idx) => idx,
        None => {
            rejection_reasons.push("Missing UID column in header".to_string());
            return ValidationResult {
                is_valid: false,
                rejection_reasons,
            };
        }
    };

    // ---- 2. UID must be non-empty ----
    let uid_value = fields.get(uid_col).map(|s| s.trim()).unwrap_or("");

    if uid_value.is_empty() {
        rejection_reasons.push(format!("Missing UID at row {row_index}"));
    }

    // ---- 3. UID must not be duplicate ----
    if !uid_value.is_empty() && seen_uids.contains(uid_value) {
        rejection_reasons.push(format!("Duplicate UID: {uid_value}"));
    }

    // ---- 4. Name column must exist ----
    let name_col =
        find_column_index(headers, "Name").or_else(|| find_column_index(headers, "Player Name"));

    let name_col = match name_col {
        Some(idx) => idx,
        None => {
            rejection_reasons.push("Missing Name column in header".to_string());
            // Continue checking numeric fields even when Name is missing.
            // Must construct result here because we still have work below.
            // Instead, we push the reason and continue.
            // But we need to handle the gap — we can't check the name field.
            // Move on to numeric checks.
            //
            // We use a sentinel so the name-empty check is skipped when
            // the column does not exist.
            usize::MAX
        }
    };

    // ---- 5. Name must be non-empty (if column exists) ----
    if name_col != usize::MAX {
        let name_value = fields.get(name_col).map(|s| s.trim()).unwrap_or("");
        if name_value.is_empty() {
            rejection_reasons.push("Missing player name".to_string());
        }
    }

    // ---- 6. Validate numeric columns ----
    for (i, header) in headers.iter().enumerate() {
        if is_non_numeric_column(header) {
            continue;
        }
        if let Some(value) = fields.get(i) {
            let trimmed = value.trim();
            if !trimmed.is_empty() && trimmed.parse::<f64>().is_err() {
                rejection_reasons.push(format!(
                    "Invalid numeric value for column '{}': '{}'",
                    header, trimmed,
                ));
            }
        }
    }

    // ---- 7. Build result ----
    let is_valid = rejection_reasons.is_empty();

    // Only record the UID as seen when the row is fully valid.
    if is_valid && !uid_value.is_empty() {
        seen_uids.insert(uid_value.to_string());
    }

    ValidationResult {
        is_valid,
        rejection_reasons,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- find_column_index ----

    #[test]
    fn test_find_column_index_exact_match() {
        let headers = vec!["UID".to_string(), "Age".to_string()];
        assert_eq!(find_column_index(&headers, "UID"), Some(0));
    }

    #[test]
    fn test_find_column_index_case_insensitive() {
        let headers = vec!["uid".to_string(), "Name".to_string()];
        assert_eq!(find_column_index(&headers, "UID"), Some(0));
        assert_eq!(find_column_index(&headers, "name"), Some(1));
    }

    #[test]
    fn test_find_column_index_not_found() {
        let headers = vec!["UID".to_string(), "Name".to_string()];
        assert_eq!(find_column_index(&headers, "Club"), None);
    }

    #[test]
    fn test_find_column_index_empty_headers() {
        let headers: Vec<String> = vec![];
        assert_eq!(find_column_index(&headers, "UID"), None);
    }

    // ---- validate_row: basic valid ----

    #[test]
    fn test_valid_row() {
        let headers = vec![
            "UID".to_string(),
            "Name".to_string(),
            "Age".to_string(),
            "Goals".to_string(),
        ];
        let fields = vec![
            "101".to_string(),
            "Test Player".to_string(),
            "25".to_string(),
            "10.5".to_string(),
        ];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 1);
        assert!(result.is_valid);
        assert!(result.rejection_reasons.is_empty());
        assert!(seen.contains("101"));
    }

    #[test]
    fn test_valid_row_empty_numeric_is_ok() {
        let headers = vec!["UID".to_string(), "Name".to_string(), "Goals".to_string()];
        let fields = vec![
            "102".to_string(),
            "Another Player".to_string(),
            "".to_string(),
        ];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 2);
        assert!(result.is_valid);
        assert!(result.rejection_reasons.is_empty());
    }

    // ---- validate_row: UID checks ----

    #[test]
    fn test_missing_uid_column() {
        let headers = vec!["Name".to_string(), "Age".to_string()];
        let fields = vec!["Test".to_string(), "25".to_string()];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 1);
        assert!(!result.is_valid);
        assert!(result
            .rejection_reasons
            .contains(&"Missing UID column in header".to_string()));
    }

    #[test]
    fn test_empty_uid() {
        let headers = vec!["UID".to_string(), "Name".to_string()];
        let fields = vec!["".to_string(), "Test".to_string()];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 5);
        assert!(!result.is_valid);
        assert!(result
            .rejection_reasons
            .contains(&"Missing UID at row 5".to_string()));
    }

    #[test]
    fn test_whitespace_uid() {
        let headers = vec!["UID".to_string(), "Name".to_string()];
        let fields = vec!["   ".to_string(), "Test".to_string()];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 3);
        assert!(!result.is_valid);
        assert!(result
            .rejection_reasons
            .contains(&"Missing UID at row 3".to_string()));
    }

    #[test]
    fn test_duplicate_uid() {
        let headers = vec!["UID".to_string(), "Name".to_string()];
        let fields1 = vec!["101".to_string(), "First".to_string()];
        let fields2 = vec!["101".to_string(), "Duplicate".to_string()];
        let mut seen = HashSet::new();

        let r1 = validate_row(&fields1, &headers, &mut seen, 1);
        assert!(r1.is_valid);

        let r2 = validate_row(&fields2, &headers, &mut seen, 2);
        assert!(!r2.is_valid);
        assert!(r2
            .rejection_reasons
            .contains(&"Duplicate UID: 101".to_string()));
    }

    // ---- validate_row: Name checks ----

    #[test]
    fn test_missing_name_column() {
        let headers = vec!["UID".to_string(), "Age".to_string()];
        let fields = vec!["101".to_string(), "25".to_string()];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 1);
        assert!(!result.is_valid);
        assert!(result
            .rejection_reasons
            .contains(&"Missing Name column in header".to_string()));
    }

    #[test]
    fn test_missing_player_name_column() {
        let headers = vec![
            "UID".to_string(),
            "Player Name".to_string(),
            "Age".to_string(),
        ];
        let fields = vec![
            "101".to_string(),
            "Test Player".to_string(),
            "25".to_string(),
        ];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 1);
        assert!(result.is_valid);
    }

    #[test]
    fn test_empty_name() {
        let headers = vec!["UID".to_string(), "Name".to_string()];
        let fields = vec!["101".to_string(), "".to_string()];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 1);
        assert!(!result.is_valid);
        assert!(result
            .rejection_reasons
            .contains(&"Missing player name".to_string()));
    }

    #[test]
    fn test_whitespace_name() {
        let headers = vec!["UID".to_string(), "Name".to_string()];
        let fields = vec!["101".to_string(), "   ".to_string()];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 1);
        assert!(!result.is_valid);
        assert!(result
            .rejection_reasons
            .contains(&"Missing player name".to_string()));
    }

    // ---- validate_row: numeric validation ----

    #[test]
    fn test_invalid_numeric_value() {
        let headers = vec!["UID".to_string(), "Name".to_string(), "Age".to_string()];
        let fields = vec![
            "101".to_string(),
            "Test".to_string(),
            "not_a_number".to_string(),
        ];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 1);
        assert!(!result.is_valid);
        assert!(result
            .rejection_reasons
            .contains(&"Invalid numeric value for column 'Age': 'not_a_number'".to_string()));
    }

    #[test]
    fn test_non_numeric_column_skipped() {
        let headers = vec![
            "UID".to_string(),
            "Name".to_string(),
            "Nation".to_string(),
            "Positions".to_string(),
            "Club".to_string(),
            "Expires".to_string(),
        ];
        // String values in non-numeric columns should NOT trigger rejection
        let fields = vec![
            "101".to_string(),
            "Test".to_string(),
            "Argentina".to_string(),
            "ST, AM(C)".to_string(),
            "FC Barcelona".to_string(),
            "30-06-2027".to_string(),
        ];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 1);
        assert!(result.is_valid);
    }

    #[test]
    fn test_multiple_numeric_errors() {
        let headers = vec![
            "UID".to_string(),
            "Name".to_string(),
            "Age".to_string(),
            "Goals".to_string(),
            "Assists".to_string(),
        ];
        let fields = vec![
            "101".to_string(),
            "Test".to_string(),
            "abc".to_string(),
            "def".to_string(),
            "15".to_string(),
        ];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 1);
        assert!(!result.is_valid);
        assert_eq!(result.rejection_reasons.len(), 2);
        assert!(result
            .rejection_reasons
            .contains(&"Invalid numeric value for column 'Age': 'abc'".to_string()));
        assert!(result
            .rejection_reasons
            .contains(&"Invalid numeric value for column 'Goals': 'def'".to_string()));
    }

    // ---- validate_row: combined failures ----

    #[test]
    fn test_empty_uid_and_empty_name() {
        let headers = vec!["UID".to_string(), "Name".to_string()];
        let fields = vec!["".to_string(), "".to_string()];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 10);
        assert!(!result.is_valid);
        assert!(result
            .rejection_reasons
            .contains(&"Missing UID at row 10".to_string()));
        assert!(result
            .rejection_reasons
            .contains(&"Missing player name".to_string()));
    }

    #[test]
    fn test_duplicate_does_not_insert_again() {
        let headers = vec!["UID".to_string(), "Name".to_string()];
        let fields1 = vec!["101".to_string(), "First".to_string()];
        let fields2_invalid = vec!["".to_string(), "No UID".to_string()];
        let mut seen = HashSet::new();

        let r1 = validate_row(&fields1, &headers, &mut seen, 1);
        assert!(r1.is_valid);
        assert!(seen.contains("101"));
        assert_eq!(seen.len(), 1);

        // Invalid row should not have its (empty) UID inserted
        let r2 = validate_row(&fields2_invalid, &headers, &mut seen, 2);
        assert!(!r2.is_valid);
        assert_eq!(seen.len(), 1);
    }

    #[test]
    fn test_row_index_in_error() {
        let headers = vec!["UID".to_string(), "Name".to_string()];
        let fields = vec!["".to_string(), "Test".to_string()];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 42);
        assert!(!result.is_valid);
        assert!(result.rejection_reasons[0].contains("42"));
    }

    // ---- Edge cases ----

    #[test]
    fn test_fields_shorter_than_headers() {
        let headers = vec!["UID".to_string(), "Name".to_string(), "Age".to_string()];
        let fields = vec!["101".to_string(), "Test".to_string()]; // missing Age field
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 1);
        // UID and Name present → valid (Age is just missing, treated as NULL)
        assert!(result.is_valid);
    }

    #[test]
    fn test_uid_case_sensitivity() {
        let headers = vec!["uid".to_string(), "name".to_string()];
        let fields = vec!["101".to_string(), "Test".to_string()];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 1);
        assert!(result.is_valid);
    }

    #[test]
    fn test_decimal_in_numeric_column_is_valid() {
        let headers = vec![
            "UID".to_string(),
            "Name".to_string(),
            "Avg Rating".to_string(),
        ];
        let fields = vec!["101".to_string(), "Test".to_string(), "7.35".to_string()];
        let mut seen = HashSet::new();

        let result = validate_row(&fields, &headers, &mut seen, 1);
        assert!(result.is_valid);
    }
}
