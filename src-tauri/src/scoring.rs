use rusqlite::params;
use serde::{Deserialize, Serialize};

/// A single metric weight entry within an archetype's in/out-of-possession list.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArchetypeWeightEntry {
    /// Metric column name (must match season_players column names).
    pub metric_name: String,
    /// Weight value (higher = more important for this archetype).
    pub weight: f64,
    /// If true, lower values of this metric are better (not used for MVP attribute weights,
    /// but present for future real-stat integration).
    #[serde(default)]
    pub inverted: bool,
}

/// In/out-of-possession weight lists for an archetype.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArchetypeWeights {
    #[serde(default)]
    pub in_possession: Vec<ArchetypeWeightEntry>,
    #[serde(default)]
    pub out_of_possession: Vec<ArchetypeWeightEntry>,
}

/// A named role definition with weights for scoring computation.
#[derive(Clone, Debug)]
pub struct ArchetypeDefinition {
    /// Unique display name (e.g., "Ball-Playing Defender").
    pub name: &'static str,
    /// Canonical base position code (e.g., "DC", "ST", "DM", "MC", "AMC", "GK").
    pub base_position: &'static str,
    /// In/out-of-possession weight lists.
    pub weights: ArchetypeWeights,
}

/// 75/25 in/out-of-possession split — primary contribution weight.
pub const IN_POSSESSION_SPLIT: f64 = 0.75;
/// 25/75 out/in-of-possession split — secondary contribution weight.
pub const OUT_POSSESSION_SPLIT: f64 = 0.25;

/// Value multiplier for cheapest players (bottom 10th percentile).
pub const VALUE_MULTIPLIER_CHEAPEST: f64 = 1.5;
/// Value multiplier for most expensive players (top 10th percentile).
pub const VALUE_MULTIPLIER_MOST_EXPENSIVE: f64 = 0.5;
/// 10th percentile floor for value multiplier range.
pub const VALUE_PERCENTILE_FLOOR: f64 = 0.10;
/// 90th percentile ceiling for value multiplier range.
pub const VALUE_PERCENTILE_CEIL: f64 = 0.90;
/// Interpolation range between 10th and 90th percentile (0.80 = 0.90 - 0.10).
pub const VALUE_INTERPOLATION_RANGE: f64 = 0.80;

/// Computes the value multiplier based on a player's transfer-value percentile.
///
/// - Bottom 10% (percentile ≤ 0.10): returns `VALUE_MULTIPLIER_CHEAPEST` (1.5)
/// - Top 10% (percentile ≥ 0.90): returns `VALUE_MULTIPLIER_MOST_EXPENSIVE` (0.5)
/// - Between 10th and 90th percentile: linear interpolation between 1.5 and 0.5
pub fn compute_value_multiplier(value_percentile: f64) -> f64 {
    if value_percentile <= VALUE_PERCENTILE_FLOOR {
        VALUE_MULTIPLIER_CHEAPEST
    } else if value_percentile >= VALUE_PERCENTILE_CEIL {
        VALUE_MULTIPLIER_MOST_EXPENSIVE
    } else {
        // Linear interpolation: 1.5 - (value_percentile - 0.10) * (1.0 / 0.80)
        let t = (value_percentile - VALUE_PERCENTILE_FLOOR) / VALUE_INTERPOLATION_RANGE;
        VALUE_MULTIPLIER_CHEAPEST - t * (VALUE_MULTIPLIER_CHEAPEST - VALUE_MULTIPLIER_MOST_EXPENSIVE)
    }
}

/// Computes the value score for a player given their quality score and transfer-value percentile.
///
/// Value score = quality_score × value_multiplier
pub fn compute_value_score(quality_score: f64, value_percentile: f64) -> f64 {
    quality_score * compute_value_multiplier(value_percentile)
}

