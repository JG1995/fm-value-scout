use crate::models::player::Player;

/// Convert a total to per-90. Returns None if minutes == 0.
fn per_90(total: f64, minutes: u32) -> Option<f64> {
    (minutes > 0).then(|| total * 90.0 / minutes as f64)
}

/// Convert per-90 to total. Returns None if per_90 is None or minutes == 0.
fn total_from_per_90(per_90: Option<f64>, minutes: u32) -> Option<f64> {
    per_90.and_then(|v| (minutes > 0).then(|| v * minutes as f64 / 90.0))
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
        player.shots_outside_box = Some(v as u16);
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
        player.chances_created = Some(v as u16);
    }
    player.clear_cut_chances_per_90 = per_90(player.clear_cut_chances as f64, player.minutes);
    player.key_passes_per_90 = per_90(player.key_passes as f64, player.minutes);

    // OP key passes: per-90 from CSV → total
    if let Some(v) = total_from_per_90(player.op_key_passes_per_90, player.minutes) {
        player.op_key_passes = Some(v as u16);
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
        player.sprints = Some(v as u16);
    }

    // Poss lost: per-90 from CSV → total
    if let Some(v) = total_from_per_90(player.poss_lost_per_90, player.minutes) {
        player.poss_lost = Some(v as u16);
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
        player.poss_won = Some(v as u16);
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
        player.headers_lost = Some(v as u16);
    }

    // === Aerial ratios ===
    player.headers_won_ratio = ratio(player.headers_won as f64, player.headers_attempted as f64);

    // Key headers: per-90 from CSV → total
    if let Some(v) = total_from_per_90(player.key_headers_per_90, player.minutes) {
        player.key_headers = Some(v as u16);
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
        player.total_saves = Some(v as u16);
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
            shots_outside_box_per_90: Some(0.5),
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
            chances_created_per_90: Some(2.0),
            clear_cut_chances: 15,
            clear_cut_chances_per_90: None,
            key_passes: 40,
            key_passes_per_90: None,
            op_key_passes: None,
            op_key_passes_per_90: Some(1.0),
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
            sprints_per_90: Some(8.0),
            poss_lost: None,
            poss_lost_per_90: Some(6.0),
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
            poss_won_per_90: Some(4.0),
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
            headers_lost_per_90: Some(0.5),
            headers_won_ratio: None,
            key_headers: None,
            key_headers_per_90: Some(0.2),
            // Goalkeeping
            clean_sheets: 5,
            clean_sheets_per_90: None,
            goals_conceded: 30,
            goals_conceded_per_90: None,
            total_saves: None,
            total_saves_per_90: Some(3.0),
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
        assert!((player.shots_outside_box.unwrap() as f64 - 15.0).abs() < 0.01);
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

    // === Helper ===
    fn assert_approx(actual: Option<f64>, expected: f64, epsilon: f64) {
        assert!(actual.is_some(), "expected {} but got None", expected);
        assert!((actual.unwrap() - expected).abs() < epsilon);
    }

    // === Per-90: Attacking ===
    #[test]
    fn test_attacking_per_90s() {
        let mut player = make_player();
        enrich(&mut player);
        assert_approx(player.goals_per_90, 0.6667, 0.001); // 20 * 90 / 2700
        assert_approx(player.goals_outside_box_per_90, 0.1667, 0.001); // 5 * 90 / 2700
        assert_approx(player.xg_per_90, 0.6, 0.001); // 18.0 * 90 / 2700
        assert_approx(player.npxg_per_90, 0.5, 0.001); // 15.0 * 90 / 2700
        assert_approx(player.xg_overperformance_per_90, 0.0667, 0.001); // 2.0 * 90 / 2700
        assert_approx(player.shots_per_90, 5.0, 0.001); // 150 * 90 / 2700
        assert_approx(player.shots_on_target_per_90, 2.0, 0.001); // 60 * 90 / 2700
        assert_approx(player.pens_taken_per_90, 0.1667, 0.001); // 5 * 90 / 2700
        assert_approx(player.pens_scored_per_90, 0.1333, 0.001); // 4 * 90 / 2700
        assert_approx(player.free_kick_shots_per_90, 0.1, 0.001); // 3 * 90 / 2700
    }

    // === Per-90: Creativity ===
    #[test]
    fn test_creativity_per_90s() {
        let mut player = make_player();
        enrich(&mut player);
        assert_approx(player.assists_per_90, 0.3333, 0.001); // 10 * 90 / 2700
        assert_approx(player.xa_per_90, 0.2667, 0.001); // 8.0 * 90 / 2700
        assert_approx(player.clear_cut_chances_per_90, 0.5, 0.001); // 15 * 90 / 2700
        assert_approx(player.key_passes_per_90, 1.3333, 0.001); // 40 * 90 / 2700
        assert_approx(player.crosses_attempted_per_90, 1.6667, 0.001); // 50 * 90 / 2700
        assert_approx(player.crosses_completed_per_90, 0.6667, 0.001); // 20 * 90 / 2700
        assert_approx(player.op_crosses_attempted_per_90, 1.0, 0.001); // 30 * 90 / 2700
        assert_approx(player.op_crosses_completed_per_90, 0.3333, 0.001); // 10 * 90 / 2700
    }

    // === Per-90: Transition ===
    #[test]
    fn test_transition_per_90s() {
        let mut player = make_player();
        enrich(&mut player);
        assert_approx(player.passes_attempted_per_90, 16.6667, 0.001); // 500 * 90 / 2700
        assert_approx(player.passes_completed_per_90, 13.3333, 0.001); // 400 * 90 / 2700
        assert_approx(player.progressive_passes_per_90, 3.3333, 0.001); // 100 * 90 / 2700
        assert_approx(player.dribbles_per_90, 1.0, 0.001); // 30 * 90 / 2700
        assert_approx(player.distance_covered_per_90, 10.8333, 0.001); // 325 * 90 / 2700
    }

    // === Per-90: Defensive ===
    #[test]
    fn test_defensive_per_90s() {
        let mut player = make_player();
        enrich(&mut player);
        assert_approx(player.tackles_attempted_per_90, 1.3333, 0.001); // 40 * 90 / 2700
        assert_approx(player.tackles_completed_per_90, 1.0, 0.001); // 30 * 90 / 2700
        assert_approx(player.key_tackles_per_90, 0.1667, 0.001); // 5 * 90 / 2700
        assert_approx(player.interceptions_per_90, 0.6667, 0.001); // 20 * 90 / 2700
        assert_approx(player.pressures_attempted_per_90, 3.3333, 0.001); // 100 * 90 / 2700
        assert_approx(player.pressures_completed_per_90, 2.0, 0.001); // 60 * 90 / 2700
        assert_approx(player.blocks_per_90, 0.5, 0.001); // 15 * 90 / 2700
        assert_approx(player.shots_blocked_per_90, 0.3333, 0.001); // 10 * 90 / 2700
        assert_approx(player.clearances_per_90, 1.6667, 0.001); // 50 * 90 / 2700
    }

    // === Per-90: Aerial ===
    #[test]
    fn test_aerial_per_90s() {
        let mut player = make_player();
        enrich(&mut player);
        assert_approx(player.headers_attempted_per_90, 1.0, 0.001); // 30 * 90 / 2700
        assert_approx(player.headers_won_per_90, 0.5, 0.001); // 15 * 90 / 2700
    }

    // === Per-90: Goalkeeping ===
    #[test]
    fn test_goalkeeping_per_90s() {
        let mut player = make_player();
        enrich(&mut player);
        assert_approx(player.clean_sheets_per_90, 0.1667, 0.001); // 5 * 90 / 2700
        assert_approx(player.goals_conceded_per_90, 1.0, 0.001); // 30 * 90 / 2700
        assert_approx(player.xgp_per_90, 0.8333, 0.001); // 25.0 * 90 / 2700
        assert_approx(player.saves_held_per_90, 0.6667, 0.001); // 20 * 90 / 2700
        assert_approx(player.saves_parried_per_90, 0.5, 0.001); // 15 * 90 / 2700
        assert_approx(player.saves_tipped_per_90, 0.3333, 0.001); // 10 * 90 / 2700
        assert_approx(player.pens_faced_per_90, 0.1, 0.001); // 3 * 90 / 2700
        assert_approx(player.pens_saved_per_90, 0.0333, 0.001); // 1 * 90 / 2700
    }

    // === Per-90: Discipline ===
    #[test]
    fn test_discipline_per_90s() {
        let mut player = make_player();
        enrich(&mut player);
        assert_approx(player.fouls_made_per_90, 0.8333, 0.001); // 25 * 90 / 2700
        assert_approx(player.fouls_against_per_90, 1.0, 0.001); // 30 * 90 / 2700
        assert_approx(player.yellow_cards_per_90, 0.1, 0.001); // 3 * 90 / 2700
        assert_approx(player.red_cards_per_90, 0.0, 0.001); // 0 * 90 / 2700
        assert_approx(player.offsides_per_90, 0.6667, 0.001); // 20 * 90 / 2700
        assert_approx(player.mlg_per_90, 0.0, 0.001); // 0 * 90 / 2700
    }

    // === Reverse Calculations: totals from per-90 CSV fields ===
    #[test]
    fn test_reverse_calculations() {
        let mut player = make_player();
        enrich(&mut player);
        // shots_outside_box_per_90 * minutes / 90 = 0.5 * 30 = 15
        assert_approx(player.shots_outside_box.map(|v| v as f64), 15.0, 0.01);
        // chances_created_per_90 * minutes / 90 = 2.0 * 30 = 60
        assert_approx(player.chances_created.map(|v| v as f64), 60.0, 0.01);
        // op_key_passes_per_90 * minutes / 90 = 1.0 * 30 = 30
        assert_approx(player.op_key_passes.map(|v| v as f64), 30.0, 0.01);
        // sprints_per_90 * minutes / 90 = 8.0 * 30 = 240
        assert_approx(player.sprints.map(|v| v as f64), 240.0, 0.01);
        // poss_lost_per_90 * minutes / 90 = 6.0 * 30 = 180
        assert_approx(player.poss_lost.map(|v| v as f64), 180.0, 0.01);
        // poss_won_per_90 * minutes / 90 = 4.0 * 30 = 120
        assert_approx(player.poss_won.map(|v| v as f64), 120.0, 0.01);
        // headers_lost_per_90 * minutes / 90 = 0.5 * 30 = 15
        assert_approx(player.headers_lost.map(|v| v as f64), 15.0, 0.01);
        // key_headers_per_90 * minutes / 90 = 0.2 * 30 = 6
        assert_approx(player.key_headers.map(|v| v as f64), 6.0, 0.01);
        // total_saves_per_90 * minutes / 90 = 3.0 * 30 = 90
        assert_approx(player.total_saves.map(|v| v as f64), 90.0, 0.01);
    }

    // === Ratios: Attacking ===
    #[test]
    fn test_attacking_ratios() {
        let mut player = make_player();
        enrich(&mut player);
        assert_approx(player.shots_on_target_ratio, 0.4, 0.001); // 60 / 150
        assert_approx(player.conversion_ratio, 0.1333, 0.001); // 20 / 150
        assert_approx(player.pens_scored_ratio, 0.8, 0.001); // 4 / 5
    }

    // === Ratios: Creativity ===
    #[test]
    fn test_creativity_ratios() {
        let mut player = make_player();
        enrich(&mut player);
        assert_approx(player.cross_completion_ratio, 0.4, 0.001); // 20 / 50
        assert_approx(player.op_cross_completion_ratio, 0.3333, 0.001); // 10 / 30
    }

    // === Ratios: Transition + Defensive ===
    #[test]
    fn test_transition_defensive_ratios() {
        let mut player = make_player();
        enrich(&mut player);
        assert_approx(player.pass_completion_ratio, 0.8, 0.001); // 400 / 500
        assert_approx(player.tackle_completion_ratio, 0.75, 0.001); // 30 / 40
        assert_approx(player.pressure_success_ratio, 0.6, 0.001); // 60 / 100
    }

    // === Ratios: Aerial + Goalkeeping + Minutes ===
    #[test]
    fn test_aerial_goalkeeping_minutes_ratios() {
        let mut player = make_player();
        enrich(&mut player);
        assert_approx(player.headers_won_ratio, 0.5, 0.001); // 15 / 30
                                                             // save_ratio = total_saves / (goals_conceded + total_saves)
                                                             // 45 / (30 + 45) = 45 / 75 = 0.6
        assert_approx(player.save_ratio, 0.6, 0.001);
        assert_approx(player.pens_saved_ratio, 0.3333, 0.001); // 1 / 3
        assert_approx(player.minutes_per_goal, 135.0, 0.01); // 2700 / 20
        assert_approx(player.minutes_per_goal_or_assist, 90.0, 0.01); // 2700 / 30
        assert_approx(player.minutes_per_assist, 270.0, 0.01); // 2700 / 10
    }

    // === Edge Cases ===
    #[test]
    fn test_per_90_zero_minutes() {
        let mut player = make_player();
        player.minutes = 0;
        enrich(&mut player);
        // All per-90 fields should remain None when minutes == 0
        assert!(player.goals_per_90.is_none());
        assert!(player.shots_per_90.is_none());
        assert!(player.assists_per_90.is_none());
        assert!(player.tackles_attempted_per_90.is_none());
        assert!(player.goals_conceded_per_90.is_none());
        assert!(player.fouls_made_per_90.is_none());
    }

    #[test]
    fn test_ratio_zero_denominator() {
        // conversion_ratio: goals=0, shots=0 → None
        let mut player = make_player();
        player.goals = 0;
        player.shots = 0;
        enrich(&mut player);
        assert!(player.conversion_ratio.is_none());

        // minutes_per_goal: goals=0 → None
        let mut player = make_player();
        player.goals = 0;
        enrich(&mut player);
        assert!(player.minutes_per_goal.is_none());

        // pass_completion_ratio: passes_attempted=0 → None
        let mut player = make_player();
        player.passes_attempted = 0;
        enrich(&mut player);
        assert!(player.pass_completion_ratio.is_none());

        // pens_scored_ratio: pens_taken=0 → None
        let mut player = make_player();
        player.pens_taken = 0;
        enrich(&mut player);
        assert!(player.pens_scored_ratio.is_none());
    }

    #[test]
    fn test_save_ratio_division_by_zero() {
        // Both goals_conceded=0 and total_saves=0 → denominator=0 → None
        let mut player = make_player();
        player.goals_conceded = 0;
        player.saves_held = 0;
        player.saves_parried = 0;
        player.saves_tipped = 0;
        enrich(&mut player);
        assert!(player.save_ratio.is_none());
    }

    #[test]
    fn test_total_from_per_90_none_input() {
        // When per_90 source field is None, total should stay None
        let mut player = make_player();
        player.chances_created_per_90 = None;
        player.sprints_per_90 = None;
        enrich(&mut player);
        assert!(player.chances_created.is_none());
        assert!(player.sprints.is_none());
    }

    #[test]
    fn test_game_win_ratio_zero_games() {
        let mut player = make_player();
        player.games_won = 0;
        player.games_drawn = 0;
        player.games_lost = 0;
        enrich(&mut player);
        assert!(player.game_win_ratio.is_none());
    }
}
