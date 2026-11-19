//! B-1008: Die Lage der Krone (am Boden, getragen, geraubt) übersteht Speichern und Laden. Der Spielstand der Insel
//! (`IslandPlayerSave`) hat kein Feld für die Krone; Go hat dasselbe (`island_save.go`), der Fehler steckt also im Format.
//! Der Test beschreibt das Sollverhalten aus dem Ticket und ist bis zur Entscheidung zum Spielstand-Format `ignore`.

use super::add_island_player;
use super::testutil::*;
use crate::island::switch_testutil::fell;

/// Eine Krone am Boden liegt nach Speichern und Laden noch am selben Ort; der Monarch bleibt ohne Krone (B-1008).
#[test]
#[ignore = "B-1008: Spielstand trägt die Krone nicht (Format-Entscheidung 🧑 offen, siehe Ticket)"]
fn crown_on_ground_survives_save_and_load() {
    let data = test_data();
    let mut isl = must_island(&data, "krone", &[0]);
    let pi = add_island_player(&mut isl, 0);
    fell(&mut isl.stages[0], pi);
    let x = isl.stages[0].players[pi].crown.as_ref().map(|c| c.x);
    assert!(x.is_some(), "Vorbedingung: Krone liegt am Boden");

    let loaded = load_island(&data, &save_json(&isl, "2026-11-19T12:00:00Z"));
    let p = &loaded.stages[0].players[pi];
    assert_eq!(
        p.crown.as_ref().map(|c| c.x),
        x,
        "Krone nach dem Laden weg oder verschoben"
    );
}
