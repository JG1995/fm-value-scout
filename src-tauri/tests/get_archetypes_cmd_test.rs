use fm_valuescout_lib::db;

#[test]
fn test_get_archetypes_all() {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::migrate_db(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();

    let archetypes = db::get_archetypes(&conn, None).unwrap();
    assert_eq!(archetypes.len(), 20);
}

#[test]
fn test_get_archetypes_filtered_dc() {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::migrate_db(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();

    let dc_archetypes = db::get_archetypes(&conn, Some("DC")).unwrap();
    assert_eq!(dc_archetypes.len(), 3);
    for a in &dc_archetypes {
        assert_eq!(a.base_position, "DC");
    }
}

#[test]
fn test_get_archetypes_filtered_gk() {
    let conn = rusqlite::Connection::open_in_memory().unwrap();
    db::create_tables(&conn).unwrap();
    db::migrate_db(&conn).unwrap();
    db::seed_archetypes(&conn).unwrap();

    let gk = db::get_archetypes(&conn, Some("GK")).unwrap();
    assert_eq!(gk.len(), 1);
    assert_eq!(gk[0].name, "Sweeper Keeper");
}
