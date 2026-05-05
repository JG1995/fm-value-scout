use fm_valuescout_lib::db;
use fm_valuescout_lib::scoring;

fn setup_full_db() -> rusqlite::Connection {
    let mut conn = rusqlite::Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::migrate_db(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();

    conn.execute(
        "INSERT INTO seasons (name, in_game_date, filename, player_count) VALUES ('Test', '1.1.2029', 'test.csv', 0)",
        [],
    ).unwrap();
    let season_id: i64 = conn.last_insert_rowid();

    let player_data = [
        ("player1", 11, 60, 55, 50, 45, 70, 65, 60, 55, 50, 1000000.0),
        ("player2", 12, 65, 60, 55, 50, 75, 70, 65, 60, 55, 2000000.0),
        ("player3", 13, 70, 65, 60, 55, 80, 75, 70, 65, 60, 3000000.0),
    ];

    for (uid, cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, tv) in player_data {
        conn.execute(
            "INSERT INTO players (uid, name, nation) VALUES (?1, ?2, ?3)",
            rusqlite::params![uid, format!("Player {}", uid), "England"],
        ).unwrap();
        conn.execute(
            "INSERT INTO season_players (season_id, uid, club, position_raw, age, minutes,
             cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, transfer_value)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
            rusqlite::params![
                season_id, uid, "Club", "DC", 25, 1500, cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, tv
            ],
        ).unwrap();
        conn.execute(
            "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (?1, ?2, ?3)",
            rusqlite::params![season_id, uid, "DC"],
        ).unwrap();
    }

    scoring::compute_percentiles(&mut conn, season_id).unwrap();
    conn
}

fn insert_empty_season(conn: &rusqlite::Connection) -> i64 {
    conn.execute(
        "INSERT INTO seasons (name, in_game_date, filename, player_count) VALUES ('Empty', '1.1.2030', 'empty.csv', 0)",
        [],
    ).unwrap();
    conn.last_insert_rowid()
}

