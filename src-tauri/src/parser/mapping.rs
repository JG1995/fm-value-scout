use std::collections::HashMap;

pub type ColumnMap = HashMap<String, usize>;

/// Build a header-name → column-index map from CSV headers.
/// This allows parsing by column name rather than index, making
/// the parser resilient to column reordering.
pub fn build_column_map(headers: &csv::StringRecord) -> ColumnMap {
    headers
        .iter()
        .enumerate()
        .map(|(i, name)| (name.to_string(), i))
        .collect()
}
