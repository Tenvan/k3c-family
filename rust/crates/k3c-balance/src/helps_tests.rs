//! Tests zu `helps.rs` (Port von `engine/balance/helps_test.go`).

use k3c_data::{Source, load_all};

use crate::bots::economy_view;
use crate::helps::{HELPS_AIDS, HELPS_FULL, apply_helps, check_helps, grade_of};
use crate::run::{Matrix, Scenario, new_island};
use crate::testutil::scenario_island;

#[test]
fn check_helps_faelle() {
    let cases = [
        ("", "", true),
        ("", "standard", true),
        ("voll", "standard", true),
        ("boss", "standard", true),
        ("voll", "", false),
        ("gibtsnicht", "standard", false),
    ];
    for (helps, world, ok) in cases {
        let r = check_helps(helps, world);
        assert_eq!(
            r.is_ok(),
            ok,
            "check_helps({helps:?}, {world:?}) = {r:?}, ok erwartet: {ok}"
        );
    }
}

/// Die Hilfen der Seite sind die des Stufen-Tests: Grad dev, Dev-Bau Stufe 5 samt Vorrat; ohne Hilfen bleibt die Insel
/// unberührt (Go: `TestNewIslandHelps`, über `new_island` aus `run.rs`).
#[test]
fn new_island_helps() {
    let mut base = Scenario {
        seed: "k3c-dev".into(),
        players: 1,
        days: 1,
        world: "standard".into(),
        ..Scenario::default()
    };
    let plain = new_island(&base).expect("Insel ohne Hilfen");
    base.helps = HELPS_FULL.into();
    let aided = new_island(&base).expect("Insel mit Hilfen");
    let hub = economy_view(&aided.stages[0]).hub_level;
    assert_eq!(hub, 5, "Hub mit Hilfen voll = {hub}, erwartet 5");
    let hub = economy_view(&plain.stages[0]).hub_level;
    assert_eq!(hub, 1, "Hub ohne Hilfen = {hub}, erwartet 1");
    let plain_world = Scenario {
        world: "standard".into(),
        ..Scenario::default()
    };
    assert!(
        grade_of(&base.grade, &base.helps) == "dev" && grade_of(&plain_world.grade, &plain_world.helps).is_empty(),
        "Grad: mit Hilfen dev, ohne unverändert"
    );
}

/// Der Teil von `new_island_helps`, den `apply_helps` und `grade_of` allein erfüllen.
#[test]
fn hilfen_voll_baut_den_hub_aus() {
    let data = load_all(&Source::Embedded).expect("eingebettete Daten");
    let plain = scenario_island("k3c-dev", 1);
    let mut aided = scenario_island("k3c-dev", 1);
    apply_helps(&mut aided, &data.difficulty, HELPS_FULL);
    let hub = economy_view(&aided.stages[0]).hub_level;
    assert_eq!(hub, 5, "Hub mit Hilfen voll = {hub}, erwartet 5");
    let hub = economy_view(&plain.stages[0]).hub_level;
    assert_eq!(hub, 1, "Hub ohne Hilfen = {hub}, erwartet 1");
    assert!(
        grade_of("", HELPS_FULL) == "dev" && grade_of("", "").is_empty(),
        "Grad: mit Hilfen dev, ohne unverändert"
    );
    assert_eq!(grade_of("hard", HELPS_FULL), "hard", "ein gesetzter Grad bleibt");
}

/// Hilfen brauchen eine Welt; mit Welt steht der Wert im Szenario (Go: `TestMatrixHelpsBrauchtWelt`).
#[test]
fn matrix_helps_braucht_welt() {
    let mut m = Matrix {
        seeds: vec!["1".into()],
        players: vec![1],
        bots: vec!["wirtschaft".into()],
        depths: vec![0],
        days: 1,
        helps: HELPS_AIDS.into(),
        ..Matrix::default()
    };
    assert!(m.scenarios().is_err(), "Hilfen ohne Welt müssen abgelehnt werden");
    m.world = "standard".into();
    let sc = m.scenarios().expect("Szenarien");
    assert_eq!(sc[0].helps, HELPS_AIDS, "Szenario = {sc:?}");
}