/// Returns the full list of seed archetypes.
pub fn get_seed_archetypes() -> Vec<ArchetypeDefinition> {
    vec![
        // ── Center Backs (DC) ──────────────────────────────────────────────
        ArchetypeDefinition {
            name: "Ball-Playing Defender",
            base_position: "DC",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "pas".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "fir".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "cmp".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "vis".into(), weight: 1.0, inverted: false },
                ],
                out_of_possession: vec![
                    ArchetypeWeightEntry { metric_name: "tck".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "hea".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "mar".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "ant".into(), weight: 1.0, inverted: false },
                ],
            },
        },
        ArchetypeDefinition {
            name: "No-Nonsense Centre Back",
            base_position: "DC",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "pas".into(), weight: 1.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "cmp".into(), weight: 1.0, inverted: false },
                ],
                out_of_possession: vec![
                    ArchetypeWeightEntry { metric_name: "tck".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "hea".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "mar".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "bra".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "pos".into(), weight: 2.0, inverted: false },
                ],
            },
        },
        ArchetypeDefinition {
            name: "Complete Centre Back",
            base_position: "DC",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "pas".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "fir".into(), weight: 1.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "cmp".into(), weight: 2.0, inverted: false },
                ],
                out_of_possession: vec![
                    ArchetypeWeightEntry { metric_name: "tck".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "hea".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "mar".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "ant".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "pos".into(), weight: 2.0, inverted: false },
                ],
            },
        },
        // ── Full Backs (DL/DR) ──────────────────────────────────────────────
        ArchetypeDefinition {
            name: "Complete Wing Back",
            base_position: "DL",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "cro".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "dri".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "pas".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "sta".into(), weight: 2.0, inverted: false },
                ],
                out_of_possession: vec![
                    ArchetypeWeightEntry { metric_name: "tck".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "pos".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "wor".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "ant".into(), weight: 1.0, inverted: false },
                ],
            },
        },
        ArchetypeDefinition {
            name: "Defensive Full Back",
            base_position: "DL",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "cro".into(), weight: 1.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "pas".into(), weight: 1.0, inverted: false },
                ],
                out_of_possession: vec![
                    ArchetypeWeightEntry { metric_name: "tck".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "pos".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "mar".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "ant".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "wor".into(), weight: 2.0, inverted: false },
                ],
            },
        },
        // ── Defensive Midfielders (DM) ──────────────────────────────────────
        ArchetypeDefinition {
            name: "Deep-Lying Playmaker",
            base_position: "DM",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "pas".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "vis".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "cmp".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "fir".into(), weight: 1.0, inverted: false },
                ],
                out_of_possession: vec![
                    ArchetypeWeightEntry { metric_name: "tck".into(), weight: 1.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "ant".into(), weight: 1.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "pos".into(), weight: 1.0, inverted: false },
                ],
            },
        },
        ArchetypeDefinition {
            name: "Ball-Winning Midfielder",
            base_position: "DM",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "pas".into(), weight: 1.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "vis".into(), weight: 1.0, inverted: false },
                ],
                out_of_possession: vec![
                    ArchetypeWeightEntry { metric_name: "tck".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "ant".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "wor".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "agg".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "pos".into(), weight: 2.0, inverted: false },
                ],
            },
        },
        ArchetypeDefinition {
            name: "Anchor Man",
            base_position: "DM",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "pas".into(), weight: 1.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "cmp".into(), weight: 1.0, inverted: false },
                ],
                out_of_possession: vec![
                    ArchetypeWeightEntry { metric_name: "tck".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "pos".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "ant".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "mar".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "cnt".into(), weight: 2.0, inverted: false },
                ],
            },
        },
        // ── Central Midfielders (MC) ───────────────────────────────────────
        ArchetypeDefinition {
            name: "Box-to-Box Midfielder",
            base_position: "MC",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "pas".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "tec".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "dri".into(), weight: 1.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "sta".into(), weight: 3.0, inverted: false },
                ],
                out_of_possession: vec![
                    ArchetypeWeightEntry { metric_name: "tck".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "wor".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "ant".into(), weight: 1.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "pos".into(), weight: 1.0, inverted: false },
                ],
            },
        },
        ArchetypeDefinition {
            name: "Advanced Playmaker",
            base_position: "MC",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "pas".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "vis".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "fir".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "dri".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "cmp".into(), weight: 2.0, inverted: false },
                ],
                out_of_possession: vec![
                    ArchetypeWeightEntry { metric_name: "tck".into(), weight: 0.5, inverted: false },
                    ArchetypeWeightEntry { metric_name: "ant".into(), weight: 0.5, inverted: false },
                ],
            },
        },
        ArchetypeDefinition {
            name: "Regista",
            base_position: "MC",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "pas".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "vis".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "fir".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "cmp".into(), weight: 2.0, inverted: false },
                ],
                out_of_possession: vec![
                    ArchetypeWeightEntry { metric_name: "ant".into(), weight: 1.0, inverted: false },
                ],
            },
        },
        // ── Attacking Midfielders (AMC) ────────────────────────────────────
        ArchetypeDefinition {
            name: "Enganche",
            base_position: "AMC",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "pas".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "vis".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "fir".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "cmp".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "dri".into(), weight: 1.0, inverted: false },
                ],
                out_of_possession: vec![],
            },
        },
        ArchetypeDefinition {
            name: "Shadow Striker",
            base_position: "AMC",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "tec".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "otb".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "dri".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "cmp".into(), weight: 1.0, inverted: false },
                ],
                out_of_possession: vec![],
            },
        },
        // ── Wingers (AML/AMR) ──────────────────────────────────────────────
        ArchetypeDefinition {
            name: "Winger",
            base_position: "AML",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "cro".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "dri".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "pac".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "acc".into(), weight: 2.0, inverted: false },
                ],
                out_of_possession: vec![
                    ArchetypeWeightEntry { metric_name: "tck".into(), weight: 0.5, inverted: false },
                    ArchetypeWeightEntry { metric_name: "wor".into(), weight: 1.0, inverted: false },
                ],
            },
        },
        ArchetypeDefinition {
            name: "Inside Forward",
            base_position: "AML",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "tec".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "dri".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "fin".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "otb".into(), weight: 2.0, inverted: false },
                ],
                out_of_possession: vec![],
            },
        },
        // ── Strikers (ST) ───────────────────────────────────────────────────
        ArchetypeDefinition {
            name: "Complete Forward",
            base_position: "ST",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "fin".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "cmp".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "otb".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "fir".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "hea".into(), weight: 2.0, inverted: false },
                ],
                out_of_possession: vec![],
            },
        },
        ArchetypeDefinition {
            name: "Advanced Forward",
            base_position: "ST",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "fin".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "cmp".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "otb".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "acc".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "pac".into(), weight: 2.0, inverted: false },
                ],
                out_of_possession: vec![],
            },
        },
        ArchetypeDefinition {
            name: "Poacher",
            base_position: "ST",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "fin".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "cmp".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "otb".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "ant".into(), weight: 2.0, inverted: false },
                ],
                out_of_possession: vec![],
            },
        },
        ArchetypeDefinition {
            name: "Target Man",
            base_position: "ST",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "hea".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "str".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "fir".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "otb".into(), weight: 2.0, inverted: false },
                ],
                out_of_possession: vec![],
            },
        },
        // ── Goalkeepers (GK) ────────────────────────────────────────────────
        ArchetypeDefinition {
            name: "Sweeper Keeper",
            base_position: "GK",
            weights: ArchetypeWeights {
                in_possession: vec![
                    ArchetypeWeightEntry { metric_name: "kic".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "rus".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "fir".into(), weight: 1.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "pas".into(), weight: 2.0, inverted: false },
                ],
                out_of_possession: vec![
                    ArchetypeWeightEntry { metric_name: "han".into(), weight: 3.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "ref_gk".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "cmd".into(), weight: 2.0, inverted: false },
                    ArchetypeWeightEntry { metric_name: "pos".into(), weight: 2.0, inverted: false },
                ],
            },
        },
    ]
}

