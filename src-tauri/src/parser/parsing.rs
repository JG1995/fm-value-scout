use crate::models::types::{Appearances, WageUnit};
use chrono::NaiveDate;

/// Parse appearances string into total and substitute counts.
///
/// Examples:
/// - "51" → Appearances { total: 51, as_sub: 0 }
/// - "46 (9)" → Appearances { total: 46, as_sub: 9 }
/// - "9 (28)" → Appearances { total: 9, as_sub: 28 }
pub fn parse_appearances(s: &str) -> Appearances {
    let s = s.trim();

    if let Some(paren_start) = s.find('(') {
        let total = s[..paren_start].trim().parse().unwrap_or(0);
        let rest = &s[paren_start + 1..];
        let as_sub = rest.trim_end_matches(')').trim().parse().unwrap_or(0);
        Appearances { total, as_sub }
    } else {
        Appearances {
            total: s.parse().unwrap_or(0),
            as_sub: 0,
        }
    }
}

/// Parse height string into centimeters.
///
/// Examples:
/// - "199 cm" → 199
/// - "195 cm" → 195
pub fn parse_height(s: &str) -> u16 {
    s.trim()
        .split_whitespace()
        .next()
        .and_then(|n| n.parse().ok())
        .unwrap_or(0)
}

/// Parse transfer value range from CSV.
/// Returns (TransferValue, currency) tuple.
///
/// Examples:
/// - "€62M - €94M" → (TransferValue { min: 62_000_000, max: 94_000_000 }, '€')
/// - "£325K - £7M" → (TransferValue { min: 325_000, max: 7_000_000 }, '£')
pub fn parse_transfer_range(s: &str) -> (crate::models::types::TransferValue, char) {
    let s = s.trim();

    // Detect currency: first non-space char, but default to '€' if it's a digit
    let first_char = s.chars().next().unwrap_or('€');
    let currency = if first_char.is_ascii_digit() {
        '€' // Bare number with no currency symbol, assume euros
    } else {
        first_char
    };

    let remainder = s.trim_start_matches(|c: char| c == '€' || c == '£' || c.is_whitespace());

    // Helper to parse value with K/M suffix
    let parse_value = |val_str: &str| -> u64 {
        let val_str = val_str.trim();
        let num_str =
            val_str.trim_end_matches(|c: char| c == 'K' || c == 'M' || c == 'k' || c == 'm');
        let multiplier = if val_str.ends_with('M') || val_str.ends_with('m') {
            1_000_000
        } else if val_str.ends_with('K') || val_str.ends_with('k') {
            1_000
        } else {
            1
        };
        num_str
            .parse::<f64>()
            .map(|n| (n * multiplier as f64) as u64)
            .unwrap_or(0)
    };

    // Helper to strip currency prefix and parse value
    let parse_with_currency = |val_str: &str| -> u64 {
        let stripped = val_str
            .trim_start_matches(|c: char| c == '€' || c == '£' || c.is_whitespace())
            .trim();
        parse_value(stripped)
    };

    if let Some(dash_idx) = remainder.find('-') {
        let (left, right) = remainder.split_at(dash_idx);
        let min = parse_with_currency(left);
        let max = parse_with_currency(right.trim_start_matches('-'));
        (crate::models::types::TransferValue { min, max }, currency)
    } else {
        let val = parse_with_currency(remainder);
        (
            crate::models::types::TransferValue { min: val, max: val },
            currency,
        )
    }
}

