//! CSV parser for FM ValueScout.
//!
//! ## Structure
//!
//! - `parsing::` — string parsing helpers
//! - `mapping::` — header-name → column-index mapping
//! - `enrichment::` — per-90, ratio, and total-from-per-90 calculations

pub mod enrichment;
pub mod mapping;
pub mod parsing;

use crate::models::csv_result::CsvResult;
use crate::models::player::Player;
use crate::models::types::FootRating;
use std::collections::HashMap;

/// Parse a CSV file and return all players with computed metrics.
pub fn parse_file(path: &str) -> Result<CsvResult, String> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path)
        .map_err(|e| format!("Failed to open CSV: {}", e))?;

    let headers = reader
        .headers()
        .map_err(|e| format!("Failed to read headers: {}", e))?
        .clone();
    let col_map = mapping::build_column_map(&headers);

    let mut currency = '€';
    let mut players = Vec::new();

    for (idx, result) in reader.records().enumerate() {
        let record = result.map_err(|e| format!("Row {}: {}", idx + 2, e))?;

        let mut player = parse_row(&record, &col_map)?;

        // Extract currency from first data row (from Transfer Value column)
        if idx == 0 {
            if let Some(val) = record.get(*col_map.get("Transfer Value").unwrap_or(&12)) {
                let (_, c) = parsing::parse_transfer_range(val);
                currency = c;
            }
        }

        enrichment::enrich(&mut player);
        players.push(player);
    }

    Ok(CsvResult { players, currency })
}

