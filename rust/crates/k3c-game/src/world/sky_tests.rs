use super::*;

fn cycle(phase: &str, progress: f64) -> CycleInfo {
    CycleInfo {
        phase: phase.to_string(),
        progress,
        ..CycleInfo::default()
    }
}

#[test]
fn dusk_weight_peaks_in_the_middle_of_dusk_and_dawn() {
    assert_eq!(dusk_weight(&cycle("day", 0.5)), 0.0);
    assert_eq!(dusk_weight(&cycle("night", 0.5)), 0.0);
    assert!((dusk_weight(&cycle("dusk", 0.5)) - 1.0).abs() < 1e-12);
    assert!((dusk_weight(&cycle("dawn", 0.5)) - 1.0).abs() < 1e-12);
    assert!(dusk_weight(&cycle("dusk", 0.0)).abs() < 1e-12);
    assert!(dusk_weight(&cycle("dusk", 1.0)).abs() < 1e-12);
}

#[test]
fn stars_are_deterministic_and_stay_in_the_upper_sky() {
    for i in 0..200 {
        let (x, y, b) = star_at(i);
        assert_eq!(star_at(i), (x, y, b));
        assert!((0.0..1.0).contains(&x), "x {x}");
        assert!((0.0..STAR_AREA).contains(&y), "y {y}");
        assert!((0.5..=1.0).contains(&b), "Helligkeit {b}");
    }
    assert_ne!(star_at(0), star_at(1));
}