/// Parse wage from CSV.
/// Returns (Wage, currency) tuple. Amount is normalized to weekly.
///
/// Examples:
/// - "€74K p/w" → (Wage { weekly_amount: 74_000, PerWeek }, '€')
/// - "€17.25K p/m" → (Wage { weekly_amount: 3_984, PerMonth }, '€') — normalized to weekly
/// - "€1M p/a" → (Wage { weekly_amount: 19_231, PerAnnum }, '€') — normalized to weekly
pub fn parse_wage(s: &str) -> (crate::models::types::Wage, char) {
    let s = s.trim();
    let currency = s.chars().next().unwrap_or('€');
    let remainder = s.trim_start_matches(|c: char| c == '€' || c == '£' || c.is_whitespace());

    // Determine unit by looking for p/w, p/m, or p/a suffix (case-insensitive)
    // The suffix comes after the number, so we need to find it in the remainder
    let remainder_lower = remainder.to_lowercase();
    let unit = if remainder_lower.contains(" p/a") || remainder_lower.contains("/a") {
        WageUnit::PerAnnum
    } else if remainder_lower.contains(" p/m") || remainder_lower.contains("/m") {
        WageUnit::PerMonth
    } else {
        // Default to per week
        WageUnit::PerWeek
    };

    // Strip the unit suffix
    let num_str = remainder
        .trim_end_matches("p/w")
        .trim_end_matches("P/W")
        .trim_end_matches("p/m")
        .trim_end_matches("P/M")
        .trim_end_matches("p/a")
        .trim_end_matches("P/A")
        .trim_end_matches("/w")
        .trim_end_matches("/m")
        .trim_end_matches("/a")
        .trim();

    let multiplier = if num_str.ends_with('K') || num_str.ends_with('k') {
        1_000.0
    } else if num_str.ends_with('M') || num_str.ends_with('m') {
        1_000_000.0
    } else {
        1.0
    };

    let clean_str =
        num_str.trim_end_matches(|c: char| c == 'K' || c == 'k' || c == 'M' || c == 'm');

    let raw_amount = clean_str
        .parse::<f64>()
        .map(|n| n * multiplier)
        .unwrap_or(0.0);
    let weekly_amount = unit.to_weekly(raw_amount);

    (
        crate::models::types::Wage {
            weekly_amount,
            unit,
        },
        currency,
    )
}

/// Parse date string in DD/M/YYYY or D/MM/YYYY format.
///
/// Examples:
/// - "30/6/2028" → June 30, 2028
/// - "1/6/2028" → June 1, 2028
pub fn parse_date(s: &str) -> Option<NaiveDate> {
    let s = s.trim();
    let parts: Vec<&str> = s.split('/').collect();

    if parts.len() == 3 {
        let day = parts[0].parse::<u32>().ok()?;
        let month = parts[1].parse::<u32>().ok()?;
        let year = parts[2].parse::<i32>().ok()?;
        NaiveDate::from_ymd_opt(year, month, day)
    } else {
        None
    }
}

/// Parse distance string into u32 (strips "km", truncates decimal).
///
/// Examples:
/// - "312.7km" → 312
/// - "250km" → 250
pub fn parse_distance(s: &str) -> u32 {
    let s = s.trim().to_lowercase();
    let num_str = s.trim_end_matches("km").trim();
    num_str
        .split_whitespace()
        .next()
        .and_then(|n| n.parse::<f64>().ok())
        .map(|n| n as u32)
        .unwrap_or(0)
}

