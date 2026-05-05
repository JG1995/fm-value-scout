use rusqlite::Connection;
use fm_valuescout_lib::db;
use fm_valuescout_lib::scoring::get_seed_archetypes;

const VALID_POSITIONS: &[&str] = &["DC", "DL", "DR", "DM", "MC", "AMC", "AML", "AMR", "ST", "GK"];

const VALID_METRICS: &[&str] = &[
    "cor", "cro", "dri", "fin", "fir", "fre", "hea", "lon", "lon_thr", "mar", "pas", "pen", "tck", "tec",
    "agg", "ant", "bra", "cmp", "cnt", "dec", "det", "fla", "inf", "lea", "otb", "pos", "tea", "vis", "wor",
    "acc", "agi", "bal", "jum", "nat_fit", "pac", "sta", "str",
    "aer", "com_gk", "ecc", "han", "kic", "one_on_one", "pun", "ref_gk", "rus", "thr", "cmd",
    "goals_xg", "npxg", "xg_op", "xg_per_shot", "shots", "shots_on_target",
    "sot_pct", "conv_pct", "pens_scored", "pens_taken",
    "assists", "xa", "key_passes", "chances_created", "op_key_passes", "through_balls",
    "dribbles_per_game", "progressive_passes", "progressive_runs",
    "passes_completed", "passes_attempted", "pass_completion_pct",
    "crosses_attempted", "crosses_completed", "cross_completion_pct",
    "tackles_per_game", "tackles_completed", "tackle_completion_pct",
    "interceptions_per_game", "clearances", "blocks",
    "possession_won", "possession_lost",
    "headers_won", "headers_lost", "headers_won_pct",
    "saves", "save_pct", "goals_conceded", "clean_sheets",
    "penalties_saved", "expected_saves",
    "fouls_made", "fouls_against", "yellow_cards", "red_cards", "offsides",
    "distance_covered", "average_rating", "player_of_match",
];

fn is_valid_metric(name: &str) -> bool {
    VALID_METRICS.contains(&name)
}

#[test]
fn test_seed_archetype_count() {
    let archetypes = get_seed_archetypes();
    assert_eq!(archetypes.len(), 20, "expected exactly 20 archetypes, got {}", archetypes.len());
}

#[test]
fn test_archetype_names_unique() {
    let archetypes = get_seed_archetypes();
    let mut names: Vec<&str> = archetypes.iter().map(|a| a.name).collect();
    names.sort();
    for i in 1..names.len() {
        assert_ne!(names[i], names[i - 1], "duplicate archetype name found: {}", names[i]);
    }
}

#[test]
fn test_valid_base_positions() {
    let archetypes = get_seed_archetypes();
    for arch in &archetypes {
        assert!(
            VALID_POSITIONS.contains(&arch.base_position),
            "invalid base_position '{}' for archetype '{}'",
            arch.base_position,
            arch.name
        );
    }
}

#[test]
fn test_valid_metric_names() {
    let archetypes = get_seed_archetypes();
    for arch in &archetypes {
        for entry in arch.weights.in_possession.iter().chain(arch.weights.out_of_possession.iter()) {
            assert!(
                is_valid_metric(&entry.metric_name),
                "invalid metric_name '{}' in archetype '{}'",
                entry.metric_name,
                arch.name
            );
        }
    }
}

#[test]
fn test_no_empty_weights() {
    let archetypes = get_seed_archetypes();
    for arch in &archetypes {
        let has_in = !arch.weights.in_possession.is_empty();
        let has_out = !arch.weights.out_of_possession.is_empty();
        assert!(
            has_in || has_out,
            "archetype '{}' has BOTH empty in_possession AND out_of_possession",
            arch.name
        );
    }
}

#[test]
fn test_seed_archetypes_db() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();
    let archetypes = db::get_archetypes(&conn, None).unwrap();
    assert_eq!(archetypes.len(), 20, "expected 20 archetypes in DB, got {}", archetypes.len());
}

#[test]
fn test_seed_archetypes_idempotent() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();
    let archetypes = db::get_archetypes(&conn, None).unwrap();
    assert_eq!(archetypes.len(), 20);
}

#[test]
fn test_get_archetypes_filtered_dc() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();
    let archetypes = db::get_archetypes(&conn, Some("DC")).unwrap();
    assert_eq!(archetypes.len(), 3, "expected 3 DC archetypes, got {}", archetypes.len());
    let names: Vec<&str> = archetypes.iter().map(|a| a.name.as_str()).collect();
    assert!(names.contains(&"Ball-Playing Defender"), "missing Ball-Playing Defender");
    assert!(names.contains(&"No-Nonsense Centre Back"), "missing No-Nonsense Centre Back");
    assert!(names.contains(&"Complete Centre Back"), "missing Complete Centre Back");
}

#[test]
fn test_get_archetypes_filtered_nonexistent() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();
    let archetypes = db::get_archetypes(&conn, Some("NONEXISTENT")).unwrap();
    assert!(archetypes.is_empty(), "expected empty for nonexistent position, got {}", archetypes.len());
}
