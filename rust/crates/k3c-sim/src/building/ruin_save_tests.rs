//! Ruinen im Spielstand (Go: `building_ruin_save_test.go`, WZ14.3, B-468/AC-03, B-489): Kennung aus dem Level, Planung im
//! Stand, Welt ohne Planung beim Laden.

use super::testutil::*;
use crate::state::common::destroy_site;

/// Go: `ruinWorld`: Test-Welt (Insel „a“, Planung in Tiefe 0) mit der Planung der Fixture; ohne Planung, wenn `planned` falsch ist.
fn ruin_world(planned: bool) -> crate::island::WorldDef {
    let mut fields = String::new();
    if planned {
        let plan = serde_json::to_string(&ruin_fixture_plan()).expect("Planung als JSON");
        fields = format!(r#","plannedChunks":{{"0":{plan}}}"#);
    }
    crate::island::testutil::parse_test_world(&fields).expect("Welt")
}

/// Go: `CreateIslandIn(world, "ruine-1", …)` mit zwei Spielern in Stufe 0.
fn ruin_island_in(world: &crate::island::WorldDef) -> (crate::island::Island, k3c_data::Data) {
    use crate::island::testutil::{ISLAND_SPEED, test_data};
    let data = test_data();
    let mut isl = crate::island::create_island_in(&data, world, "ruine-1", &[0], ISLAND_SPEED, Default::default())
        .expect("Insel");
    crate::island::add_island_player(&mut isl, 0);
    crate::island::add_island_player(&mut isl, 0);
    (isl, data)
}

/// Go: `ParseIslandSave` und `FromIslandSaveIn` der Welt `world` aus dem Spielstand-Text.
fn load_in(data: &k3c_data::Data, raw: &str, world: &crate::island::WorldDef) -> crate::island::Island {
    use crate::island::testutil::{ISLAND_SPEED, standard_world};
    use crate::island::{SaveTables, from_island_save_in, parse_island_save};
    let std_world = standard_world(data);
    let defaults = crate::state::links::values_of(&std_world).expect("Basis-Wertesatz");
    let t = SaveTables {
        data,
        defaults: &defaults,
        standard_world: &std_world,
    };
    let s = parse_island_save(raw, &t).expect("Spielstand lesen");
    from_island_save_in(&t, &s, ISLAND_SPEED, world).expect("Insel laden")
}

/// Ruinen im Spielstand (kind@x, keine neue Version) und Vorrang der Mauerlinien beim Bezahlen (WZ14.3, B-468/AC-03).
#[test]
fn ruine_im_spielstand() {
    use crate::island::testutil::save_json;
    let world = ruin_world(true);
    let (mut isl, data) = ruin_island_in(&world);
    let w = &mut isl.stages[0];
    let st = ruin_of(w, "storage");
    w.sites[st].hp = 20.0;
    let (sm, sid) = (ruin_of(w, "smithy"), w.sites[ruin_of(w, "smithy")].id);
    let _ = sm;
    destroy_site(w, sid);
    let at = "2026-10-21T10:00:00Z";
    assert_eq!(crate::island::ISLAND_SAVE_VERSION, 10, "Version");
    let first = save_json(&isl, at);
    assert!(
        !first.contains("\"ruin\":"),
        "die Kennung gehört nicht in den Spielstand (Herkunft aus dem Level)"
    );
    let loaded = load_in(&data, &first, &world);
    let lw = &loaded.stages[0];
    let s = &lw.sites[ruin_of(lw, "storage")];
    assert!(
        s.state == "built" && s.hp == 20.0,
        "beschädigte Ruine nach Laden: {s:?}"
    );
    let s = &lw.sites[ruin_of(lw, "smithy")];
    assert!(s.state == "unpaid" && s.hp == 0.0, "zerstörte Ruine nach Laden: {s:?}");
    let s = &lw.sites[ruin_of(lw, "barracks")];
    assert!(
        s.state == "built" && s.hp == ruin_start_hp(lw, "barracks"),
        "unberührte Ruine nach Laden: {s:?}"
    );
    assert_eq!(lw.players.len(), 2, "Spieler nach Laden");
    assert_eq!(
        save_json(&loaded, at),
        first,
        "Speichern, Laden, Speichern ergibt etwas anderes"
    );
}

/// Die Planung steht im Stand (B-489): Ändert sich die Planung der Welt danach, lädt der Stand seine Stufe so, wie er sie
/// gebaut hat; die Ruinen und ihre Plätze bleiben (vorher verlor der Stand sie, WZ24.2).
#[test]
fn ruine_im_spielstand_bleibt_bei_geaenderter_welt_planung() {
    use crate::island::testutil::save_json;
    let (isl, data) = ruin_island_in(&ruin_world(true));
    let raw = save_json(&isl, "2026-10-21T10:00:00Z");
    let loaded = load_in(&data, &raw, &ruin_world(false)); // die Welt hat die Planung inzwischen verloren
    for r in &isl.stages[0].level.ruins {
        let found = site_at(&loaded.stages[0], &r.kind, r.x);
        assert!(
            found.is_some_and(|i| loaded.stages[0].sites[i].ruin),
            "Ruine {}@{} nach dem Laden verloren",
            r.kind,
            r.x
        );
    }
    assert_eq!(
        loaded.stages[0].sites.len(),
        isl.stages[0].sites.len(),
        "Plätze nach Laden"
    );
}
