use fm_valuescout_lib::db;
use fm_valuescout_lib::scoring;

/// Shared setup for in-memory DB with one DC player at 1500 minutes.
fn setup_base_db() -> (rusqlite::Connection, i64) {
    let mut conn = rusqlite::Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::migrate_db(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();

    conn.execute(
        "INSERT INTO seasons (name, in_game_date, filename, player_count) VALUES ('AdvTest', '1.1.2029', 'adv.csv', 0)",
        [],
    )
    .unwrap();
    let season_id: i64 = conn.last_insert_rowid();

    conn.execute(
        "INSERT INTO players (uid, name, nation) VALUES ('base', 'Base Player', 'England')",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_players (season_id, uid, club, position_raw, age, minutes,
         cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, transfer_value)
         VALUES (?1, 'base', 'Club', 'DC', 25, 1500, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 1000000.0)",
        rusqlite::params![season_id],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (?1, 'base', 'DC')",
        rusqlite::params![season_id],
    )
    .unwrap();

    (conn, season_id)
}

/// Test: transfer_value = 0 should still produce archetype scores.
/// FR-042: cheapest players get VALUE_MULTIPLIER_CHEAPEST (1.5).
/// Value percentile falls back to 0.0 → multiplier 1.5.
#[test]
fn test_zero_transfer_value() {
    let (mut conn, season_id) = setup_base_db();

    conn.execute(
        "INSERT INTO players (uid, name, nation) VALUES ('zero_tv', 'Zero TV', 'England')",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_players (season_id, uid, club, position_raw, age, minutes,
         cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, transfer_value)
         VALUES (?1, 'zero_tv', 'Club', 'DC', 25, 1500, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 0.0)",
        rusqlite::params![season_id],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (?1, 'zero_tv', 'DC')",
        rusqlite::params![season_id],
    )
    .unwrap();

    scoring::compute_percentiles(&mut conn, season_id).unwrap();
    scoring::compute_archetype_scores(&mut conn, season_id).unwrap();

    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM archetype_scores WHERE season_id = ?1 AND uid = 'zero_tv'",
            rusqlite::params![season_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 3, "zero_tv should have 3 archetype scores despite transfer_value=0");

    let row = conn
        .query_row::<(f64, f64), _, _>(
            "SELECT quality_score, value_score FROM archetype_scores
             WHERE season_id = ?1 AND uid = 'zero_tv' LIMIT 1",
            rusqlite::params![season_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    let (quality_score, value_score) = row;
    let expected_value_score = quality_score * 1.5;
    assert!(
        (value_score - expected_value_score).abs() < 0.001,
        "value_score ({}) should equal quality_score * 1.5 ({})",
        value_score,
        expected_value_score
    );
}

/// Test: transfer_value = NULL should fall back to 0.0 percentile (cheapest multiplier 1.5).
#[test]
fn test_null_transfer_value() {
    let (mut conn, season_id) = setup_base_db();

    conn.execute(
        "INSERT INTO players (uid, name, nation) VALUES ('null_tv', 'Null TV', 'England')",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_players (season_id, uid, club, position_raw, age, minutes,
         cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, transfer_value)
         VALUES (?1, 'null_tv', 'Club', 'DC', 25, 1500, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, NULL)",
        rusqlite::params![season_id],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (?1, 'null_tv', 'DC')",
        rusqlite::params![season_id],
    )
    .unwrap();

    scoring::compute_percentiles(&mut conn, season_id).unwrap();
    scoring::compute_archetype_scores(&mut conn, season_id).unwrap();

    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM archetype_scores WHERE season_id = ?1 AND uid = 'null_tv'",
            rusqlite::params![season_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 3, "null_tv should have 3 archetype scores despite NULL transfer_value");

    let row = conn
        .query_row::<(f64, f64), _, _>(
            "SELECT quality_score, value_score FROM archetype_scores
             WHERE season_id = ?1 AND uid = 'null_tv' LIMIT 1",
            rusqlite::params![season_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    let (quality_score, value_score) = row;
    let expected_value_score = quality_score * 1.5;
    assert!(
        (value_score - expected_value_score).abs() < 0.001,
        "NULL transfer_value should use cheapest multiplier 1.5: got {} vs {}",
        value_score,
        expected_value_score
    );
}

/// Test: Player in season_players but NOT in season_player_positions
/// should not receive any archetype scores (JOIN excludes them).
#[test]
fn test_player_with_no_positions() {
    let (mut conn, season_id) = setup_base_db();

    conn.execute(
        "INSERT INTO players (uid, name, nation) VALUES ('no_pos', 'No Position', 'England')",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_players (season_id, uid, club, position_raw, age, minutes,
         cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, transfer_value)
         VALUES (?1, 'no_pos', 'Club', 'DC', 25, 1500, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 5000000.0)",
        rusqlite::params![season_id],
    )
    .unwrap();
    // NOTE: intentionally NOT inserting into season_player_positions.

    scoring::compute_percentiles(&mut conn, season_id).unwrap();
    scoring::compute_archetype_scores(&mut conn, season_id).unwrap();

    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM archetype_scores WHERE season_id = ?1 AND uid = 'no_pos'",
            rusqlite::params![season_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 0, "player with no position row should have 0 archetype scores");

    let base_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM archetype_scores WHERE season_id = ?1 AND uid = 'base'",
            rusqlite::params![season_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(base_count, 3, "base player should still have 3 archetype scores");
}

/// Test: Negative transfer_value should not crash.
#[test]
fn test_negative_transfer_value() {
    let (mut conn, season_id) = setup_base_db();

    conn.execute(
        "INSERT INTO players (uid, name, nation) VALUES ('neg_tv', 'Neg TV', 'England')",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_players (season_id, uid, club, position_raw, age, minutes,
         cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, transfer_value)
         VALUES (?1, 'neg_tv', 'Club', 'DC', 25, 1500, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, -1000000.0)",
        rusqlite::params![season_id],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (?1, 'neg_tv', 'DC')",
        rusqlite::params![season_id],
    )
    .unwrap();

    scoring::compute_percentiles(&mut conn, season_id).unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        scoring::compute_archetype_scores(&mut conn, season_id)
    }));
    assert!(result.is_ok(), "compute_archetype_scores should not panic with negative transfer_value");

    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM archetype_scores WHERE season_id = ?1 AND uid = 'neg_tv'",
            rusqlite::params![season_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 3, "neg_tv should have 3 archetype scores");
}

/// Test: Player with exactly minutes=1000 should be INCLUDED.
#[test]
fn test_exactly_1000_minutes() {
    let (mut conn, season_id) = setup_base_db();

    conn.execute(
        "INSERT INTO players (uid, name, nation) VALUES ('exact_1000', 'Exact 1000', 'England')",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_players (season_id, uid, club, position_raw, age, minutes,
         cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, transfer_value)
         VALUES (?1, 'exact_1000', 'Club', 'DC', 25, 1000, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 1000000.0)",
        rusqlite::params![season_id],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (?1, 'exact_1000', 'DC')",
        rusqlite::params![season_id],
    )
    .unwrap();

    scoring::compute_percentiles(&mut conn, season_id).unwrap();
    scoring::compute_archetype_scores(&mut conn, season_id).unwrap();

    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM archetype_scores WHERE season_id = ?1 AND uid = 'exact_1000'",
            rusqlite::params![season_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 3, "player with exactly 1000 minutes should be included");
}

/// Test: Player with minutes=999 should be EXCLUDED. FR-031.
#[test]
fn test_999_minutes() {
    let (mut conn, season_id) = setup_base_db();

    conn.execute(
        "INSERT INTO players (uid, name, nation) VALUES ('min_999', 'Min 999', 'England')",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_players (season_id, uid, club, position_raw, age, minutes,
         cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, transfer_value)
         VALUES (?1, 'min_999', 'Club', 'DC', 25, 999, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 1000000.0)",
        rusqlite::params![season_id],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (?1, 'min_999', 'DC')",
        rusqlite::params![season_id],
    )
    .unwrap();

    scoring::compute_percentiles(&mut conn, season_id).unwrap();
    scoring::compute_archetype_scores(&mut conn, season_id).unwrap();

    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM archetype_scores WHERE season_id = ?1 AND uid = 'min_999'",
            rusqlite::params![season_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 0, "player with 999 minutes should be excluded (FR-031)");
}
