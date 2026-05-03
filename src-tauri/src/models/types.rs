use serde::{Deserialize, Serialize};

/// Foot strength rating, ordered from weakest to strongest.
/// Used for both left and right foot analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FootRating {
    Weak,
    Reasonable,
    FairlyStrong,
    Strong,
    VeryStrong,
}

impl FootRating {
    /// Parse from the CSV string representation.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "Weak" => Some(Self::Weak),
            "Reasonable" => Some(Self::Reasonable),
            "Fairly Strong" => Some(Self::FairlyStrong),
            "Strong" => Some(Self::Strong),
            "Very Strong" => Some(Self::VeryStrong),
            _ => None,
        }
    }
}

impl std::fmt::Display for FootRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Weak => write!(f, "Weak"),
            Self::Reasonable => write!(f, "Reasonable"),
            Self::FairlyStrong => write!(f, "Fairly Strong"),
            Self::Strong => write!(f, "Strong"),
            Self::VeryStrong => write!(f, "Very Strong"),
        }
    }
}

/// Wage payment frequency, used for display purposes.
/// Internally, all wages are normalized to weekly values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WageUnit {
    PerWeek,
    PerMonth,
    PerAnnum,
}

impl WageUnit {
    /// Convert an amount in this unit to a weekly amount.
    pub fn to_weekly(&self, amount: f64) -> f64 {
        match self {
            Self::PerWeek => amount,
            Self::PerMonth => amount / 4.33, // ~52 weeks / 12 months
            Self::PerAnnum => amount / 52.0,
        }
    }
}

/// Appearances in a season, split into total and substitute appearances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Appearances {
    /// Total appearances including as substitute.
    pub total: u16,
    /// Appearances as a substitute (included in total).
    pub as_sub: u16,
}

impl Appearances {
    /// Calculate starts = total - as_sub.
    pub fn starts(&self) -> u16 {
        self.total.saturating_sub(self.as_sub)
    }
}

/// Transfer value range from CSV, parsed into numeric values.
/// Currency is stored at season level, not per-player.
/// Example: "€62M - €94M" → TransferValue { min: 62_000_000, max: 94_000_000 }
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TransferValue {
    pub min: u64,
    pub max: u64,
}

impl TransferValue {
    /// Midpoint value for sorting/filtering.
    pub fn midpoint(&self) -> f64 {
        (self.min as f64 + self.max as f64) / 2.0
    }
}

/// Wage from CSV, normalized to weekly value internally.
/// The original unit is preserved for display purposes.
/// Example: "€74K p/w" → Wage { weekly_amount: 74_000, unit: PerWeek }
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Wage {
    /// Weekly-normalized amount (original value converted via unit.to_weekly).
    pub weekly_amount: f64,
    /// Original payment frequency (for display only).
    pub unit: WageUnit,
}

impl Wage {
    /// Weekly amount (same as weekly_amount field, for API compatibility).
    pub fn weekly(&self) -> f64 {
        self.weekly_amount
    }

    /// Annual wage estimate (52 weeks, not accounting for contract length).
    /// Note: Currency symbol should come from season metadata.
    pub fn annual_estimate(&self) -> f64 {
        self.weekly_amount * 52.0
    }
}