#[test]
fn test_compute_archetype_scores_basic() {
    let mut conn = setup_full_db();
    let season_id: i64 = conn
        .query_row("SELECT id FROM seasons WHERE name = 'Test'", [], |r| r.get(0))
        .unwrap();

    let row_count = scoring::compute_archetype_scores(&mut conn, season_id).unwrap();
    assert_eq!(row_count, 9, "Expected 9 archetype score rows, got {row_count}");

    let mut stmt = conn
        .prepare("SELECT quality_score FROM archetype_scores WHERE season_id = ?1")
        .unwrap();
    let scores: Vec<f64> = stmt
        .query_map(rusqlite::params![season_id], |r| r.get(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    assert_eq!(scores.len(), 9);
    for score in &scores {
        assert!((*score >= 0.0) && (*score <= 100.0), "quality_score {score} out of range [0, 100]");
    }

    let mut arch_stmt = conn
        .prepare("SELECT DISTINCT a.name FROM archetype_scores ast JOIN archetypes a ON ast.archetype_id = a.id WHERE ast.season_id = ?1")
        .unwrap();
    let arch_names: Vec<String> = arch_stmt
        .query_map(rusqlite::params![season_id], |r| r.get(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    assert_eq!(arch_names.len(), 3);
    assert!(arch_names.contains(&"Ball-Playing Defender".to_string()));
    assert!(arch_names.contains(&"No-Nonsense Centre Back".to_string()));
    assert!(arch_names.contains(&"Complete Centre Back".to_string()));
}

#[test]
fn test_compute_archetype_scores_minutes_filter() {
    let mut conn = rusqlite::Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::migrate_db(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();

    conn.execute(
        "INSERT INTO seasons (name, in_game_date, filename, player_count) VALUES ('Test2', '1.1.2029', 'test2.csv', 0)",
        [],
    ).unwrap();
    let season_id: i64 = conn.last_insert_rowid();

    conn.execute(
        "INSERT INTO players (uid, name, nation) VALUES ('low_min', 'Low Min Player', 'England')",
        [],
    ).unwrap();
    conn.execute(
        "INSERT INTO season_players (season_id, uid, club, position_raw, age, minutes,
         cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, transfer_value)
         VALUES (?1, 'low_min', 'Club', 'DC', 25, 500, 70, 65, 60, 55, 50, 75, 70, 65, 60, 55, 1000000.0)",
        rusqlite::params![season_id],
    ).unwrap();
    conn.execute(
        "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (?1, 'low_min', 'DC')",
        rusqlite::params![season_id],
    ).unwrap();

    conn.execute(
        "INSERT INTO players (uid, name, nation) VALUES ('high_min', 'High Min Player', 'England')",
        [],
    ).unwrap();
    conn.execute(
        "INSERT INTO season_players (season_id, uid, club, position_raw, age, minutes,
         cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, transfer_value)
         VALUES (?1, 'high_min', 'Club', 'DC', 25, 1500, 80, 70, 65, 60, 55, 80, 75, 70, 65, 60, 2000000.0)",
        rusqlite::params![season_id],
    ).unwrap();
    conn.execute(
        "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (?1, 'high_min', 'DC')",
        rusqlite::params![season_id],
    ).unwrap();

    scoring::compute_percentiles(&mut conn, season_id).unwrap();
    let row_count = scoring::compute_archetype_scores(&mut conn, season_id).unwrap();
    assert_eq!(row_count, 3, "Expected 3 rows, got {row_count}");

    let low_min_scores: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM archetype_scores WHERE season_id = ?1 AND uid = 'low_min'",
            rusqlite::params![season_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(low_min_scores, 0, "low_min should have no scores");

    let high_min_scores: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM archetype_scores WHERE season_id = ?1 AND uid = 'high_min'",
            rusqlite::params![season_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(high_min_scores, 3, "high_min should have 3 scores");
}

#[test]
fn test_compute_archetype_scores_value_score() {
    let mut conn = rusqlite::Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::migrate_db(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();

    conn.execute(
        "INSERT INTO seasons (name, in_game_date, filename, player_count) VALUES ('Test3', '1.1.2029', 'test3.csv', 0)",
        [],
    ).unwrap();
    let season_id: i64 = conn.last_insert_rowid();

    // player_a: HIGH quality (90), cheap (500K) -> quality=90, multiplier=1.5 -> value_score = 135
    // player_c: LOW quality (10), expensive (10M) -> quality=10, multiplier=0.5 -> value_score = 5
    // player_a should have a strictly higher value_score than player_c.
    let player_data = [
        ("player_a", 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 500000.0),
        ("player_b", 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 5000000.0),
        ("player_c", 10, 20, 20, 20, 20, 20, 20, 20, 20, 20, 10000000.0),
    ];

    for (uid, cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, tv) in player_data {
        conn.execute(
            "INSERT INTO players (uid, name, nation) VALUES (?1, ?2, ?3)",
            rusqlite::params![uid, uid, "England"],
        ).unwrap();
        conn.execute(
            "INSERT INTO season_players (season_id, uid, club, position_raw, age, minutes,
             cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, transfer_value)
             VALUES (?1, ?2, 'Club', 'DC', 25, 1500, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            rusqlite::params![season_id, uid, cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, tv],
        ).unwrap();
        conn.execute(
            "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (?1, ?2, 'DC')",
            rusqlite::params![season_id, uid],
        ).unwrap();
    }

    scoring::compute_percentiles(&mut conn, season_id).unwrap();
    scoring::compute_archetype_scores(&mut conn, season_id).unwrap();

    let value_a: f64 = conn
        .query_row(
            "SELECT ast.value_score FROM archetype_scores ast
             JOIN archetypes a ON ast.archetype_id = a.id
             WHERE ast.season_id = ?1 AND ast.uid = 'player_a' AND a.base_position = 'DC'
             LIMIT 1",
            rusqlite::params![season_id],
            |r| r.get(0),
        )
        .unwrap();

    let value_c: f64 = conn
        .query_row(
            "SELECT ast.value_score FROM archetype_scores ast
             JOIN archetypes a ON ast.archetype_id = a.id
             WHERE ast.season_id = ?1 AND ast.uid = 'player_c' AND a.base_position = 'DC'
             LIMIT 1",
            rusqlite::params![season_id],
            |r| r.get(0),
        )
        .unwrap();

    assert!(
        value_a > value_c,
        "player_a ({}) should have higher value_score than player_c ({})",
        value_a, value_c
    );
}

#[test]
fn test_compute_archetype_scores_empty_season() {
    let mut conn = setup_full_db();
    let empty_season_id = insert_empty_season(&conn);
    let row_count = scoring::compute_archetype_scores(&mut conn, empty_season_id).unwrap();
    assert_eq!(row_count, 0, "Expected 0 rows for empty season");
}

#[test]
fn test_compute_archetype_scores_idempotent() {
    let mut conn = setup_full_db();
    let season_id: i64 = conn
        .query_row("SELECT id FROM seasons WHERE name = 'Test'", [], |r| r.get(0))
        .unwrap();

    let first_count = scoring::compute_archetype_scores(&mut conn, season_id).unwrap();
    assert_eq!(first_count, 9);

    let second_count = scoring::compute_archetype_scores(&mut conn, season_id).unwrap();
    assert_eq!(second_count, first_count);

    let first_scores: Vec<(String, i64, f64, f64)> = {
        let mut stmt = conn
            .prepare("SELECT uid, archetype_id, quality_score, value_score FROM archetype_scores WHERE season_id = ?1 ORDER BY uid, archetype_id")
            .unwrap();
        stmt.query_map(rusqlite::params![season_id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect()
    };

    let second_scores: Vec<(String, i64, f64, f64)> = {
        let mut stmt = conn
            .prepare("SELECT uid, archetype_id, quality_score, value_score FROM archetype_scores WHERE season_id = ?1 ORDER BY uid, archetype_id")
            .unwrap();
        stmt.query_map(rusqlite::params![season_id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect()
    };

    assert_eq!(first_scores, second_scores);
}
