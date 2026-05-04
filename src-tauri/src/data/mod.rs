use crate::models::archetype::Archetype;

/// Load default archetypes from the embedded JSON file.
/// Returns an empty Vec if the file is `[]` (placeholder state).
pub fn load_default_archetypes() -> Vec<Archetype> {
    let json = include_str!("default_archetypes.json");
    serde_json::from_str(json).expect("default_archetypes.json is valid JSON")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_defaults_returns_empty() {
        let archetypes = load_default_archetypes();
        assert!(archetypes.is_empty());
    }
}
