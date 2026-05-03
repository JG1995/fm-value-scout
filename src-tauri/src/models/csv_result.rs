use serde::{Deserialize, Serialize};

use crate::models::player::Player;

/// Result of parsing a CSV file.
/// Holds all players and the currency extracted from the data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvResult {
    pub players: Vec<Player>,
    pub currency: char,
}

impl CsvResult {
    pub fn new(currency: char) -> Self {
        Self {
            players: Vec::new(),
            currency,
        }
    }

    pub fn len(&self) -> usize {
        self.players.len()
    }

    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }
}
