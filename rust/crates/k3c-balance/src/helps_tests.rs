//! Tests zu `helps.rs` (Port von `engine/balance/helps_test.go`).

use k3c_data::{Source, load_all};

use crate::bots::economy_view;
use crate::helps::{HELPS_FULL, apply_helps, check_helps, grade_of};
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
/// unberührt. Go: `TestNewIslandHelps` baut die Inseln über `newIsland` (RU25.5, `island_world.rs`). Was ohne sie prüfbar
/// ist, deckt `hilfen_voll_baut_den_hub_aus` ab.
#[test]
#[ignore = "wartet auf RU25.5"]
fn new_island_helps() {}

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

/// Go: `TestMatrixHelpsBrauchtWelt` prüft `Matrix.Scenarios` (RU25.4, `run.rs`); die Prüfung selbst ist `check_helps`.
#[test]
#[ignore = "wartet auf RU25.4"]
fn matrix_helps_braucht_welt() {}
