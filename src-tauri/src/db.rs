use rusqlite::{params, Connection};
use serde::Serialize;

/// A single player's scouting data for the database view.
#[derive(Debug, Clone, Serialize)]
pub struct ScoutingPlayer {
    pub uid: String,
    pub name: String,
    pub nation: String,
    pub club: String,
    pub age: i64,
    pub position_raw: String,
    pub canonical_positions: Vec<String>,
    pub minutes: i64,
    pub transfer_value: Option<f64>,
    pub average_rating: Option<f64>,
    pub percentiles: Vec<PercentileEntry>,
}

/// A percentile entry for a single metric.
#[derive(Debug, Clone, Serialize)]
pub struct PercentileEntry {
    pub metric_name: String,
    pub percent_rank: f64,
}

/// A player with archetype scoring for role search results.
#[derive(Debug, Clone, Serialize)]
pub struct RoleSearchPlayer {
    pub uid: String,
    pub name: String,
    pub nation: String,
    pub club: String,
    pub age: i64,
    pub position_raw: String,
    pub canonical_positions: Vec<String>,
    pub minutes: i64,
    pub transfer_value: Option<f64>,
    pub quality_score: f64,
    pub value_score: f64,
    pub top_metrics: Vec<TopMetricEntry>,
}

/// A top-weighted metric entry for role search display.
#[derive(Debug, Clone, Serialize)]
pub struct TopMetricEntry {
    pub metric_name: String,
    pub percentile: f64,
    pub weight: f64,
}

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
            -- Transfer
            transfer_value REAL,
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

// Migration: add transfer_value column if it doesn't exist (for databases created before this change)
pub fn migrate_db(conn: &Connection) -> rusqlite::Result<()> {
    let has_column: bool = conn
        .prepare("SELECT transfer_value FROM season_players LIMIT 0")
        .is_ok();
    if !has_column {
        conn.execute_batch("ALTER TABLE season_players ADD COLUMN transfer_value REAL;")?;
    }
    Ok(())
}

/// Get a setting value by key. Returns Ok(None) when the key doesn't exist.
pub fn get_setting(conn: &Connection, key: &str) -> rusqlite::Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
    let mut rows = stmt.query_map(params![key], |row| row.get::<_, String>(0))?;
    match rows.next() {
        Some(Ok(value)) => Ok(Some(value)),
        _ => Ok(None),
    }
}

/// Set a setting key-value pair (upsert). If the key already exists, it is updated.
pub fn set_setting(conn: &Connection, key: &str, value: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
        params![key, value],
    )?;
    Ok(())
}

/// A summary of a past CSV import season, returned to the frontend.
#[derive(Debug, Clone, Serialize)]
pub struct SeasonInfo {
    pub id: i64,
    pub name: String,
    pub in_game_date: String,
    pub import_date: String,
    pub filename: String,
    pub player_count: i64,
}

/// Retrieve all seasons, most recent first.
pub fn get_seasons(conn: &Connection) -> rusqlite::Result<Vec<SeasonInfo>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, in_game_date, import_date, filename, player_count \
         FROM seasons ORDER BY import_date DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(SeasonInfo {
            id: row.get(0)?,
            name: row.get(1)?,
            in_game_date: row.get(2)?,
            import_date: row.get(3)?,
            filename: row.get(4)?,
            player_count: row.get(5)?,
        })
    })?;
    let mut seasons = Vec::new();
    for row in rows {
        seasons.push(row?);
    }
    Ok(seasons)
}

/// Seed default archetypes into the database.
/// Uses INSERT OR IGNORE so re-running is safe.
pub fn seed_archetypes(conn: &Connection) -> rusqlite::Result<()> {
    let archetypes = crate::scoring::get_seed_archetypes();
    let mut stmt = conn.prepare(
        "INSERT OR IGNORE INTO archetypes (name, base_position, metric_weights_json) VALUES (?1, ?2, ?3)",
    )?;
    for arch in &archetypes {
        let weights_json =
            serde_json::to_string(&arch.weights).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        stmt.execute(rusqlite::params![arch.name, arch.base_position, weights_json])?;
    }
    Ok(())
}

