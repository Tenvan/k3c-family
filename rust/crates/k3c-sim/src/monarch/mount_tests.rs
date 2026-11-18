//! Standard-Reittier (Go: `monarch_mount_test.go`, S1.3, B-152/AC-02, docs/rules/monarch.md § 7): erwartete Werte aus
//! monarch.json › mount. Tests über `step` (`pure`) und Inseln (`decode_world_story`, RU19) tragen `ignore`. Die Tests ab
//! `own_` haben kein Go-Gegenstück und prüfen das Tempo ohne `step`.

use super::mount_of;
use super::passives::speed_of;
use crate::enemies::testutil::{add_player, quiet_world, test_values};
use crate::island::add_island_player;
use crate::island::testutil::{must_island, test_data};
use crate::state::{PlayerCommand, World, WorldValues};
use crate::world::testutil::DT;
use crate::world::{step, vals};

/// Go: `mountSpeeds`: zwei Spieler laufen 5 s nach rechts, Spieler 1 sprintet; Ergebnis `vx` beider.
fn mount_speeds(w: &mut World) -> (f64, f64) {
    let (p0, p1) = (add_player(w), add_player(w));
    let cmds = [
        PlayerCommand {
            move_x: 1.0,
            ..PlayerCommand::default()
        },
        PlayerCommand {
            move_x: 1.0,
            sprint: true,
            ..PlayerCommand::default()
        },
    ];
    for _ in 0..150 {
        step(w, &cmds, None, DT);
    }
    (w.players[p0].vx, w.players[p1].vx)
}

/// Go: `wantSpeeds`.
fn want_speeds(v: &WorldValues) -> (f64, f64) {
    let m = &v.monarch.mount;
    let walk = v.monarch.base.speed * m.speed_factor;
    (walk, walk * v.monarch.sprint_multiplier * m.sprint_factor)
}

/// (a) Jeder Monarch hat beim Beitritt das Reittier aus den Daten: Welt mit 2 Spielern und Insel mit 2 Stufen.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story: must_island lädt die Standard-Welt)"]
fn mount_beim_beitritt() {
    let data = test_data();
    let mut w = quiet_world();
    let (a, b) = (add_player(&mut w), add_player(&mut w));
    let mut isl = must_island(&data, "reittier-a", &[0, 1]);
    let (c, d) = (add_island_player(&mut isl, 0), add_island_player(&mut isl, 1));
    let monarch = test_values().monarch;
    for (stage, pi) in [(&w, a), (&w, b), (&isl.stages[0], c), (&isl.stages[1], d)] {
        let v = vals(stage);
        assert_eq!(
            mount_of(&v, pi),
            &monarch.mount,
            "Spieler {}: Reittier",
            stage.players[pi].index
        );
    }
    let m = &monarch.mount;
    assert!(
        !m.id.is_empty() && m.speed_factor > 0.0 && m.sprint_factor > 0.0,
        "monarch.json › mount unvollständig: {m:?}"
    );
}

/// (b) Tempo = Basis × Faktor, Sprint = Basis × Faktor × sprintMultiplier × Sprintfaktor.
#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn mount_geschwindigkeit() {
    let mut w = quiet_world();
    let (walk, sprint) = mount_speeds(&mut w);
    let (ww, ws) = want_speeds(&vals(&w));
    assert!(
        (walk - ww).abs() <= 1e-6 && (sprint - ws).abs() <= 1e-6,
        "Tempo {walk} (erwartet {ww}), Sprint {sprint} (erwartet {ws})"
    );
}

