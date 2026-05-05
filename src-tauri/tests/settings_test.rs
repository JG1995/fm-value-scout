use fm_valuescout_lib::db;
use rusqlite::Connection;

#[test]
fn test_set_and_get_setting() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();

    db::set_setting(&conn, "managed_club", "Liverpool").unwrap();
    let result = db::get_setting(&conn, "managed_club").unwrap();
    assert_eq!(result, Some("Liverpool".to_string()));
}

#[test]
fn test_get_nonexistent_key() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();

    let result = db::get_setting(&conn, "nonexistent_key").unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_upsert_updates_value() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();

    db::set_setting(&conn, "theme", "dark").unwrap();
    let first = db::get_setting(&conn, "theme").unwrap();
    assert_eq!(first, Some("dark".to_string()));

    db::set_setting(&conn, "theme", "light").unwrap();
    let second = db::get_setting(&conn, "theme").unwrap();
    assert_eq!(second, Some("light".to_string()));
}

#[test]
fn test_managed_club_workflow() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();

    db::set_setting(&conn, "managed_club", "Manchester United").unwrap();
    let club = db::get_setting(&conn, "managed_club").unwrap();
    assert_eq!(club, Some("Manchester United".to_string()));

    db::set_setting(&conn, "managed_club", "Chelsea").unwrap();
    let updated_club = db::get_setting(&conn, "managed_club").unwrap();
    assert_eq!(updated_club, Some("Chelsea".to_string()));
}

#[test]
fn test_multiple_settings() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();

    db::set_setting(&conn, "key1", "value1").unwrap();
    db::set_setting(&conn, "key2", "value2").unwrap();
    db::set_setting(&conn, "key3", "value3").unwrap();

    assert_eq!(
        db::get_setting(&conn, "key1").unwrap(),
        Some("value1".to_string())
    );
    assert_eq!(
        db::get_setting(&conn, "key2").unwrap(),
        Some("value2".to_string())
    );
    assert_eq!(
        db::get_setting(&conn, "key3").unwrap(),
        Some("value3".to_string())
    );
}

#[test]
fn test_empty_string_value() {
    let conn = Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();

    db::set_setting(&conn, "empty_key", "").unwrap();
    let result = db::get_setting(&conn, "empty_key").unwrap();
    assert_eq!(result, Some("".to_string()));
}
