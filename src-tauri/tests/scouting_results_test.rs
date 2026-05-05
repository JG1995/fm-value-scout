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
    )
    .unwrap();
    let season_id: i64 = conn.last_insert_rowid();

    let player_data = [
        ("player1", "ClubA", 11, 60, 55, 50, 45, 70, 65, 60, 55, 50, 1000000.0),
        ("player2", "ClubB", 12, 65, 60, 55, 50, 75, 70, 65, 60, 55, 2000000.0),
        ("player3", "ClubA", 13, 70, 65, 60, 55, 80, 75, 70, 65, 60, 3000000.0),
    ];

    for (uid, club, cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, tv) in player_data {
        conn.execute(
            "INSERT INTO players (uid, name, nation) VALUES (?1, ?2, ?3)",
            rusqlite::params![uid, format!("Player {}", uid), "England"],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO season_players (season_id, uid, club, position_raw, age, minutes,
             cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, transfer_value, average_rating)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
            rusqlite::params![
                season_id, uid, club, "DC", 25, 1500, cor, pas, fir, cmp, vis, tck, hea, mar, ant, pos, tv, 7.5
            ],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO season_player_positions (season_id, uid, canonical_position) VALUES (?1, ?2, ?3)",
            rusqlite::params![season_id, uid, "DC"],
        )
        .unwrap();
    }

    scoring::compute_percentiles(&mut conn, season_id).unwrap();
    scoring::compute_archetype_scores(&mut conn, season_id).unwrap();
    conn
}

#[test]
fn test_get_scouting_players_basic() {
    let conn = setup_full_db();
    let players = db::get_scouting_players(&conn, "all", None).unwrap();

    assert_eq!(players.len(), 3);
    assert!(players.iter().all(|p| !p.percentiles.is_empty()));
}

#[test]
fn test_get_scouting_players_position_pool() {
    let conn = setup_full_db();
    let players = db::get_scouting_players(&conn, "position", None).unwrap();

    assert_eq!(players.len(), 3);
    for player in &players {
        assert!(!player.canonical_positions.is_empty());
    }
}

#[test]
fn test_get_scouting_players_invalid_pool_type() {
    let conn = setup_full_db();
    let result = db::get_scouting_players(&conn, "invalid", None);
    assert!(result.is_err());
}

#[test]
fn test_get_scouting_players_excludes_managed_club() {
    let conn = setup_full_db();
    let players = db::get_scouting_players(&conn, "all", Some("ClubA")).unwrap();

    assert_eq!(players.len(), 1);
    assert_eq!(players[0].uid, "player2");
}

#[test]
fn test_get_scouting_players_empty_for_unknown_club() {
    let conn = setup_full_db();
    let players = db::get_scouting_players(&conn, "all", Some("NonexistentClub")).unwrap();
    assert_eq!(players.len(), 3);
}

#[test]
fn test_get_role_search_results_basic() {
    let conn = setup_full_db();

    // Get archetype that has computed data (inner join with archetype_scores)
    let archetype_id: i64 = conn
        .query_row(
            "SELECT a.id FROM archetypes a JOIN archetype_scores ac ON a.id = ac.archetype_id LIMIT 1",
            [],
            |r| r.get(0),
        )
        .unwrap();

    let results = db::get_role_search_results(&conn, archetype_id, "quality", None).unwrap();
    assert!(!results.is_empty());

    for player in &results {
        assert!(!player.top_metrics.is_empty());
        assert!(player.quality_score >= 0.0 && player.quality_score <= 100.0);
    }
}

#[test]
fn test_get_role_search_results_value_mode() {
    let conn = setup_full_db();
    // Get archetype that has computed data
    let archetype_id: i64 = conn
        .query_row(
            "SELECT a.id FROM archetypes a JOIN archetype_scores ac ON a.id = ac.archetype_id LIMIT 1",
            [],
            |r| r.get(0),
        )
        .unwrap();

    let results = db::get_role_search_results(&conn, archetype_id, "value", None).unwrap();
    assert!(!results.is_empty());
}

#[test]
fn test_get_role_search_results_invalid_score_mode() {
    let conn = setup_full_db();
    let archetype_id: i64 = conn
        .query_row("SELECT id FROM archetypes LIMIT 1", [], |r| r.get(0))
        .unwrap();

    let result = db::get_role_search_results(&conn, archetype_id, "invalid", None);
    assert!(result.is_err());
}

#[test]
fn test_get_role_search_results_excludes_managed_club() {
    let conn = setup_full_db();
    // Get archetype that has computed data
    let archetype_id: i64 = conn
        .query_row(
            "SELECT a.id FROM archetypes a JOIN archetype_scores ac ON a.id = ac.archetype_id LIMIT 1",
            [],
            |r| r.get(0),
        )
        .unwrap();

    let all_results = db::get_role_search_results(&conn, archetype_id, "quality", None).unwrap();
    let filtered_results = db::get_role_search_results(&conn, archetype_id, "quality", Some("ClubA")).unwrap();

    assert!(filtered_results.len() < all_results.len());
}

#[test]
fn test_get_role_search_results_sorted_by_score() {
    let conn = setup_full_db();
    // Get archetype that has computed data
    let archetype_id: i64 = conn
        .query_row(
            "SELECT a.id FROM archetypes a JOIN archetype_scores ac ON a.id = ac.archetype_id LIMIT 1",
            [],
            |r| r.get(0),
        )
        .unwrap();

    let results = db::get_role_search_results(&conn, archetype_id, "quality", None).unwrap();

    for i in 1..results.len() {
        assert!(
            results[i - 1].quality_score >= results[i].quality_score,
            "Results should be sorted by quality_score descending"
        );
    }
}

#[test]
fn test_run_scouting_pipeline() {
    let mut conn = setup_full_db();
    let (pct_rows, score_rows) = scoring::run_scouting_pipeline(&mut conn).unwrap();

    assert!(pct_rows > 0, "Should compute percentiles");
    assert!(score_rows > 0, "Should compute archetype scores");
}

#[test]
fn test_run_scouting_pipeline_idempotent() {
    let mut conn = setup_full_db();
    let (pct1, score1) = scoring::run_scouting_pipeline(&mut conn).unwrap();
    let (pct2, score2) = scoring::run_scouting_pipeline(&mut conn).unwrap();

    assert_eq!(pct1, pct2);
    assert_eq!(score1, score2);
}