/// (c) Ein anderer Faktor in einer Kopie der Daten ändert beide Geschwindigkeiten ohne Codeänderung.
#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn mount_faktor_aus_daten() {
    let mut w0 = quiet_world();
    let (base_walk, base_sprint) = mount_speeds(&mut w0);
    let mut w = quiet_world();
    let mut v = (*vals(&w)).clone();
    (v.monarch.mount.speed_factor, v.monarch.mount.sprint_factor) = (1.5, 1.2);
    let (ww, ws) = want_speeds(&v);
    w.vals = Some(std::sync::Arc::new(v));
    let (walk, sprint) = mount_speeds(&mut w);
    assert!(
        (walk - ww).abs() <= 1e-6 && (sprint - ws).abs() <= 1e-6 && walk != base_walk && sprint != base_sprint,
        "Faktor 1,5/1,2: Tempo {walk} (erwartet {ww}), Sprint {sprint} (erwartet {ws})"
    );
}

/// (d) Zweimal derselbe Seed und dieselben Eingaben ergeben denselben Weltzustand.
#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn mount_deterministisch() {
    let run = || {
        let mut w = quiet_world();
        add_player(&mut w);
        add_player(&mut w);
        let cmds = [
            PlayerCommand {
                move_x: 1.0,
                sprint: true,
                ..PlayerCommand::default()
            },
            PlayerCommand {
                move_x: -1.0,
                ..PlayerCommand::default()
            },
        ];
        for _ in 0..300 {
            step(&mut w, &cmds, None, DT);
        }
        serde_json::to_string(&w).expect("Weltzustand als JSON")
    };
    assert_eq!(run(), run(), "gleicher Seed, gleiche Eingaben: Weltzustand weicht ab");
}

/// (e) Datenprüfung: mount.sprite ist ein Reittier aus data/paper.json › reittiere (ein unbekannter Schlüssel lässt diesen
/// Test scheitern, nicht das Spiel).
#[test]
fn mount_sprite_bekannt() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../data/paper.json");
    let raw = std::fs::read_to_string(path).expect("data/paper.json");
    let paper: serde_json::Value = serde_json::from_str(&raw).expect("paper.json ist JSON");
    let sprite = test_values().monarch.mount.sprite;
    assert!(
        paper["reittiere"].get(&sprite).is_some(),
        "monarch.json › mount.sprite {sprite:?} fehlt in paper.json › reittiere"
    );
}

/// (f) Ein Stufenwechsel ändert das Reittier nicht: Der Wertesatz einer Stufe ist derselbe wie der der Welt.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story, pure)"]
fn mount_nach_stufenwechsel() {
    let data = test_data();
    let mut isl = must_island(&data, "reittier-f", &[0, 1]);
    let a = add_island_player(&mut isl, 0);
    let before = mount_of(&vals(&isl.stages[0]), a).clone();
    let monarch = test_values().monarch;
    assert_eq!(before, monarch.mount);
    let after = mount_of(&vals(&isl.stages[1]), 0).clone();
    assert!(
        after == before && after == monarch.mount,
        "Reittier nach Wechsel {after:?}, vorher {before:?}"
    );
}

/// Zwei Spieler reiten dasselbe Reittier; das Tempo folgt dem Faktor der Daten (ohne `step` gerechnet).
#[test]
fn own_tempo_folgt_dem_reittier_faktor_fuer_zwei_spieler() {
    let mut w = quiet_world();
    let (p0, p1) = (add_player(&mut w), add_player(&mut w));
    let v = vals(&w);
    let (ww, _) = want_speeds(&v);
    assert!((speed_of(&w, &v, p0) - ww).abs() < 1e-9 && (speed_of(&w, &v, p1) - ww).abs() < 1e-9);
    assert_eq!(mount_of(&v, p0), mount_of(&v, p1), "jeder reitet das Standard-Reittier");
    let mut v2 = (*v).clone();
    (v2.monarch.mount.speed_factor, v2.monarch.mount.sprint_factor) = (1.5, 1.2);
    let (ww2, _) = want_speeds(&v2);
    assert!(
        ww2 != ww && (speed_of(&w, &v2, p0) - ww2).abs() < 1e-9,
        "anderer Faktor ändert das Tempo ohne Codeänderung"
    );
}
