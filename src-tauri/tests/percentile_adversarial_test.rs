//! Adversarial tests for compute_percentiles function.

use fm_valuescout_lib::db;
use fm_valuescout_lib::scoring::compute_percentiles;
use rusqlite::params;

/// The exact count of metrics in PERCENTILE_METRICS — used to verify the const
/// covers all expected columns and no SQL injection is possible.
const PERCENTILE_METRIC_COUNT: usize = 99;

/// All metric names from PERCENTILE_METRICS const in scoring.rs.
/// If the const is ever bypassed, format! interpolation would allow injection
/// since these are directly interpolated into SQL column references.
const PERCENTILE_METRIC_NAMES: &[&str] = &[
    // Technical (INTEGER) - 14
    "cor", "cro", "dri", "fin", "fir", "fre", "hea", "lon", "lon_thr", "mar", "pas", "pen", "tck",
    "tec",
    // Mental (INTEGER) - 15
    "agg", "ant", "bra", "cmp", "cnt", "dec", "det", "fla", "inf", "lea", "otb", "pos", "tea", "vis",
    "wor",
    // Physical (INTEGER) - 8
    "acc", "agi", "bal", "jum", "nat_fit", "pac", "sta", "str",
    // Goalkeeper (INTEGER) - 11
    "aer", "com_gk", "ecc", "han", "kic", "one_on_one", "pun", "ref_gk", "rus", "thr", "cmd",
    // Attacking (REAL) - 10
    "goals_xg", "npxg", "xg_op", "xg_per_shot", "shots", "shots_on_target", "sot_pct", "conv_pct",
    "pens_scored", "pens_taken",
    // Creativity (REAL) - 6
    "assists", "xa", "key_passes", "chances_created", "op_key_passes", "through_balls",
    // Transition (REAL) - 9
    "dribbles_per_game", "progressive_passes", "progressive_runs", "passes_completed",
    "passes_attempted", "pass_completion_pct", "crosses_attempted", "crosses_completed",
    "cross_completion_pct",
    // Defensive (REAL) - 8
    "tackles_per_game", "tackles_completed", "tackle_completion_pct", "interceptions_per_game",
    "clearances", "blocks", "possession_won", "possession_lost",
    // Aerial (REAL) - 3
    "headers_won", "headers_lost", "headers_won_pct",
    // Goalkeeping Stats (REAL) - 6
    "saves", "save_pct", "goals_conceded", "clean_sheets", "penalties_saved", "expected_saves",
    // Discipline (REAL) - 5
    "fouls_made", "fouls_against", "yellow_cards", "red_cards", "offsides",
    // Match Impact (REAL) - 3
    "distance_covered", "average_rating", "player_of_match",
    // Transfer (REAL) - 1
    "transfer_value",
];

fn setup_db() -> rusqlite::Connection {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::migrate_db(&conn).unwrap();
    conn.execute(
        "INSERT INTO seasons (name, in_game_date, filename, player_count) VALUES ('Test', '1.1.2029', 'test.csv', 0)",
        [],
    )
    .unwrap();
    conn
}

/// Inserts a player with a single REAL metric set (xg_op) and canonical position.
/// Other metrics remain NULL.
fn insert_player_with_real_metric(
    conn: &rusqlite::Connection,
    uid: &str,
    name: &str,
    nation: &str,
    club: &str,
    position_raw: &str,
    age: i64,
    xg_op: Option<f64>,
) {
    conn.execute(
        "INSERT INTO players (uid, name, nation) VALUES (?1, ?2, ?3)",
        params![uid, name, nation],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO season_players (season_id, uid, club, position_raw, age, xg_op) VALUES (1, ?1, ?2, ?3, ?4, ?5)",
        params![uid, club, position_raw, age, xg_op],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (1, ?1, ?2)",
        params![uid, position_raw],
    )
    .unwrap();
}

