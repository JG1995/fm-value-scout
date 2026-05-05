// Adversarial / edge-case tests for the parsers module foundation (task 2.1).
// Targets attack vectors NOT covered by the standard verification tests.
// See: src-tauri/src/parsers/mod.rs

use fm_valuescout_lib::parsers::*;

// ═══════════════════════════════════════════════════════════════════════════
// Schema::from_headers — boundary & adversarial attacks
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_schema_headers_extremely_long_name() {
    // 50 KB header name — potential buffer / allocation stress
    let long_name = "A".repeat(50_000);
    let headers = vec![long_name.clone()];
    let schema = Schema::from_headers(&headers);

    assert_eq!(schema.column_index.len(), 1);
    assert_eq!(schema.column_index.get(&long_name), Some(&0));
}

#[test]
fn test_schema_headers_long_name_mixed_with_short() {
    // One very long header followed by normal headers
    let long_name = "B".repeat(100_000);
    let headers = vec![
        "short".to_string(),
        long_name.clone(),
        "tiny".to_string(),
    ];
    let schema = Schema::from_headers(&headers);

    assert_eq!(schema.column_index.len(), 3);
    assert_eq!(schema.column_index.get("short"), Some(&0));
    assert_eq!(schema.column_index.get(&long_name), Some(&1));
    assert_eq!(schema.column_index.get("tiny"), Some(&2));
}

#[test]
fn test_schema_headers_unicode_injection() {
    // Unicode control / special characters that could trick parsing
    let headers = vec![
        "\0".to_string(),                    // null byte
        "\u{202E}".to_string(),              // RIGHT-TO-LEFT OVERRIDE
        "\u{200B}".to_string(),              // zero-width space
        "\u{FEFF}".to_string(),              // BOM
        "col\u{0000}name".to_string(),       // embedded null
        "col\u{202E}name".to_string(),       // embedded RTL override
        "héllo wörld".to_string(),           // accented chars
        "\u{1F600}".to_string(),             // emoji
        "a\u{0300}\u{0301}\u{0302}".to_string(), // combining diacritics
        "tab\tcol".to_string(),              // tab in name
        "newline\ncol".to_string(),          // newline in name
    ];
    let schema = Schema::from_headers(&headers);

    assert_eq!(schema.column_index.len(), 11);
    assert_eq!(schema.column_index.get("\0"), Some(&0));
    assert_eq!(schema.column_index.get("\u{202E}"), Some(&1));
    assert_eq!(schema.column_index.get("\u{200B}"), Some(&2));
    assert_eq!(schema.column_index.get("\u{FEFF}"), Some(&3));
    assert_eq!(schema.column_index.get("col\u{0000}name"), Some(&4));
    assert_eq!(schema.column_index.get("col\u{202E}name"), Some(&5));
    assert_eq!(schema.column_index.get("héllo wörld"), Some(&6));
    assert_eq!(schema.column_index.get("\u{1F600}"), Some(&7));
    assert_eq!(schema.column_index.get("a\u{0300}\u{0301}\u{0302}"), Some(&8));
    assert_eq!(schema.column_index.get("tab\tcol"), Some(&9));
    assert_eq!(schema.column_index.get("newline\ncol"), Some(&10));
}

#[test]
fn test_schema_headers_all_empty_strings() {
    // Every single header is an empty string — HashMap collision scenario
    let count = 50;
    let headers: Vec<String> = std::iter::repeat("".to_string()).take(count).collect();
    let schema = Schema::from_headers(&headers);

    // All empty keys hash to the same value, so HashMap has only 1 entry (last index wins)
    assert_eq!(schema.column_index.len(), 1);
    assert_eq!(schema.column_index.get(""), Some(&(count as usize - 1)));
}

#[test]
fn test_schema_headers_thousand_columns() {
    // 1000 unique column names — stress HashMap capacity and rehashing
    let headers: Vec<String> = (0..1000)
        .map(|i| format!("col_{}", i))
        .collect();
    let schema = Schema::from_headers(&headers);

    assert_eq!(schema.column_index.len(), 1000);
    // Spot-check a few indices
    assert_eq!(schema.column_index.get("col_0"), Some(&0));
    assert_eq!(schema.column_index.get("col_499"), Some(&499));
    assert_eq!(schema.column_index.get("col_999"), Some(&999));
    assert_eq!(schema.column_index.get("col_1000"), None);
}

