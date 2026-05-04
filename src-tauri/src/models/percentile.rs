use std::collections::HashMap;

use super::archetype::PlayerMetric;
use super::player::Player;

// ---------------------------------------------------------------------------
// PercentileCache — pre-computed sorted distributions per (metric, position)
// ---------------------------------------------------------------------------

/// Minimum number of players required in a position group for a meaningful
/// percentile. Below this, fall back to the all-players distribution.
const MIN_PLAYERS: usize = 5;

/// A pre-computed set of sorted value distributions, keyed by metric and
/// optional position group (`None` = all players, `Some("DC")` = DC only).
#[derive(Debug, Clone)]
pub struct PercentileCache {
    distributions: HashMap<(PlayerMetric, Option<String>), Vec<f64>>,
}

impl PercentileCache {
    // ------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------

    /// Build distributions from a player list: for every metric variant,
    /// collect all non-`None` values (all-players) and per-position subsets,
    /// sort each, and store.
    pub fn from_players(players: &[Player]) -> Self {
        let metrics = PlayerMetric::all_variants();
        let mut distributions: HashMap<(PlayerMetric, Option<String>), Vec<f64>> = HashMap::new();

        // --- collect raw values ---
        // We collect into Vecs first, deduplicate + sort later.
        let mut raw: HashMap<(PlayerMetric, Option<String>), Vec<f64>> = HashMap::new();

        for player in players {
            let position = player.positions.first().map(|s| s.to_string());

            for &metric in &metrics {
                if let Some(value) = metric.extract(player) {
                    if value.is_finite() {
                        // All-players
                        raw.entry((metric, None)).or_default().push(value);

                        // Position group
                        if let Some(ref pos) = position {
                            raw.entry((metric, Some(pos.clone())))
                                .or_default()
                                .push(value);
                        }
                    }
                }
            }
        }

        // --- sort all collected vectors ---
        for (key, mut values) in raw {
            values.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            distributions.insert(key, values);
        }

        Self { distributions }
    }

    // ------------------------------------------------------------------
    // Query
    // ------------------------------------------------------------------

    /// Return the percentile rank (0.0–1.0) of `value` within the
    /// distribution for `(metric, position_group)`.
    ///
    /// - `position_group: None` queries the all-players distribution.
    /// - `position_group: Some("DC")` queries the DC-only distribution,
    ///   falling back to all-players if the group has fewer than
    ///   `MIN_PLAYERS` entries.
    ///
    /// Returns `None` when no distribution exists (no data for that
    /// metric) or when the reference set is empty.
    pub fn percentile(
        &self,
        metric: PlayerMetric,
        position_group: Option<&str>,
        value: f64,
    ) -> Option<f64> {
        if !value.is_finite() {
            return None;
        }

        // Try position-group first
        if let Some(ref pos) = position_group {
            let key = (metric, Some(pos.to_string()));
            if let Some(dist) = self.distributions.get(&key) {
                if dist.len() >= MIN_PLAYERS {
                    return percentile_rank(dist, value);
                }
            }
            // Fall through to all-players
        }

        let key = (metric, None);
        let dist = self.distributions.get(&key)?;
        percentile_rank(dist, value)
    }

    /// Return the number of stored distributions.
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.distributions.len()
    }

    /// Total entries across all distributions (useful for validation).
    #[cfg(test)]
    pub fn total_entries(&self) -> usize {
        self.distributions.values().map(|v| v.len()).sum()
    }
}

// ---------------------------------------------------------------------------
// Percentile rank calculation
// ---------------------------------------------------------------------------