/// SQL injection test via metric name: PERCENTILE_METRICS const is the only guard.
/// Since we cannot bypass the const, we verify that all metrics in the const
/// are valid season_players columns (exist in the schema). If format! interpolation
/// were used with a malicious value, the const prevents it.
#[test]
fn test_sql_injection_metric_name() {
    let conn = setup_db();

    // Verify the metric count matches our expected count
    assert_eq!(
        PERCENTILE_METRIC_NAMES.len(),
        PERCENTILE_METRIC_COUNT,
        "PERCENTILE_METRIC_COUNT should be 98"
    );

    // Verify each metric in the const is a valid column in season_players
    for metric in PERCENTILE_METRIC_NAMES {
        let sql = format!("SELECT {} FROM season_players LIMIT 1", metric);
        let result = conn.prepare(&sql);
        assert!(
            result.is_ok(),
            "Metric '{}' from PERCENTILE_METRICS is not a valid season_players column: {:?}",
            metric,
            result
        );
    }

    // Verify there are no extra columns in season_players that aren't in PERCENTILE_METRICS
    // (This ensures the const is complete and no untracked columns exist)
    let mut stmt = conn
        .prepare("PRAGMA table_info(season_players)")
        .unwrap();
    let columns: Vec<String> = stmt
        .query_map([], |row| row.get::<_, String>(1))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    // Known non-metric columns
    let non_metrics: Vec<&str> = vec![
        "id", "season_id", "uid", "club", "position_raw", "age", "minutes", "starts", "subs",
        "expires",
    ];

    let mut found_metrics = Vec::new();
    for col in &columns {
        if !non_metrics.contains(&col.as_str()) {
            found_metrics.push(col.as_str());
        }
    }

    // All found columns should be in PERCENTILE_METRICS
    for col in &found_metrics {
        assert!(
            PERCENTILE_METRIC_NAMES.contains(col),
            "Column '{}' exists in season_players but is not in PERCENTILE_METRICS const",
            col
        );
    }

    // All PERCENTILE_METRICS should be accounted for
    assert_eq!(
        found_metrics.len(),
        PERCENTILE_METRIC_COUNT,
        "Column count mismatch: found {} non-schema columns, expected {}",
        found_metrics.len(),
        PERCENTILE_METRIC_COUNT
    );
}

/// Test: all players have NULL for a metric — no percentile entries should be created.
#[test]
fn test_percentile_with_all_nulls() {
    let mut conn = setup_db();

    // Insert 5 players with NULL xg_op
    for i in 0..5 {
        insert_player_with_real_metric(
            &conn,
            &format!("player{}", i),
            &format!("Player {}", i),
            "ENG",
            "ClubA",
            "DC",
            25,
            None,
        );
    }

    let result = compute_percentiles(&mut conn, 1);
    assert!(result.is_ok(), "compute_percentiles should not error on all-null metric");

    let rows_inserted = result.unwrap();
    // With all NULL, no rows should be inserted for xg_op (WHERE sp.xg_op IS NOT NULL filters them all)
    assert_eq!(
        rows_inserted, 0,
        "All-null metric should produce 0 rows, got {}",
        rows_inserted
    );

    // Verify no percentile entries exist for xg_op
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM player_percentiles WHERE season_id = 1 AND metric_name = 'xg_op'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count, 0, "Expected 0 percentile rows for all-null xg_op, got {}", count);
}

