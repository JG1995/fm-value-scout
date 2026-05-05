use fm_valuescout_lib::scoring::{
    compute_value_multiplier, compute_value_score, VALUE_MULTIPLIER_CHEAPEST,
    VALUE_MULTIPLIER_MOST_EXPENSIVE,
};

#[test]
fn test_value_multiplier_cheapest() {
    assert_eq!(compute_value_multiplier(0.0), VALUE_MULTIPLIER_CHEAPEST);
}

#[test]
fn test_value_multiplier_tenth_percentile() {
    assert_eq!(compute_value_multiplier(0.10), VALUE_MULTIPLIER_CHEAPEST);
}

#[test]
fn test_value_multiplier_ninetieth_percentile() {
    assert_eq!(
        compute_value_multiplier(0.90),
        VALUE_MULTIPLIER_MOST_EXPENSIVE
    );
}

#[test]
fn test_value_multiplier_most_expensive() {
    assert_eq!(
        compute_value_multiplier(1.0),
        VALUE_MULTIPLIER_MOST_EXPENSIVE
    );
}

#[test]
fn test_value_multiplier_middle() {
    // At percentile 0.50, linear interpolation should give 1.0
    // 1.5 - (0.50 - 0.10) / 0.80 * (1.5 - 0.5) = 1.5 - 0.5 * 1.0 = 1.0
    assert_eq!(compute_value_multiplier(0.50), 1.0);
}

#[test]
fn test_value_score_basic() {
    // quality=80.0, percentile=0.0 → 80.0 * 1.5 = 120.0
    assert_eq!(compute_value_score(80.0, 0.0), 120.0);
}

#[test]
fn test_value_score_expensive() {
    // quality=80.0, percentile=1.0 → 80.0 * 0.5 = 40.0
    assert_eq!(compute_value_score(80.0, 1.0), 40.0);
}

#[test]
fn test_value_score_middle() {
    // quality=80.0, percentile=0.50 → 80.0 * 1.0 = 80.0
    assert_eq!(compute_value_score(80.0, 0.50), 80.0);
}
