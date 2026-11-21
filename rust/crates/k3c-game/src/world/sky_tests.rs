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

#[test]
fn clouds_are_deterministic_clamped_and_drift() {
    for i in 0..12 {
        for step in 0..40 {
            let t = step as f64 * 17.3;
            let boxes = cloud_boxes(i, t, 800.0, 400.0, 0.004);
            assert_eq!(boxes, cloud_boxes(i, t, 800.0, 400.0, 0.004));
            for [x, y, w, h] in boxes {
                assert!(
                    x >= 0.0 && x + w <= 800.0 + 1e-9,
                    "Wolke {i} ragt aus der Zelle: {x} {w}"
                );
                assert!(
                    y >= 0.0 && y + h <= 400.0 * CLOUD_AREA + 800.0 * 0.16 * 1.4 * 0.28,
                    "Höhe {y}"
                );
                assert!(w > 0.0 && h > 0.0);
            }
        }
    }
    let at = |t: f64| cloud_boxes(3, t, 800.0, 400.0, 0.004).first().map(|b| b[0]);
    assert_ne!(at(0.0), at(60.0), "die Wolke zieht");
}