fn parse_row(
    record: &csv::StringRecord,
    col_map: &HashMap<String, usize>,
) -> Result<Player, String> {
    let get = |name: &str| -> &str { record.get(*col_map.get(name).unwrap_or(&0)).unwrap_or("") };

    // Build raw Player from CSV fields
    Ok(Player {
        // === Identity fields ===
        unique_id: get("Unique ID").parse().unwrap_or(0),
        name: get("Player").to_string(),
        nation: get("Nation").to_string(),
        second_nation: {
            let s = get("2nd Nat");
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        },
        club: get("Club").to_string(),
        position: get("Position").to_string(),
        positions: parsing::parse_positions(get("Position")),

        // === Physical fields ===
        age: get("Age").parse().unwrap_or(0),
        height_cm: parsing::parse_height(get("Height")),
        left_foot: FootRating::from_str(get("Left Foot")).unwrap_or(FootRating::Weak),
        right_foot: FootRating::from_str(get("Right Foot")).unwrap_or(FootRating::Weak),

        // === Ability fields ===
        current_ability: get("CA").parse().unwrap_or(0),
        potential_ability: get("PA").parse().unwrap_or(0),

        // === Contract fields ===
        transfer_value: parsing::parse_transfer_range(get("Transfer Value")).0,
        weekly_wage: parsing::parse_wage(get("Wage")).0,
        contract_expires: parsing::parse_date(get("Expires")),

        // === Availability ===
        appearances: parsing::parse_appearances(get("Appearances")),
        minutes: get("Minutes").parse().unwrap_or(0),

        // === Attacking metrics ===
        goals: get("Goals").parse().unwrap_or(0),
        goals_per_90: None,
        goals_outside_box: get("Goals From Outside The Box").parse().unwrap_or(0),
        goals_outside_box_per_90: None,
        xg: get("xG").parse().unwrap_or(0.0),
        xg_per_90: None,
        npxg: get("NP-xG").parse().unwrap_or(0.0),
        npxg_per_90: None,
        xg_overperformance: get("xG-OP").parse().unwrap_or(0.0),
        xg_overperformance_per_90: None,
        xg_per_shot: get("xG/shot").parse().unwrap_or(0.0),
        shots: get("Shots").parse().unwrap_or(0),
        shots_per_90: None,
        shots_outside_box: None,
        shots_outside_box_per_90: Some(
            get("Shots From Outside The Box Per 90 minutes")
                .parse()
                .unwrap_or(0.0),
        ),
        shots_on_target: get("ShT").parse().unwrap_or(0),
        shots_on_target_per_90: None,
        shots_on_target_ratio: None,
        conversion_ratio: None,
        pens_taken: get("Pens").parse().unwrap_or(0),
        pens_taken_per_90: None,
        pens_scored: get("Pens S").parse().unwrap_or(0),
        pens_scored_per_90: None,
        pens_scored_ratio: None,
        free_kick_shots: get("Free Kick Shots").parse().unwrap_or(0),
        free_kick_shots_per_90: None,
        minutes_per_goal: None,
        minutes_per_goal_or_assist: None,

        // === Creativity metrics ===
        assists: get("Assists").parse().unwrap_or(0),
        assists_per_90: None,
        xa: get("xA").parse().unwrap_or(0.0),
        xa_per_90: None,
        chances_created: None,
        chances_created_per_90: Some(get("Ch C/90").parse().unwrap_or(0.0)),
        clear_cut_chances: get("CCC").parse().unwrap_or(0),
        clear_cut_chances_per_90: None,
        key_passes: get("Key").parse().unwrap_or(0),
        key_passes_per_90: None,
        op_key_passes: None,
        op_key_passes_per_90: Some(get("OP-KP/90").parse().unwrap_or(0.0)),
        crosses_attempted: get("Cr A").parse().unwrap_or(0),
        crosses_attempted_per_90: None,
        crosses_completed: get("Cr C").parse().unwrap_or(0),
        crosses_completed_per_90: None,
        cross_completion_ratio: None,
        op_crosses_attempted: get("OP-Crs A").parse().unwrap_or(0),
        op_crosses_attempted_per_90: None,
        op_crosses_completed: get("OP-Crs C").parse().unwrap_or(0),
        op_crosses_completed_per_90: None,
        op_cross_completion_ratio: None,
        minutes_per_assist: None,

        // === Transition metrics ===
        passes_attempted: get("Pas A").parse().unwrap_or(0),
        passes_attempted_per_90: None,
        passes_completed: get("Ps C").parse().unwrap_or(0),
        passes_completed_per_90: None,
        pass_completion_ratio: None,
        progressive_passes: get("PsP").parse().unwrap_or(0),
        progressive_passes_per_90: None,
        dribbles: get("Drb").parse().unwrap_or(0),
        dribbles_per_90: None,
        distance_covered: parsing::parse_distance(get("Distance")),
        distance_covered_per_90: None,
        sprints: None,
        sprints_per_90: Some(get("Sprints/90").parse().unwrap_or(0.0)),
        poss_lost: None,
        poss_lost_per_90: Some(get("Poss Lost/90").parse().unwrap_or(0.0)),

        // === Defensive metrics ===
        tackles_attempted: get("Tck A").parse().unwrap_or(0),
        tackles_attempted_per_90: None,
        tackles_completed: get("Tck C").parse().unwrap_or(0),
        tackles_completed_per_90: None,
        tackle_completion_ratio: None,
        key_tackles: get("K Tck").parse().unwrap_or(0),
        key_tackles_per_90: None,
        interceptions: get("Itc").parse().unwrap_or(0),
        interceptions_per_90: None,
        poss_won: None,
        poss_won_per_90: Some(get("Poss Won/90").parse().unwrap_or(0.0)),
        pressures_attempted: get("Pres A").parse().unwrap_or(0),
        pressures_attempted_per_90: None,
        pressures_completed: get("Pres C").parse().unwrap_or(0),
        pressures_completed_per_90: None,
        pressure_success_ratio: None,
        blocks: get("Blk").parse().unwrap_or(0),
        blocks_per_90: None,
        shots_blocked: get("Shts Blckd").parse().unwrap_or(0),
        shots_blocked_per_90: None,
        clearances: get("Clearances").parse().unwrap_or(0),
        clearances_per_90: None,

        // === Aerial metrics ===
        headers_attempted: get("Hdrs A").parse().unwrap_or(0),
        headers_attempted_per_90: None,
        headers_won: get("Hdrs").parse().unwrap_or(0),
        headers_won_per_90: None,
        headers_lost: None,
        headers_lost_per_90: Some(get("Hdrs L/90").parse().unwrap_or(0.0)),
        headers_won_ratio: None,
        key_headers: None,
        key_headers_per_90: Some(get("K Hdrs/90").parse().unwrap_or(0.0)),

        // === Goalkeeping metrics ===
        clean_sheets: get("Clean Sheets").parse().unwrap_or(0),
        clean_sheets_per_90: None,
        goals_conceded: get("Goals Conceded").parse().unwrap_or(0),
        goals_conceded_per_90: None,
        total_saves: None,
        total_saves_per_90: Some(get("Saves/90").parse().unwrap_or(0.0)),
        save_ratio: None,
        xsv_percent: get("xSv %").parse().unwrap_or(0.0),
        xgp: get("xGP").parse().unwrap_or(0.0),
        xgp_per_90: None,
        saves_held: get("Svh").parse().unwrap_or(0),
        saves_held_per_90: None,
        saves_parried: get("Svp").parse().unwrap_or(0),
        saves_parried_per_90: None,
        saves_tipped: get("Svt").parse().unwrap_or(0),
        saves_tipped_per_90: None,
        pens_faced: get("Pens Faced").parse().unwrap_or(0),
        pens_faced_per_90: None,
        pens_saved: get("Pens Saved").parse().unwrap_or(0),
        pens_saved_per_90: None,
        pens_saved_ratio: None,

        // === Discipline metrics ===
        fouls_made: get("Fouls Made").parse().unwrap_or(0),
        fouls_made_per_90: None,
        fouls_against: get("Fouls Against").parse().unwrap_or(0),
        fouls_against_per_90: None,
        yellow_cards: get("Yel").parse().unwrap_or(0),
        yellow_cards_per_90: None,
        red_cards: get("Red cards").parse().unwrap_or(0),
        red_cards_per_90: None,
        offsides: get("Off").parse().unwrap_or(0),
        offsides_per_90: None,
        mlg: get("MLG").parse().unwrap_or(0),
        mlg_per_90: None,

        // === Match impact metrics ===
        rating: get("Rating").parse().unwrap_or(0.0),
        pom: get("PoM").parse().unwrap_or(0),
        game_win_ratio: None,
        games_won: get("Games Won").parse().unwrap_or(0),
        games_drawn: get("Games Drawn").parse().unwrap_or(0),
        games_lost: get("Games Lost").parse().unwrap_or(0),
        team_goals: get("Team Goals").parse().unwrap_or(0),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_integration() {
        let path = "../docs/notes/test-files/Test_CSV_2026_04_29.csv";
        let result = parse_file(path);
        match result {
            Ok(csv_result) => {
                assert!(csv_result.len() > 0, "Expected at least one player");
                assert_eq!(csv_result.currency, '€');
                // First player should have enrichment computed
                let first = &csv_result.players[0];
                assert!(first.goals_per_90.is_some() || first.goals == 0);
            }
            Err(e) => panic!("Failed to parse CSV: {}", e),
        }
    }
}
