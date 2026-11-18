//! Port von `native/k3c-game/game/pacing_test.go` (`TestPacer`) gegen die Plattform-Zeitsammlung `k3c_platform::Clock`.
//! `pacing.go` selbst entfällt; Schalter `--pacing` und `Paced()` gibt es nicht mehr.

use k3c_platform::{Clock, MAX_STEPS};

/// Bei 60 Bildern je Sekunde läuft in jedem zweiten Bild ein Takt, alpha wechselt zwischen 0,5 und 0; ein Hänger holt
/// höchstens `MAX_STEPS` nach.
#[test]
fn pacer() {
    let mut c = Clock::new();
    let mut ticks = 0;
    for i in 0..120 {
        let (n, a) = c.advance(1.0 / 60.0);
        ticks += n;
        let want = if i % 2 == 1 { 0.0 } else { 0.5 };
        assert!(
            (a - want).abs() <= 1e-6 || (a - want - 1.0).abs() <= 1e-6,
            "Bild {i}: alpha {a}, erwartet {want}"
        );
    }
    assert!((59..=60).contains(&ticks), "2 Sekunden: {ticks} Takte");
    check_pacer_hitch(&mut c);
}

/// Prüft nach einem Hänger die Grenze der Takte und alpha bei 144 Hz.
fn check_pacer_hitch(c: &mut Clock) {
    let (n, a) = c.advance(1.0); // Hänger von einer Sekunde
    assert!(n == MAX_STEPS && a == 0.0, "Hänger: {n} Takte, alpha {a}");
    for i in 0..10 {
        let (_, a) = c.advance(1.0 / 144.0); // 144 Hz: alpha bleibt unter 1
        assert!((0.0..1.0).contains(&a), "144 Hz, Bild {i}: alpha {a}");
    }
}
