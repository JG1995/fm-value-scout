use serde::{Deserialize, Serialize};

use crate::models::player::Player;

/// Result of parsing a CSV file.
/// Holds all players and the currency extracted from the data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvResult {
    pub players: Vec<Player>,
    pub currency: char,
}
