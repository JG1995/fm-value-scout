pub mod archetypes;

use std::sync::Mutex;

use crate::models::csv_result::CsvResult;
use crate::models::percentile::PercentileCache;
use crate::models::player::Player;
use crate::parser;
use serde::{Deserialize, Serialize};

use crate::models::archetype::PlayerMetric;

// ---------------------------------------------------------------------------
// DataStore — holds parsed players and their percentile distributions
// ---------------------------------------------------------------------------

pub struct DataStore {
    pub inner: Mutex<DataStoreInner>,
}

pub struct DataStoreInner {
    pub players: Vec<Player>,
    pub percentile_cache: PercentileCache,
}

impl DataStore {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(DataStoreInner {
                players: Vec::new(),
                percentile_cache: PercentileCache::from_players(&[]),
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// Parse a CSV file, populate the DataStore, and return the result.
#[tauri::command]
pub fn parse_csv(state: tauri::State<DataStore>, path: &str) -> Result<CsvResult, String> {
    let result = parser::parse_file(path)?;

    let mut inner = state.inner.lock().map_err(|e| e.to_string())?;
    let cache = PercentileCache::from_players(&result.players);
    inner.players = result.players.clone();
    inner.percentile_cache = cache;

    Ok(result)
}

/// A single percentile query from the frontend.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PercentileQuery {
    pub metric: PlayerMetric,
    /// Optional position group (e.g. "DC", "GK"). `null` for all players.
    pub position_group: Option<String>,
    pub value: f64,
}

/// Result of a single percentile lookup.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PercentileResult {
    pub metric: PlayerMetric,
    pub position_group: Option<String>,
    pub value: f64,
    /// 0.0–1.0, or `null` when unavailable.
    pub percentile: Option<f64>,
}

/// Batch percentile lookup.
#[tauri::command]
pub fn get_percentiles(
    state: tauri::State<DataStore>,
    queries: Vec<PercentileQuery>,
) -> Result<Vec<PercentileResult>, String> {
    let inner = state.inner.lock().map_err(|e| e.to_string())?;
    let cache = &inner.percentile_cache;

    let results = queries
        .into_iter()
        .map(|q| {
            let percentile = cache.percentile(q.metric, q.position_group.as_deref(), q.value);
            PercentileResult {
                metric: q.metric,
                position_group: q.position_group,
                value: q.value,
                percentile,
            }
        })
        .collect();

    Ok(results)
}
