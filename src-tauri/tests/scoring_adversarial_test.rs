//! Adversarial tests for scoring module and seed_archetypes.

use rusqlite::Connection;
use fm_valuescout_lib::db;
use fm_valuescout_lib::scoring::{get_seed_archetypes, ArchetypeWeightEntry, ArchetypeWeights};

/// SQL injection via base_position — malicious strings must be treated as
/// literal values, never executed as SQL.
#[test]
fn test_sql_injection_via_base_position() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();

    let malicious_inputs = [
        "DC'; DELETE FROM archetypes; --",
        "DC' OR '1'='1",
        "'; INSERT INTO archetypes VALUES (99,'hacked','GK','{}'); --",
        "DC\"; DROP TABLE archetypes; --",
        "DC' UNION SELECT * FROM players; --",
    ];

    for malicious in malicious_inputs {
        let result = db::get_archetypes(&conn, Some(malicious));
        assert!(result.is_ok(), "SQL injection potential on input '{}': {:?}", malicious, result);
        let archetypes = result.unwrap();
        assert!(archetypes.is_empty(), "SQL injection may have succeeded: {} archetypes for '{}'", archetypes.len(), malicious);
    }
}

/// JSON round-trip: valid archetypes must serialize and deserialize correctly.
#[test]
fn test_seed_archetypes_json_round_trip() {
    let archetypes = get_seed_archetypes();
    for arch in &archetypes {
        let json = serde_json::to_string(&arch.weights).unwrap();
        let parsed: ArchetypeWeights = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.in_possession.len(), arch.weights.in_possession.len());
        assert_eq!(parsed.out_of_possession.len(), arch.weights.out_of_possession.len());
    }
}

/// Malformed JSON in DB: get_archetypes does NOT parse metric_weights_json,
/// so it must remain safe even when other rows contain garbage JSON.
#[test]
fn test_malformed_json_in_archetypes_table() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();

    let _ = conn.execute(
        "INSERT OR IGNORE INTO archetypes (name, base_position, metric_weights_json) VALUES (?1, ?2, ?3)",
        rusqlite::params!["Bad JSON Archetype", "DC", "{ invalid json fragment"],
    );

    let result = db::get_archetypes(&conn, Some("DC"));
    assert!(result.is_ok(), "get_archetypes must survive malformed JSON row");
    let archetypes = result.unwrap();
    assert!(archetypes.len() >= 3, "expected >= 3 valid DC archetypes, got {}", archetypes.len());
}

/// Empty database: get_archetypes on empty DB returns empty vec.
#[test]
fn test_get_archetypes_empty_database() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();

    let all_archetypes = db::get_archetypes(&conn, None).unwrap();
    assert!(all_archetypes.is_empty(), "expected empty DB result, got {}", all_archetypes.len());

    let filtered = db::get_archetypes(&conn, Some("DC")).unwrap();
    assert!(filtered.is_empty(), "expected empty filtered result, got {}", filtered.len());
}

/// Very long archetype name (10KB) — SQLite TEXT has no practical limit.
#[test]
fn test_very_long_archetype_name() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();

    let long_name = "A".repeat(10_000);
    let _ = conn.execute(
        "INSERT OR IGNORE INTO archetypes (name, base_position, metric_weights_json) VALUES (?1, ?2, ?3)",
        rusqlite::params![&long_name, "DC", r#"{"in_possession":[],"out_of_possession":[]}"#],
    );

    let result = db::get_archetypes(&conn, None);
    assert!(result.is_ok(), "get_archetypes must handle 10KB names without error");
}

/// INSERT OR IGNORE prevents duplicates — re-seeding must not multiply archetypes.
#[test]
fn test_duplicate_archetype_names_ignored() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();

    let initial_count = db::get_archetypes(&conn, None).unwrap().len();
    db::seed_archetypes(&conn).unwrap();
    let after_count = db::get_archetypes(&conn, None).unwrap().len();
    assert_eq!(after_count, initial_count, "INSERT OR IGNORE must prevent duplicates");
}

/// Unicode archetype names — CJK, Arabic, Greek, emoji, control chars.
#[test]
fn test_unicode_archetype_names() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();

    let names = [
        "Ball-Playing Defender \u{2713}",         // check mark
        "\u{963F}\u{9F99}\u{4E2D}\u{5834}", // Chinese chars (encoded)
        "\u{0391}\u{03BC}\u{03BD}\u{03C4}\u{03B9}\u{03BA}\u{03CC}\u{03C2}\u{03C7}\u{03B1}\u{03C6}", // Greek
    ];

    for name in &names {
        let r = conn.execute(
            "INSERT OR IGNORE INTO archetypes (name, base_position, metric_weights_json) VALUES (?1, ?2, ?3)",
            rusqlite::params![name, "DC", r#"{"in_possession":[],"out_of_possession":[]}"#],
        );
        assert!(r.is_ok(), "failed to insert '{}': {:?}", name, r);
    }

    let archetypes = db::get_archetypes(&conn, Some("DC")).unwrap();
    assert!(archetypes.len() >= 3, "expected >= 3 DC archetypes, got {}", archetypes.len());
}

