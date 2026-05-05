use crate::parsers::expansion::expand_position;
use crate::parsers::row_parser::parse_row;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashSet;

/// A single rejected row with human-readable reasons.
#[derive(Debug, Clone, Serialize)]
pub struct RejectedRow {
    pub row_number: usize,
    pub reasons: Vec<String>,
}

/// Summary of a completed CSV import.
#[derive(Debug, Clone, Serialize)]
pub struct ImportResult {
    pub season_id: i64,
    pub season_name: String,
    pub players_imported: usize,
    pub rows_rejected: Vec<RejectedRow>,
}

/// Run the full CSV import pipeline inside an atomic transaction.
///
/// 1. Reads the file and detects its encoding (UTF-8 → windows-1252 fallback).
/// 2. Parses CSV rows on the semicolon delimiter.
/// 3. Validates and converts each row through the parsers pipeline.
/// 4. Expands compound FM position codes into canonical positions.
/// 5. Inserts everything in a single SQLite transaction (RAII — auto-rollback on error).
/// 6. Returns an `ImportResult` summary.
pub fn run_import(
    conn: &mut Connection,
    file_path: &str,
    in_game_date: &str,
) -> Result<ImportResult, String> {
    // ---- 1. Read file and detect encoding ----
    let bytes = std::fs::read(file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let decoded = crate::encoding::detect_and_decode(&bytes)?;

    // ---- 2. Set up CSV reader ----
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .flexible(true)
        .from_reader(decoded.content.as_bytes());

    let headers: Vec<String> = reader
        .headers()
        .map_err(|e| format!("Failed to read CSV headers: {}", e))?
        .iter()
        .map(|s| s.to_string())
        .collect();

    if headers.is_empty() {
        return Err("CSV file has no headers".to_string());
    }

    let mut seen_uids: HashSet<String> = HashSet::new();
    let mut rows_rejected: Vec<RejectedRow> = Vec::new();
    let mut players_imported: usize = 0;

    // Season label derived from in-game date
    let season_name = format!("{} Season", in_game_date);

    // Extract filename for the season record
    let filename = std::path::Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown.csv")
        .to_string();

    // ---- 3. Begin RAII transaction (auto-rollback on drop) ----
    // If any operation fails, rusqlite rolls back the transaction when `tx` is dropped.
    let tx = conn
        .transaction()
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    // Create season record
    tx.execute(
        "INSERT INTO seasons (name, in_game_date, filename, player_count) VALUES (?1, ?2, ?3, 0)",
        rusqlite::params![season_name, in_game_date, filename],
    )
    .map_err(|e| format!("Failed to create season record: {}", e))?;

    let season_id: i64 = tx.last_insert_rowid();

    // ---- 4. Prepare INSERT statements and process records ----
    // NB: Prepared statements are scoped inside this block so they drop
    // before `tx.commit()` below (they borrow `tx`).
    {
        let mut insert_player = tx
            .prepare(
                "INSERT OR IGNORE INTO players (uid, name, nation, second_nationality, height, \
         left_foot_score, right_foot_score, left_foot_raw, right_foot_raw) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            )
            .map_err(|e| format!("Failed to prepare player insert: {}", e))?;

        let mut insert_position = tx
            .prepare(
                "INSERT OR IGNORE INTO player_positions (uid, canonical_position) VALUES (?1, ?2)",
            )
            .map_err(|e| format!("Failed to prepare position insert: {}", e))?;

        let mut insert_spp = tx.prepare(
        "INSERT OR IGNORE INTO season_player_positions (season_id, uid, canonical_position) VALUES (?1, ?2, ?3)"
    ).map_err(|e| format!("Failed to prepare SPP insert: {}", e))?;

        // 107-column season_players insert
        let mut insert_season_player = tx.prepare(
        "INSERT OR IGNORE INTO season_players \
         (season_id, uid, club, position_raw, age, minutes, starts, subs, expires, \
          cor, cro, dri, fin, fir, fre, hea, lon, lon_thr, mar, pas, pen, tck, tec, \
          agg, ant, bra, cmp, cnt, dec, det, fla, inf, lea, otb, pos, tea, vis, wor, \
          acc, agi, bal, jum, nat_fit, pac, sta, str, \
          aer, com_gk, ecc, han, kic, one_on_one, pun, ref_gk, rus, thr, cmd, \
          goals_xg, npxg, xg_op, xg_per_shot, shots, shots_on_target, sot_pct, conv_pct, pens_scored, pens_taken, \
          assists, xa, key_passes, chances_created, op_key_passes, through_balls, \
          dribbles_per_game, progressive_passes, progressive_runs, \
          passes_completed, passes_attempted, pass_completion_pct, \
          crosses_attempted, crosses_completed, cross_completion_pct, \
          tackles_per_game, tackles_completed, tackle_completion_pct, \
          interceptions_per_game, clearances, blocks, \
          possession_won, possession_lost, \
          headers_won, headers_lost, headers_won_pct, \
          saves, save_pct, goals_conceded, clean_sheets, penalties_saved, expected_saves, \
          fouls_made, fouls_against, yellow_cards, red_cards, offsides, \
          distance_covered, average_rating, player_of_match) \
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9, \
          ?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23, \
          ?24,?25,?26,?27,?28,?29,?30,?31,?32,?33,?34,?35,?36,?37,?38, \
          ?39,?40,?41,?42,?43,?44,?45,?46, \
          ?47,?48,?49,?50,?51,?52,?53,?54,?55,?56,?57, \
          ?58,?59,?60,?61,?62,?63,?64,?65,?66,?67, \
          ?68,?69,?70,?71,?72,?73, \
          ?74,?75,?76, \
          ?77,?78,?79, \
          ?80,?81,?82, \
          ?83,?84,?85, \
          ?86,?87,?88, \
          ?89,?90, \
          ?91,?92,?93, \
          ?94,?95,?96,?97,?98,?99, \
          ?100,?101,?102,?103,?104, \
          ?105,?106,?107)"
    ).map_err(|e| format!("Failed to prepare season_player insert: {}", e))?;

        // ---- 5. Process each CSV record ----
        for (row_idx, result) in reader.records().enumerate() {
            let row_number = row_idx + 2; // 1-indexed: row 1 = headers, row 2+ = data
            let record = match result {
                Ok(r) => r,
                Err(e) => {
                    rows_rejected.push(RejectedRow {
                        row_number,
                        reasons: vec![format!("CSV parse error: {}", e)],
                    });
                    continue;
                }
            };

            let fields: Vec<String> = record.iter().map(|s| s.to_string()).collect();

            match parse_row(&fields, &headers, &mut seen_uids) {
                Ok(player) => {
                    // Validator guarantees uid is present after validate_row passes
                    let uid = player.uid.as_deref().expect(
                        "parse_row succeeded but uid is None — validator should have rejected this",
                    );

                    // Extract footedness raw strings safely
                    let (left_raw_str, left_score) =
                        match (&player.left_foot_raw, player.left_foot_score) {
                            (Some(r), s) => (r.as_str(), s),
                            (None, _) => ("", None),
                        };
                    let (right_raw_str, right_score) =
                        match (&player.right_foot_raw, player.right_foot_score) {
                            (Some(r), s) => (r.as_str(), s),
                            (None, _) => ("", None),
                        };

                    // INSERT into players (OR IGNORE handles existing UIDs)
                    insert_player
                        .execute(rusqlite::params![
                            uid,
                            player.name,
                            player.nation,
                            player.second_nationality,
                            player.height,
                            left_score,
                            right_score,
                            left_raw_str,
                            right_raw_str,
                        ])
                        .map_err(|e| format!("Insert player error at row {}: {}", row_number, e))?;

                    // Expand and insert positions
                    if let Some(ref pos_raw) = player.position_raw {
                        for single_pos in pos_raw.split(", ") {
                            for canon in &expand_position(single_pos) {
                                insert_position
                                    .execute(rusqlite::params![uid, canon])
                                    .map_err(|e| {
                                        format!(
                                            "Insert position error at row {}: {}",
                                            row_number, e
                                        )
                                    })?;
                                insert_spp
                                    .execute(rusqlite::params![season_id, uid, canon])
                                    .map_err(|e| {
                                        format!("Insert SPP error at row {}: {}", row_number, e)
                                    })?;
                            }
                        }
                    }

                    // INSERT into season_players (107 params)
                    insert_season_player
                        .execute(rusqlite::params![
                            season_id,
                            uid,
                            player.club,
                            player.position_raw,
                            player.age,
                            player.minutes,
                            player.starts,
                            player.subs,
                            player.expires,
                            player.cor,
                            player.cro,
                            player.dri,
                            player.fin,
                            player.fir,
                            player.fre,
                            player.hea,
                            player.lon,
                            player.lon_thr,
                            player.mar,
                            player.pas,
                            player.pen,
                            player.tck,
                            player.tec,
                            player.agg,
                            player.ant,
                            player.bra,
                            player.cmp,
                            player.cnt,
                            player.dec,
                            player.det,
                            player.fla,
                            player.inf,
                            player.lea,
                            player.otb,
                            player.pos,
                            player.tea,
                            player.vis,
                            player.wor,
                            player.acc,
                            player.agi,
                            player.bal,
                            player.jum,
                            player.nat_fit,
                            player.pac,
                            player.sta,
                            player.str,
                            player.aer,
                            player.com_gk,
                            player.ecc,
                            player.han,
                            player.kic,
                            player.one_on_one,
                            player.pun,
                            player.ref_gk,
                            player.rus,
                            player.thr,
                            player.cmd,
                            player.goals_xg,
                            player.npxg,
                            player.xg_op,
                            player.xg_per_shot,
                            player.shots,
                            player.shots_on_target,
                            player.sot_pct,
                            player.conv_pct,
                            player.pens_scored,
                            player.pens_taken,
                            player.assists,
                            player.xa,
                            player.key_passes,
                            player.chances_created,
                            player.op_key_passes,
                            player.through_balls,
                            player.dribbles_per_game,
                            player.progressive_passes,
                            player.progressive_runs,
                            player.passes_completed,
                            player.passes_attempted,
                            player.pass_completion_pct,
                            player.crosses_attempted,
                            player.crosses_completed,
                            player.cross_completion_pct,
                            player.tackles_per_game,
                            player.tackles_completed,
                            player.tackle_completion_pct,
                            player.interceptions_per_game,
                            player.clearances,
                            player.blocks,
                            player.possession_won,
                            player.possession_lost,
                            player.headers_won,
                            player.headers_lost,
                            player.headers_won_pct,
                            player.saves,
                            player.save_pct,
                            player.goals_conceded,
                            player.clean_sheets,
                            player.penalties_saved,
                            player.expected_saves,
                            player.fouls_made,
                            player.fouls_against,
                            player.yellow_cards,
                            player.red_cards,
                            player.offsides,
                            player.distance_covered,
                            player.average_rating,
                            player.player_of_match,
                        ])
                        .map_err(|e| {
                            format!("Insert season_player error at row {}: {}", row_number, e)
                        })?;

                    players_imported += 1;
                }
                Err(reasons) => {
                    rows_rejected.push(RejectedRow {
                        row_number,
                        reasons,
                    });
                }
            }
        }
    } // <-- prepared statements dropped here

    // Update player count on seasons table
    tx.execute(
        "UPDATE seasons SET player_count = ?1 WHERE id = ?2",
        rusqlite::params![players_imported, season_id],
    )
    .map_err(|e| format!("Failed to update player count: {}", e))?;

    // ---- 6. Commit (RAII drops the transaction guard, committing automatically) ----
    tx.commit()
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    Ok(ImportResult {
        season_id,
        season_name,
        players_imported,
        rows_rejected,
    })
}
