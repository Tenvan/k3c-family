use super::testutil::{TempDir, test_session};
use crate::view::session::Session as _;
use crate::view::settings::default_settings;
use crate::view::stress::{GROWTH_EXP, StressTest};

// B-896: Entwicklermodus und die zuletzt eingestellten Stresstest-Werte überleben Speichern und Neustart.
#[test]
fn settings_stress_persist() {
    let dir = TempDir::new();
    let mut want = default_settings();
    want.dev_mode = true;
    want.stress = StressTest {
        preset: "volleBurg".into(),
        players: 2,
        depth: 1,
        seed: 7,
        enemies: 20,
        growth: GROWTH_EXP.into(),
        factor: 1.5,
        build: 5,
        troops: 120,
        timescale: 2,
        waves: 5,
        minutes: 10,
        enemy: "skeleton".into(),
        troop: "warrior".into(),
        pure: false,
    };
    test_session(&dir).set_settings(want.clone()).expect("speichern");
    let got = test_session(&dir).settings();
    assert_eq!(got, want, "nach Neustart {:?} / {}", got.stress, got.dev_mode);
}
