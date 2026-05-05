use rusqlite::params;
use fm_valuescout_lib::scoring::compute_percentiles;

fn setup_db() -> rusqlite::Connection {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    fm_valuescout_lib::db::create_tables(&conn).unwrap();
    conn.execute(
        "INSERT INTO seasons (name, in_game_date, filename, player_count) VALUES ('Test', '1.1.2029', 'test.csv', 0)",
        [],
    )
    .unwrap();
    conn
}

fn insert_player(conn: &rusqlite::Connection, uid: &str, name: &str, nation: &str, club: &str, position_raw: &str, age: i64, cor: Option<i64>) {
    conn.execute(
        "INSERT INTO players (uid, name, nation) VALUES (?1, ?2, ?3)",
        params![uid, name, nation],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO season_players (season_id, uid, club, position_raw, age, cor) VALUES (1, ?1, ?2, ?3, ?4, ?5)",
        params![uid, club, position_raw, age, cor],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (1, ?1, ?2)",
        params![uid, position_raw],
    )
    .unwrap();
}

/// Test: basic percentile computation for 5 DC players with cor values 1-5.
/// Verifies position pool has 5 entries with percentiles 0.0, 0.25, 0.5, 0.75, 1.0
/// and all pool has same.
#[test]
fn test_compute_percentiles_basic() {
    let mut conn = setup_db();

    // Insert 5 DC players with cor values 1, 2, 3, 4, 5
    for (i, cor_val) in [1, 2, 3, 4, 5].iter().enumerate() {
        let uid = format!("player{}", i + 1);
        insert_player(&conn, &uid, &format!("Player {}", i + 1), "ENG", "ClubA", "DC", 25, Some(*cor_val));
    }

    let result = compute_percentiles(&mut conn, 1);
    assert!(result.is_ok());
    let total_rows = result.unwrap();
    // 1 metric (cor) * 2 pools (position + all) * 5 players = 10 rows
    assert_eq!(total_rows, 10);

    // Verify position pool percentiles for DC
    let mut stmt = conn
        .prepare("SELECT uid, percent_rank FROM player_percentiles WHERE season_id = 1 AND metric_name = 'cor' AND pool_type = 'position' AND canonical_position = 'DC' ORDER BY percent_rank")
        .unwrap();
    let rows: Vec<(String, f64)> = stmt
        .query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    assert_eq!(rows.len(), 5);
    assert_eq!(rows[0].1, 0.0);  // cor=1 → rank=0.0
    assert_eq!(rows[1].1, 0.25); // cor=2 → rank=0.25
    assert_eq!(rows[2].1, 0.5);  // cor=3 → rank=0.5
    assert_eq!(rows[3].1, 0.75); // cor=4 → rank=0.75
    assert_eq!(rows[4].1, 1.0);  // cor=5 → rank=1.0

    // Verify all pool percentiles
    let mut stmt = conn
        .prepare("SELECT uid, percent_rank FROM player_percentiles WHERE season_id = 1 AND metric_name = 'cor' AND pool_type = 'all' ORDER BY percent_rank")
        .unwrap();
    let rows: Vec<(String, f64)> = stmt
        .query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    assert_eq!(rows.len(), 5);
    assert_eq!(rows[0].1, 0.0);
    assert_eq!(rows[1].1, 0.25);
    assert_eq!(rows[2].1, 0.5);
    assert_eq!(rows[3].1, 0.75);
    assert_eq!(rows[4].1, 1.0);
}

/// Test: NULL cor values are excluded from percentile computation.
#[test]
fn test_compute_percentiles_null_handling() {
    let mut conn = setup_db();

    // Insert 3 players: cor=1, cor=NULL, cor=3
    insert_player(&conn, "player1", "Player 1", "ENG", "ClubA", "DC", 25, Some(1));
    insert_player(&conn, "player2", "Player 2", "ENG", "ClubA", "DC", 25, None);
    insert_player(&conn, "player3", "Player 3", "ENG", "ClubA", "DC", 25, Some(3));

    let result = compute_percentiles(&mut conn, 1);
    assert!(result.is_ok());

    // Only 2 players have non-NULL cor, so 2 entries per pool
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM player_percentiles WHERE season_id = 1 AND metric_name = 'cor' AND pool_type = 'position' AND canonical_position = 'DC'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count, 2);

    let count_all: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM player_percentiles WHERE season_id = 1 AND metric_name = 'cor' AND pool_type = 'all'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count_all, 2);
}