/// All metric columns for which to compute percentile ranks.
const PERCENTILE_METRICS: &[&str] = &[
    // Technical (INTEGER)
    "cor", "cro", "dri", "fin", "fir", "fre", "hea", "lon", "lon_thr", "mar", "pas", "pen", "tck",
    "tec",
    // Mental (INTEGER)
    "agg", "ant", "bra", "cmp", "cnt", "dec", "det", "fla", "inf", "lea", "otb", "pos", "tea", "vis",
    "wor",
    // Physical (INTEGER)
    "acc", "agi", "bal", "jum", "nat_fit", "pac", "sta", "str",
    // Goalkeeper (INTEGER)
    "aer", "com_gk", "ecc", "han", "kic", "one_on_one", "pun", "ref_gk", "rus", "thr", "cmd",
    // Attacking (REAL)
    "goals_xg", "npxg", "xg_op", "xg_per_shot", "shots", "shots_on_target", "sot_pct", "conv_pct",
    "pens_scored", "pens_taken",
    // Creativity (REAL)
    "assists", "xa", "key_passes", "chances_created", "op_key_passes", "through_balls",
    // Transition (REAL)
    "dribbles_per_game", "progressive_passes", "progressive_runs", "passes_completed",
    "passes_attempted", "pass_completion_pct", "crosses_attempted", "crosses_completed",
    "cross_completion_pct",
    // Defensive (REAL)
    "tackles_per_game", "tackles_completed", "tackle_completion_pct", "interceptions_per_game",
    "clearances", "blocks", "possession_won", "possession_lost",
    // Aerial (REAL)
    "headers_won", "headers_lost", "headers_won_pct",
    // Goalkeeping Stats (REAL)
    "saves", "save_pct", "goals_conceded", "clean_sheets", "penalties_saved", "expected_saves",
    // Discipline (REAL)
    "fouls_made", "fouls_against", "yellow_cards", "red_cards", "offsides",
    // Match Impact (REAL)
    "distance_covered", "average_rating", "player_of_match",
    // Transfer (REAL)
    "transfer_value",
];

