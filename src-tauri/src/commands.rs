use crate::models::csv_result::CsvResult;
use crate::parser;

#[tauri::command]
pub fn parse_csv(path: &str) -> Result<CsvResult, String> {
    parser::parse_file(path)
}