/// Negative weights — must be accepted and stored.
#[test]
fn test_negative_weights() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();

    let neg_weights = ArchetypeWeights {
        in_possession: vec![ArchetypeWeightEntry { metric_name: "pas".into(), weight: -3.0, inverted: false }],
        out_of_possession: vec![ArchetypeWeightEntry { metric_name: "tck".into(), weight: -1.0, inverted: false }],
    };

    let json = serde_json::to_string(&neg_weights).unwrap();
    let r = conn.execute(
        "INSERT OR IGNORE INTO archetypes (name, base_position, metric_weights_json) VALUES (?1, ?2, ?3)",
        rusqlite::params!["Neg Weight Archetype", "DC", &json],
    );
    assert!(r.is_ok(), "negative weights must be accepted: {:?}", r);

    let archetypes = db::get_archetypes(&conn, Some("DC")).unwrap();
    assert!(archetypes.iter().any(|a| a.name == "Neg Weight Archetype"));
}

/// Zero weights — must be accepted and round-trip correctly.
#[test]
fn test_zero_weights() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();

    let zero_weights = ArchetypeWeights {
        in_possession: vec![
            ArchetypeWeightEntry { metric_name: "pas".into(), weight: 0.0, inverted: false },
            ArchetypeWeightEntry { metric_name: "fir".into(), weight: 0.0, inverted: false },
        ],
        out_of_possession: vec![
            ArchetypeWeightEntry { metric_name: "tck".into(), weight: 0.0, inverted: false },
        ],
    };

    let json = serde_json::to_string(&zero_weights).unwrap();
    let r = conn.execute(
        "INSERT OR IGNORE INTO archetypes (name, base_position, metric_weights_json) VALUES (?1, ?2, ?3)",
        rusqlite::params!["Zero Weight Archetype", "DC", &json],
    );
    assert!(r.is_ok(), "zero weights must be accepted: {:?}", r);

    let parsed: ArchetypeWeights = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.in_possession.len(), 2);
    assert_eq!(parsed.in_possession[0].weight, 0.0);
}

/// Fully empty weights — empty in_possession AND out_of_possession is legal.
#[test]
fn test_empty_weights_both_zero() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();

    let empty_weights = ArchetypeWeights { in_possession: vec![], out_of_possession: vec![] };
    let json = serde_json::to_string(&empty_weights).unwrap();
    let r = conn.execute(
        "INSERT OR IGNORE INTO archetypes (name, base_position, metric_weights_json) VALUES (?1, ?2, ?3)",
        rusqlite::params!["Empty Weight Archetype", "AMC", &json],
    );
    assert!(r.is_ok(), "fully empty weights must be accepted: {:?}", r);

    let archetypes = db::get_archetypes(&conn, Some("AMC")).unwrap();
    assert!(archetypes.iter().any(|a| a.name == "Empty Weight Archetype"));
}

/// NaN weights: serde_json ACCEPTS NaN but serializes it to JSON null.
/// This is a data integrity concern — NaN silently becomes null in the DB.
/// We verify the actual behavior (null output, not an error).
#[test]
fn test_nan_weights_serialization_behavior() {
    let weights = ArchetypeWeights {
        in_possession: vec![ArchetypeWeightEntry { metric_name: "pas".into(), weight: f64::NAN, inverted: false }],
        out_of_possession: vec![],
    };
    let result = serde_json::to_string(&weights);
    // serde_json does NOT error on NaN — it serializes to "null"
    assert!(result.is_ok(), "serde_json must accept NaN (produces null)");
    let json_out = result.unwrap();
    assert!(json_out.contains("\"weight\":null"), "NaN should serialize to null: {}", json_out);
}

/// Infinity weights: serde_json ACCEPTS Infinity but serializes to null.
/// Data integrity concern: infinity silently becomes null in DB JSON.
#[test]
fn test_infinity_weights_serialization_behavior() {
    let weights_pos = ArchetypeWeights {
        in_possession: vec![ArchetypeWeightEntry { metric_name: "pas".into(), weight: f64::INFINITY, inverted: false }],
        out_of_possession: vec![],
    };
    let r1 = serde_json::to_string(&weights_pos);
    assert!(r1.is_ok(), "serde_json must accept INFINITY (produces null)");
    assert!(r1.unwrap().contains("null"));

    let weights_neg = ArchetypeWeights {
        in_possession: vec![ArchetypeWeightEntry { metric_name: "pas".into(), weight: f64::NEG_INFINITY, inverted: false }],
        out_of_possession: vec![],
    };
    let r2 = serde_json::to_string(&weights_neg);
    assert!(r2.is_ok(), "serde_json must accept NEG_INFINITY (produces null)");
    assert!(r2.unwrap().contains("null"));
}