/// Computes percentile ranks for all players in a given season and stores them in the
/// `player_percentiles` table. Runs two INSERT...SELECT statements per metric — one for the
/// "position" pool (ranked within canonical position group) and one for the "all" pool (ranked
/// across all players). Uses `INSERT OR REPLACE` so re-computation is idempotent. Returns the total
/// number of rows inserted.
pub fn compute_percentiles(conn: &mut rusqlite::Connection, season_id: i64) -> rusqlite::Result<usize> {
    let tx = conn.transaction()?;
    let mut total_rows = 0;

    for metric in PERCENTILE_METRICS {
        // Position pool: rank within canonical_position partition
        let sql_position = format!(
            "INSERT OR REPLACE INTO player_percentiles
               (season_id, uid, metric_name, pool_type, canonical_position, percent_rank)
            SELECT
                sp.season_id,
                sp.uid,
                ?,
                'position',
                spp.canonical_position,
                PERCENT_RANK() OVER (
                    PARTITION BY spp.canonical_position
                    ORDER BY sp.{metric}
                )
            FROM season_players sp
            JOIN season_player_positions spp
              ON sp.season_id = spp.season_id AND sp.uid = spp.uid
            WHERE sp.season_id = ? AND sp.{metric} IS NOT NULL",
            metric = metric
        );
        let rows = tx.execute(&sql_position, params![metric, season_id])?;
        total_rows += rows;

        // All pool: rank across all players
        let sql_all = format!(
            "INSERT OR REPLACE INTO player_percentiles
               (season_id, uid, metric_name, pool_type, canonical_position, percent_rank)
            SELECT
                sp.season_id,
                sp.uid,
                ?,
                'all',
                spp.canonical_position,
                PERCENT_RANK() OVER (ORDER BY sp.{metric})
            FROM season_players sp
            JOIN season_player_positions spp
              ON sp.season_id = spp.season_id AND sp.uid = spp.uid
            WHERE sp.season_id = ? AND sp.{metric} IS NOT NULL",
            metric = metric
        );
        let rows = tx.execute(&sql_all, params![metric, season_id])?;
        total_rows += rows;
    }

    tx.commit()?;
    Ok(total_rows)
}

/// Compute the quality score for a single player-archetype combination.
///
/// Returns the quality score (0.0 - 100.0) or None if the player doesn't have
/// percentile data for any of the archetype's required metrics.
pub fn compute_quality_score(
    archetype_weights: &ArchetypeWeights,
    player_percentiles: &std::collections::HashMap<String, f64>,
    _pool_type: &str,
) -> Option<f64> {
    // Process in_possession entries
    let mut in_raw_score = 0.0;
    let mut in_max_score = 0.0;
    let mut in_found_any = false;

    for entry in &archetype_weights.in_possession {
        let max_contrib = entry.weight * 1.0;
        in_max_score += max_contrib;

        if let Some(&percentile) = player_percentiles.get(&entry.metric_name) {
            in_found_any = true;
            let value = if entry.inverted { 1.0 - percentile } else { percentile };
            in_raw_score += entry.weight * value;
        }
    }

    // Process out_of_possession entries
    let mut out_raw_score = 0.0;
    let mut out_max_score = 0.0;
    let mut out_found_any = false;

    for entry in &archetype_weights.out_of_possession {
        let max_contrib = entry.weight * 1.0;
        out_max_score += max_contrib;

        if let Some(&percentile) = player_percentiles.get(&entry.metric_name) {
            out_found_any = true;
            let value = if entry.inverted { 1.0 - percentile } else { percentile };
            out_raw_score += entry.weight * value;
        }
    }

    // If no percentiles were found at all, return None
    if !in_found_any && !out_found_any {
        return None;
    }

    // Normalize each sub-score independently to [0.0, 1.0]
    let in_norm = if in_max_score > 0.0 { in_raw_score / in_max_score } else { 0.0 };
    let out_norm = if out_max_score > 0.0 { out_raw_score / out_max_score } else { 0.0 };

    // Apply possession split weights
    let quality_score = if in_max_score > 0.0 && out_max_score > 0.0 {
        // Both present: true 75/25 split
        (in_norm * IN_POSSESSION_SPLIT + out_norm * OUT_POSSESSION_SPLIT) * 100.0
    } else if in_max_score > 0.0 {
        // Only in-possession: full range
        in_norm * 100.0
    } else if out_max_score > 0.0 {
        // Only out-of-possession: full range
        out_norm * 100.0
    } else {
        return None;
    };

    // Clamp to [0.0, 100.0] to handle floating point edge cases
    Some(quality_score.clamp(0.0, 100.0))
}

