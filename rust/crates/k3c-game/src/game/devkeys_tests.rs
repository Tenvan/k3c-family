//! Port von `native/k3c-game/game/devkeys_test.go`. `TestParseArgsDevFlags` steht in `scene_tests`/`flags_tests`.

use k3c_platform::{InputState, Key};

use super::*;
use crate::game::Args;
use crate::game::play_testutil::play_game_with;
use crate::game::testutil::{FakeSim, Probe};
use crate::view::settings::{RIVER_TILES, default_settings};

/// Setzt die Eingabe des Takts so, dass genau die Tasten in `down` frisch gedrückt sind (Go: `keysDown`).
pub(super) fn keys_down(g: &mut Game, down: &[Key]) {
    let mut raw = InputState::new();
    for k in down {
        raw.keys.press(*k);
    }
    g.raw_input = raw;
}

/// Ein Spiel im Spielmodus mit Standard-Einstellungen im Frame (Go: `devGame`); `dev` hängt die Entwickler-Rolle ein.
pub(super) fn dev_game(dev: bool) -> (Game, Probe) {
    let (sim, probe) = FakeSim::new(2);
    let sim = if dev { sim.with_dev() } else { sim };
    let (mut g, probe) = play_game_with(sim, probe, Vec::new());
    g.frame.settings = default_settings();
    (g, probe)
}

// AC-05 (B-816): F3 ändert die Fluss-Fassung in den Einstellungen und kehrt beim zweiten Druck zurück; der Rest der
// Einstellungen bleibt, F2 allein ändert die Fluss-Fassung nicht.
#[test]
fn dev_key_river_toggles() {
    let (mut g, probe) = dev_game(false);
    keys_down(&mut g, &[Key::F3]);
    g.dev_keys();
    let set = probe.set.borrow().clone();
    assert_eq!(set.len(), 1, "F3: {set:?}");
    assert!(!set[0].river.is_empty() && !set[0].stats, "F3: {set:?}");
    assert!(
        (set[0].music - 1.0).abs() < f64::EPSILON,
        "Rest der Einstellungen bleibt"
    );
    let first = set[0].river.clone();
    g.frame.settings = set[0].clone();
    g.dev_keys();
    let set = probe.set.borrow().clone();
    assert!(
        set.len() == 2 && set[1].river != first && !set[1].river.is_empty(),
        "zweiter Druck: {set:?}"
    );
    g.frame.settings = set[1].clone();
    keys_down(&mut g, &[Key::F2]);
    g.dev_keys();
    let set = probe.set.borrow().clone();
    assert!(
        set.len() == 3 && set[2].river == set[1].river && set[2].stats,
        "F2: {set:?}"
    );
}

// Ohne Taste, im Aufnahme-Modus und vor dem ersten Frame wird nichts gespeichert; beide Tasten zugleich: ein Speichern.
#[test]
fn dev_key_ignored() {
    let (mut g, probe) = dev_game(false);
    keys_down(&mut g, &[Key::A]);
    g.dev_keys();
    keys_down(&mut g, &[Key::F3, Key::F2]);
    g.args.shots = "x".to_string();
    g.dev_keys();
    g.args.shots.clear();
    g.frame.settings = Settings::default();
    g.dev_keys();
    assert!(probe.set.borrow().is_empty(), "gespeichert: {:?}", probe.set.borrow());
    g.frame.settings = default_settings();
    g.dev_keys();
    let set = probe.set.borrow().clone();
    assert!(
        set.len() == 1 && !set[0].river.is_empty() && set[0].stats,
        "beide Tasten: {set:?}"
    );
}

#[test]
fn apply_dev_args() {
    let (mut g, probe) = dev_game(false);
    g.apply_dev_args();
    assert!(probe.set.borrow().is_empty(), "ohne Schalter gespeichert");
    g.args = Args {
        river: RIVER_TILES.to_string(),
        stats: true,
        ..g.args.clone()
    };
    g.apply_dev_args();
    let set = probe.set.borrow().clone();
    assert!(
        set.len() == 1 && set[0].river == RIVER_TILES && set[0].stats,
        "--river --stats: {set:?}"
    );
}
