use std::path::PathBuf;
use std::sync::Mutex;

use serde::Serialize;

use super::DataStore;
use crate::data;
use crate::models::archetype::{Archetype, MetricInfo, PlayerMetric};

// ---------------------------------------------------------------------------
// ArchetypeStore
// ---------------------------------------------------------------------------

/// Managed Tauri state holding both default and user-created archetypes.
pub struct ArchetypeStore {
    /// Default archetypes loaded from embedded JSON at startup.
    pub defaults: Vec<Archetype>,
    /// Mutable state for user archetypes.
    pub inner: Mutex<ArchetypeInner>,
}

pub struct ArchetypeInner {
    pub user_archetypes: Vec<Archetype>,
    pub user_file: PathBuf,
}

impl ArchetypeStore {
    /// Create a new store, loading defaults and existing user archetypes.
    pub fn new(app_data_dir: PathBuf) -> Self {
        let defaults = data::load_default_archetypes();

        let user_file = app_data_dir.join("user_archetypes.json");
        let user_archetypes = if user_file.exists() {
            std::fs::read_to_string(&user_file)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        Self {
            defaults,
            inner: Mutex::new(ArchetypeInner {
                user_archetypes,
                user_file,
            }),
        }
    }

    /// Persist user archetypes to disk.
    fn persist(&self) -> Result<(), String> {
        let inner = self.inner.lock().map_err(|e| e.to_string())?;
        let json =
            serde_json::to_string_pretty(&inner.user_archetypes).map_err(|e| e.to_string())?;
        if let Some(parent) = inner.user_file.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(&inner.user_file, json).map_err(|e| e.to_string())?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// Return all archetypes (defaults + user created), user ones at the end.
#[tauri::command]
pub fn list_archetypes(state: tauri::State<ArchetypeStore>) -> Result<Vec<Archetype>, String> {
    let inner = state.inner.lock().map_err(|e| e.to_string())?;
    let mut all = state.defaults.clone();
    all.extend(inner.user_archetypes.clone());
    Ok(all)
}

/// Return all available metrics with their display names.
#[tauri::command]
pub fn list_available_metrics() -> Vec<MetricInfo> {
    PlayerMetric::all_variants()
        .into_iter()
        .map(|m| MetricInfo {
            id: m,
            display_name: m.display_name().to_string(),
        })
        .collect()
}

/// Save (create or update) a user archetype.
/// Generates an ID if one is not set; sets `is_user_created = true`.
#[tauri::command]
pub fn save_user_archetype(
    state: tauri::State<ArchetypeStore>,
    mut archetype: Archetype,
) -> Result<Archetype, String> {
    // Enforce user-created flag
    archetype.is_user_created = true;

    // Generate ID if not set
    if archetype.id.is_empty() {
        archetype.id =
            Archetype::generate_id(&archetype.position, archetype.phase, &archetype.name);
    }

    // Basic validation
    if archetype.metrics.is_empty() {
        return Err("At least one metric is required".into());
    }
    if archetype.name.is_empty() {
        return Err("Archetype name is required".into());
    }
    if archetype.position.is_empty() {
        return Err("Archetype position is required".into());
    }
    // Validate weights are positive
    for metric in &archetype.metrics {
        if metric.weight <= 0.0 {
            return Err(format!(
                "Metric '{}' must have a positive weight",
                metric.metric.display_name()
            ));
        }
    }

    let mut inner = state.inner.lock().map_err(|e| e.to_string())?;

    // Update if existing, otherwise push
    if let Some(pos) = inner
        .user_archetypes
        .iter_mut()
        .find(|a| a.id == archetype.id)
    {
        *pos = archetype.clone();
    } else {
        inner.user_archetypes.push(archetype.clone());
    }

    // Persist
    let store_ref: &ArchetypeStore = &state;
    store_ref.persist()?;

    Ok(archetype)
}

/// Delete a user-created archetype by ID.
/// Returns an error if the archetype is a default (non-user-created) or not found.
#[tauri::command]
pub fn delete_user_archetype(
    state: tauri::State<ArchetypeStore>,
    id: String,
) -> Result<(), String> {
    let mut inner = state.inner.lock().map_err(|e| e.to_string())?;

    // Check it's not a default archetype
    if state.defaults.iter().any(|a| a.id == id) {
        return Err(format!(
            "Archetype '{}' is a built-in default and cannot be deleted",
            id
        ));
    }

    let len_before = inner.user_archetypes.len();
    inner.user_archetypes.retain(|a| a.id != id);

    if inner.user_archetypes.len() == len_before {
        return Err(format!("Archetype '{}' not found", id));
    }

    // Persist
    let store_ref: &ArchetypeStore = &state;
    store_ref.persist()?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Archetype scoring
// ---------------------------------------------------------------------------

/// Per-player archetype fit result.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchetypeFitScore {
    pub player_unique_id: u64,
    /// Archetype quality score (0.0 – 1.0), or null when no metrics available.
    pub quality: Option<f64>,
    /// Value-for-money: quality / normalized transfer cost. Higher = better value.
    /// Null when quality is null.
    pub value: Option<f64>,
}

/// Score every player in the DataStore against a given archetype.
/// Returns one result per player sorted by quality descending (nulls last).
#[tauri::command]
pub fn score_archetype_fit(
    archetype_store: tauri::State<ArchetypeStore>,
    data_store: tauri::State<DataStore>,
    archetype_id: String,
) -> Result<Vec<ArchetypeFitScore>, String> {
    // --- find the archetype ---
    let archetype = {
        let inner = archetype_store.inner.lock().map_err(|e| e.to_string())?;
        let all = archetype_store
            .defaults
            .iter()
            .chain(inner.user_archetypes.iter())
            .find(|a| a.id == archetype_id)
            .cloned()
            .ok_or_else(|| format!("Archetype '{}' not found", archetype_id))?;
        all
    };

    // --- access players and percentile cache ---
    let inner = data_store.inner.lock().map_err(|e| e.to_string())?;
    let players = &inner.players;
    let cache = &inner.percentile_cache;

    if players.is_empty() {
        return Ok(Vec::new());
    }

    // --- max transfer value for cost normalization ---
    let max_transfer = players
        .iter()
        .map(|p| p.transfer_value.max)
        .max()
        .unwrap_or(1) as f64;

    // --- score each player ---
    let mut scores: Vec<ArchetypeFitScore> = players
        .iter()
        .map(|player| {
            let position = player.positions.first().map(|s| s.as_str());

            let (total_weighted, total_weight) =
                archetype
                    .metrics
                    .iter()
                    .fold((0.0, 0.0), |(acc_w, acc_tw), m| {
                        if let Some(raw) = m.metric.extract(player) {
                            let pct = cache.percentile(m.metric, position, raw).unwrap_or(0.5);
                            let adj = if m.inverted {
                                (1.0 - pct) * m.weight
                            } else {
                                pct * m.weight
                            };
                            (acc_w + adj, acc_tw + m.weight)
                        } else {
                            (acc_w, acc_tw)
                        }
                    });

            let quality = if total_weight > 0.0 {
                Some(total_weighted / total_weight)
            } else {
                None
            };

            let value = quality.map(|q| {
                let cost_ratio =
                    (player.transfer_value.max as f64 / max_transfer.max(1.0)).max(0.01);
                q / cost_ratio
            });

            ArchetypeFitScore {
                player_unique_id: player.unique_id,
                quality,
                value,
            }
        })
        .collect();

    // --- sort: highest quality first, nulls last ---
    scores.sort_by(|a, b| match (a.quality, b.quality) {
        (Some(qa), Some(qb)) => qb.partial_cmp(&qa).unwrap_or(std::cmp::Ordering::Equal),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });

    Ok(scores)
}