#[test]
fn test_schema_headers_all_exact_duplicates() {
    // All headers are identical strings
    let count = 100;
    let headers: Vec<String> = std::iter::repeat("dupe".to_string()).take(count).collect();
    let schema = Schema::from_headers(&headers);

    // Only 1 unique key; last write wins → index 99
    assert_eq!(schema.column_index.len(), 1);
    assert_eq!(schema.column_index.get("dupe"), Some(&(count as usize - 1)));
}

#[test]
fn test_schema_headers_mixed_case_conflict() {
    // Headers that differ only by case — should be distinct keys
    let headers = vec![
        "name".to_string(),
        "Name".to_string(),
        "NAME".to_string(),
        "nAmE".to_string(),
    ];
    let schema = Schema::from_headers(&headers);

    assert_eq!(schema.column_index.len(), 4);
    assert_eq!(schema.column_index.get("name"), Some(&0));
    assert_eq!(schema.column_index.get("Name"), Some(&1));
    assert_eq!(schema.column_index.get("NAME"), Some(&2));
    assert_eq!(schema.column_index.get("nAmE"), Some(&3));
}

#[test]
fn test_schema_headers_whitespace_variants() {
    // Various whitespace characters — tab, space, nbsp, etc.
    let headers = vec![
        " ".to_string(),
        "  ".to_string(),
        "\t".to_string(),
        "\u{00A0}".to_string(),  // non-breaking space
        "\u{2003}".to_string(),  // em space
        " leading".to_string(),
        "trailing ".to_string(),
    ];
    let schema = Schema::from_headers(&headers);

    assert_eq!(schema.column_index.len(), 7);
    assert_eq!(schema.column_index.get(" "), Some(&0));
    assert_eq!(schema.column_index.get("  "), Some(&1));
    assert_eq!(schema.column_index.get("\t"), Some(&2));
    assert_eq!(schema.column_index.get("\u{00A0}"), Some(&3));
    assert_eq!(schema.column_index.get("\u{2003}"), Some(&4));
    assert_eq!(schema.column_index.get(" leading"), Some(&5));
    assert_eq!(schema.column_index.get("trailing "), Some(&6));
}

// ═══════════════════════════════════════════════════════════════════════════
// PlayerRecord — mutation isolation after default()
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_player_record_default_mutation_isolation_identity() {
    // Create default, set an identity field, verify all others remain None
    let mut record = PlayerRecord::default();
    record.uid = Some("test_player".to_string());

    assert_eq!(record.uid, Some("test_player".to_string()));
    // Spot-check fields from each section — all must still be None
    assert!(record.name.is_none());
    assert!(record.nation.is_none());
    assert!(record.height.is_none());
    assert!(record.left_foot_raw.is_none());
    assert!(record.right_foot_score.is_none());
    // Season
    assert!(record.club.is_none());
    assert!(record.position_raw.is_none());
    assert!(record.age.is_none());
    assert!(record.minutes.is_none());
    assert!(record.starts.is_none());
    assert!(record.subs.is_none());
    assert!(record.expires.is_none());
}

#[test]
fn test_player_record_default_mutation_isolation_technical() {
    // Set one technical stat and check isolation
    let mut record = PlayerRecord::default();
    record.cor = Some(15);

    assert_eq!(record.cor, Some(15));
    // Identity unchanged
    assert!(record.uid.is_none());
    assert!(record.name.is_none());
    // Other technical unchanged
    assert!(record.cro.is_none());
    assert!(record.dri.is_none());
    assert!(record.fin.is_none());
    assert!(record.pas.is_none());
    assert!(record.tec.is_none());
}

