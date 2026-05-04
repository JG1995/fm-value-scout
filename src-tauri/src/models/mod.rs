//! Data models for the FM ValueScout application.
//!
//! ## Module structure
//!
//! - `player::Player` — 82-field player record
//! - `types::` — domain types: `FootRating`, `WageUnit`, `Appearances`, `TransferValue`, `Wage`
//! - `csv_result::CsvResult` — result of parsing a CSV file

pub mod archetype;
pub mod csv_result;
pub mod player;
pub mod types;