/// Run the full scouting pipeline for the most recent season:
/// 1. Compute percentiles
/// 2. Compute archetype scores
pub fn run_scouting_pipeline(conn: &mut rusqlite::Connection) -> rusqlite::Result<(usize, usize)> {
    // Get most recent season
    let season_id: i64 = {
        let mut stmt = conn.prepare("SELECT id FROM seasons ORDER BY import_date DESC LIMIT 1")?;
        stmt.query_row([], |row| row.get(0))?
    };

    let pct_rows = compute_percentiles(conn, season_id)?;
    let score_rows = compute_archetype_scores(conn, season_id)?;
    Ok((pct_rows, score_rows))
}

/// Pre-compute quality and value scores for all players in a season for all archetypes.
pub fn compute_archetype_scores(
    conn: &mut rusqlite::Connection,
    season_id: i64,
) -> rusqlite::Result<usize> {
    let tx = conn.transaction()?;
    let mut total_rows = 0;

    // Wrap in block so all prepared statements are dropped before commit
    let total_rows = {
        // Query all archetypes
        let mut arch_stmt = tx.prepare("SELECT id, metric_weights_json FROM archetypes")?;
        let archetype_rows = arch_stmt.query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?;

        for arch_result in archetype_rows {
            let (archetype_id, weights_json) = arch_result?;

            // Deserialize metric weights
            let weights: ArchetypeWeights = serde_json::from_str(&weights_json)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

            // Get base_position for this archetype
            let base_pos: String = tx.query_row(
                "SELECT base_position FROM archetypes WHERE id = ?1",
                params![archetype_id],
                |row| row.get(0),
            )?;

            // Query all players in this season with this archetype's base_position AND minutes >= 1000
            let mut player_stmt = tx.prepare(
                "SELECT DISTINCT sp.uid, spp.canonical_position
                 FROM season_players sp
                 JOIN season_player_positions spp ON sp.season_id = spp.season_id AND sp.uid = spp.uid
                 WHERE sp.season_id = ?1 AND spp.canonical_position = ?2 AND sp.minutes >= 1000",
            )?;

            let player_rows = player_stmt.query_map(params![season_id, &base_pos], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;

            for player_result in player_rows {
                let (uid, _canonical_position) = player_result?;

                // Query player percentiles for "all" pool type
                let mut pct_stmt = tx.prepare(
                    "SELECT metric_name, percent_rank FROM player_percentiles
                     WHERE season_id = ?1 AND uid = ?2 AND pool_type = 'all'",
                )?;

                let mut percentiles = std::collections::HashMap::new();
                let pct_rows = pct_stmt.query_map(params![season_id, &uid], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
                })?;

                for pct_result in pct_rows {
                    let (metric_name, percent_rank) = pct_result?;
                    percentiles.insert(metric_name, percent_rank);
                }
                drop(pct_stmt);

                // Compute quality score
                let quality_score = match compute_quality_score(&weights, &percentiles, "all") {
                    Some(score) => score,
                    None => continue,
                };

                let value_percentile = percentiles.get("transfer_value").copied().unwrap_or(0.0);

                let value_score = compute_value_score(quality_score, value_percentile);

                // INSERT OR REPLACE into archetype_scores
                tx.execute(
                    "INSERT OR REPLACE INTO archetype_scores
                     (season_id, uid, archetype_id, quality_score, value_score)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![season_id, &uid, archetype_id, quality_score, value_score],
                )?;
                total_rows += 1;
            }
        }

        total_rows
    };

    tx.commit()?;
    Ok(total_rows)
}