use phf::phf_map;

/// Static compile-time hash map mapping compound FM position strings to their
/// canonical position code expansions.
///
/// Keys are compound position descriptors as they appear in Football Manager
/// data exports (e.g. `D(C)`, `M/AM(R)`, `D(RLC)`). Values are comma-separated
/// lists of canonical position codes.
static POSITION_EXPANSIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "D(C)"            => "DC",
    "D(L)"            => "DL",
    "D(R)"            => "DR",
    "D(LC)"           => "DC,DL",
    "D(RC)"           => "DC,DR",
    "D(RL)"           => "DL,DR",
    "D(RLC)"          => "DC,DL,DR",
    "DM(C)"           => "DMC",
    "DM(L)"           => "DML",
    "DM(R)"           => "DMR",
    "M(C)"            => "MC",
    "M(L)"            => "ML",
    "M(R)"            => "MR",
    "M(LC)"           => "MC,ML",
    "M(RC)"           => "MC,MR",
    "M(RL)"           => "ML,MR",
    "M(RLC)"          => "MC,ML,MR",
    "AM(C)"           => "AMC",
    "AM(L)"           => "AML",
    "AM(R)"           => "AMR",
    "AM(LC)"          => "AMC,AML",
    "AM(RC)"          => "AMC,AMR",
    "AM(RL)"          => "AML,AMR",
    "WB/M(L)"         => "WB_L,ML",
    "WB/M(R)"         => "WB_R,MR",
    "D(RC)/WB(R)"     => "DC,DR,WB_R",
    "D(LC)/WB(L)"     => "DC,DL,WB_L",
    "D(C)/DM(C)"      => "DC,DMC",
    "DM(C)/M(C)"      => "DMC,MC",
    "M(C)/AM(C)"      => "MC,AMC",
    "AM(C)/ST(C)"     => "AMC,ST",
    "AM(L)/ST(L)"     => "AML,ST_L",
    "AM(R)/ST(R)"     => "AMR,ST_R",
    "AM(RL)/ST(RL)"   => "AML,AMR,ST_L,ST_R",
    "WB/M/AM(L)"      => "WB_L,ML,AML",
    "WB/M/AM(R)"      => "WB_R,MR,AMR",
    "D(L)/WB(L)"      => "DL,WB_L",
    "D(R)/WB(R)"      => "DR,WB_R",
    "DM/M/AM(C)"      => "DMC,MC,AMC",
    "DM/M(C)"         => "DMC,MC",
    "M/AM(C)"         => "MC,AMC",
    "M/AM(L)"         => "ML,AML",
    "M/AM(R)"         => "MR,AMR",
    "ST(C)"           => "ST",
    "ST(L)"           => "ST_L",
    "ST(R)"           => "ST_R",
    "GK"              => "GK",
};

/// Look up a compound FM position string and return its expanded canonical
/// position codes.
///
/// If the position is found in the static map, its comma-separated list of
/// canonical codes is split and returned as a `Vec<String>`.
///
/// If the position is not found in the map (i.e. it is already a canonical
/// code or an unrecognised string), a single-element vec containing the input
/// as-is is returned.
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::expansion::expand_position;
///
/// // Compound position — expanded into multiple canonical codes
/// let result = expand_position("D(RLC)");
/// assert_eq!(result, vec!["DC", "DL", "DR"]);
///
/// // Single-position compound
/// let result = expand_position("D(C)");
/// assert_eq!(result, vec!["DC"]);
///
/// // Already canonical — returned as-is
/// let result = expand_position("ST");
/// assert_eq!(result, vec!["ST"]);
///
/// // Compound with slash notation
/// let result = expand_position("M/AM(C)");
/// assert_eq!(result, vec!["MC", "AMC"]);
/// ```
pub fn expand_position(position: &str) -> Vec<String> {
    if let Some(expansion) = POSITION_EXPANSIONS.get(position) {
        expansion.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        vec![position.to_string()]
    }
}