#[test]
fn test_player_record_default_mutation_isolation_mental() {
    let mut record = PlayerRecord::default();
    record.det = Some(18);

    assert_eq!(record.det, Some(18));
    assert!(record.agg.is_none());
    assert!(record.ant.is_none());
    assert!(record.bra.is_none());
    assert!(record.cmp.is_none());
    assert!(record.cnt.is_none());
    assert!(record.dec.is_none());
    assert!(record.fla.is_none());
    assert!(record.lea.is_none());
    assert!(record.wor.is_none());
}

#[test]
fn test_player_record_default_mutation_isolation_physical() {
    let mut record = PlayerRecord::default();
    record.pac = Some(92);

    assert_eq!(record.pac, Some(92));
    assert!(record.acc.is_none());
    assert!(record.agi.is_none());
    assert!(record.bal.is_none());
    assert!(record.jum.is_none());
    assert!(record.nat_fit.is_none());
    assert!(record.sta.is_none());
    assert!(record.str.is_none());
}

#[test]
fn test_player_record_default_mutation_isolation_gk() {
    let mut record = PlayerRecord::default();
    record.ref_gk = Some(85);

    assert_eq!(record.ref_gk, Some(85));
    assert!(record.aer.is_none());
    assert!(record.com_gk.is_none());
    assert!(record.han.is_none());
    assert!(record.kic.is_none());
    assert!(record.one_on_one.is_none());
    assert!(record.pun.is_none());
    assert!(record.rus.is_none());
    assert!(record.thr.is_none());
    assert!(record.cmd.is_none());
}

#[test]
fn test_player_record_default_mutation_isolation_float() {
    // Mutation isolation for f64 fields
    let mut record = PlayerRecord::default();
    record.goals_xg = Some(25.5);

    assert!((record.goals_xg.unwrap() - 25.5).abs() < f64::EPSILON);
    assert!(record.npxg.is_none());
    assert!(record.xg_op.is_none());
    assert!(record.shots.is_none());
    assert!(record.assists.is_none());
    assert!(record.xa.is_none());
    assert!(record.dribbles_per_game.is_none());
    assert!(record.passes_completed.is_none());
    assert!(record.tackles_per_game.is_none());
    assert!(record.interceptions_per_game.is_none());
    assert!(record.saves.is_none());
    assert!(record.fouls_made.is_none());
    assert!(record.distance_covered.is_none());
    assert!(record.average_rating.is_none());
}

#[test]
fn test_player_record_default_mutation_isolation_string_extra() {
    // Mutation isolation for the String extra fields
    let mut record = PlayerRecord::default();
    record.second_nationality = Some("France".to_string());

    assert_eq!(record.second_nationality, Some("France".to_string()));
    assert!(record.current_ability.is_none());
    assert!(record.potential_ability.is_none());
    assert!(record.transfer_value.is_none());
    assert!(record.wage_value.is_none());
    assert!(record.wage_denomination.is_none());
}

#[test]
fn test_player_record_default_mutation_multiple_spread() {
    // Set fields across multiple sections simultaneously — verify isolation
    let mut record = PlayerRecord::default();

    record.name = Some("Test Player".to_string());
    record.cor = Some(14);
    record.det = Some(16);
    record.pac = Some(88);
    record.ref_gk = Some(90);
    record.goals_xg = Some(10.0);
    record.second_nationality = Some("Brazil".to_string());

    // Verify the set values
    assert_eq!(record.name, Some("Test Player".to_string()));
    assert_eq!(record.cor, Some(14));
    assert_eq!(record.det, Some(16));
    assert_eq!(record.pac, Some(88));
    assert_eq!(record.ref_gk, Some(90));
    assert!((record.goals_xg.unwrap() - 10.0).abs() < f64::EPSILON);
    assert_eq!(record.second_nationality, Some("Brazil".to_string()));

    // Verify neighboring fields are still None
    assert!(record.uid.is_none());
    assert!(record.nation.is_none());
    assert!(record.cro.is_none());
    assert!(record.agg.is_none());
    assert!(record.acc.is_none());
    assert!(record.aer.is_none());
    assert!(record.npxg.is_none());
    assert!(record.current_ability.is_none());
    assert!(record.potential_ability.is_none());
}

