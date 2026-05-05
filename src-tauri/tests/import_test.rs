//! Integration tests for the CSV import pipeline.

use rusqlite::Connection;
use std::io::Write;

fn make_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    fm_valuescout_lib::db::create_tables(&conn).unwrap();
    conn
}

fn write_test_csv(content: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir();
    // Use a unique filename based on content hash to avoid cross-test interference
    let hash = content
        .bytes()
        .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
    let path = dir.join(format!("fm_import_test_{}.csv", hash));
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(content.as_bytes()).unwrap();
    path
}

#[test]
fn test_import_single_player_row() {
    let mut conn = make_test_db();
    let csv = "UID;Name;Nation;Age;Club;Position\n1001;Test Player;ENG;25;FC Test;ST\n";
    let path = write_test_csv(csv);
    let result =
        fm_valuescout_lib::import::run_import(&mut conn, path.to_str().unwrap(), "15.6.2029")
            .unwrap();
    assert_eq!(result.players_imported, 1);
    assert!(result.rows_rejected.is_empty());
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM seasons", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1);
    let player_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM players", [], |r| r.get(0))
        .unwrap();
    assert_eq!(player_count, 1);
    let season_name: String = conn
        .query_row("SELECT name FROM seasons", [], |r| r.get(0))
        .unwrap();
    assert_eq!(season_name, "15.6.2029 Season");
}

#[test]
fn test_import_multiple_player_rows() {
    let mut conn = make_test_db();
    let csv = "UID;Name;Nation;Age;Club;Position\n1001;Player One;ENG;22;Alpha Utd;DM\n1002;Player Two;FRA;28;Beta City;AMC\n1003;Player Three;GER;31;Gamma Rovers;GK\n";
    let path = write_test_csv(csv);
    let result =
        fm_valuescout_lib::import::run_import(&mut conn, path.to_str().unwrap(), "1.1.2030")
            .unwrap();
    assert_eq!(result.players_imported, 3);
    assert!(result.rows_rejected.is_empty());
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM seasons", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1);
    let player_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM players", [], |r| r.get(0))
        .unwrap();
    assert_eq!(player_count, 3);
    let spp_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM season_player_positions", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert!(spp_count > 0);
}

#[test]
fn test_import_rejected_row_missing_uid() {
    let mut conn = make_test_db();
    let csv = "UID;Name;Nation;Age;Club;Position\n1001;Valid Player;ENG;25;Alpha;ST\n;Missing UID;ENG;23;Beta;MC\n";
    let path = write_test_csv(csv);
    let result =
        fm_valuescout_lib::import::run_import(&mut conn, path.to_str().unwrap(), "15.6.2029")
            .unwrap();
    assert_eq!(result.players_imported, 1);
    assert_eq!(result.rows_rejected.len(), 1);
    assert_eq!(result.rows_rejected[0].row_number, 3);
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM seasons", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1);
}

#[test]
fn test_import_rejected_row_missing_name() {
    let mut conn = make_test_db();
    let csv = "UID;Name;Nation;Age;Club;Position\n1001;Valid Player;ENG;25;Alpha;ST\n1002;;ENG;22;Beta;MC\n";
    let path = write_test_csv(csv);
    let result =
        fm_valuescout_lib::import::run_import(&mut conn, path.to_str().unwrap(), "15.6.2029")
            .unwrap();
    assert_eq!(result.players_imported, 1);
    assert_eq!(result.rows_rejected.len(), 1);
    assert_eq!(result.rows_rejected[0].row_number, 3);
}

#[test]
fn test_import_rejected_row_duplicate_uid() {
    let mut conn = make_test_db();
    let csv = "UID;Name;Nation;Age;Club;Position\n1001;First Player;ENG;25;Alpha;ST\n1001;Duplicate Player;FRA;28;Beta;MC\n";
    let path = write_test_csv(csv);
    let result =
        fm_valuescout_lib::import::run_import(&mut conn, path.to_str().unwrap(), "15.6.2029")
            .unwrap();
    assert_eq!(result.players_imported, 1);
    assert_eq!(result.rows_rejected.len(), 1);
    assert!(result.rows_rejected[0]
        .reasons
        .iter()
        .any(|r| r.contains("Duplicate")));
}

#[test]
fn test_import_player_count_updated_in_season() {
    let mut conn = make_test_db();
    let csv = "UID;Name;Nation;Age;Club;Position\n1001;Player One;ENG;22;Alpha Utd;DM\n1002;Player Two;FRA;28;Beta City;AMC\n";
    let path = write_test_csv(csv);
    let result =
        fm_valuescout_lib::import::run_import(&mut conn, path.to_str().unwrap(), "15.6.2029")
            .unwrap();
    assert_eq!(result.players_imported, 2);
    let stored_count: i64 = conn
        .query_row(
            "SELECT player_count FROM seasons WHERE id = ?1",
            [result.season_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stored_count, 2);
}

#[test]
fn test_import_produces_correct_season_name() {
    let mut conn = make_test_db();
    let csv = "UID;Name;Nation;Age;Club;Position\n1001;Test;ENG;25;Test Club;ST\n";
    let path = write_test_csv(csv);
    let result =
        fm_valuescout_lib::import::run_import(&mut conn, path.to_str().unwrap(), "30.11.2031")
            .unwrap();
    assert_eq!(result.season_name, "30.11.2031 Season");
}

#[test]
fn test_import_compound_position_expands() {
    let mut conn = make_test_db();
    let csv = "UID;Name;Nation;Age;Club;Position\n1001;Multi Pos Player;ESP;24;Test Utd;DM,AMC\n";
    let path = write_test_csv(csv);
    let result =
        fm_valuescout_lib::import::run_import(&mut conn, path.to_str().unwrap(), "15.6.2029")
            .unwrap();
    assert_eq!(result.players_imported, 1);
    assert!(result.rows_rejected.is_empty());
    let pos_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM season_player_positions WHERE uid = '1001' AND season_id = ?1",
            [result.season_id],
            |r| r.get(0),
        )
        .unwrap();
    assert!(
        pos_count >= 2,
        "expected at least 2 positions, got {}",
        pos_count
    );
}

#[test]
fn test_import_non_ascii_characters() {
    let mut conn = make_test_db();
    let csv = "UID;Name;Nation;Age;Club;Position\n1001;José García;ESP;30;Club Ñ Sofía;ST\n";
    let path = write_test_csv(csv);
    let result =
        fm_valuescout_lib::import::run_import(&mut conn, path.to_str().unwrap(), "15.6.2029")
            .unwrap();
    assert_eq!(result.players_imported, 1);
    assert!(result.rows_rejected.is_empty());
    let stored_name: String = conn
        .query_row("SELECT name FROM players WHERE uid = '1001'", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(stored_name, "José García");
}
