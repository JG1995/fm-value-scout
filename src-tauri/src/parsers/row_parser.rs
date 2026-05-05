use super::appearances::parse_appearances;
use super::currency::parse_currency;
use super::footedness::parse_footedness;
use super::nationality::lookup_nationality;
use super::positions::parse_positions;
use super::units::{parse_unit_field, UnitPrecision};
use super::validator::validate_row;
use super::wage::parse_wage;
use super::*;
use std::collections::HashSet;

// ---------------------------------------------------------------------------
// Column-lookup helpers
// ---------------------------------------------------------------------------

/// Find the index of a header column matching any of the given patterns
/// (case-insensitive exact match). Returns the index of the first match, or
/// `None` when no header matches any pattern.
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::row_parser::find_header_index;
///
/// let headers = vec!["UID".to_string(), "Name".to_string(), "Age".to_string()];
/// assert_eq!(find_header_index(&headers, &["UID"]), Some(0));
/// assert_eq!(find_header_index(&headers, &["uid"]), Some(0));
/// assert_eq!(find_header_index(&headers, &["Age", "AGE"]), Some(2));
/// assert_eq!(find_header_index(&headers, &["Club"]), None);
/// ```
pub fn find_header_index(headers: &[String], patterns: &[&str]) -> Option<usize> {
    let lower_patterns: Vec<String> = patterns.iter().map(|p| p.to_lowercase()).collect();
    headers.iter().position(|h| {
        let lower = h.to_lowercase();
        lower_patterns.iter().any(|p| lower == *p)
    })
}

/// Return `true` when any of the given header patterns exists (case-insensitive).
fn has_any_header(headers: &[String], patterns: &[&str]) -> bool {
    find_header_index(headers, patterns).is_some()
}

// ---------------------------------------------------------------------------
// Field-value extraction helpers
// ---------------------------------------------------------------------------

/// Extract the raw string value for a column identified by any of `patterns`.
/// Returns `None` if the column is missing or the field is empty/whitespace.
fn get_field<'a>(fields: &'a [String], headers: &[String], patterns: &[&str]) -> Option<&'a str> {
    let idx = find_header_index(headers, patterns)?;
    let trimmed = fields.get(idx).map(|s| s.trim()).unwrap_or("");
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Parse a column as `i64`. Returns `None` on missing/empty/unparseable input.
fn parse_i64_field(fields: &[String], headers: &[String], patterns: &[&str]) -> Option<i64> {
    let raw = get_field(fields, headers, patterns)?;
    raw.parse::<i64>().ok()
}

/// Parse a column as `f64`. Returns `None` on missing/empty/unparseable input.
fn parse_f64_field(fields: &[String], headers: &[String], patterns: &[&str]) -> Option<f64> {
    let raw = get_field(fields, headers, patterns)?;
    raw.parse::<f64>().ok()
}

/// Get a raw string value for a column (not through a dedicated parser).
fn get_string_field(fields: &[String], headers: &[String], patterns: &[&str]) -> Option<String> {
    let raw = get_field(fields, headers, patterns)?;
    Some(raw.to_string())
}

/// Handle the "Thr" column which can mean either Long Throws (technical) or
/// Throwing (goalkeeper). If the CSV headers contain any GK-specific columns,
/// "Thr" is treated as Throwing → `thr`. Otherwise it's Long Throws → `lon_thr`.
fn handle_thr_column(record: &mut PlayerRecord, fields: &[String], headers: &[String]) {
    let gk_indicators = &[
        "Aer",
        "Cmd",
        "Com",
        "Com Gk",
        "Ecc",
        "Han",
        "Kic",
        "1v1",
        "One On One",
        "One-on-One",
        "Pun",
        "Ref",
        "Rus",
    ];
    if has_any_header(headers, gk_indicators) {
        // GK context → "Thr" means Throwing
        record.thr = parse_i64_field(fields, headers, &["Thr"]);
    } else {
        // Outfield context → "Thr" means Long Throws
        record.lon_thr = parse_i64_field(fields, headers, &["Thr"]);
    }
}

// ---------------------------------------------------------------------------
// parse_row — main orchestrator
// ---------------------------------------------------------------------------

