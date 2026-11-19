//! Das Kennzeichen der Doppel-Truhe: deterministisch und im Mittel jede n-te (RU62.3, AC-02).
use super::is_double_chest;

#[test]
fn gleicher_seed_gleiche_kennzeichen() {
    let a: Vec<bool> = (0..200)
        .map(|i| is_double_chest("insel-7", 1, f64::from(i) * 13.37, 4))
        .collect();
    let b: Vec<bool> = (0..200)
        .map(|i| is_double_chest("insel-7", 1, f64::from(i) * 13.37, 4))
        .collect();
    assert_eq!(a, b);
    assert!(a.contains(&true) && a.contains(&false));
}

#[test]
fn seed_stufe_und_ort_aendern_das_kennzeichen() {
    let base: Vec<bool> = (0..200)
        .map(|i| is_double_chest("insel-7", 1, f64::from(i) * 7.0, 4))
        .collect();
    let seed: Vec<bool> = (0..200)
        .map(|i| is_double_chest("insel-8", 1, f64::from(i) * 7.0, 4))
        .collect();
    let stage: Vec<bool> = (0..200)
        .map(|i| is_double_chest("insel-7", 2, f64::from(i) * 7.0, 4))
        .collect();
    assert_ne!(base, seed);
    assert_ne!(base, stage);
}

#[test]
fn im_mittel_jede_n_te_truhe() {
    for n in [3, 4, 8] {
        let hits = (0..4000_i32)
            .filter(|i| is_double_chest(&format!("seed-{}", i % 40), i64::from(i % 3), f64::from(*i) * 5.5, n))
            .count() as f64;
        let ratio = hits / 4000.0;
        let want = 1.0 / n as f64;
        assert!((ratio - want).abs() < want * 0.25, "n={n}: Anteil {ratio}");
    }
}

#[test]
fn every_nth_eins_kennzeichnet_jede() {
    assert!((0..50).all(|i| is_double_chest("x", 0, f64::from(i), 1)));
}