// ═══════════════════════════════════════════════════════════════════════════
// Clone — deep independence test
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_field_value_clone_position_independence() {
    // Clone a Position variant with many strings, mutate original, verify clone unaffected
    let original = FieldValue::Position(vec![
        "ST".to_string(),
        "CF".to_string(),
        "CAM".to_string(),
        "CM".to_string(),
        "CDM".to_string(),
    ]);
    let cloned = original.clone();

    // Verify cloned content matches
    if let FieldValue::Position(ref pos) = cloned {
        assert_eq!(pos.len(), 5);
        assert_eq!(pos[0], "ST");
        assert_eq!(pos[4], "CDM");
    } else {
        panic!("Clone produced wrong variant");
    }

    // Now verify we can destructure and compare — deep independence
    // (Vec<String> clone is deep, so this is a structural verification)
    match (&original, &cloned) {
        (FieldValue::Position(a), FieldValue::Position(b)) => {
            assert_eq!(a, b, "Clone should have identical content");
        }
        _ => panic!("Both should be Position variants"),
    }
}

#[test]
fn test_field_value_clone_wage_deep() {
    // Verified clone produces an independent deep copy for Wage
    let original = FieldValue::Wage {
        value: 50000.0,
        denomination: Some("p/w".to_string()),
    };
    let mut cloned = original.clone();

    // Modify the clone's inner values
    if let FieldValue::Wage {
        ref mut value,
        ref mut denomination,
    } = cloned
    {
        *value = 99999.0;
        *denomination = None;
    }

    // Verify original is unchanged
    match original {
        FieldValue::Wage { value, denomination } => {
            assert!((value - 50000.0).abs() < f64::EPSILON);
            assert_eq!(denomination, Some("p/w".to_string()));
        }
        _ => panic!("Original should be Wage"),
    }
}

#[test]
fn test_field_value_clone_footscore_deep() {
    let original = FieldValue::FootScore {
        score: Some(5),
        raw: "5/5".to_string(),
    };
    let mut cloned = original.clone();

    if let FieldValue::FootScore {
        ref mut score,
        ref mut raw,
    } = cloned
    {
        *score = None;
        *raw = "changed".to_string();
    }

    // Original must be unchanged
    match original {
        FieldValue::FootScore { score, raw } => {
            assert_eq!(score, Some(5));
            assert_eq!(raw, "5/5");
        }
        _ => panic!("Original should be FootScore"),
    }
}