/// Parse a single CSV row into a [`PlayerRecord`].
///
/// Dispatch flow:
/// 1. Validates the row via [`validate_row`] — returns `Err(rejection_reasons)`
///    on failure.
/// 2. Matches each header column to its appropriate parser and populates the
///    corresponding [`PlayerRecord`] field.
///
/// Empty fields are treated as `None` (missing data). Parse failures for
/// individual metrics are silently treated as `None` (graceful degradation).
///
/// # Arguments
///
/// * `fields`  — The raw CSV field values for this row, in column order.
/// * `headers` — The CSV header names (must be the same length as or longer
///               than `fields`).
/// * `seen_uids` — Mutable set of previously-seen UIDs (for duplicate detection).
///
/// # Returns
///
/// * `Ok(PlayerRecord)` — successfully parsed row.
/// * `Err(Vec<String>)` — validation failure; the vector contains
///   human-readable rejection reasons.
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
/// use fm_valuescout_lib::parsers::row_parser::parse_row;
/// use fm_valuescout_lib::parsers::PlayerRecord;
///
/// let headers = vec![
///     "UID".to_string(), "Name".to_string(), "Age".to_string(),
///     "Club".to_string(), "Position".to_string(),
/// ];
/// let fields = vec![
///     "1001".to_string(), "Player One".to_string(), "25".to_string(),
///     "FC Barcelona".to_string(), "ST".to_string(),
/// ];
/// let mut seen = HashSet::new();
/// let result = parse_row(&fields, &headers, &mut seen);
/// assert!(result.is_ok());
/// let record = result.unwrap();
/// assert_eq!(record.uid.as_deref(), Some("1001"));
/// assert_eq!(record.name.as_deref(), Some("Player One"));
/// assert_eq!(record.age, Some(25));
/// ```
pub fn parse_row(
    fields: &[String],
    headers: &[String],
    seen_uids: &mut HashSet<String>,
) -> Result<PlayerRecord, Vec<String>> {
    // ---- 1. Validate ----
    let validation = validate_row(fields, headers, seen_uids, 0);
    if !validation.is_valid {
        return Err(validation.rejection_reasons);
    }

    let mut record = PlayerRecord::default();

    // =====================================================================
    // Identity columns
    // =====================================================================

    record.uid = get_string_field(fields, headers, &["UID"]);
    record.name = get_string_field(fields, headers, &["Name"]);
    if let Some(raw) = get_field(fields, headers, &["Nation", "Nationality"]) {
        if let FieldValue::String(v) = lookup_nationality(raw) {
            record.nation = Some(v);
        }
    }
    if let Some(raw) = get_field(fields, headers, &["Height"]) {
        if let FieldValue::Int(v) = parse_unit_field(raw, UnitPrecision::Height) {
            record.height = Some(v);
        }
    }
    if let Some(raw) = get_field(fields, headers, &["Left Foot", "Left_Foot"]) {
        if let FieldValue::FootScore { score, raw: r } = parse_footedness(raw) {
            record.left_foot_raw = Some(r);
            record.left_foot_score = score;
        }
    }
    if let Some(raw) = get_field(fields, headers, &["Right Foot", "Right_Foot"]) {
        if let FieldValue::FootScore { score, raw: r } = parse_footedness(raw) {
            record.right_foot_raw = Some(r);
            record.right_foot_score = score;
        }
    }

    // =====================================================================
    // Season columns
    // =====================================================================

    record.club = get_string_field(fields, headers, &["Club"]);
    if let Some(raw) = get_field(fields, headers, &["Position", "Positions"]) {
        if let FieldValue::Position(v) = parse_positions(raw) {
            record.position_raw = Some(v.join(", "));
        }
    }
    record.age = parse_i64_field(fields, headers, &["Age"]);
    record.minutes = parse_i64_field(fields, headers, &["Minutes"]);

    // Starts/Apps column → parse_appearances extracts both starts and subs
    if let Some(raw) = get_field(fields, headers, &["Starts", "Apps"]) {
        if let FieldValue::Appearances { starts, subs } = parse_appearances(raw) {
            record.starts = Some(starts);
            record.subs = Some(subs);
        }
    } else {
        // No Starts/Apps column; try standalone Subs
        record.subs = parse_i64_field(fields, headers, &["Subs"]);
    }

    record.expires = get_string_field(fields, headers, &["Expires"]);

    // =====================================================================
    // Technical attributes (i64)
    // =====================================================================

    record.cor = parse_i64_field(fields, headers, &["Cor"]);
    record.cro = parse_i64_field(fields, headers, &["Cro"]);
    record.dri = parse_i64_field(fields, headers, &["Dri"]);
    record.fin = parse_i64_field(fields, headers, &["Fin"]);
    record.fir = parse_i64_field(fields, headers, &["Fir"]);
    record.fre = parse_i64_field(fields, headers, &["Fre"]);
    record.hea = parse_i64_field(fields, headers, &["Hea"]);
    record.lon = parse_i64_field(fields, headers, &["Lon"]);
    // "Thr" is ambiguous — delegated to handle_thr_column
    handle_thr_column(&mut record, fields, headers);
    record.mar = parse_i64_field(fields, headers, &["Mar"]);
    record.pas = parse_i64_field(fields, headers, &["Pas"]);
    record.pen = parse_i64_field(fields, headers, &["Pen"]);
    record.tck = parse_i64_field(fields, headers, &["Tck"]);
    record.tec = parse_i64_field(fields, headers, &["Tec"]);

    // =====================================================================
    // Mental attributes (i64)
    // =====================================================================

    record.agg = parse_i64_field(fields, headers, &["Agg"]);
    record.ant = parse_i64_field(fields, headers, &["Ant"]);
    record.bra = parse_i64_field(fields, headers, &["Bra"]);
    record.cmp = parse_i64_field(fields, headers, &["Cmp"]);
    record.cnt = parse_i64_field(fields, headers, &["Cnt"]);
    record.dec = parse_i64_field(fields, headers, &["Dec"]);
    record.det = parse_i64_field(fields, headers, &["Det"]);
    record.fla = parse_i64_field(fields, headers, &["Fla"]);
    record.inf = parse_i64_field(fields, headers, &["Inf"]);
    record.lea = parse_i64_field(fields, headers, &["Lea"]);
    record.otb = parse_i64_field(fields, headers, &["Otb"]);
    record.pos = parse_i64_field(fields, headers, &["Pos"]);
    record.tea = parse_i64_field(fields, headers, &["Tea"]);
    record.vis = parse_i64_field(fields, headers, &["Vis"]);
    record.wor = parse_i64_field(fields, headers, &["Wor"]);

    // =====================================================================
    // Physical attributes (i64)
    // =====================================================================

    record.acc = parse_i64_field(fields, headers, &["Acc"]);
    record.agi = parse_i64_field(fields, headers, &["Agi"]);
    record.bal = parse_i64_field(fields, headers, &["Bal"]);
    record.jum = parse_i64_field(fields, headers, &["Jmp", "Jum"]);
    record.nat_fit = parse_i64_field(fields, headers, &["Nat", "Nat Fit"]);
    record.pac = parse_i64_field(fields, headers, &["Pac"]);
    record.sta = parse_i64_field(fields, headers, &["Sta"]);
    record.str = parse_i64_field(fields, headers, &["Str"]);

    // =====================================================================
    // Goalkeeper attributes (i64)
    // =====================================================================

    record.aer = parse_i64_field(fields, headers, &["Aer"]);
    record.cmd = parse_i64_field(fields, headers, &["Cmd"]);
    record.com_gk = parse_i64_field(fields, headers, &["Com", "Com Gk"]);
    record.ecc = parse_i64_field(fields, headers, &["Ecc"]);
    record.han = parse_i64_field(fields, headers, &["Han"]);
    record.kic = parse_i64_field(fields, headers, &["Kic"]);
    record.one_on_one = parse_i64_field(fields, headers, &["1v1", "One On One", "One-on-One"]);
    record.pun = parse_i64_field(fields, headers, &["Pun"]);
    record.ref_gk = parse_i64_field(fields, headers, &["Ref"]);
    record.rus = parse_i64_field(fields, headers, &["Rus"]);
    // "Thr" in GK context is already handled by handle_thr_column above

    // =====================================================================
    // Attacking & Finishing (f64)
    // =====================================================================

    record.goals_xg = parse_f64_field(fields, headers, &["Goals XG", "Goals_xG", "Gls_xG"]);
    record.npxg = parse_f64_field(fields, headers, &["NPXG", "Non-Penalty xG"]);
    record.xg_op = parse_f64_field(fields, headers, &["XG OP", "xG_op"]);
    record.xg_per_shot = parse_f64_field(fields, headers, &["XG Per Shot", "xG/Shot"]);
    record.shots = parse_f64_field(fields, headers, &["Shots", "Sh"]);
    record.shots_on_target =
        parse_f64_field(fields, headers, &["Shots OT", "SoT", "Shots on Target"]);
    record.sot_pct = parse_f64_field(fields, headers, &["SoT %", "SOT%"]);
    record.conv_pct = parse_f64_field(fields, headers, &["Conv %", "Conv%"]);
    record.pens_scored = parse_f64_field(fields, headers, &["Pens Scored", "Pen Scored"]);
    record.pens_taken = parse_f64_field(fields, headers, &["Pens Taken", "Pen Taken"]);

    // =====================================================================
    // Creativity & Chance Creation (f64)
    // =====================================================================

    record.assists = parse_f64_field(fields, headers, &["Assists", "Ast"]);
    record.xa = parse_f64_field(fields, headers, &["XA", "x Assists"]);
    record.key_passes = parse_f64_field(fields, headers, &["Key Passes", "KeyP"]);
    record.chances_created = parse_f64_field(fields, headers, &["Chances Created", "ChC"]);
    record.op_key_passes = parse_f64_field(fields, headers, &["OP Key Passes", "OP KeyP"]);
    record.through_balls = parse_f64_field(fields, headers, &["Through Balls", "ThrB"]);

    // =====================================================================
    // Transition & Ball Progression (f64)
    // =====================================================================

    record.dribbles_per_game = parse_f64_field(fields, headers, &["Dribbles P90", "Drb/90"]);
    record.progressive_passes = parse_f64_field(fields, headers, &["Prog Passes", "PrgP"]);
    record.progressive_runs = parse_f64_field(fields, headers, &["Prog Runs", "PrgR", "Carries"]);
    record.passes_completed = parse_f64_field(fields, headers, &["Passes C", "Pas C"]);
    record.passes_attempted = parse_f64_field(fields, headers, &["Passes A", "Pas A"]);
    record.pass_completion_pct = parse_f64_field(fields, headers, &["Pass %", "Pas%"]);
    record.crosses_attempted = parse_f64_field(fields, headers, &["Crosses A", "Crs A"]);
    record.crosses_completed = parse_f64_field(fields, headers, &["Crosses C", "Crs C"]);
    record.cross_completion_pct = parse_f64_field(fields, headers, &["Cross %", "Crs%"]);

    // =====================================================================
    // Defensive Actions (f64)
    // =====================================================================

    record.tackles_per_game = parse_f64_field(fields, headers, &["Tackles P90", "Tck/90"]);
    record.tackles_completed = parse_f64_field(fields, headers, &["Tackles C", "Tck C"]);
    record.tackle_completion_pct = parse_f64_field(fields, headers, &["Tackle %", "Tck%"]);
    record.interceptions_per_game = parse_f64_field(fields, headers, &["Int P90", "Int/90"]);
    record.clearances = parse_f64_field(fields, headers, &["Clearances", "Clr"]);
    record.blocks = parse_f64_field(fields, headers, &["Blocks", "Blk"]);
    record.possession_won = parse_f64_field(fields, headers, &["Poss Won", "PW"]);
    record.possession_lost = parse_f64_field(fields, headers, &["Poss Lost", "PL"]);

    // =====================================================================
    // Aerial Presence (f64)
    // =====================================================================

    record.headers_won = parse_f64_field(fields, headers, &["Headers W", "Hdr W"]);
    record.headers_lost = parse_f64_field(fields, headers, &["Headers L", "Hdr L"]);
    record.headers_won_pct = parse_f64_field(fields, headers, &["Hdr %", "Header %"]);

    // =====================================================================
    // Goalkeeping Stats (f64)
    // =====================================================================

    record.saves = parse_f64_field(fields, headers, &["Saves", "Sv"]);
    record.save_pct = parse_f64_field(fields, headers, &["Save %", "Sv%"]);
    record.goals_conceded = parse_f64_field(fields, headers, &["Goals Conceded", "GA"]);
    record.clean_sheets = parse_f64_field(fields, headers, &["Clean Sheets", "CS"]);
    record.penalties_saved = parse_f64_field(fields, headers, &["Pen Saves", "Pen Sv"]);
    record.expected_saves = parse_f64_field(fields, headers, &["x Saves", "xSv"]);

    // =====================================================================
    // Discipline (f64)
    // =====================================================================

    record.fouls_made = parse_f64_field(fields, headers, &["Fouls Made", "Fls"]);
    record.fouls_against = parse_f64_field(fields, headers, &["Fouls Against", "Fls A"]);
    record.yellow_cards = parse_f64_field(fields, headers, &["Yellow Cards", "Yel"]);
    record.red_cards = parse_f64_field(fields, headers, &["Red Cards", "Red"]);
    record.offsides = parse_f64_field(fields, headers, &["Offsides", "Off"]);

    // =====================================================================
    // Match Impact (f64)
    // =====================================================================

    if let Some(raw) = get_field(fields, headers, &["Distance", "Dist"]) {
        if let FieldValue::Float(v) = parse_unit_field(raw, UnitPrecision::Distance) {
            record.distance_covered = Some(v);
        }
    }
    record.average_rating = parse_f64_field(fields, headers, &["Avg Rating", "Rat"]);
    record.player_of_match = parse_f64_field(fields, headers, &["POM", "PoM", "Player of Match"]);

    // =====================================================================
    // Optional extra columns
    // =====================================================================

    record.current_ability = parse_i64_field(fields, headers, &["CA"]);
    record.potential_ability = parse_i64_field(fields, headers, &["PA"]);
    if let Some(raw) = get_field(fields, headers, &["Value", "Transfer Value"]) {
        if let FieldValue::Currency(v) = parse_currency(raw) {
            record.transfer_value = Some(v);
        }
    }
    if let Some(raw) = get_field(fields, headers, &["Wage"]) {
        if let FieldValue::Wage {
            value,
            denomination,
        } = parse_wage(raw)
        {
            record.wage_value = Some(value);
            record.wage_denomination = denomination;
        }
    }
    if let Some(raw) = get_field(fields, headers, &["Second Nat", "2nd Nation"]) {
        if let FieldValue::String(v) = lookup_nationality(raw) {
            record.second_nationality = Some(v);
        }
    }

    Ok(record)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    /// Quick shorthand: build `headers` and `fields` from parallel arrays.
    fn build_row(headers: &[&str], fields: &[&str]) -> (Vec<String>, Vec<String>) {
        (
            headers.iter().map(|s| s.to_string()).collect(),
            fields.iter().map(|s| s.to_string()).collect(),
        )
    }

    /// Parse a row and assert success, returning the record.
    fn assert_ok(headers: &[&str], fields: &[&str], seen: &mut HashSet<String>) -> PlayerRecord {
        let (h, f) = build_row(headers, fields);
        parse_row(&f, &h, seen).expect("expected Ok, got Err")
    }

    /// Parse a row and assert Err, returning the rejection reasons.
    fn assert_err(headers: &[&str], fields: &[&str], seen: &mut HashSet<String>) -> Vec<String> {
        let (h, f) = build_row(headers, fields);
        parse_row(&f, &h, seen).expect_err("expected Err, got Ok")
    }

    // ---- Find header index tests ----

    #[test]
    fn test_find_header_index_exact() {
        let h = vec!["UID".to_string(), "Age".to_string()];
        assert_eq!(find_header_index(&h, &["UID"]), Some(0));
        assert_eq!(find_header_index(&h, &["Age"]), Some(1));
    }

    #[test]
    fn test_find_header_index_case_insensitive() {
        let h = vec!["uid".to_string(), "NAME".to_string()];
        assert_eq!(find_header_index(&h, &["UID"]), Some(0));
        assert_eq!(find_header_index(&h, &["Name"]), Some(1));
    }

    #[test]
    fn test_find_header_index_multiple_patterns() {
        let h = vec!["Cor".to_string(), "Age".to_string()];
        assert_eq!(find_header_index(&h, &["NotHere", "Cor"]), Some(0));
        assert_eq!(find_header_index(&h, &["Jmp", "Jum"]), None);
    }

    #[test]
    fn test_find_header_index_not_found() {
        let h = vec!["UID".to_string(), "Name".to_string()];
        assert_eq!(find_header_index(&h, &["Club"]), None);
    }

    // ---- Validation failure tests ----

    #[test]
    fn test_missing_uid_column() {
        let mut seen = HashSet::new();
        let reasons = assert_err(&["Name", "Age"], &["Test", "25"], &mut seen);
        assert!(reasons.iter().any(|r| r.contains("UID column")));
    }

    #[test]
    fn test_empty_uid_rejected() {
        let mut seen = HashSet::new();
        let reasons = assert_err(&["UID", "Name"], &["", "Test"], &mut seen);
        assert!(reasons.iter().any(|r| r.contains("UID")));
    }

    #[test]
    fn test_duplicate_uid_rejected() {
        let mut seen = HashSet::new();
        // First row succeeds and records the UID
        assert_ok(&["UID", "Name"], &["101", "First"], &mut seen);
        // Second row with same UID is rejected
        let reasons = assert_err(&["UID", "Name"], &["101", "Duplicate"], &mut seen);
        assert!(reasons.iter().any(|r| r.contains("Duplicate")));
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut seen = HashSet::new();
        let reasons = assert_err(&["UID", "Name"], &["101", ""], &mut seen);
        assert!(reasons
            .iter()
            .any(|r| r.contains("player name") || r.contains("Name")));
    }

    // ---- Happy path: full basic row ----

    #[test]
    fn test_full_basic_row() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Age", "Club", "Position"],
            &["1001", "Player One", "25", "FC Barcelona", "ST"],
            &mut seen,
        );
        assert_eq!(record.uid.as_deref(), Some("1001"));
        assert_eq!(record.name.as_deref(), Some("Player One"));
        assert_eq!(record.age, Some(25));
        assert_eq!(record.club.as_deref(), Some("FC Barcelona"));
        assert_eq!(record.position_raw.as_deref(), Some("ST"));
    }

    // ---- Case-insensitive header matching ----

    #[test]
    fn test_case_insensitive_headers() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["uid", "name", "age", "club", "nation"],
            &["2001", "Player Two", "30", "Juventus", "ITA"],
            &mut seen,
        );
        assert_eq!(record.uid.as_deref(), Some("2001"));
        assert_eq!(record.name.as_deref(), Some("Player Two"));
        assert_eq!(record.age, Some(30));
        assert_eq!(record.nation.as_deref(), Some("Italy"));
    }

    // ---- Identity columns ----

    #[test]
    fn test_nation_lookup() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Nation"],
            &["101", "Test", "ENG"],
            &mut seen,
        );
        assert_eq!(record.nation.as_deref(), Some("England"));
    }

    #[test]
    fn test_natinality_alternative_header() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Nationality"],
            &["102", "Test", "FRA"],
            &mut seen,
        );
        assert_eq!(record.nation.as_deref(), Some("France"));
    }

    #[test]
    fn test_height_parsed() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Height"],
            &["103", "Tall Player", "192cm"],
            &mut seen,
        );
        assert_eq!(record.height, Some(192));
    }

    #[test]
    fn test_footedness() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Left Foot", "Right Foot"],
            &["104", "Player", "Very Strong", "Weak"],
            &mut seen,
        );
        assert_eq!(record.left_foot_score, Some(20));
        assert_eq!(record.left_foot_raw.as_deref(), Some("Very Strong"));
        assert_eq!(record.right_foot_score, Some(5));
        assert_eq!(record.right_foot_raw.as_deref(), Some("Weak"));
    }

    // ---- Season columns ----

    #[test]
    fn test_season_columns() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Minutes", "Subs", "Expires"],
            &["105", "Player", "1500", "12", "30.6.2026"],
            &mut seen,
        );
        assert_eq!(record.minutes, Some(1500));
        assert_eq!(record.subs, Some(12));
        assert_eq!(record.starts, None);
        assert_eq!(record.expires.as_deref(), Some("30.6.2026"));
    }

    #[test]
    fn test_appearances_starts_apps_column() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Starts", "Subs"],
            &["106", "Player", "43 (3)", "5"],
            &mut seen,
        );
        // Starts/Apps column takes precedence; "Subs" is ignored when Starts/Apps is present
        assert_eq!(record.starts, Some(43));
        assert_eq!(record.subs, Some(3));
    }

    #[test]
    fn test_appearances_apps_alias() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Apps"],
            &["107", "Player", "30 (10)"],
            &mut seen,
        );
        assert_eq!(record.starts, Some(30));
        assert_eq!(record.subs, Some(10));
    }

    // ---- Technical attributes ----

    #[test]
    fn test_technical_attributes() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &[
                "UID", "Name", "Cor", "Cro", "Dri", "Fin", "Fir", "Fre", "Hea", "Lon", "Mar",
                "Pas", "Pen", "Tck", "Tec",
            ],
            &[
                "200",
                "Tech Player",
                "12",
                "14",
                "16",
                "18",
                "10",
                "8",
                "15",
                "13",
                "11",
                "17",
                "9",
                "14",
                "12",
            ],
            &mut seen,
        );
        assert_eq!(record.cor, Some(12));
        assert_eq!(record.cro, Some(14));
        assert_eq!(record.dri, Some(16));
        assert_eq!(record.fin, Some(18));
        assert_eq!(record.fir, Some(10));
        assert_eq!(record.fre, Some(8));
        assert_eq!(record.hea, Some(15));
        assert_eq!(record.lon, Some(13));
        assert_eq!(record.mar, Some(11));
        assert_eq!(record.pas, Some(17));
        assert_eq!(record.pen, Some(9));
        assert_eq!(record.tck, Some(14));
        assert_eq!(record.tec, Some(12));
    }

    // ---- Mental attributes ----

    #[test]
    fn test_mental_attributes() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &[
                "UID", "Name", "Agg", "Ant", "Bra", "Cmp", "Cnt", "Dec", "Det", "Fla", "Inf",
                "Lea", "Otb", "Pos", "Tea", "Vis", "Wor",
            ],
            &[
                "300",
                "Mental Player",
                "15",
                "14",
                "13",
                "12",
                "11",
                "10",
                "9",
                "8",
                "7",
                "6",
                "5",
                "4",
                "3",
                "2",
                "1",
            ],
            &mut seen,
        );
        assert_eq!(record.agg, Some(15));
        assert_eq!(record.ant, Some(14));
        assert_eq!(record.bra, Some(13));
        assert_eq!(record.cmp, Some(12));
        assert_eq!(record.cnt, Some(11));
        assert_eq!(record.dec, Some(10));
        assert_eq!(record.det, Some(9));
        assert_eq!(record.fla, Some(8));
        assert_eq!(record.inf, Some(7));
        assert_eq!(record.lea, Some(6));
        assert_eq!(record.otb, Some(5));
        assert_eq!(record.pos, Some(4));
        assert_eq!(record.tea, Some(3));
        assert_eq!(record.vis, Some(2));
        assert_eq!(record.wor, Some(1));
    }

    // ---- Physical attributes with aliases ----

    #[test]
    fn test_physical_attributes_with_aliases() {
        let mut seen = HashSet::new();
        // Use Jmp (alias for Jum) and Nat Fit (exact name)
        let record = assert_ok(
            &[
                "UID", "Name", "Acc", "Agi", "Bal", "Jmp", "Nat Fit", "Pac", "Sta", "Str",
            ],
            &[
                "400",
                "Phys Player",
                "18",
                "17",
                "16",
                "15",
                "14",
                "13",
                "12",
                "11",
            ],
            &mut seen,
        );
        assert_eq!(record.acc, Some(18));
        assert_eq!(record.agi, Some(17));
        assert_eq!(record.bal, Some(16));
        assert_eq!(record.jum, Some(15));
        assert_eq!(record.nat_fit, Some(14));
        assert_eq!(record.pac, Some(13));
        assert_eq!(record.sta, Some(12));
        assert_eq!(record.str, Some(11));
    }

    #[test]
    fn test_physical_attributes_nat_alias() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Nat"],
            &["401", "Fit Player", "18"],
            &mut seen,
        );
        assert_eq!(record.nat_fit, Some(18));
    }

    // ---- Goalkeeper attributes ----

    #[test]
    fn test_goalkeeper_attributes() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &[
                "UID", "Name", "Aer", "Cmd", "Com", "Ecc", "Han", "Kic", "1v1", "Pun", "Ref",
                "Rus", "Thr",
            ],
            &[
                "500",
                "GK Player",
                "16",
                "15",
                "14",
                "13",
                "12",
                "11",
                "10",
                "9",
                "8",
                "7",
                "6",
            ],
            &mut seen,
        );
        assert_eq!(record.aer, Some(16));
        assert_eq!(record.cmd, Some(15));
        assert_eq!(record.com_gk, Some(14));
        assert_eq!(record.ecc, Some(13));
        assert_eq!(record.han, Some(12));
        assert_eq!(record.kic, Some(11));
        assert_eq!(record.one_on_one, Some(10));
        assert_eq!(record.pun, Some(9));
        assert_eq!(record.ref_gk, Some(8));
        assert_eq!(record.rus, Some(7));
        // Thr in GK context → thr (throwing)
        assert_eq!(record.thr, Some(6));
        // lon_thr should be None (GK context uses thr)
        assert_eq!(record.lon_thr, None);
    }

    #[test]
    fn test_gk_com_gk_alias() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Com Gk", "Aer"],
            &["502", "GK2", "18", "20"],
            &mut seen,
        );
        assert_eq!(record.com_gk, Some(18));
    }

    #[test]
    fn test_gk_one_on_one_aliases() {
        let mut seen = HashSet::new();
        // "One On One" instead of "1v1"
        let record = assert_ok(
            &["UID", "Name", "One On One", "Aer"],
            &["503", "GK3", "15", "17"],
            &mut seen,
        );
        assert_eq!(record.one_on_one, Some(15));
    }

    // ---- Thr disambiguation: outfield context (no GK columns) ----

    #[test]
    fn test_thr_outfield_context() {
        let mut seen = HashSet::new();
        // No GK-specific columns → Thr means Long Throws → lon_thr
        let record = assert_ok(
            &["UID", "Name", "Thr", "Pas"],
            &["600", "Outfield", "14", "16"],
            &mut seen,
        );
        assert_eq!(record.lon_thr, Some(14));
        assert_eq!(record.thr, None);
    }

    // ---- Float metrics ----

    #[test]
    fn test_attacking_float_metrics() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &[
                "UID",
                "Name",
                "Goals XG",
                "NPXG",
                "XG OP",
                "XG Per Shot",
                "Shots",
                "Shots OT",
                "SoT %",
                "Conv %",
                "Pens Scored",
                "Pens Taken",
            ],
            &[
                "700",
                "Att Player",
                "15.5",
                "12.3",
                "2.1",
                "0.25",
                "62",
                "28",
                "45.2",
                "24.2",
                "5",
                "6",
            ],
            &mut seen,
        );
        assert!((record.goals_xg.unwrap() - 15.5).abs() < 0.001);
        assert!((record.npxg.unwrap() - 12.3).abs() < 0.001);
        assert!((record.xg_op.unwrap() - 2.1).abs() < 0.001);
        assert!((record.xg_per_shot.unwrap() - 0.25).abs() < 0.001);
        assert!((record.shots.unwrap() - 62.0).abs() < 0.001);
        assert!((record.shots_on_target.unwrap() - 28.0).abs() < 0.001);
        assert!((record.sot_pct.unwrap() - 45.2).abs() < 0.001);
        assert!((record.conv_pct.unwrap() - 24.2).abs() < 0.001);
        assert!((record.pens_scored.unwrap() - 5.0).abs() < 0.001);
        assert!((record.pens_taken.unwrap() - 6.0).abs() < 0.001);
    }

    #[test]
    fn test_column_name_aliases_float() {
        let mut seen = HashSet::new();
        // Use alternative column aliases
        let record = assert_ok(
            &["UID", "Name", "Gls_xG", "Sh", "Ast", "KeyP", "PrgP", "PrgR"],
            &["800", "Alias Player", "10.0", "40", "8", "22", "55", "33"],
            &mut seen,
        );
        assert!((record.goals_xg.unwrap() - 10.0).abs() < 0.001);
        assert!((record.shots.unwrap() - 40.0).abs() < 0.001);
        assert!((record.assists.unwrap() - 8.0).abs() < 0.001);
        assert!((record.key_passes.unwrap() - 22.0).abs() < 0.001);
        assert!((record.progressive_passes.unwrap() - 55.0).abs() < 0.001);
        assert!((record.progressive_runs.unwrap() - 33.0).abs() < 0.001);
    }

    #[test]
    fn test_defensive_float_metrics() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &[
                "UID",
                "Name",
                "Tackles P90",
                "Tackles C",
                "Tackle %",
                "Int P90",
                "Clearances",
                "Blocks",
                "Poss Won",
                "Poss Lost",
            ],
            &[
                "900",
                "Def Player",
                "3.5",
                "45",
                "78.3",
                "2.1",
                "120",
                "30",
                "55",
                "40",
            ],
            &mut seen,
        );
        assert!((record.tackles_per_game.unwrap() - 3.5).abs() < 0.001);
        assert!((record.tackles_completed.unwrap() - 45.0).abs() < 0.001);
        assert!((record.tackle_completion_pct.unwrap() - 78.3).abs() < 0.001);
        assert!((record.interceptions_per_game.unwrap() - 2.1).abs() < 0.001);
        assert!((record.clearances.unwrap() - 120.0).abs() < 0.001);
        assert!((record.blocks.unwrap() - 30.0).abs() < 0.001);
        assert!((record.possession_won.unwrap() - 55.0).abs() < 0.001);
        assert!((record.possession_lost.unwrap() - 40.0).abs() < 0.001);
    }

    #[test]
    fn test_discipline_and_misc_float() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &[
                "UID",
                "Name",
                "Fouls Made",
                "Yellow Cards",
                "Red Cards",
                "Offsides",
                "Avg Rating",
                "POM",
            ],
            &["1000", "Disc Player", "25", "6", "1", "8", "7.35", "3"],
            &mut seen,
        );
        assert!((record.fouls_made.unwrap() - 25.0).abs() < 0.001);
        assert!((record.yellow_cards.unwrap() - 6.0).abs() < 0.001);
        assert!((record.red_cards.unwrap() - 1.0).abs() < 0.001);
        assert!((record.offsides.unwrap() - 8.0).abs() < 0.001);
        assert!((record.average_rating.unwrap() - 7.35).abs() < 0.001);
        assert!((record.player_of_match.unwrap() - 3.0).abs() < 0.001);
    }

    // ---- Optional extra columns ----

    #[test]
    fn test_optional_columns_ca_pa() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "CA", "PA"],
            &["1100", "Prospect", "120", "-85"],
            &mut seen,
        );
        assert_eq!(record.current_ability, Some(120));
        assert_eq!(record.potential_ability, Some(-85));
    }

    #[test]
    fn test_optional_columns_value_wage() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Value", "Wage"],
            &["1200", "Star Player", "€55M", "€100K p/w"],
            &mut seen,
        );
        assert!((record.transfer_value.unwrap() - 55_000_000.0).abs() < 1.0);
        let wage_val = record.wage_value.unwrap();
        assert!((wage_val - 100_000.0).abs() < 1.0);
        assert_eq!(record.wage_denomination.as_deref(), Some("p/w"));
    }

    #[test]
    fn test_optional_columns_transfer_value_alias() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Transfer Value"],
            &["1300", "Valuable", "€10M"],
            &mut seen,
        );
        assert!((record.transfer_value.unwrap() - 10_000_000.0).abs() < 1.0);
    }

    #[test]
    fn test_optional_columns_second_nation() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Second Nat"],
            &["1400", "Dual", "ARG"],
            &mut seen,
        );
        assert_eq!(record.second_nationality.as_deref(), Some("Argentina"));
    }

    #[test]
    fn test_optional_columns_second_nation_alias() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "2nd Nation"],
            &["1401", "Dual2", "BRA"],
            &mut seen,
        );
        assert_eq!(record.second_nationality.as_deref(), Some("Brazil"));
    }

    // ---- Empty field handling ----

    #[test]
    fn test_empty_fields_are_none() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &[
                "UID",
                "Name",
                "Age",
                "Minutes",
                "Starts",
                "Subs",
                "Cor",
                "Fin",
                "Pas",
                "Goals XG",
                "Avg Rating",
            ],
            &["1500", "Partial", "", "", "", "", "", "", "", "", ""],
            &mut seen,
        );
        assert_eq!(record.uid.as_deref(), Some("1500"));
        assert_eq!(record.name.as_deref(), Some("Partial"));
        assert_eq!(record.age, None);
        assert_eq!(record.minutes, None);
        assert_eq!(record.starts, None);
        assert_eq!(record.subs, None);
        assert_eq!(record.cor, None);
        assert_eq!(record.fin, None);
        assert_eq!(record.pas, None);
        assert_eq!(record.goals_xg, None);
        assert_eq!(record.average_rating, None);
    }

    #[test]
    fn test_partial_attributes_some_set() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Cor", "Fin", "Pas", "Agg", "Pac"],
            &["1600", "Partial2", "15", "", "", "12", ""],
            &mut seen,
        );
        assert_eq!(record.cor, Some(15));
        assert_eq!(record.fin, None);
        assert_eq!(record.pas, None);
        assert_eq!(record.agg, Some(12));
        assert_eq!(record.pac, None);
    }

    // ---- Distance parsed with UnitPrecision ----

    #[test]
    fn test_distance_covered() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Distance"],
            &["1700", "Runner", "12.5km"],
            &mut seen,
        );
        assert!((record.distance_covered.unwrap() - 12.5).abs() < 0.001);
    }

    #[test]
    fn test_distance_alias() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Dist"],
            &["1701", "Runner2", "10.2km"],
            &mut seen,
        );
        assert!((record.distance_covered.unwrap() - 10.2).abs() < 0.001);
    }

    // ---- Headers shorter than fields ----

    #[test]
    fn test_fields_longer_than_headers_ignored() {
        let mut seen = HashSet::new();
        // Extra fields beyond headers are silently ignored
        let (h, f) = build_row(&["UID", "Name"], &["1800", "Player", "ExtraValue"]);
        let result = parse_row(&f, &h, &mut seen);
        assert!(result.is_ok());
    }

    // ---- Aerial metrics ----

    #[test]
    fn test_aerial_metrics() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Headers W", "Headers L", "Hdr %"],
            &["1900", "Tall", "120", "60", "66.7"],
            &mut seen,
        );
        assert!((record.headers_won.unwrap() - 120.0).abs() < 0.001);
        assert!((record.headers_lost.unwrap() - 60.0).abs() < 0.001);
        assert!((record.headers_won_pct.unwrap() - 66.7).abs() < 0.001);
    }

    // ---- GK stats f64 ----

    #[test]
    fn test_gk_stats_float() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &[
                "UID",
                "Name",
                "Saves",
                "Save %",
                "Goals Conceded",
                "Clean Sheets",
                "Pen Saves",
                "x Saves",
            ],
            &["2000", "GK Stats", "85", "78.5", "32", "12", "3", "6.2"],
            &mut seen,
        );
        assert!((record.saves.unwrap() - 85.0).abs() < 0.001);
        assert!((record.save_pct.unwrap() - 78.5).abs() < 0.001);
        assert!((record.goals_conceded.unwrap() - 32.0).abs() < 0.001);
        assert!((record.clean_sheets.unwrap() - 12.0).abs() < 0.001);
        assert!((record.penalties_saved.unwrap() - 3.0).abs() < 0.001);
        assert!((record.expected_saves.unwrap() - 6.2).abs() < 0.001);
    }

    // ---- Cross metrics ----

    #[test]
    fn test_cross_metrics() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Crosses A", "Crosses C", "Cross %"],
            &["2100", "Crosser", "150", "42", "28.0"],
            &mut seen,
        );
        assert!((record.crosses_attempted.unwrap() - 150.0).abs() < 0.001);
        assert!((record.crosses_completed.unwrap() - 42.0).abs() < 0.001);
        assert!((record.cross_completion_pct.unwrap() - 28.0).abs() < 0.001);
    }

    // ---- Creativity metrics ----

    #[test]
    fn test_creativity_metrics() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &[
                "UID",
                "Name",
                "XA",
                "Chances Created",
                "OP Key Passes",
                "Through Balls",
            ],
            &["2200", "Creator", "8.5", "55", "12", "18"],
            &mut seen,
        );
        assert!((record.xa.unwrap() - 8.5).abs() < 0.001);
        assert!((record.chances_created.unwrap() - 55.0).abs() < 0.001);
        assert!((record.op_key_passes.unwrap() - 12.0).abs() < 0.001);
        assert!((record.through_balls.unwrap() - 18.0).abs() < 0.001);
    }

    // ---- Passing metrics ----

    #[test]
    fn test_passing_metrics() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Passes C", "Passes A", "Pass %"],
            &["2300", "Passer", "850", "950", "89.5"],
            &mut seen,
        );
        assert!((record.passes_completed.unwrap() - 850.0).abs() < 0.001);
        assert!((record.passes_attempted.unwrap() - 950.0).abs() < 0.001);
        assert!((record.pass_completion_pct.unwrap() - 89.5).abs() < 0.001);
    }

    // ---- Unparseable numeric fields produce None (graceful degradation) ----

    #[test]
    fn test_unparseable_numeric_rejected() {
        let mut seen = HashSet::new();
        let result = parse_row(
            &["2400", "Bad Data", "not_a_number", "also_bad"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            &["UID", "Name", "Age", "Cor"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            &mut seen,
        );
        assert!(
            result.is_err(),
            "expected Err for unparseable numeric values"
        );
        let reasons = result.unwrap_err();
        assert!(
            reasons.iter().any(|r| r.contains("not_a_number")),
            "should flag Age value"
        );
        assert!(
            reasons.iter().any(|r| r.contains("also_bad")),
            "should flag Cor value"
        );
    }

    // ---- Validates via seen_uids tracking across calls ----

    #[test]
    fn test_seen_uids_tracked() {
        let mut seen = HashSet::new();
        let r1 = assert_ok(&["UID", "Name"], &["99", "First"], &mut seen);
        assert_eq!(r1.uid.as_deref(), Some("99"));
        assert!(seen.contains("99"));

        // Second row with different UID succeeds
        let r2 = assert_ok(&["UID", "Name"], &["100", "Second"], &mut seen);
        assert_eq!(r2.uid.as_deref(), Some("100"));
        assert!(seen.contains("100"));
        assert_eq!(seen.len(), 2);
    }

    // ---- Player of match aliases ----

    #[test]
    fn test_pom_aliases() {
        let mut seen = HashSet::new();

        // "POM"
        let record = assert_ok(
            &["UID", "Name", "POM"],
            &["2500", "POM Player", "5"],
            &mut seen,
        );
        assert!((record.player_of_match.unwrap() - 5.0).abs() < 0.001);

        // "Player of Match"
        let record2 = assert_ok(
            &["UID", "Name", "Player of Match"],
            &["2501", "POM Player2", "3"],
            &mut seen,
        );
        assert!((record2.player_of_match.unwrap() - 3.0).abs() < 0.001);
    }

    // ---- Position aliases ----

    #[test]
    fn test_position_via_positions_header() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Positions"],
            &["2600", "MultiPos", "ST, AML, AMR"],
            &mut seen,
        );
        assert_eq!(record.position_raw.as_deref(), Some("ST, AML, AMR"));
    }

    // ---- Wages ----

    #[test]
    fn test_wage_column() {
        let mut seen = HashSet::new();
        let record = assert_ok(
            &["UID", "Name", "Wage"],
            &["2700", "WageEarner", "€200K p/w"],
            &mut seen,
        );
        let wage_val = record.wage_value.unwrap();
        assert!((wage_val - 200_000.0).abs() < 1.0);
        assert_eq!(record.wage_denomination.as_deref(), Some("p/w"));
    }
}
