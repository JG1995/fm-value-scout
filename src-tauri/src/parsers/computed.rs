/// Compute a per-90 value from a raw total and minutes played.
///
/// Formula: `total / minutes * 90.0`
///
/// Returns `0.0` when minutes is zero or negative (guard against division by zero).
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::computed::compute_per_90;
///
/// let result = compute_per_90(100.0, 900.0);
/// assert!((result - 10.0).abs() < f64::EPSILON);
///
/// assert_eq!(compute_per_90(50.0, 0.0), 0.0);
/// assert_eq!(compute_per_90(50.0, -10.0), 0.0);
/// ```
pub fn compute_per_90(total: f64, minutes: f64) -> f64 {
    if minutes <= 0.0 {
        return 0.0;
    }
    total / minutes * 90.0
}

/// Recover a raw total from a per-90 value and minutes played.
///
/// Formula: `per90 * minutes / 90.0`
///
/// Returns `0.0` when minutes is zero or negative (guard against division by zero).
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::computed::compute_total_from_per_90;
///
/// let result = compute_total_from_per_90(10.0, 900.0);
/// assert!((result - 100.0).abs() < f64::EPSILON);
///
/// assert_eq!(compute_total_from_per_90(10.0, 0.0), 0.0);
/// assert_eq!(compute_total_from_per_90(10.0, -5.0), 0.0);
/// ```
pub fn compute_total_from_per_90(per90: f64, minutes: f64) -> f64 {
    if minutes <= 0.0 {
        return 0.0;
    }
    per90 * minutes / 90.0
}

/// Compute a percentage-style ratio, clamped to the range `[0.0, 1.0]`.
///
/// Formula: `completed / attempted`
///
/// Returns `0.0` when attempted is zero or negative (division by zero guard).
/// The result is clamped to `[0.0, 1.0]` to represent valid completion percentages.
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::computed::compute_ratio;
///
/// let result = compute_ratio(75.0, 100.0);
/// assert!((result - 0.75).abs() < f64::EPSILON);
///
/// // Division by zero guard
/// assert_eq!(compute_ratio(50.0, 0.0), 0.0);
///
/// // Clamped to [0.0, 1.0]
/// assert_eq!(compute_ratio(200.0, 100.0), 1.0);
/// assert_eq!(compute_ratio(-10.0, 100.0), 0.0);
/// ```
pub fn compute_ratio(completed: f64, attempted: f64) -> f64 {
    if attempted <= 0.0 {
        return 0.0;
    }
    let ratio = completed / attempted;
    ratio.clamp(0.0, 1.0)
}