/// Compute the percentile rank of `value` inside a **sorted** slice.
///
/// Uses PERCENTRANK.INC semantics: `count_below / (N - 1)`, clamped to [0, 1].
/// Returns `None` when the slice has fewer than 2 entries.
fn percentile_rank(sorted: &[f64], value: f64) -> Option<f64> {
    if sorted.len() < 2 {
        return None;
    }

    // Count entries strictly below `value`
    let rank = sorted.partition_point(|&x| x < value);
    let percentile = rank as f64 / (sorted.len() as f64 - 1.0);
    Some(percentile.clamp(0.0, 1.0))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- helpers ---

    fn dc_player(id: u64, tackles: f64, interceptions: f64) -> Player {
        let mut p = Player::default();
        p.unique_id = id;
        p.positions = vec!["DC".into()];
        p.tackles_completed_per_90 = Some(tackles);
        p.interceptions_per_90 = Some(interceptions);
        p
    }

    fn gk_player(id: u64, saves: f64) -> Player {
        let mut p = Player::default();
        p.unique_id = id;
        p.positions = vec!["GK".into()];
        p.total_saves_per_90 = Some(saves);
        p
    }

    // --- construction ---

    #[test]
    fn empty_players_produces_empty_cache() {
        let cache = PercentileCache::from_players(&[]);
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn builds_distributions_for_all_metrics() {
        let players = vec![dc_player(1, 2.0, 1.0), dc_player(2, 3.0, 2.0)];
        let cache = PercentileCache::from_players(&players);
        // We should have at least the two metrics we set (all-players + DC)
        assert!(cache.len() > 0, "cache should have entries");
        assert!(cache.total_entries() > 0, "cache should have data points");
    }

    // --- percentile_rank ---

    #[test]
    fn percentile_rank_min_value_is_zero() {
        let sorted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(percentile_rank(&sorted, 1.0), Some(0.0));
    }

    #[test]
    fn percentile_rank_max_value_is_one() {
        let sorted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(percentile_rank(&sorted, 5.0), Some(1.0));
    }

    #[test]
    fn percentile_rank_mid_value() {
        let sorted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(percentile_rank(&sorted, 3.0), Some(0.5));
    }

    #[test]
    fn percentile_rank_ties_use_strictly_below() {
        // Two copies of 3.0: [1, 2, 3, 3, 4, 5]
        let sorted = vec![1.0, 2.0, 3.0, 3.0, 4.0, 5.0];
        // rank = count below 3.0 = 2, N-1 = 5
        assert_eq!(percentile_rank(&sorted, 3.0), Some(2.0 / 5.0));
    }

    #[test]
    fn percentile_rank_below_min() {
        let sorted = vec![1.0, 2.0, 3.0];
        assert_eq!(percentile_rank(&sorted, 0.5), Some(0.0));
    }

    #[test]
    fn percentile_rank_above_max() {
        let sorted = vec![1.0, 2.0, 3.0];
        assert_eq!(percentile_rank(&sorted, 10.0), Some(1.0));
    }

    #[test]
    fn percentile_rank_empty_returns_none() {
        assert_eq!(percentile_rank(&[], 1.0), None);
    }

    #[test]
    fn percentile_rank_single_element_returns_none() {
        assert_eq!(percentile_rank(&[1.0], 1.0), None);
    }

    // --- query ---

    #[test]
    fn position_group_query() {
        let players = vec![
            dc_player(1, 1.0, 0.0),
            dc_player(2, 2.0, 0.0),
            dc_player(3, 3.0, 0.0),
            dc_player(4, 4.0, 0.0),
            dc_player(5, 5.0, 0.0),
        ];
        let cache = PercentileCache::from_players(&players);

        // Medium value among DCs
        let p = cache.percentile(PlayerMetric::TacklesCompletedPer90, Some("DC"), 3.0);
        assert_eq!(p, Some(0.5));
    }

    #[test]
    fn all_players_query() {
        let players = vec![
            dc_player(1, 1.0, 0.0),
            dc_player(2, 2.0, 0.0),
            dc_player(3, 3.0, 0.0),
        ];
        let cache = PercentileCache::from_players(&players);

        let p = cache.percentile(PlayerMetric::TacklesCompletedPer90, None, 2.0);
        assert_eq!(p, Some(0.5));
    }

    #[test]
    fn fallback_when_position_group_too_small() {
        // Only 2 DCs — below MIN_PLAYERS (5). Should fall back to all-players.
        let players = vec![
            dc_player(1, 1.0, 0.0),
            dc_player(2, 5.0, 0.0),
            // Add some non-DCs to the all-players pool
            {
                let mut p = Player::default();
                p.unique_id = 3;
                p.positions = vec!["STC".into()];
                p.tackles_completed_per_90 = Some(2.0);
                p
            },
            {
                let mut p = Player::default();
                p.unique_id = 4;
                p.positions = vec!["STC".into()];
                p.tackles_completed_per_90 = Some(3.0);
                p
            },
            {
                let mut p = Player::default();
                p.unique_id = 5;
                p.positions = vec!["STC".into()];
                p.tackles_completed_per_90 = Some(4.0);
                p
            },
        ];
        let cache = PercentileCache::from_players(&players);

        // DC position-group has only 2 entries → should fall back to all-players (5 entries)
        let p = cache.percentile(PlayerMetric::TacklesCompletedPer90, Some("DC"), 3.0);
        // All-players: [1.0, 2.0, 3.0, 4.0, 5.0], value 3.0 → rank 2/4 = 0.5
        assert_eq!(p, Some(0.5));
    }

    #[test]
    fn missing_metric_returns_none() {
        let players = vec![dc_player(1, 1.0, 0.0)];
        let cache = PercentileCache::from_players(&players);

        // Metric that was never set (all players have None for it)
        let p = cache.percentile(PlayerMetric::CleanSheetsRatio, None, 0.5);
        assert_eq!(p, None);
    }

    #[test]
    fn nan_value_returns_none() {
        let players = vec![dc_player(1, 1.0, 0.0), dc_player(2, 2.0, 0.0)];
        let cache = PercentileCache::from_players(&players);

        let p = cache.percentile(PlayerMetric::TacklesCompletedPer90, None, f64::NAN);
        assert_eq!(p, None);
    }

    #[test]
    fn players_with_no_positions_contribute_to_all_players_only() {
        let mut no_pos = dc_player(1, 1.0, 0.0);
        no_pos.positions = vec![];

        let players = vec![no_pos, dc_player(2, 2.0, 0.0)];
        let cache = PercentileCache::from_players(&players);

        // All-players should have 2 entries: sorted [1.0, 2.0]
        // value 1.5 → partition_point returns 1 → 1/(2-1) = 1.0
        let p_all = cache.percentile(PlayerMetric::TacklesCompletedPer90, None, 1.5);
        assert_eq!(p_all, Some(1.0));
    }

    #[test]
    fn position_group_distribution_excludes_other_positions() {
        let players = vec![
            dc_player(1, 1.0, 0.0),
            dc_player(2, 2.0, 0.0),
            gk_player(3, 3.0),
            gk_player(4, 4.0),
        ];
        let cache = PercentileCache::from_players(&players);

        // DC query should only see DC values
        // (but 2 DCs < MIN_PLAYERS, falls back to all 4 players)
        // So let's make enough DCs
        let players = vec![
            dc_player(1, 1.0, 0.0),
            dc_player(2, 2.0, 0.0),
            dc_player(3, 3.0, 0.0),
            dc_player(4, 4.0, 0.0),
            dc_player(5, 5.0, 0.0),
            gk_player(6, 0.0),
            gk_player(7, 0.0),
        ];
        let cache = PercentileCache::from_players(&players);

        // DC distribution should be [1.0, 2.0, 3.0, 4.0, 5.0]
        let p = cache.percentile(PlayerMetric::TacklesCompletedPer90, Some("DC"), 3.0);
        assert_eq!(p, Some(0.5));

        // GK distribution for total saves should only include GK saves
        let p_gk = cache.percentile(PlayerMetric::TotalSavesPer90, Some("GK"), 0.0);
        // Both GKs have 0.0, so [0.0, 0.0] → percentile of 0.0 is rank 0 / 1 = 0.0
        assert_eq!(p_gk, Some(0.0));
    }
}
