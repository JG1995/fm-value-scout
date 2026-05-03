use crate::models::player::Player;

/// Convert a total to per-90. Returns None if minutes == 0.
fn per_90(total: f64, minutes: u32) -> Option<f64> {
    (minutes > 0).then(|| total * 90.0 / minutes as f64)
}

/// Convert per-90 to total. Returns None if minutes == 0.
fn total_from_per_90(per_90: f64, minutes: u32) -> Option<f64> {
    (minutes > 0).then(|| per_90 * minutes as f64 / 90.0)
}

/// Compute a / b. Returns None if b == 0.
fn ratio(numerator: f64, denominator: f64) -> Option<f64> {
    (denominator != 0.0).then(|| numerator / denominator)
}

/// Enrich a player in-place: compute all per-90s, ratios, and totals-from-per-90s.
pub fn enrich(player: &mut Player) {
    // === Attacking per-90s ===
    player.goals_per_90 = per_90(player.goals as f64, player.minutes);
    player.goals_outside_box_per_90 = per_90(player.goals_outside_box as f64, player.minutes);
    player.xg_per_90 = per_90(player.xg, player.minutes);
    player.npxg_per_90 = per_90(player.npxg, player.minutes);
    player.xg_overperformance_per_90 = per_90(player.xg_overperformance, player.minutes);
    player.shots_per_90 = per_90(player.shots as f64, player.minutes);
    player.shots_on_target_per_90 = per_90(player.shots_on_target as f64, player.minutes);
    player.pens_taken_per_90 = per_90(player.pens_taken as f64, player.minutes);
    player.pens_scored_per_90 = per_90(player.pens_scored as f64, player.minutes);
    player.free_kick_shots_per_90 = per_90(player.free_kick_shots as f64, player.minutes);

    // Per-90 from CSV per-90 → total
    if let Some(v) = total_from_per_90(player.shots_outside_box_per_90, player.minutes) {
        player.shots_outside_box = Some(v);
    }

    // === Attacking ratios ===
    player.shots_on_target_ratio = ratio(player.shots_on_target as f64, player.shots as f64);
    player.conversion_ratio = ratio(player.goals as f64, player.shots as f64);
    player.pens_scored_ratio = ratio(player.pens_scored as f64, player.pens_taken as f64);

    // === Minutes-based ===
    player.minutes_per_goal = ratio(player.minutes as f64, player.goals as f64);
    player.minutes_per_goal_or_assist = ratio(
        player.minutes as f64,
        (player.goals + player.assists) as f64,
    );
    player.minutes_per_assist = ratio(player.minutes as f64, player.assists as f64);

    // === Creativity per-90s ===
    player.assists_per_90 = per_90(player.assists as f64, player.minutes);
    player.xa_per_90 = per_90(player.xa, player.minutes);
    if let Some(v) = total_from_per_90(player.chances_created_per_90, player.minutes) {
        player.chances_created = Some(v);
    }
    player.clear_cut_chances_per_90 = per_90(player.clear_cut_chances as f64, player.minutes);
    player.key_passes_per_90 = per_90(player.key_passes as f64, player.minutes);

    // OP key passes: per-90 from CSV → total
    if let Some(v) = total_from_per_90(player.op_key_passes_per_90, player.minutes) {
        player.op_key_passes = Some(v);
    }

    player.crosses_attempted_per_90 = per_90(player.crosses_attempted as f64, player.minutes);
    player.crosses_completed_per_90 = per_90(player.crosses_completed as f64, player.minutes);
    player.op_crosses_attempted_per_90 = per_90(player.op_crosses_attempted as f64, player.minutes);
    player.op_crosses_completed_per_90 = per_90(player.op_crosses_completed as f64, player.minutes);

    // === Creativity ratios ===
    player.cross_completion_ratio = ratio(
        player.crosses_completed as f64,
        player.crosses_attempted as f64,
    );
    player.op_cross_completion_ratio = ratio(
        player.op_crosses_completed as f64,
        player.op_crosses_attempted as f64,
    );

    // === Transition per-90s ===
    player.passes_attempted_per_90 = per_90(player.passes_attempted as f64, player.minutes);
    player.passes_completed_per_90 = per_90(player.passes_completed as f64, player.minutes);
    player.pass_completion_ratio = ratio(
        player.passes_completed as f64,
        player.passes_attempted as f64,
    );
    player.progressive_passes_per_90 = per_90(player.progressive_passes as f64, player.minutes);
    player.dribbles_per_90 = per_90(player.dribbles as f64, player.minutes);
    player.distance_covered_per_90 = per_90(player.distance_covered as f64, player.minutes);

    // Sprints: per-90 from CSV → total
    if let Some(v) = total_from_per_90(player.sprints_per_90, player.minutes) {
        player.sprints = Some(v);
    }

    // Poss lost: per-90 from CSV → total
    if let Some(v) = total_from_per_90(player.poss_lost_per_90, player.minutes) {
        player.poss_lost = Some(v);
    }

    // === Defensive per-90s ===
    player.tackles_attempted_per_90 = per_90(player.tackles_attempted as f64, player.minutes);
    player.tackles_completed_per_90 = per_90(player.tackles_completed as f64, player.minutes);
    player.key_tackles_per_90 = per_90(player.key_tackles as f64, player.minutes);
    player.interceptions_per_90 = per_90(player.interceptions as f64, player.minutes);
    player.pressures_attempted_per_90 = per_90(player.pressures_attempted as f64, player.minutes);
    player.pressures_completed_per_90 = per_90(player.pressures_completed as f64, player.minutes);
    player.blocks_per_90 = per_90(player.blocks as f64, player.minutes);
    player.shots_blocked_per_90 = per_90(player.shots_blocked as f64, player.minutes);
    player.clearances_per_90 = per_90(player.clearances as f64, player.minutes);

    // Poss won: per-90 from CSV → total
    if let Some(v) = total_from_per_90(player.poss_won_per_90, player.minutes) {
        player.poss_won = Some(v);
    }

    // === Defensive ratios ===
    player.tackle_completion_ratio = ratio(
        player.tackles_completed as f64,
        player.tackles_attempted as f64,
    );
    player.pressure_success_ratio = ratio(
        player.pressures_completed as f64,
        player.pressures_attempted as f64,
    );

    // === Aerial per-90s ===
    player.headers_attempted_per_90 = per_90(player.headers_attempted as f64, player.minutes);
    player.headers_won_per_90 = per_90(player.headers_won as f64, player.minutes);
    // Headers lost: per-90 from CSV → total
    if let Some(v) = total_from_per_90(player.headers_lost_per_90, player.minutes) {
        player.headers_lost = Some(v);
    }

    // === Aerial ratios ===
    player.headers_won_ratio = ratio(player.headers_won as f64, player.headers_attempted as f64);

    // Key headers: per-90 from CSV → total
    if let Some(v) = total_from_per_90(player.key_headers_per_90, player.minutes) {
        player.key_headers = Some(v);
    }

    // === Goalkeeping per-90s ===
    player.clean_sheets_per_90 = per_90(player.clean_sheets as f64, player.minutes);
    player.goals_conceded_per_90 = per_90(player.goals_conceded as f64, player.minutes);
    player.xgp_per_90 = per_90(player.xgp, player.minutes);
    player.saves_held_per_90 = per_90(player.saves_held as f64, player.minutes);
    player.saves_parried_per_90 = per_90(player.saves_parried as f64, player.minutes);
    player.saves_tipped_per_90 = per_90(player.saves_tipped as f64, player.minutes);
    player.pens_faced_per_90 = per_90(player.pens_faced as f64, player.minutes);
    player.pens_saved_per_90 = per_90(player.pens_saved as f64, player.minutes);

    // Total saves: per-90 from CSV → total
    if let Some(v) = total_from_per_90(player.total_saves_per_90, player.minutes) {
        player.total_saves = Some(v);
    }

    // === Goalkeeping ratios ===
    let total_saves = player.saves_held + player.saves_parried + player.saves_tipped;
    player.save_ratio = ratio(
        total_saves as f64,
        player.goals_conceded as f64 + total_saves as f64,
    );
    player.pens_saved_ratio = ratio(player.pens_saved as f64, player.pens_faced as f64);

    // === Discipline per-90s ===
    player.fouls_made_per_90 = per_90(player.fouls_made as f64, player.minutes);
    player.fouls_against_per_90 = per_90(player.fouls_against as f64, player.minutes);
    player.yellow_cards_per_90 = per_90(player.yellow_cards as f64, player.minutes);
    player.red_cards_per_90 = per_90(player.red_cards as f64, player.minutes);
    player.offsides_per_90 = per_90(player.offsides as f64, player.minutes);
    player.mlg_per_90 = per_90(player.mlg as f64, player.minutes);

    // === Match impact ratios ===
    let total_games = player.games_won + player.games_drawn + player.games_lost;
    player.game_win_ratio = ratio(player.games_won as f64, total_games as f64);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_player() -> Player {
        Player {
            // Identity
            unique_id: 1,
            name: "Test Player".to_string(),
            nation: "England".to_string(),
            second_nation: None,
            club: "Test FC".to_string(),
            position: "ST".to_string(),
            // Physical
            age: 25,
            height_cm: 180,
            left_foot: crate::models::types::FootRating::Strong,
            right_foot: crate::models::types::FootRating::VeryStrong,
            // Ability
            current_ability: 140,
            potential_ability: 160,
            // Contract
            transfer_value: crate::models::types::TransferValue {
                min: 10_000_000,
                max: 15_000_000,
            },
            weekly_wage: crate::models::types::Wage {
                weekly_amount: 50_000.0,
                unit: crate::models::types::WageUnit::PerWeek,
            },
            contract_expires: None,
            // Availability
            appearances: crate::models::types::Appearances {
                total: 30,
                as_sub: 5,
            },
            minutes: 2700, // 30 games * 90 mins
            // Attacking
            goals: 20,
            goals_per_90: None,
            goals_outside_box: 5,
            goals_outside_box_per_90: None,
            xg: 18.0,
            xg_per_90: None,
            npxg: 15.0,
            npxg_per_90: None,
            xg_overperformance: 2.0,
            xg_overperformance_per_90: None,
            xg_per_shot: 0.12,
            shots: 150,
            shots_per_90: None,
            shots_outside_box: None,
            shots_outside_box_per_90: 0.5,
            shots_on_target: 60,
            shots_on_target_per_90: None,
            shots_on_target_ratio: None,
            conversion_ratio: None,
            pens_taken: 5,
            pens_taken_per_90: None,
            pens_scored: 4,
            pens_scored_per_90: None,
            pens_scored_ratio: None,
            free_kick_shots: 3,
            free_kick_shots_per_90: None,
            minutes_per_goal: None,
            minutes_per_goal_or_assist: None,
            // Creativity
            assists: 10,
            assists_per_90: None,
            xa: 8.0,
            xa_per_90: None,
            chances_created: None,
            chances_created_per_90: 2.0,
            clear_cut_chances: 15,
            clear_cut_chances_per_90: None,
            key_passes: 40,
            key_passes_per_90: None,
            op_key_passes: None,
            op_key_passes_per_90: 1.0,
            crosses_attempted: 50,
            crosses_attempted_per_90: None,
            crosses_completed: 20,
            crosses_completed_per_90: None,
            cross_completion_ratio: None,
            op_crosses_attempted: 30,
            op_crosses_attempted_per_90: None,
            op_crosses_completed: 10,
            op_crosses_completed_per_90: None,
            op_cross_completion_ratio: None,
            minutes_per_assist: None,
            // Transition
            passes_attempted: 500,
            passes_attempted_per_90: None,
            passes_completed: 400,
            passes_completed_per_90: None,
            pass_completion_ratio: None,
            progressive_passes: 100,
            progressive_passes_per_90: None,
            dribbles: 30,
            dribbles_per_90: None,
            distance_covered: 325,
            distance_covered_per_90: None,
            sprints: None,
            sprints_per_90: 8.0,
            poss_lost: None,
            poss_lost_per_90: 6.0,
            // Defensive
            tackles_attempted: 40,
            tackles_attempted_per_90: None,
            tackles_completed: 30,
            tackles_completed_per_90: None,
            tackle_completion_ratio: None,
            key_tackles: 5,
            key_tackles_per_90: None,
            interceptions: 20,
            interceptions_per_90: None,
            poss_won: None,
            poss_won_per_90: 4.0,
            pressures_attempted: 100,
            pressures_attempted_per_90: None,
            pressures_completed: 60,
            pressures_completed_per_90: None,
            pressure_success_ratio: None,
            blocks: 15,
            blocks_per_90: None,
            shots_blocked: 10,
            shots_blocked_per_90: None,
            clearances: 50,
            clearances_per_90: None,
            // Aerial
            headers_attempted: 30,
            headers_attempted_per_90: None,
            headers_won: 15,
            headers_won_per_90: None,
            headers_lost: None,
            headers_lost_per_90: 0.5,
            headers_won_ratio: None,
            key_headers: None,
            key_headers_per_90: 0.2,
            // Goalkeeping
            clean_sheets: 5,
            clean_sheets_per_90: None,
            goals_conceded: 30,
            goals_conceded_per_90: None,
            total_saves: None,
            total_saves_per_90: 3.0,
            save_ratio: None,
            xsv_percent: 0.7,
            xgp: 25.0,
            xgp_per_90: None,
            saves_held: 20,
            saves_held_per_90: None,
            saves_parried: 15,
            saves_parried_per_90: None,
            saves_tipped: 10,
            saves_tipped_per_90: None,
            pens_faced: 3,
            pens_faced_per_90: None,
            pens_saved: 1,
            pens_saved_per_90: None,
            pens_saved_ratio: None,
            // Discipline
            fouls_made: 25,
            fouls_made_per_90: None,
            fouls_against: 30,
            fouls_against_per_90: None,
            yellow_cards: 3,
            yellow_cards_per_90: None,
            red_cards: 0,
            red_cards_per_90: None,
            offsides: 20,
            offsides_per_90: None,
            mlg: 0,
            mlg_per_90: None,
            // Match impact
            rating: 7.2,
            pom: 2,
            game_win_ratio: None,
            games_won: 15,
            games_drawn: 8,
            games_lost: 7,
            team_goals: 50,
        }
    }

    #[test]
    fn test_goals_per_90() {
        let mut player = make_player();
        enrich(&mut player);
        // 20 goals in 2700 minutes = 20 * 90 / 30 = 0.6667
        assert!((player.goals_per_90.unwrap() - 0.6667).abs() < 0.001);
    }

    #[test]
    fn test_shots_outside_box_total() {
        let mut player = make_player();
        enrich(&mut player);
        // 0.5 per 90 * 30 games = 15 total
        assert!((player.shots_outside_box.unwrap() - 15.0).abs() < 0.01);
    }

    #[test]
    fn test_conversion_ratio() {
        let mut player = make_player();
        enrich(&mut player);
        // 20 goals / 150 shots = 0.133
        assert!((player.conversion_ratio.unwrap() - 0.1333).abs() < 0.001);
    }

    #[test]
    fn test_minutes_per_goal() {
        let mut player = make_player();
        enrich(&mut player);
        // 2700 / 20 = 135
        assert!((player.minutes_per_goal.unwrap() - 135.0).abs() < 0.01);
    }

    #[test]
    fn test_game_win_ratio() {
        let mut player = make_player();
        enrich(&mut player);
        // 15 / 30 = 0.5
        assert!((player.game_win_ratio.unwrap() - 0.5).abs() < 0.001);
    }
}