/// Compute a ratio without clamping the result.
///
/// Formula: `numerator / denominator`
///
/// Returns `0.0` when denominator is zero or negative (division by zero guard).
/// Does **not** clamp the result — preserves negative values,
/// which is needed for metrics like xG OP/xGP where negative ratios are meaningful.
///
/// # Examples
///
/// ```
/// use fm_valuescout_lib::parsers::computed::compute_ratio_unclamped;
///
/// let result = compute_ratio_unclamped(75.0, 100.0);
/// assert!((result - 0.75).abs() < f64::EPSILON);
///
/// // Division by zero guard
/// assert_eq!(compute_ratio_unclamped(50.0, 0.0), 0.0);
///
/// // Negative values are preserved
/// assert!(compute_ratio_unclamped(-30.0, 100.0) < 0.0);
/// assert!((compute_ratio_unclamped(-30.0, 100.0) + 0.3).abs() < f64::EPSILON);
/// ```
pub fn compute_ratio_unclamped(numerator: f64, denominator: f64) -> f64 {
    if denominator <= 0.0 {
        return 0.0;
    }
    numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // compute_per_90
    // -----------------------------------------------------------------------

    #[test]
    fn test_per_90_standard() {
        let result = compute_per_90(100.0, 900.0);
        assert!((result - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_per_90_zero_minutes() {
        assert_eq!(compute_per_90(50.0, 0.0), 0.0);
    }

    #[test]
    fn test_per_90_negative_minutes() {
        assert_eq!(compute_per_90(50.0, -10.0), 0.0);
    }

    #[test]
    fn test_per_90_zero_total() {
        let result = compute_per_90(0.0, 900.0);
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_per_90_negative_total() {
        // Negative totals should propagate (e.g. xG OP can be negative)
        let result = compute_per_90(-50.0, 900.0);
        assert!((result - (-5.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_per_90_partial_minutes() {
        // 45 minutes = half a match
        let result = compute_per_90(5.0, 45.0);
        assert!((result - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_per_90_extrapolate_from_small_minutes() {
        // 1 minute sample extrapolated to 90
        let result = compute_per_90(1.0, 1.0);
        assert!((result - 90.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_per_90_fractional_total() {
        let result = compute_per_90(0.5, 90.0);
        assert!((result - 0.5).abs() < f64::EPSILON);
    }

    // -----------------------------------------------------------------------
    // compute_total_from_per_90
    // -----------------------------------------------------------------------

    #[test]
    fn test_total_from_per_90_standard() {
        let result = compute_total_from_per_90(10.0, 900.0);
        assert!((result - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_total_from_per_90_zero_minutes() {
        assert_eq!(compute_total_from_per_90(10.0, 0.0), 0.0);
    }

    #[test]
    fn test_total_from_per_90_negative_minutes() {
        assert_eq!(compute_total_from_per_90(10.0, -5.0), 0.0);
    }

    #[test]
    fn test_total_from_per_90_zero_per90() {
        let result = compute_total_from_per_90(0.0, 900.0);
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_total_from_per_90_negative_per90() {
        let result = compute_total_from_per_90(-5.0, 900.0);
        assert!((result - (-50.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_total_from_per_90_partial_minutes() {
        let result = compute_total_from_per_90(10.0, 45.0);
        assert!((result - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_total_from_per_90_one_minute() {
        let result = compute_total_from_per_90(90.0, 1.0);
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_total_from_per_90_fractional() {
        let result = compute_total_from_per_90(0.5, 90.0);
        assert!((result - 0.5).abs() < f64::EPSILON);
    }

    // -----------------------------------------------------------------------
    // compute_ratio
    // -----------------------------------------------------------------------

    #[test]
    fn test_ratio_standard() {
        let result = compute_ratio(75.0, 100.0);
        assert!((result - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ratio_division_by_zero() {
        assert_eq!(compute_ratio(50.0, 0.0), 0.0);
    }

    #[test]
    fn test_ratio_negative_attempted() {
        assert_eq!(compute_ratio(50.0, -10.0), 0.0);
    }

    #[test]
    fn test_ratio_clamp_above_one() {
        assert_eq!(compute_ratio(200.0, 100.0), 1.0);
    }

    #[test]
    fn test_ratio_clamp_below_zero() {
        assert_eq!(compute_ratio(-10.0, 100.0), 0.0);
    }

    #[test]
    fn test_ratio_exactly_one() {
        let result = compute_ratio(100.0, 100.0);
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ratio_exactly_zero() {
        let result = compute_ratio(0.0, 100.0);
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ratio_both_zero() {
        // 0 / 0 → attempted <= 0 → return 0.0
        assert_eq!(compute_ratio(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_ratio_fractional_values() {
        let result = compute_ratio(0.5, 1.0);
        assert!((result - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ratio_rounding() {
        // 1/3 should give ~0.333...
        let result = compute_ratio(1.0, 3.0);
        assert!((result - (1.0 / 3.0)).abs() < f64::EPSILON);
    }

    // -----------------------------------------------------------------------
    // compute_ratio_unclamped
    // -----------------------------------------------------------------------

    #[test]
    fn test_ratio_unclamped_standard() {
        let result = compute_ratio_unclamped(75.0, 100.0);
        assert!((result - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ratio_unclamped_division_by_zero() {
        assert_eq!(compute_ratio_unclamped(50.0, 0.0), 0.0);
    }

    #[test]
    fn test_ratio_unclamped_negative_denominator() {
        assert_eq!(compute_ratio_unclamped(50.0, -10.0), 0.0);
    }

    #[test]
    fn test_ratio_unclamped_negative_numerator() {
        let result = compute_ratio_unclamped(-30.0, 100.0);
        assert!((result + 0.3).abs() < f64::EPSILON);
        assert!(result < 0.0);
    }

    #[test]
    fn test_ratio_unclamped_above_one() {
        // Unlike compute_ratio, values > 1.0 are preserved
        let result = compute_ratio_unclamped(200.0, 100.0);
        assert!((result - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ratio_unclamped_negative_denominator_returns_zero() {
        // Negative denominator is an error condition (FM data never has negative
        // denominators for completed/attempted fields) → guard returns 0.
        assert_eq!(compute_ratio_unclamped(-30.0, -10.0), 0.0);
    }

    #[test]
    fn test_ratio_unclamped_zero_numerator() {
        let result = compute_ratio_unclamped(0.0, 100.0);
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ratio_unclamped_both_zero() {
        assert_eq!(compute_ratio_unclamped(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_ratio_unclamped_fractional() {
        let result = compute_ratio_unclamped(1.0, 3.0);
        assert!((result - (1.0 / 3.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ratio_unclamped_negative_result() {
        // Negative numerator, positive denominator → negative result preserved
        let result = compute_ratio_unclamped(-5.0, 2.0);
        assert!((result - (-2.5)).abs() < f64::EPSILON);
        assert!(result < 0.0);
    }
}
