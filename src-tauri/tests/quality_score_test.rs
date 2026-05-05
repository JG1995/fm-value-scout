use fm_valuescout_lib::scoring::*;
use std::collections::HashMap;

fn make_percentiles(pairs: &[(&str, f64)]) -> HashMap<String, f64> {
    pairs.iter().map(|(k, v)| ((*k).to_string(), *v)).collect()
}

fn archetype_both_in_out() -> ArchetypeWeights {
    ArchetypeWeights {
        in_possession: vec![
            ArchetypeWeightEntry { metric_name: "pas".into(), weight: 2.0, inverted: false },
            ArchetypeWeightEntry { metric_name: "fir".into(), weight: 1.0, inverted: false },
        ],
        out_of_possession: vec![
            ArchetypeWeightEntry { metric_name: "tck".into(), weight: 3.0, inverted: false },
            ArchetypeWeightEntry { metric_name: "hea".into(), weight: 1.0, inverted: false },
        ],
    }
}

#[test]
fn test_quality_perfect_both() {
    let pct = make_percentiles(&[("pas", 1.0), ("fir", 1.0), ("tck", 1.0), ("hea", 1.0)]);
    let arch = archetype_both_in_out();
    let result = compute_quality_score(&arch, &pct, "all");
    assert!(result.is_some());
    assert!((result.unwrap() - 100.0).abs() < 1e-9);
}

#[test]
fn test_quality_perfect_in_zero_out() {
    let pct = make_percentiles(&[("pas", 1.0), ("fir", 1.0), ("tck", 0.0), ("hea", 0.0)]);
    let arch = archetype_both_in_out();
    let result = compute_quality_score(&arch, &pct, "all");
    assert!(result.is_some());
    assert!((result.unwrap() - 75.0).abs() < 1e-9);
}

#[test]
fn test_quality_zero_in_perfect_out() {
    let pct = make_percentiles(&[("pas", 0.0), ("fir", 0.0), ("tck", 1.0), ("hea", 1.0)]);
    let arch = archetype_both_in_out();
    let result = compute_quality_score(&arch, &pct, "all");
    assert!(result.is_some());
    assert!((result.unwrap() - 25.0).abs() < 1e-9);
}

#[test]
fn test_quality_empty_out_weights() {
    let seed = get_seed_archetypes();
    let poacher = seed.iter().find(|a| a.name == "Poacher").unwrap();
    assert!(poacher.weights.out_of_possession.is_empty());
    let pct = make_percentiles(&[("fin", 1.0), ("cmp", 1.0), ("otb", 1.0), ("ant", 1.0)]);
    let result = compute_quality_score(&poacher.weights, &pct, "all");
    assert!(result.is_some());
    assert!((result.unwrap() - 100.0).abs() < 1e-9);
}

#[test]
fn test_quality_no_percentiles() {
    let arch = archetype_both_in_out();
    let pct: HashMap<String, f64> = HashMap::new();
    let result = compute_quality_score(&arch, &pct, "all");
    assert!(result.is_none());
}

#[test]
fn test_quality_partial_percentiles() {
    let pct = make_percentiles(&[("pas", 1.0), ("tck", 1.0)]);
    let arch = archetype_both_in_out();
    let result = compute_quality_score(&arch, &pct, "all");
    assert!(result.is_some());
    let expected = (0.75 * (2.0_f64 / 3.0) + 0.25 * (3.0_f64 / 4.0)) * 100.0;
    assert!((result.unwrap() - expected).abs() < 1e-9);
}

#[test]
fn test_quality_inverted_metric() {
    let arch = ArchetypeWeights {
        in_possession: vec![
            ArchetypeWeightEntry { metric_name: "pas".into(), weight: 2.0, inverted: true },
        ],
        out_of_possession: vec![],
    };
    let pct = make_percentiles(&[("pas", 0.0)]);
    let result = compute_quality_score(&arch, &pct, "all");
    assert!(result.is_some());
    assert!((result.unwrap() - 100.0).abs() < 1e-9);

    let pct2 = make_percentiles(&[("pas", 1.0)]);
    let result2 = compute_quality_score(&arch, &pct2, "all");
    assert!(result2.is_some());
    assert!((result2.unwrap() - 0.0).abs() < 1e-9);
}

#[test]
fn test_quality_ball_playing_defender() {
    let seed = get_seed_archetypes();
    let bp = seed.iter().find(|a| a.name == "Ball-Playing Defender").unwrap();
    let pct = make_percentiles(&[
        ("pas", 1.0), ("fir", 1.0), ("cmp", 1.0), ("vis", 1.0),
        ("tck", 1.0), ("hea", 1.0), ("mar", 1.0), ("ant", 1.0),
    ]);
    let result = compute_quality_score(&bp.weights, &pct, "all");
    assert!(result.is_some());
    assert!((result.unwrap() - 100.0).abs() < 1e-9);
}