/// Test: players with negative metric values — percentiles must sort correctly.
#[test]
fn test_percentile_with_negative_values() {
    let mut conn = setup_db();

    // Insert players with negative xg_op values
    insert_player_with_real_metric(&conn, "p1", "Neg XG Player 1", "ENG", "ClubA", "ST", 25, Some(-2.5));
    insert_player_with_real_metric(&conn, "p2", "Neg XG Player 2", "ENG", "ClubA", "ST", 25, Some(-1.0));
    insert_player_with_real_metric(&conn, "p3", "Neg XG Player 3", "ENG", "ClubA", "ST", 25, Some(-0.5));
    insert_player_with_real_metric(&conn, "p4", "Neg XG Player 4", "ENG", "ClubA", "ST", 25, Some(0.0));
    insert_player_with_real_metric(&conn, "p5", "Neg XG Player 5", "ENG", "ClubA", "ST", 25, Some(1.5));

    let result = compute_percentiles(&mut conn, 1);
    assert!(result.is_ok());

    // 1 metric * 2 pools * 5 players = 10 rows expected
    assert_eq!(result.unwrap(), 10);

    // Verify position pool percentiles are correctly ordered
    // -2.5 (lowest) should have rank 0.0
    // -1.0 should have rank 0.25
    // -0.5 should have rank 0.5
    // 0.0 should have rank 0.75
    // 1.5 (highest) should have rank 1.0
    let mut stmt = conn
        .prepare(
            "SELECT uid, percent_rank FROM player_percentiles
             WHERE season_id = 1 AND metric_name = 'xg_op' AND pool_type = 'position' AND canonical_position = 'ST'
             ORDER BY (SELECT xg_op FROM season_players sp WHERE sp.uid = player_percentiles.uid)",
        )
        .unwrap();

    let rows: Vec<(String, f64)> = stmt
        .query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    assert_eq!(rows.len(), 5);
    assert_eq!(rows[0], ("p1".to_string(), 0.0)); // -2.5
    assert_eq!(rows[1], ("p2".to_string(), 0.25)); // -1.0
    assert_eq!(rows[2], ("p3".to_string(), 0.5)); // -0.5
    assert_eq!(rows[3], ("p4".to_string(), 0.75)); // 0.0
    assert_eq!(rows[4], ("p5".to_string(), 1.0)); // 1.5

    // Also verify the 'all' pool
    let mut stmt_all = conn
        .prepare(
            "SELECT uid, percent_rank FROM player_percentiles
             WHERE season_id = 1 AND metric_name = 'xg_op' AND pool_type = 'all'
             ORDER BY (SELECT xg_op FROM season_players sp WHERE sp.uid = player_percentiles.uid)",
        )
        .unwrap();

    let rows_all: Vec<(String, f64)> = stmt_all
        .query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    assert_eq!(rows_all.len(), 5);
    assert_eq!(rows_all[0], ("p1".to_string(), 0.0));
    assert_eq!(rows_all[1], ("p2".to_string(), 0.25));
    assert_eq!(rows_all[2], ("p3".to_string(), 0.5));
    assert_eq!(rows_all[3], ("p4".to_string(), 0.75));
    assert_eq!(rows_all[4], ("p5".to_string(), 1.0));
}

/// Test: very large values (1e15) — no overflow in percentile computation.
#[test]
fn test_percentile_with_very_large_values() {
    let mut conn = setup_db();

    // Insert players with very large values
    insert_player_with_real_metric(&conn, "p1", "Large Player 1", "ENG", "ClubA", "ST", 25, Some(1e15));
    insert_player_with_real_metric(&conn, "p2", "Large Player 2", "ENG", "ClubA", "ST", 25, Some(2e15));
    insert_player_with_real_metric(&conn, "p3", "Large Player 3", "ENG", "ClubA", "ST", 25, Some(3e15));

    let result = compute_percentiles(&mut conn, 1);
    assert!(result.is_ok(), "compute_percentiles should not overflow with large values: {:?}", result);

    let rows_inserted = result.unwrap();
    assert_eq!(rows_inserted, 6, "Expected 6 rows for large values test"); // 1 metric * 2 pools * 3 players

    // Verify percentiles are correctly computed
    let mut stmt = conn
        .prepare(
            "SELECT uid, percent_rank FROM player_percentiles
             WHERE season_id = 1 AND metric_name = 'xg_op' AND pool_type = 'position' AND canonical_position = 'ST'
             ORDER BY (SELECT xg_op FROM season_players sp WHERE sp.uid = player_percentiles.uid)",
        )
        .unwrap();

    let rows: Vec<(String, f64)> = stmt
        .query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0], ("p1".to_string(), 0.0));
    assert_eq!(rows[1], ("p2".to_string(), 0.5));
    assert_eq!(rows[2], ("p3".to_string(), 1.0));
}

