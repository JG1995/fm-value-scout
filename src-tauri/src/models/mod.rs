//! Data models for the FM ValueScout application.
//!
//! ## Module structure
//!
//! - `player::Player` — 82-field player record
//! - `types::` — domain types: `FootRating`, `WageUnit`, `Appearances`, `TransferValue`, `Wage`
//! - `csv_result::CsvResult` — result of parsing a CSV file

pub mod csv_result;
pub mod parsing;
pub mod player;
pub mod types;

// Re-export the main types for convenience
pub use csv_result::CsvResult;
pub use player::Player;
pub use types::{Appearances, FootRating, TransferValue, Wage, WageUnit};

// Re-export parsing functions
pub use parsing::{parse_appearances, parse_date, parse_height, parse_transfer_range, parse_wage};