/// Parse an FM position string into granular position codes.
///
/// Handles multi-segment strings like `"D (LC), WB (L)"`,
/// position groups separated by `/`, and deduplication.
///
/// # Examples
///
/// ```ignore
/// // "AM (RC), ST (C)" → ["AMR", "AMC", "STC"]
/// // "D (LC), WB (L)" → ["DL", "DC", "WBL"]
/// // "GK" → ["GK"]
/// ```
pub fn parse_positions(raw: &str) -> Vec<String> {
    if raw.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::new();
    for segment in raw.split(',') {
        let segment = segment.trim();
        if segment.is_empty() {
            continue;
        }
        if let Some(paren_idx) = segment.find('(') {
            let groups_part = segment[..paren_idx].trim();
            let after_paren = &segment[paren_idx + 1..];
            let qualifiers = after_paren.trim_end_matches(')').trim();
            for group in groups_part.split('/') {
                let group = group.trim();
                for q in qualifiers.chars() {
                    result.push(format!("{}{}", group, q));
                }
            }
        } else {
            result.push(segment.to_string());
        }
    }
    // Deduplicate while preserving order
    let mut seen = std::collections::HashSet::new();
    result.retain(|p| seen.insert(p.clone()));
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::types::{FootRating, TransferValue, WageUnit};
    use chrono::Datelike;

    #[test]
    fn test_parse_appearances_with_subs() {
        let result = parse_appearances("46 (9)");
        assert_eq!(result.total, 46);
        assert_eq!(result.as_sub, 9);
        assert_eq!(result.starts(), 37);
    }

    #[test]
    fn test_parse_appearances_no_subs() {
        let result = parse_appearances("51");
        assert_eq!(result.total, 51);
        assert_eq!(result.as_sub, 0);
        assert_eq!(result.starts(), 51);
    }

    #[test]
    fn test_parse_transfer_millions() {
        let (result, currency) = parse_transfer_range("€62M - €94M");
        assert_eq!(result.min, 62_000_000);
        assert_eq!(result.max, 94_000_000);
        assert_eq!(currency, '€');
        assert!((result.midpoint() - 78_000_000.0).abs() < 0.01);
    }

    #[test]
    fn test_parse_transfer_thousands() {
        let (result, currency) = parse_transfer_range("€325K - €7M");
        assert_eq!(result.min, 325_000);
        assert_eq!(result.max, 7_000_000);
        assert_eq!(currency, '€');
    }

    #[test]
    fn test_parse_transfer_pound() {
        let (result, currency) = parse_transfer_range("£10M - £15M");
        assert_eq!(result.min, 10_000_000);
        assert_eq!(result.max, 15_000_000);
        assert_eq!(currency, '£');
    }

    #[test]
    fn test_parse_transfer_zero() {
        // Zero value with currency
        let (result, currency) = parse_transfer_range("€0");
        assert_eq!(result.min, 0);
        assert_eq!(result.max, 0);
        assert_eq!(currency, '€');

        // Bare zero with no currency symbol
        let (result, currency) = parse_transfer_range("0");
        assert_eq!(result.min, 0);
        assert_eq!(result.max, 0);
        assert_eq!(currency, '€'); // Defaults to euro
    }

    #[test]
    fn test_parse_wage_weekly() {
        let (result, currency) = parse_wage("€74K p/w");
        assert_eq!(result.weekly_amount, 74_000.0);
        assert_eq!(result.unit, WageUnit::PerWeek);
        assert_eq!(currency, '€');
    }

    #[test]
    fn test_parse_wage_decimal() {
        let (result, currency) = parse_wage("€17.25K p/w");
        assert_eq!(result.weekly_amount, 17_250.0);
        assert_eq!(result.unit, WageUnit::PerWeek);
        assert_eq!(currency, '€');
    }

    #[test]
    fn test_parse_wage_monthly() {
        let (result, currency) = parse_wage("€100K p/m");
        assert_eq!(result.unit, WageUnit::PerMonth);
        // 100,000 / 4.33 ≈ 23,094
        assert!((result.weekly_amount - 23_094.0).abs() < 1.0);
        assert_eq!(currency, '€');
    }

    #[test]
    fn test_parse_wage_annum() {
        let (result, currency) = parse_wage("€1M p/a");
        assert_eq!(result.unit, WageUnit::PerAnnum);
        // 1,000,000 / 52 ≈ 19,231
        assert!((result.weekly_amount - 19_230.77).abs() < 2.0);
        assert_eq!(currency, '€');
    }

    #[test]
    fn test_parse_height() {
        assert_eq!(parse_height("199 cm"), 199);
        assert_eq!(parse_height("195 cm"), 195);
    }

    #[test]
    fn test_parse_date() {
        let date = parse_date("30/6/2028").unwrap();
        assert_eq!(date.day(), 30);
        assert_eq!(date.month(), 6);
        assert_eq!(date.year(), 2028);
    }

    #[test]
    fn test_foot_rating_ordering() {
        use FootRating::*;
        assert!(VeryStrong > Strong);
        assert!(Strong > FairlyStrong);
        assert!(FairlyStrong > Reasonable);
        assert!(Reasonable > Weak);
    }

    #[test]
    fn test_parse_distance() {
        assert_eq!(parse_distance("312.7km"), 312);
        assert_eq!(parse_distance("250km"), 250);
        assert_eq!(parse_distance(""), 0);
    }
}

#[test]
fn test_parse_positions() {
    // From the format table
    assert_eq!(parse_positions("AM (C), ST (C)"), vec!["AMC", "STC"]);
    assert_eq!(
        parse_positions("D (L), M/AM(C), ST (C)"),
        vec!["DL", "MC", "AMC", "STC"]
    );
    assert_eq!(parse_positions("D (LC), WB (L)"), vec!["DL", "DC", "WBL"]);
    assert_eq!(
        parse_positions("AM (RC), ST (C)"),
        vec!["AMR", "AMC", "STC"]
    );
    assert_eq!(parse_positions("AM (RLC)"), vec!["AMR", "AML", "AMC"]);
    assert_eq!(parse_positions("GK"), vec!["GK"]);

    // Edge cases
    assert!(parse_positions("").is_empty());
    assert_eq!(parse_positions("  GK  "), vec!["GK"]);
    // Deduplication
    assert_eq!(parse_positions("D (C), D (C)"), vec!["DC"]);
}