/// Test: a player with 2 positions gets entries in both position pools.
#[test]
fn test_compute_percentiles_multiple_positions() {
    let mut conn = setup_db();

    // Insert player with 2 positions
    conn.execute(
        "INSERT INTO players (uid, name, nation) VALUES ('player1', 'Player 1', 'ENG')",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_players (season_id, uid, club, position_raw, age, cor) VALUES (1, 'player1', 'ClubA', 'DC/DM', 25, 3)",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (1, 'player1', 'DC')",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (1, 'player1', 'DM')",
        [],
    )
    .unwrap();

    let result = compute_percentiles(&mut conn, 1);
    assert!(result.is_ok());

    // Verify both DC and DM position entries exist
    let dc_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM player_percentiles WHERE season_id = 1 AND metric_name = 'cor' AND pool_type = 'position' AND canonical_position = 'DC'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(dc_count, 1);

    let dm_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM player_percentiles WHERE season_id = 1 AND metric_name = 'cor' AND pool_type = 'position' AND canonical_position = 'DM'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(dm_count, 1);
}

/// Test: empty season returns Ok(0).
#[test]
fn test_compute_percentiles_empty_season() {
    let mut conn = setup_db();

    // Create a second empty season
    conn.execute(
        "INSERT INTO seasons (name, in_game_date, filename, player_count) VALUES ('Empty', '1.1.2029', 'empty.csv', 0)",
        [],
    )
    .unwrap();

    let result = compute_percentiles(&mut conn, 2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

/// Test: running twice produces same results and no error.
#[test]
fn test_compute_percentiles_recompute() {
    let mut conn = setup_db();

    for (i, cor_val) in [1, 2, 3].iter().enumerate() {
        let uid = format!("player{}", i + 1);
        insert_player(&conn, &uid, &format!("Player {}", i + 1), "ENG", "ClubA", "DC", 25, Some(*cor_val));
    }

    let result1 = compute_percentiles(&mut conn, 1);
    assert!(result1.is_ok());
    let rows1 = result1.unwrap();

    let result2 = compute_percentiles(&mut conn, 1);
    assert!(result2.is_ok());
    let rows2 = result2.unwrap();

    assert_eq!(rows1, rows2);

    // Verify values are identical after recompute
    let rows: Vec<(String, f64)> = {
        let mut stmt = conn
            .prepare("SELECT uid, percent_rank FROM player_percentiles WHERE season_id = 1 AND metric_name = 'cor' AND pool_type = 'position' AND canonical_position = 'DC' ORDER BY uid")
            .unwrap();
        stmt.query_map([], |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    };
    assert_eq!(rows.len(), 3);
}

/// Test: single player always gets PERCENT_RANK = 0.0.
#[test]
fn test_compute_percentiles_single_player() {
    let mut conn = setup_db();

    insert_player(&conn, "player1", "Player 1", "ENG", "ClubA", "DC", 25, Some(5));

    let result = compute_percentiles(&mut conn, 1);
    assert!(result.is_ok());

    let rank: f64 = conn
        .query_row(
            "SELECT percent_rank FROM player_percentiles WHERE season_id = 1 AND metric_name = 'cor' AND pool_type = 'position' AND canonical_position = 'DC'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(rank, 0.0);

    let rank_all: f64 = conn
        .query_row(
            "SELECT percent_rank FROM player_percentiles WHERE season_id = 1 AND metric_name = 'cor' AND pool_type = 'all'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(rank_all, 0.0);
}
