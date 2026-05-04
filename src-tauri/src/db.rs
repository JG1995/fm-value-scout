use rusqlite::Connection;

/// Create all application tables and indexes if they do not already exist.
pub fn create_tables(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "
        -- 1. PLAYERS: immutable identity
        CREATE TABLE IF NOT EXISTS players (
            uid         TEXT PRIMARY KEY,
            name        TEXT NOT NULL,
            nation      TEXT NOT NULL,
            second_nationality TEXT,
            height      INTEGER,
            left_foot_score INTEGER,
            right_foot_score INTEGER,
            left_foot_raw TEXT,
            right_foot_raw TEXT,
            created_at  TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- 2. PLAYER_POSITIONS: canonical positions per player
        CREATE TABLE IF NOT EXISTS player_positions (
            uid                TEXT NOT NULL REFERENCES players(uid),
            canonical_position TEXT NOT NULL,
            PRIMARY KEY (uid, canonical_position)
        );

        -- 3. SEASONS: import metadata
        CREATE TABLE IF NOT EXISTS seasons (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            name         TEXT NOT NULL,
            in_game_date TEXT NOT NULL,
            import_date  TEXT NOT NULL DEFAULT (datetime('now')),
            filename     TEXT NOT NULL,
            player_count INTEGER NOT NULL
        );

        -- 4. SEASON_PLAYERS: per-player-per-season stats (wide table, 82+ metrics)
        CREATE TABLE IF NOT EXISTS season_players (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            season_id    INTEGER NOT NULL REFERENCES seasons(id),
            uid          TEXT NOT NULL REFERENCES players(uid),
            club         TEXT NOT NULL,
            position_raw TEXT NOT NULL,
            age          INTEGER NOT NULL,
            minutes      INTEGER NOT NULL DEFAULT 0,
            starts       INTEGER NOT NULL DEFAULT 0,
            subs         INTEGER NOT NULL DEFAULT 0,
            expires      TEXT,
            -- Technical
            cor INTEGER, cro INTEGER, dri INTEGER, fin INTEGER, fir INTEGER, fre INTEGER,
            hea INTEGER, lon INTEGER, lon_thr INTEGER, mar INTEGER, pas INTEGER, pen INTEGER,
            tck INTEGER, tec INTEGER,
            -- Mental
            agg INTEGER, ant INTEGER, bra INTEGER, cmp INTEGER, cnt INTEGER, dec INTEGER,
            det INTEGER, fla INTEGER, inf INTEGER, lea INTEGER, otb INTEGER,
            pos INTEGER, tea INTEGER, vis INTEGER, wor INTEGER,
            -- Physical
            acc INTEGER, agi INTEGER, bal INTEGER, jum INTEGER, nat_fit INTEGER, pac INTEGER, sta INTEGER, str INTEGER,
            -- Goalkeeper
            aer INTEGER, com_gk INTEGER, ecc INTEGER, han INTEGER, kic INTEGER,
            one_on_one INTEGER, pun INTEGER, ref_gk INTEGER, rus INTEGER, thr INTEGER, cmd INTEGER,
            -- Attacking & Finishing
            goals_xg REAL, npxg REAL, xg_op REAL, xg_per_shot REAL, shots REAL, shots_on_target REAL,
            sot_pct REAL, conv_pct REAL, pens_scored REAL, pens_taken REAL,
            -- Creativity & Chance Creation
            assists REAL, xa REAL, key_passes REAL, chances_created REAL, op_key_passes REAL, through_balls REAL,
            -- Transition & Ball Progression
            dribbles_per_game REAL, progressive_passes REAL, progressive_runs REAL,
            passes_completed REAL, passes_attempted REAL, pass_completion_pct REAL,
            crosses_attempted REAL, crosses_completed REAL, cross_completion_pct REAL,
            -- Defensive Actions
            tackles_per_game REAL, tackles_completed REAL, tackle_completion_pct REAL,
            interceptions_per_game REAL, clearances REAL, blocks REAL,
            possession_won REAL, possession_lost REAL,
            -- Aerial Presence
            headers_won REAL, headers_lost REAL, headers_won_pct REAL,
            -- Goalkeeping Stats
            saves REAL, save_pct REAL, goals_conceded REAL, clean_sheets REAL,
            penalties_saved REAL, expected_saves REAL,
            -- Discipline
            fouls_made REAL, fouls_against REAL, yellow_cards REAL, red_cards REAL, offsides REAL,
            -- Match Impact
            distance_covered REAL, average_rating REAL, player_of_match REAL,
            UNIQUE(season_id, uid)
        );

        -- 5. SEASON_PLAYER_POSITIONS: expanded canonical positions per season per player
        CREATE TABLE IF NOT EXISTS season_player_positions (
            season_id          INTEGER NOT NULL REFERENCES seasons(id),
            uid                TEXT NOT NULL REFERENCES players(uid),
            canonical_position TEXT NOT NULL,
            PRIMARY KEY (season_id, uid, canonical_position)
        );

        -- 6. PLAYER_PERCENTILES: pre-computed percentile ranks
        CREATE TABLE IF NOT EXISTS player_percentiles (
            season_id          INTEGER NOT NULL REFERENCES seasons(id),
            uid                TEXT NOT NULL REFERENCES players(uid),
            metric_name        TEXT NOT NULL,
            pool_type          TEXT NOT NULL CHECK(pool_type IN ('position', 'all')),
            canonical_position TEXT NOT NULL DEFAULT '',
            percent_rank       REAL NOT NULL,
            PRIMARY KEY (season_id, uid, metric_name, pool_type, canonical_position)
        );

        -- 7. ARCHETYPES: role definitions
        CREATE TABLE IF NOT EXISTS archetypes (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            name            TEXT NOT NULL UNIQUE,
            base_position   TEXT NOT NULL,
            metric_weights_json  TEXT NOT NULL
        );

        -- 8. ARCHETYPE_SCORES: pre-computed quality and value scores
        CREATE TABLE IF NOT EXISTS archetype_scores (
            season_id     INTEGER NOT NULL REFERENCES seasons(id),
            uid           TEXT NOT NULL REFERENCES players(uid),
            archetype_id  INTEGER NOT NULL REFERENCES archetypes(id),
            quality_score REAL NOT NULL,
            value_score   REAL NOT NULL,
            PRIMARY KEY (season_id, uid, archetype_id)
        );

        -- 9. SETTINGS: generic key-value store for user preferences
        CREATE TABLE IF NOT EXISTS settings (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        -- Indexes for query performance
        CREATE INDEX IF NOT EXISTS idx_sp_season_mins ON season_players(season_id, minutes);
        CREATE INDEX IF NOT EXISTS idx_spp_season_pos ON season_player_positions(season_id, canonical_position);
        CREATE INDEX IF NOT EXISTS idx_as_season_arch_score ON archetype_scores(season_id, archetype_id, quality_score DESC);
        ",
    )
}

/// Seed default archetypes into the database.
/// This is a stub to be implemented in task 5.0.
pub fn seed_archetypes(_conn: &Connection) -> rusqlite::Result<()> {
    Ok(())
}