/// Archetype info returned to the frontend.
#[derive(Debug, Clone, Serialize)]
pub struct ArchetypeInfo {
    pub id: i64,
    pub name: String,
    pub base_position: String,
}

/// Retrieve all archetypes, optionally filtered by base_position.
pub fn get_archetypes(
    conn: &Connection,
    base_position: Option<&str>,
) -> rusqlite::Result<Vec<ArchetypeInfo>> {
    let sql = match base_position {
        Some(_) => "SELECT id, name, base_position FROM archetypes WHERE base_position = ?1 ORDER BY name",
        None => "SELECT id, name, base_position FROM archetypes ORDER BY base_position, name",
    };
    let mut stmt = conn.prepare(sql)?;
    let mut result = Vec::new();
    if let Some(pos) = base_position {
        let rows = stmt.query_map(params![pos], |row| {
            Ok(ArchetypeInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                base_position: row.get(2)?,
            })
        })?;
        for row in rows {
            result.push(row?);
        }
    } else {
        let rows = stmt.query_map([], |row| {
            Ok(ArchetypeInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                base_position: row.get(2)?,
            })
        })?;
        for row in rows {
            result.push(row?);
        }
    }
    Ok(result)
}

/// Get all players for the most recent season with their percentile data.
/// pool_type: "position" or "all" — determines which percentile pool to use.
/// managed_club: if Some, exclude players from this club (FR-032).
pub fn get_scouting_players(
    conn: &Connection,
    pool_type: &str,
    managed_club: Option<&str>,
) -> rusqlite::Result<Vec<ScoutingPlayer>> {
    // Validate pool_type
    if pool_type != "position" && pool_type != "all" {
        return Err(rusqlite::Error::InvalidParameterName(
            "pool_type must be 'position' or 'all'".into(),
        ));
    }

    // Get most recent season
    let season_id: i64 = conn.query_row(
        "SELECT id FROM seasons ORDER BY import_date DESC LIMIT 1",
        [],
        |row| row.get(0),
    )?;

    // Build exclusion clause for managed club
    let club_filter = match managed_club {
        Some(club) => format!("AND sp.club != '{}'", club.replace('\'', "''")),
        None => String::new(),
    };

    // Get all players with their data
    let sql = format!(
        "SELECT DISTINCT sp.uid, p.name, p.nation, sp.club, sp.age, sp.position_raw, sp.minutes, \
         sp.transfer_value, sp.average_rating \
         FROM season_players sp \
         JOIN players p ON sp.uid = p.uid \
         WHERE sp.season_id = ?1 {} \
         ORDER BY sp.minutes DESC",
        club_filter
    );

    let mut stmt = conn.prepare(&sql)?;
    let players: Vec<(String, String, String, String, i64, String, i64, Option<f64>, Option<f64>)> = stmt
        .query_map(params![season_id], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
            ))
        })?
        .filter_map(|r| r.ok())
        .collect();

    let mut results = Vec::new();
    for (uid, name, nation, club, age, pos_raw, minutes, transfer_value, avg_rating) in players {
        // Get canonical positions
        let mut pos_stmt = conn.prepare(
            "SELECT canonical_position FROM season_player_positions WHERE season_id = ?1 AND uid = ?2",
        )?;
        let canonical_positions: Vec<String> = pos_stmt
            .query_map(params![season_id, &uid], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        drop(pos_stmt);

        // Get percentiles for this player
        let mut pct_stmt = conn.prepare(
            "SELECT metric_name, percent_rank FROM player_percentiles \
             WHERE season_id = ?1 AND uid = ?2 AND pool_type = ?3 \
             ORDER BY metric_name",
        )?;
        let percentiles: Vec<PercentileEntry> = pct_stmt
            .query_map(params![season_id, &uid, pool_type], |row| {
                Ok(PercentileEntry {
                    metric_name: row.get(0)?,
                    percent_rank: row.get(1)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        results.push(ScoutingPlayer {
            uid,
            name,
            nation,
            club,
            age,
            position_raw: pos_raw,
            canonical_positions,
            minutes,
            transfer_value,
            average_rating: avg_rating,
            percentiles,
        });
    }

    Ok(results)
}

/// Get role search results for a specific archetype, sorted by score.
/// score_mode: "quality" or "value"
/// managed_club: if Some, exclude players from this club (FR-032).
pub fn get_role_search_results(
    conn: &Connection,
    archetype_id: i64,
    score_mode: &str,
    managed_club: Option<&str>,
) -> rusqlite::Result<Vec<RoleSearchPlayer>> {
    if score_mode != "quality" && score_mode != "value" {
        return Err(rusqlite::Error::InvalidParameterName(
            "score_mode must be 'quality' or 'value'".into(),
        ));
    }

    let sort_column = match score_mode {
        "quality" => "quality_score",
        "value" => "value_score",
        _ => "quality_score",
    };

    // Get most recent season
    let season_id: i64 = conn.query_row(
        "SELECT id FROM seasons ORDER BY import_date DESC LIMIT 1",
        [],
        |row| row.get(0),
    )?;

    let club_filter = match managed_club {
        Some(club) => format!("AND sp.club != '{}'", club.replace('\'', "''")),
        None => String::new(),
    };

    let sql = format!(
        "SELECT ascore.uid, p.name, p.nation, sp.club, sp.age, sp.position_raw, sp.minutes, \
         sp.transfer_value, ascore.quality_score, ascore.value_score \
         FROM archetype_scores ascore \
         JOIN season_players sp ON ascore.season_id = sp.season_id AND ascore.uid = sp.uid \
         JOIN players p ON ascore.uid = p.uid \
         WHERE ascore.season_id = ?1 AND ascore.archetype_id = ?2 {} \
         ORDER BY ascore.{} DESC",
        club_filter, sort_column
    );

    let mut stmt = conn.prepare(&sql)?;
    let rows: Vec<(String, String, String, String, i64, String, i64, Option<f64>, f64, f64)> = stmt
        .query_map(params![season_id, archetype_id], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
                row.get(9)?,
            ))
        })?
        .filter_map(|r| r.ok())
        .collect();

    // Get archetype weights for top_metrics
    let weights_json: String = conn.query_row(
        "SELECT metric_weights_json FROM archetypes WHERE id = ?1",
        params![archetype_id],
        |row| row.get(0),
    )?;
    let weights: crate::scoring::ArchetypeWeights = serde_json::from_str(&weights_json)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    // Collect all metric entries and sort by weight descending, take top 5
    let mut all_entries: Vec<&crate::scoring::ArchetypeWeightEntry> = weights
        .in_possession
        .iter()
        .chain(weights.out_of_possession.iter())
        .collect();
    all_entries.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));
    let top_entries: Vec<_> = all_entries.into_iter().take(5).collect();

    let mut results = Vec::new();
    for (uid, name, nation, club, age, pos_raw, minutes, transfer_value, quality_score, value_score) in rows {
        // Get canonical positions
        let mut pos_stmt = conn.prepare(
            "SELECT canonical_position FROM season_player_positions WHERE season_id = ?1 AND uid = ?2",
        )?;
        let canonical_positions: Vec<String> = pos_stmt
            .query_map(params![season_id, &uid], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        drop(pos_stmt);

        // Get percentiles for top metrics
        let mut top_metrics = Vec::new();
        for entry in &top_entries {
            let pct: Option<f64> = conn
                .query_row(
                    "SELECT percent_rank FROM player_percentiles \
                     WHERE season_id = ?1 AND uid = ?2 AND metric_name = ?3 AND pool_type = 'all'",
                    params![season_id, &uid, entry.metric_name],
                    |row| row.get(0),
                )
                .ok();
            top_metrics.push(TopMetricEntry {
                metric_name: entry.metric_name.clone(),
                percentile: pct.unwrap_or(0.0),
                weight: entry.weight,
            });
        }

        results.push(RoleSearchPlayer {
            uid,
            name,
            nation,
            club,
            age,
            position_raw: pos_raw,
            canonical_positions,
            minutes,
            transfer_value,
            quality_score,
            value_score,
            top_metrics,
        });
    }

    Ok(results)
}
