use std::path::PathBuf;
use std::sync::Mutex;

use crate::models::archetype::{Archetype, MetricInfo, PlayerMetric};
use crate::data;

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
        archetype.id = Archetype::generate_id(&archetype.position, archetype.phase, &archetype.name);
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
    if let Some(pos) = inner.user_archetypes.iter_mut().find(|a| a.id == archetype.id) {
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