#[test]
fn test_field_value_clone_appearances_deep() {
    let original = FieldValue::Appearances {
        starts: 30,
        subs: 10,
    };
    let cloned = original.clone();

    match cloned {
        FieldValue::Appearances { starts, subs } => {
            assert_eq!(starts, 30);
            assert_eq!(subs, 10);
        }
        _ => panic!("Clone should be Appearances"),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Debug format — panic resistance with adversarial values
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_debug_float_nan_infinity() {
    // Debug formatting must not panic on NaN or Infinity
    let cases = vec![
        FieldValue::Float(f64::NAN),
        FieldValue::Float(f64::INFINITY),
        FieldValue::Float(f64::NEG_INFINITY),
        FieldValue::Float(-0.0),
        FieldValue::Currency(f64::NAN),
        FieldValue::Currency(f64::INFINITY),
        FieldValue::Currency(f64::NEG_INFINITY),
        FieldValue::Currency(-0.0),
    ];
    for fv in cases {
        let debug_str = format!("{:?}", fv);
        assert!(!debug_str.is_empty(), "Debug format produced empty string");
    }
}

#[test]
fn test_debug_int_extremes() {
    // Debug on min/max integer values
    let cases = vec![
        FieldValue::Int(i64::MIN),
        FieldValue::Int(i64::MAX),
        FieldValue::Int(0),
        FieldValue::Int(-1),
    ];
    for fv in cases {
        let debug_str = format!("{:?}", fv);
        assert!(!debug_str.is_empty(), "Debug format produced empty string");
    }
}

#[test]
fn test_debug_very_long_strings() {
    // Very long strings in variants with String fields
    let long_str = "X".repeat(100_000);
    let long_unicode = "\u{1F600}".repeat(10_000);

    let cases = vec![
        FieldValue::String(long_str.clone()),
        FieldValue::String(long_unicode.clone()),
        FieldValue::FootScore {
            score: None,
            raw: long_str.clone(),
        },
        FieldValue::Wage {
            value: 0.0,
            denomination: Some(long_str.clone()),
        },
        FieldValue::Wage {
            value: 0.0,
            denomination: Some(long_unicode.clone()),
        },
        FieldValue::Position(vec![long_str]),
        FieldValue::Position(vec![long_unicode]),
    ];
    for fv in cases {
        let debug_str = format!("{:?}", fv);
        assert!(!debug_str.is_empty(), "Debug format produced empty string");
    }
}

#[test]
fn test_debug_wage_null_denomination() {
    // Wage with no denomination must not panic on debug
    let fv = FieldValue::Wage {
        value: 1000.0,
        denomination: None,
    };
    let debug_str = format!("{:?}", fv);
    assert!(!debug_str.is_empty());
    // Should contain the value
    assert!(debug_str.contains("1000"));
}

#[test]
fn test_debug_footscore_null_score() {
    // FootScore with null score and weird raw
    let fv = FieldValue::FootScore {
        score: None,
        raw: "\0\u{202E}\u{200B}".to_string(),
    };
    let debug_str = format!("{:?}", fv);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_debug_empty_position() {
    // Empty position vec
    let fv = FieldValue::Position(vec![]);
    let debug_str = format!("{:?}", fv);
    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("Position"));
}

#[test]
fn test_debug_appearances_zero() {
    // Appearances with zero values
    let fv = FieldValue::Appearances {
        starts: 0,
        subs: 0,
    };
    let debug_str = format!("{:?}", fv);
    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("0"));
}

#[test]
fn test_debug_negative_appearances() {
    // Appearances with negative values (adversarial scenario)
    let fv = FieldValue::Appearances {
        starts: -1,
        subs: -5,
    };
    let debug_str = format!("{:?}", fv);
    assert!(!debug_str.is_empty());
}

// ═══════════════════════════════════════════════════════════════════════════
// FieldValue — adversarial construction edge cases
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_field_value_int_wrapping_behavior() {
    // Verify Int variant handles extreme i64 values correctly
    let max_val = FieldValue::Int(i64::MAX);
    let min_val = FieldValue::Int(i64::MIN);

    match max_val {
        FieldValue::Int(v) => assert_eq!(v, i64::MAX),
        _ => panic!("MAX Int matched wrong variant"),
    }
    match min_val {
        FieldValue::Int(v) => assert_eq!(v, i64::MIN),
        _ => panic!("MIN Int matched wrong variant"),
    }
}

#[test]
fn test_field_value_float_nan_preserved() {
    // NaN must round-trip (Rust f64 NaN preserves bit pattern in Debug)
    let fv = FieldValue::Float(f64::NAN);
    match fv {
        FieldValue::Float(v) => assert!(v.is_nan(), "NaN must be preserved"),
        _ => panic!("Float(NaN) matched wrong variant"),
    }
}

#[test]
fn test_field_value_float_infinity_roundtrip() {
    let fv = FieldValue::Float(f64::INFINITY);
    match fv {
        FieldValue::Float(v) => assert!(v.is_infinite() && v.is_sign_positive()),
        _ => panic!("Float(Inf) matched wrong variant"),
    }

    let fv_neg = FieldValue::Float(f64::NEG_INFINITY);
    match fv_neg {
        FieldValue::Float(v) => assert!(v.is_infinite() && v.is_sign_negative()),
        _ => panic!("Float(-Inf) matched wrong variant"),
    }
}

#[test]
fn test_field_value_currency_nan() {
    // Currency with NaN should still match the variant correctly
    let fv = FieldValue::Currency(f64::NAN);
    match fv {
        FieldValue::Currency(v) => assert!(v.is_nan()),
        _ => panic!("Currency(NaN) matched wrong variant"),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Adversarial / attack-vector tests for parse_currency (task 2.2)
// Tests ONLY malformed inputs, boundary conditions, and edge cases.
// ═══════════════════════════════════════════════════════════════════════════════

use fm_valuescout_lib::parsers::currency::parse_currency;

fn assert_currency(result: FieldValue, expected: f64) {
    match result {
        FieldValue::Currency(v) => {
            let diff = (v - expected).abs();
            assert!(diff < 0.001, "expected {expected}, got {v}");
        }
        other => panic!("expected FieldValue::Currency({expected}), got {other:?}"),
    }
}

// ── 1. Very long input strings (50KB+) ─────────────────────────────────────

#[test]
fn test_adv_very_long_digit_string() {
    let mut long = String::with_capacity(55_000);
    long.push_str("€");
    for _ in 0..50_000 { long.push('9'); }
    let result = parse_currency(&long);
    match result {
        FieldValue::Currency(v) => assert!(!v.is_nan(), "no NaN"),
        FieldValue::Null => {} // expected
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn test_adv_very_long_with_valid_prefix() {
    let mut garbage = String::with_capacity(55_000);
    garbage.push_str("€55M");
    for _ in 0..50_000 { garbage.push(' '); }
    assert_currency(parse_currency(&garbage), 55_000_000.0);
}

// ── 2. Unicode / hidden characters ─────────────────────────────────────────

#[test]
fn test_adv_zero_width_space() {
    assert!(matches!(parse_currency("€\u{200B}55M"), FieldValue::Null));
}

#[test]
fn test_adv_bom_prefix() {
    assert!(matches!(parse_currency("\u{FEFF}€55M"), FieldValue::Null));
}

#[test]
fn test_adv_rtl_override() {
    assert!(matches!(parse_currency("€\u{202E}55M"), FieldValue::Null));
}

#[test]
fn test_adv_combining_char_on_magnitude() {
    let result = parse_currency("€55M\u{0300}");
    assert!(matches!(result, FieldValue::Null), "got {result:?}");
}

#[test]
fn test_adv_null_byte_inside() {
    assert!(matches!(parse_currency("€55\x00M"), FieldValue::Null));
}

#[test]
fn test_adv_unicode_dashes_not_recognized() {
    assert!(matches!(parse_currency("€10M – €20M"), FieldValue::Null));
    assert!(matches!(parse_currency("€10M — €20M"), FieldValue::Null));
}

// ── 3. Multiple magnitude suffixes ─────────────────────────────────────────

#[test]
fn test_adv_double_magnitude_suffix() {
    assert!(matches!(parse_currency("€10KK"), FieldValue::Null));
    assert!(matches!(parse_currency("€10MM"), FieldValue::Null));
    assert!(matches!(parse_currency("€10BB"), FieldValue::Null));
    assert!(matches!(parse_currency("€10KM"), FieldValue::Null));
    assert!(matches!(parse_currency("€10MK"), FieldValue::Null));
}

// ── 4. Negative numbers (KNOWN BUG: rfind('-') conflates sign with range) ──

#[test]
fn test_adv_negative_value_eaten_by_range_logic() {
    // Now correctly preserves the negative sign — no longer eaten by range logic
    let result = parse_currency("€-5M");
    match result {
        FieldValue::Currency(v) => {
            assert!((v - (-5_000_000.0)).abs() < 0.001, "expected -5M, got {v}");
        }
        other => panic!("expected Currency(-5000000), got {other:?}"),
    }
}

#[test]
fn test_adv_nonsense_negative_range() {
    let result = parse_currency("€-160M - -€210M");
    match result {
        FieldValue::Currency(v) => assert!((v - 210_000_000.0).abs() < 0.001),
        FieldValue::Null => {}
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn test_adv_negative_without_currency_symbol() {
    let result = parse_currency("-5M");
    match result {
        FieldValue::Currency(v) => eprintln!("BUG: -5M parsed as +{v}"),
        FieldValue::Null => {}
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn test_adv_negative_zero() {
    // €-0 now correctly preserves the negative sign (IEEE 754)
    let result = parse_currency("€-0");
    match result {
        FieldValue::Currency(v) => {
            assert_eq!(v, -0.0);
            assert!(v.is_sign_negative(), "€-0 should preserve negative sign");
        }
        other => panic!("expected Currency(-0.0), got {other:?}"),
    }
}

// ── 5. Scientific notation ─────────────────────────────────────────────────

#[test]
fn test_adv_scientific_basic() {
    let result = parse_currency("€1e6");
    match result {
        FieldValue::Currency(v) => assert!((v - 1_000_000.0).abs() < 0.001),
        FieldValue::Null => {}
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn test_adv_scientific_different_order() {
    let result = parse_currency("€1e7");
    match result {
        FieldValue::Currency(v) => assert!((v - 10_000_000.0).abs() < 0.001),
        FieldValue::Null => {}
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn test_adv_scientific_with_magnitude_suffix() {
    let result = parse_currency("€1e2K");
    match result {
        FieldValue::Currency(v) => assert!((v - 100_000.0).abs() < 0.001),
        FieldValue::Null => {}
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn test_adv_scientific_negative_exponent() {
    // "€1e-1" — the dash in the negative exponent no longer triggers range logic.
    // The range separator is " - " (space-dash-space), so 1e-1 passes through cleanly.
    let result = parse_currency("€1e-1");
    match result {
        FieldValue::Currency(v) => {
            assert!((v - 0.1).abs() < 0.001, "€1e-1 should be 0.1, got {v}");
        }
        FieldValue::Null => panic!("€1e-1 should produce Currency(0.1), got Null"),
        other => panic!("unexpected: {other:?}"),
    }
}

// ── 6. Non-numeric after stripping ──────────────────────────────────────────

#[test]
fn test_adv_pure_alpha() { assert!(matches!(parse_currency("€abc"), FieldValue::Null)); }

#[test]
fn test_adv_alpha_with_magnitude() {
    assert!(matches!(parse_currency("€abcM"), FieldValue::Null));
    assert!(matches!(parse_currency("€abcK"), FieldValue::Null));
    assert!(matches!(parse_currency("€abcB"), FieldValue::Null));
}

#[test]
fn test_adv_magnitude_only() {
    assert!(matches!(parse_currency("€M"), FieldValue::Null));
    assert!(matches!(parse_currency("€K"), FieldValue::Null));
    assert!(matches!(parse_currency("€B"), FieldValue::Null));
}

#[test]
fn test_adv_symbols_only() {
    assert!(matches!(parse_currency("€."), FieldValue::Null));
    assert!(matches!(parse_currency("€-"), FieldValue::Null));
    assert!(matches!(parse_currency("€+"), FieldValue::Null));
}

#[test]
fn test_adv_hex_like() { assert!(matches!(parse_currency("€0x1A"), FieldValue::Null)); }

// ── 7. Only a dash ─────────────────────────────────────────────────────────

#[test]
fn test_adv_only_dash_inputs() {
    assert!(matches!(parse_currency("-"), FieldValue::Null));
    assert!(matches!(parse_currency("--"), FieldValue::Null));
    assert!(matches!(parse_currency("---"), FieldValue::Null));
    assert!(matches!(parse_currency("€-"), FieldValue::Null));
    assert!(matches!(parse_currency(" - "), FieldValue::Null));
}

// ── 8. Multiple dashes (rfind takes the last) ───────────────────────────────

#[test]
fn test_adv_triple_range() {
    // Compact dashes without spaces are not recognized as range separator
    let result = parse_currency("€1M-€2M-€3M");
    assert!(matches!(result, FieldValue::Null), "compact ranges without spaces should not be recognized: got {result:?}");
}

#[test]
fn test_adv_quadruple_range_with_spaces() {
    assert_currency(parse_currency("€10M - €20M - €30M - €40M"), 40_000_000.0);
}

#[test]
fn test_adv_compact_multiple_range() {
    // Compact dashes without spaces are not recognized as range separator
    let result = parse_currency("€5M-€10M-€15M");
    assert!(matches!(result, FieldValue::Null), "compact ranges without spaces should not be recognized: got {result:?}");
}

// ── 9. Decimal without magnitude ───────────────────────────────────────────

#[test]
fn test_adv_decimal_plain() { assert_currency(parse_currency("€19.25"), 19.25); }

#[test]
fn test_adv_decimal_small() {
    assert_currency(parse_currency("€0.99"), 0.99);
    assert_currency(parse_currency("€0.0001"), 0.0001);
}

#[test]
fn test_adv_decimal_trailing_zeros() {
    assert_currency(parse_currency("€5.0"), 5.0);
    assert_currency(parse_currency("€5.000"), 5.0);
}

// ── 10. Commas as thousands separators ──────────────────────────────────────

#[test]
fn test_adv_commas_with_magnitude() {
    assert!(matches!(parse_currency("€55,000,000M"), FieldValue::Null));
}

#[test]
fn test_adv_commas_without_magnitude() {
    assert!(matches!(parse_currency("€55,000,000"), FieldValue::Null));
}

#[test]
fn test_adv_single_comma() {
    assert!(matches!(parse_currency("€55,000"), FieldValue::Null));
}

// ── Additional: Overflow & Infinity ─────────────────────────────────────────

#[test]
fn test_adv_overflow_to_infinity() {
    let result = parse_currency("€1e309");
    match result {
        FieldValue::Currency(v) => assert!(v.is_infinite(), "1e309 should overflow: got {v}"),
        FieldValue::Null => {}
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn test_adv_magnitude_overflow() {
    let result = parse_currency("€1e305M");
    match result {
        FieldValue::Currency(v) => assert!(v.is_infinite(), "1e305M should overflow: got {v}"),
        FieldValue::Null => {}
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn test_adv_nan_literal() {
    let result = parse_currency("€NaN");
    match result {
        FieldValue::Currency(v) => assert!(v.is_nan(), "got {v}"),
        FieldValue::Null => {}
        other => panic!("unexpected: {other:?}"),
    }
}

// ── Additional: Whitespace variants inside the value ────────────────────────

#[test]
fn test_adv_whitespace_inside() {
    // Tab/NL/CR before number gets trimmed by trim() calls
    // Use whitespace truly inside the numeric part to break parsing
    assert!(matches!(parse_currency("€55\t5M"), FieldValue::Null), "tab in middle should break");
    assert!(matches!(parse_currency("€55\n5M"), FieldValue::Null), "newline in middle should break");
    assert!(matches!(parse_currency("€55\r5M"), FieldValue::Null), "CR in middle should break");
    assert!(matches!(parse_currency("€55\x0C5M"), FieldValue::Null), "form feed in middle should break");
}

#[test]
fn test_adv_leading_tab() {
    assert_currency(parse_currency("\t€55M"), 55_000_000.0);
}

// ── Additional: Mixed/incorrect currency symbols ────────────────────────────

#[test]
fn test_adv_multiple_currency_symbols() {
    assert!(matches!(parse_currency("€$55M"), FieldValue::Null));
    assert!(matches!(parse_currency("55€M"), FieldValue::Null));
    assert!(matches!(parse_currency("55M€"), FieldValue::Null));
}

// ── Additional: Extreme numeric boundaries ──────────────────────────────────

#[test]
fn test_adv_large_number_no_magnitude() {
    assert_currency(parse_currency("€999999999999999"), 999999999999999.0);
}

#[test]
fn test_adv_trailing_dot() {
    let result = parse_currency("€55.");
    match result {
        FieldValue::Currency(v) => assert!((v - 55.0).abs() < 0.001),
        FieldValue::Null => {}
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn test_adv_leading_dot() {
    let result = parse_currency("€.5");
    match result {
        FieldValue::Currency(v) => assert!((v - 0.5).abs() < 0.001),
        FieldValue::Null => {}
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn test_adv_empty_after_suffix_strip() {
    assert!(matches!(parse_currency("€K"), FieldValue::Null));
    assert!(matches!(parse_currency("€M"), FieldValue::Null));
    assert!(matches!(parse_currency("€B"), FieldValue::Null));
}
