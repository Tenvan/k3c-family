//! Spielstand Version 4 (Go: `island_save_v4_test.go`, S2.2, B-147): Tag und Phase beim Speichern; Version 3 ohne die Felder
//! lädt weiter. `island_save_v4_rundlauf` und `island_save_v4_v3_laedt_insel` bauen oder laden eine Insel (RU14) und tragen
//! `ignore`; das Lesen der Fixtures v3 und v4 läuft.

use serde_json::Value;

use super::save_tests::{Env, fixture};
use super::testutil::*;
use super::{ISLAND_SAVE_VERSION, add_island_player, step_island};

/// Go: `nightIsland`: Insel mit zwei Spielern in den Stufen 0 und 1, bis in die Nacht gerechnet (schneller Zyklus).
fn night_island(data: &k3c_data::Data) -> super::Island {
    let mut isl = must_island(data, "familie", &[0, 1]);
    let (a, b) = (add_island_player(&mut isl, 0), add_island_player(&mut isl, 1));
    isl.stages[0].players[a].gold = 23;
    isl.stages[1].players[b].gold = 8;
    isl.stock.stone = 12;
    let mut i = 0;
    while isl.stages[0].cycle.phase != "night" {
        assert!(
            i <= 30 * 60,
            "keine Nacht nach 60 s, Phase {:?}",
            isl.stages[0].cycle.phase
        );
        step_island(&mut isl, &[], &data.difficulty, 1.0 / 30.0);
        i += 1;
    }
    isl
}

/// Go: `TestIslandSaveV4Rundlauf`: `to_save` schreibt Tag, Phase und Tiefen; Speichern, Laden, Speichern ist gleich.
#[test]
fn island_save_v4_rundlauf() {
    let data = test_data();
    let isl = night_island(&data);
    let s = isl.to_save("2026-10-04T22:00:00Z").expect("Spielstand");
    assert!(
        s.version == ISLAND_SAVE_VERSION && s.phase == "night" && s.day == isl.stages[0].cycle.day && s.day >= 1,
        "Version {}, Tag {}, Phase {:?}",
        s.version,
        s.day,
        s.phase
    );
    assert!(
        s.players.len() == 2 && s.players[0].depth == 0 && s.players[1].depth == 1,
        "Spieler: {:?}",
        s.players
    );
    let first = save_json(&isl, "2026-10-04T22:00:00Z");
    let loaded = load_island(&data, &first);
    assert_eq!(
        first,
        save_json(&loaded, "2026-10-04T22:00:00Z"),
        "Speichern, Laden, Speichern"
    );
}

/// Go: `TestIslandSaveV4FixtureUndV3` (Lese-Seite): Das Fixture v4 liest mit Tag und Phase; das Fixture v3 (ohne die Felder)
/// liest mit leeren Feldern als Version 9 und schreibt kein leeres Feld.
#[test]
fn island_save_v4_fixture_und_v3() {
    let env = Env::standard();
    let s = env.parse(&fixture(4)).expect("v4");
    assert!(
        s.version == ISLAND_SAVE_VERSION && s.phase == "night" && s.day >= 1 && !s.saved_at.is_empty(),
        "v4: Version {}, Tag {}, Phase {:?}, savedAt {:?}",
        s.version,
        s.day,
        s.phase,
        s.saved_at
    );
    let old = env.parse(&fixture(3)).expect("v3");
    assert!(
        old.version == ISLAND_SAVE_VERSION && old.day == 0 && old.phase.is_empty(),
        "v3: Version {}, Tag {}, Phase {:?}",
        old.version,
        old.day,
        old.phase
    );
    let out: Value = serde_json::to_value(&old).expect("schreiben");
    assert!(
        out.get("phase").is_none(),
        "ein Stand ohne Phase schreibt kein leeres Feld"
    );
    assert!(out.get("day").is_none(), "ein Stand ohne Tag schreibt kein leeres Feld");
}

/// Go: `TestIslandSaveV4FixtureUndV3` (Lade-Seite): Das Fixture v3 lädt als Insel.
#[test]
fn island_save_v4_v3_laedt_insel() {
    let data = test_data();
    let isl = load_island(&data, &fixture(3));
    assert!(!isl.stages.is_empty(), "v3: Insel aus dem Stand");
}