/// Test: zero values are not treated as NULL.
#[test]
fn test_percentile_with_zero_values() {
    let mut conn = setup_db();

    // Insert players with zero and non-zero xg_op
    insert_player_with_real_metric(&conn, "p1", "Zero Player 1", "ENG", "ClubA", "ST", 25, Some(0.0));
    insert_player_with_real_metric(&conn, "p2", "Zero Player 2", "ENG", "ClubA", "ST", 25, Some(1.0));
    insert_player_with_real_metric(&conn, "p3", "Zero Player 3", "ENG", "ClubA", "ST", 25, Some(2.0));

    let result = compute_percentiles(&mut conn, 1);
    assert!(result.is_ok());

    let rows_inserted = result.unwrap();
    // 1 metric * 2 pools * 3 players = 6 rows expected (zero IS included)
    assert_eq!(rows_inserted, 6, "Zero values should be included in percentiles");

    // Verify zero is not treated as NULL
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM player_percentiles WHERE season_id = 1 AND metric_name = 'xg_op' AND pool_type = 'position'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count, 3, "Zero should produce 3 percentile entries, got {}", count);

    // Verify the zero player has a valid percentile rank
    let rank: f64 = conn
        .query_row(
            "SELECT percent_rank FROM player_percentiles WHERE uid = 'p1' AND metric_name = 'xg_op' AND pool_type = 'position'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(rank, 0.0, "Zero value should have rank 0.0");
}

/// Test: valid computation followed by query returns consistent data.
#[test]
fn test_percentile_consistency_after_compute() {
    let mut conn = setup_db();

    insert_player_with_real_metric(&conn, "p1", "Player 1", "ENG", "ClubA", "ST", 25, Some(1.0));
    insert_player_with_real_metric(&conn, "p2", "Player 2", "ENG", "ClubA", "ST", 25, Some(2.0));
    insert_player_with_real_metric(&conn, "p3", "Player 3", "ENG", "ClubA", "ST", 25, Some(3.0));

    let result1 = compute_percentiles(&mut conn, 1);
    assert!(result1.is_ok());
    let rows1 = result1.unwrap();

    // Query the data back - use block to drop stmt borrow before next compute_percentiles
    let rows_before: Vec<(String, f64)> = {
        let mut stmt = conn
            .prepare(
                "SELECT uid, percent_rank FROM player_percentiles
                 WHERE season_id = 1 AND metric_name = 'xg_op' AND pool_type = 'position' AND canonical_position = 'ST'
                 ORDER BY percent_rank",
            )
            .unwrap();
        stmt.query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    };

    // Re-run compute_percentiles
    let result2 = compute_percentiles(&mut conn, 1);
    assert!(result2.is_ok());
    let rows2 = result2.unwrap();

    assert_eq!(rows1, rows2, "Re-computation should produce same row count");

    // Query again - use block to drop stmt borrow
    let rows_after: Vec<(String, f64)> = {
        let mut stmt = conn
            .prepare(
                "SELECT uid, percent_rank FROM player_percentiles
                 WHERE season_id = 1 AND metric_name = 'xg_op' AND pool_type = 'position' AND canonical_position = 'ST'
                 ORDER BY percent_rank",
            )
            .unwrap();
        stmt.query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    };

    assert_eq!(
        rows_before, rows_after,
        "Percentile data should be consistent after re-computation"
    );
}

/// Test: UNIQUE constraint prevents duplicate uid-position combinations.
/// season_player_positions has PRIMARY KEY(season_id, uid, canonical_position).
#[test]
fn test_percentile_with_duplicate_uid_positions() {
    let conn = setup_db();

    // Insert a player
    conn.execute(
        "INSERT INTO players (uid, name, nation) VALUES ('player1', 'Player 1', 'ENG')",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO season_players (season_id, uid, club, position_raw, age) VALUES (1, 'player1', 'ClubA', 'DC', 25)",
        [],
    )
    .unwrap();

    // Insert first position
    conn.execute(
        "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (1, 'player1', 'DC')",
        [],
    )
    .unwrap();

    // Try to insert the same position again — should fail due to UNIQUE constraint
    let result = conn.execute(
        "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (1, 'player1', 'DC')",
        [],
    );

    assert!(
        result.is_err(),
        "Duplicate uid-position should fail due to PRIMARY KEY constraint"
    );

    // Verify only one position entry exists
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM season_player_positions WHERE season_id = 1 AND uid = 'player1'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count, 1, "Expected exactly 1 position entry, got {}", count);
}