/// Check whether a position string contains parentheses, indicating it is a
/// compound (non-canonical) position descriptor.
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::expansion::is_compound_position;
///
/// assert!(is_compound_position("D(RLC)"));
/// assert!(!is_compound_position("DC"));
/// assert!(!is_compound_position("ST"));
/// assert!(!is_compound_position("GK"));
/// ```
pub fn is_compound_position(position: &str) -> bool {
    position.contains('(') && position.contains(')')
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- expand_position: compound positions ----

    #[test]
    fn test_expand_single_defence() {
        assert_eq!(expand_position("D(C)"), vec!["DC"]);
    }

    #[test]
    fn test_expand_full_back_line() {
        assert_eq!(expand_position("D(RLC)"), vec!["DC", "DL", "DR"]);
    }

    #[test]
    fn test_expand_centre_back_pair() {
        assert_eq!(expand_position("D(LC)"), vec!["DC", "DL"]);
    }

    #[test]
    fn test_expand_defensive_mid() {
        assert_eq!(expand_position("DM(C)"), vec!["DMC"]);
    }

    #[test]
    fn test_expand_midfield_centre() {
        assert_eq!(expand_position("M(C)"), vec!["MC"]);
    }

    #[test]
    fn test_expand_midfield_three() {
        assert_eq!(expand_position("M(RLC)"), vec!["MC", "ML", "MR"]);
    }

    #[test]
    fn test_expand_attacking_mid_wide() {
        assert_eq!(expand_position("AM(RL)"), vec!["AML", "AMR"]);
    }

    #[test]
    fn test_expand_attacking_mid_centre() {
        assert_eq!(expand_position("AM(C)"), vec!["AMC"]);
    }

    #[test]
    fn test_expand_striker() {
        assert_eq!(expand_position("ST(C)"), vec!["ST"]);
    }

    #[test]
    fn test_expand_gk() {
        assert_eq!(expand_position("GK"), vec!["GK"]);
    }

    #[test]
    fn test_expand_slash_notation_dm_m_am() {
        assert_eq!(expand_position("DM/M/AM(C)"), vec!["DMC", "MC", "AMC"]);
    }

    #[test]
    fn test_expand_slash_notation_dm_m() {
        assert_eq!(expand_position("DM/M(C)"), vec!["DMC", "MC"]);
    }

    #[test]
    fn test_expand_slash_notation_m_am() {
        assert_eq!(expand_position("M/AM(C)"), vec!["MC", "AMC"]);
    }

    #[test]
    fn test_expand_wb_m_left() {
        assert_eq!(expand_position("WB/M(L)"), vec!["WB_L", "ML"]);
    }

    #[test]
    fn test_expand_wb_m_right() {
        assert_eq!(expand_position("WB/M(R)"), vec!["WB_R", "MR"]);
    }

    #[test]
    fn test_expand_striker_wide_left() {
        assert_eq!(expand_position("ST(L)"), vec!["ST_L"]);
    }

    #[test]
    fn test_expand_striker_wide_right() {
        assert_eq!(expand_position("ST(R)"), vec!["ST_R"]);
    }

    #[test]
    fn test_expand_am_st_wide_left() {
        assert_eq!(expand_position("AM(L)/ST(L)"), vec!["AML", "ST_L"]);
    }

    #[test]
    fn test_expand_am_st_wide_right() {
        assert_eq!(expand_position("AM(R)/ST(R)"), vec!["AMR", "ST_R"]);
    }

    #[test]
    fn test_expand_am_st_wide_both() {
        assert_eq!(
            expand_position("AM(RL)/ST(RL)"),
            vec!["AML", "AMR", "ST_L", "ST_R"]
        );
    }

    #[test]
    fn test_expand_wb_m_am_left() {
        assert_eq!(expand_position("WB/M/AM(L)"), vec!["WB_L", "ML", "AML"]);
    }

    #[test]
    fn test_expand_wb_m_am_right() {
        assert_eq!(expand_position("WB/M/AM(R)"), vec!["WB_R", "MR", "AMR"]);
    }

    #[test]
    fn test_expand_d_wb_left() {
        assert_eq!(expand_position("D(L)/WB(L)"), vec!["DL", "WB_L"]);
    }

    #[test]
    fn test_expand_d_wb_right() {
        assert_eq!(expand_position("D(R)/WB(R)"), vec!["DR", "WB_R"]);
    }

    #[test]
    fn test_expand_d_rc_wb_right() {
        assert_eq!(expand_position("D(RC)/WB(R)"), vec!["DC", "DR", "WB_R"]);
    }

    #[test]
    fn test_expand_d_lc_wb_left() {
        assert_eq!(expand_position("D(LC)/WB(L)"), vec!["DC", "DL", "WB_L"]);
    }

    // ---- expand_position: already canonical / unknown ----

    #[test]
    fn test_expand_already_canonical() {
        assert_eq!(expand_position("DC"), vec!["DC"]);
    }

    #[test]
    fn test_expand_already_canonical_st() {
        assert_eq!(expand_position("ST"), vec!["ST"]);
    }

    #[test]
    fn test_expand_already_canonical_amc() {
        assert_eq!(expand_position("AMC"), vec!["AMC"]);
    }

    #[test]
    fn test_expand_unknown_position() {
        assert_eq!(expand_position("XYZ"), vec!["XYZ"]);
    }

    #[test]
    fn test_expand_empty_string() {
        assert_eq!(expand_position(""), vec![""]);
    }

    // ---- is_compound_position ----

    #[test]
    fn test_is_compound_with_parentheses() {
        assert!(is_compound_position("D(RLC)"));
    }

    #[test]
    fn test_is_compound_single_letter() {
        assert!(is_compound_position("M(C)"));
    }

    #[test]
    fn test_is_compound_slash_notation() {
        assert!(is_compound_position("DM/M/AM(C)"));
    }

    #[test]
    fn test_is_not_compound_canonical() {
        assert!(!is_compound_position("DC"));
    }

    #[test]
    fn test_is_not_compound_short_code() {
        assert!(!is_compound_position("ST"));
    }

    #[test]
    fn test_is_not_compound_gk() {
        assert!(!is_compound_position("GK"));
    }

    #[test]
    fn test_is_not_compound_empty() {
        assert!(!is_compound_position(""));
    }

    #[test]
    fn test_is_not_compound_position_without_close_paren() {
        // Only one parenthesis — not a valid compound
        assert!(!is_compound_position("D(RLC"));
    }
}
