//! Port von `engine/sim/world_values_room_test.go` (BAL8.2, B-681, welten.md § 1/§ 6): Jede Insel rechnet mit dem
//! Wertesatz ihrer Welt; Welten mit verschiedenen Werten laufen nebeneinander, ohne sich zu beeinflussen; ohne Overrides
//! gilt die Basis byte-gleich. Die Go-Tests bauen auf der Standard-Welt (`StandardWorld` liest `story`: RU19) und rechnen
//! Inseln (`create_world`, `Step`: RU18); sie tragen `ignore`. Die `own_*`-Tests prüfen dieselben Regeln von
//! `resolve_world` und `values_of` an einer Test-Welt ohne Geschichte.

use std::collections::BTreeMap;

use serde_json::{Value, json};

use super::base::base;
use super::pending::site_cost;
use super::values_room::{default_values, resolve_world, values, values_of};
use crate::island::switch_testutil::{test_island, test_world};
use crate::island::testutil::{ISLAND_SPEED, parse_world_in, standard_world, test_data};
use crate::island::worlds::{STANDARD_WORLD_ID, world_path};
use crate::island::{CreateOptions, Island, WorldDef, add_island_player, create_island_in, step_island};
use crate::state::World;
use crate::state::shared::CombatSettings;

/// Go: `standardLookup`: nur die Standard-Welt ist bekannt.
fn standard_lookup(id: &str) -> Result<WorldDef, String> {
    if id == STANDARD_WORLD_ID {
        Ok(standard_world(&test_data()))
    } else {
        Err(format!("Welt {id:?} unbekannt"))
    }
}

/// Go: `ResolveWorld(w, lookup)` auf den eingebetteten Daten.
fn resolve(w: &WorldDef, lookup: &dyn Fn(&str) -> Result<WorldDef, String>) -> Result<WorldDef, String> {
    let b = base()?;
    resolve_world(w, &k3c_data::Source::Embedded, &b.data, &b.rules, lookup)
}

fn overrides(files: &[(&str, Value)]) -> BTreeMap<String, Value> {
    files.iter().map(|(k, v)| ((*k).to_string(), v.clone())).collect()
}

/// Go: `teuerWorld`: erbt von der Standard-Welt, Mauer Gold 9, Bauern-Tempo 5.
fn teuer_world() -> WorldDef {
    let mut w = standard_world(&test_data());
    (w.id, w.name, w.extends) = ("teuer".into(), "Teuer".into(), STANDARD_WORLD_ID.into());
    w.overrides = overrides(&[
        ("buildings", json!({"wall": {"cost": {"gold": 9}}})),
        ("troops", json!({"peasant": {"speed": 5}})),
    ]);
    resolve(&w, &standard_lookup).unwrap_or_else(|e| panic!("{e}"))
}

fn island_in(world: &WorldDef, seed: &str) -> Island {
    let data = test_data();
    let depths = world.islands[0].depths.clone();
    create_island_in(&data, world, seed, &depths, ISLAND_SPEED, CreateOptions::default())
        .unwrap_or_else(|e| panic!("{e}"))
}

/// Go: `worldRun`: erste Insel der Welt, zwei Spieler, 40 Sekunden; die Stufen als JSON.
fn world_run(world: &WorldDef) -> String {
    let data = test_data();
    let mut isl = island_in(world, "werte-je-raum");
    add_island_player(&mut isl, 0);
    add_island_player(&mut isl, 0); // 2 Spieler wie 1
    for _ in 0..40 * 60 {
        step_island(&mut isl, &[], &data.difficulty, 1.0 / 60.0);
    }
    serde_json::to_string(&isl.stages).unwrap_or_else(|e| panic!("{e}"))
}

/// Go: `wallCost`: Gold der ersten Mauer in Stufe 0.
fn wall_cost(world: &WorldDef) -> i64 {
    let isl = island_in(world, "mauer");
    let w = &isl.stages[0];
    let s = w
        .sites
        .iter()
        .find(|s| s.kind == "wall")
        .unwrap_or_else(|| panic!("keine Mauer in Stufe 0"));
    site_cost(w, s).gold
}

/// B-681/AC-01: Zwei Welten mit verschiedenen Kosten rechnen nebeneinander (Threads) mit ihren eigenen Werten.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story) und RU18 (site_cost, step_players)"]
fn wertesatz_je_welt_parallel_unabhaengig() {
    let (std, teuer) = (standard_world(&test_data()), teuer_world());
    let base = default_values();
    let (base_wall, base_speed) = (base.buildings["wall"].cost.gold, base.troops["peasant"].speed);
    assert_eq!(wall_cost(&std), base_wall, "Standard-Welt: Mauer wie Basis");
    assert_eq!(wall_cost(&teuer), 9, "Welt teuer: Mauer 9 Gold (Override)");
    let (solo_std, solo_teuer) = (world_run(&std), world_run(&teuer));
    assert_ne!(solo_std, solo_teuer, "Bauern-Tempo der Welt teuer wirkt nicht");
    let worlds = [&std, &teuer, &std, &teuer];
    let got: Vec<String> = std::thread::scope(|s| {
        let hs: Vec<_> = worlds.iter().map(|w| s.spawn(|| world_run(w))).collect();
        hs.into_iter().map(|h| h.join().unwrap_or_default()).collect()
    });
    for (i, want) in [&solo_std, &solo_teuer, &solo_std, &solo_teuer].iter().enumerate() {
        assert_eq!(&got[i], *want, "Lauf {i} nebenläufig anders als allein");
    }
    let after = default_values();
    assert!(after.buildings["wall"].cost.gold == base_wall && after.troops["peasant"].speed == base_speed);
}

/// B-681/AC-02: Ohne Overrides in der Kette gilt der Basis-Wertesatz; der Zustand ist byte-gleich zur Insel der Basis.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story) und RU18 (create_world: new_hub_site, step_players)"]
fn wertesatz_ohne_overrides_byte_gleich() {
    let mut kind = standard_world(&test_data());
    (kind.id, kind.extends) = ("kind".into(), STANDARD_WORLD_ID.into());
    let kind = resolve(&kind, &standard_lookup).unwrap_or_else(|e| panic!("{e}"));
    let mut got = (**kind.values.as_ref().unwrap_or_else(|| panic!("Wertesatz"))).clone();
    got.combat = CombatSettings::default(); // die Kampfvariante steht in der Welt-Datei, nicht in data/
    assert_eq!(
        got,
        *default_values(),
        "Welt ohne Overrides: Wertesatz weicht von der Basis ab"
    );
    let base = world_run(&standard_world(&test_data()));
    for w in [standard_world(&test_data()), kind] {
        assert_eq!(
            world_run(&w),
            base,
            "Welt {}: Zustand nicht byte-gleich zur Basis",
            w.id
        );
    }
}

/// Ein ungültiger Override lehnt die Welt beim Auflösen ab (welten.md § 1.2), die Meldung nennt die Welt-Datei.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn resolve_world_ungueltiger_override() {
    let mut w = standard_world(&test_data());
    w.id = "kaputt".into();
    w.overrides = overrides(&[("buildings", json!({"wall": {"cost": {"gold": "viel"}}}))]);
    let err = resolve(&w, &standard_lookup).err().unwrap_or_default();
    assert!(err.contains("data/worlds/kaputt.json"), "Fehler {err:?}");
    let depths = w.islands[0].depths.clone();
    let r = create_island_in(&test_data(), &w, "kaputt", &depths, 1.0, CreateOptions::default());
    assert!(r.is_err(), "Insel einer Welt mit ungültigem Override angelegt");
}

/// Eine Zahl kommt nie aus zwei Quellen: Münzen je Treffer, Truhen- und Morgengold folgen demselben Wertesatz.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story) und RU18 (values_from: check_presets; dawn_gold)"]
fn wertesatz_eine_quelle_je_wert() {
    let mut w = standard_world(&test_data());
    (w.id, w.name, w.extends) = ("stark".into(), "Stark".into(), STANDARD_WORLD_ID.into());
    w.overrides = overrides(&[
        ("troops", json!({"peasant": {"hp": 77}, "vagrant": {"hp": 66}})),
        ("monarch", json!({"hitCoins": 3})),
        ("economy", json!({"chestGold": [50, 60], "dawnGoldPerDepth": 7})),
    ]);
    let world = resolve(&w, &standard_lookup).unwrap_or_else(|e| panic!("{e}"));
    let mut isl = island_in(&world, "eine-quelle");
    let pi = add_island_player(&mut isl, 0);
    let st = &mut isl.stages[0];
    let v = values(st);
    assert!(
        v.monarch.hit_coins == 3 && st.players[pi].crown.is_none(),
        "Monarch hitCoins"
    );
    assert_eq!(v.economy.chest_gold, [50, 60], "chestGold");
    assert_eq!(
        dawn_gold(st),
        v.economy.dawn_gold_per_player + 7 * st.biome.depth,
        "dawnGold"
    );
    let hub = st.hub_x;
    let ti = crate::unit::spawn_vagrant(st, hub, hub);
    assert!(
        st.troops[ti].hp == 66.0 && st.troops[ti].max_hp == 66.0,
        "Landstreicher 66/66"
    );
}

/// Go: `dawnGold` (economy.go), RU18.
fn dawn_gold(_w: &World) -> i64 {
    panic!("RU18: dawn_gold")
}

/// Test-Welt `id` ohne Geschichte (Kopf wie `skins_world`), optional mit `extends`.
fn plain_world(id: &str, extends: &str) -> WorldDef {
    let extra = if extends.is_empty() {
        String::new()
    } else {
        format!(r#""extends":"{extends}","#)
    };
    let raw = test_world(&test_island("a", ""))
        .replacen("\"islands\"", &format!("{extra}\"islands\""), 1)
        .replacen(r#""id":"test""#, &format!(r#""id":"{id}""#), 1);
    parse_world_in(&raw, id).unwrap_or_else(|e| panic!("{e}"))
}

fn lookup_in(worlds: &[WorldDef]) -> impl Fn(&str) -> Result<WorldDef, String> + '_ {
    move |id| {
        worlds
            .iter()
            .find(|w| w.id == id)
            .cloned()
            .ok_or_else(|| format!("{}: Datei fehlt", world_path(id)))
    }
}

/// Wie `TestWertesatzOhneOverridesByteGleich` (Teil Wertesatz): Ohne Overrides, auch über `extends`, ist der Wertesatz der
/// Basis-Wertesatz (bis auf die Kampfvariante der Welt) und wird geteilt, nicht kopiert.
#[test]
fn own_resolve_world_ohne_overrides_gleich_basis() {
    let (eltern, kind) = (plain_world("eltern", ""), plain_world("kind", "eltern"));
    let r = resolve(&kind, &lookup_in(&[eltern])).unwrap_or_else(|e| panic!("{e}"));
    let got = r.values.unwrap_or_else(|| panic!("Wertesatz"));
    let mut plain = (*got).clone();
    plain.combat = CombatSettings::default();
    assert_eq!(
        plain,
        *default_values(),
        "Welt ohne Overrides: Wertesatz weicht von der Basis ab"
    );
    let again = values_of(&plain_world("eltern", "")).unwrap_or_else(|e| panic!("{e}"));
    if again.combat == default_values().combat {
        assert!(
            std::sync::Arc::ptr_eq(&again, &default_values()),
            "Basis-Wertesatz wird geteilt"
        );
    }
}

/// Wie `TestResolveWorldUngueltigerOverride`: Fehler mit Welt-Datei, und keine Insel aus der Welt.
#[test]
fn own_resolve_world_ungueltiger_override() {
    let mut w = plain_world("kaputt", "");
    w.overrides = overrides(&[("buildings", json!({"wall": {"cost": {"gold": "viel"}}}))]);
    let err = resolve(&w, &lookup_in(&[])).err().unwrap_or_default();
    assert_eq!(
        err,
        "data/worlds/kaputt.json: overrides › data/buildings.json › wall.cost.gold: Ganzzahl erwartet, ist Text"
    );
    let depths = w.islands[0].depths.clone();
    let r = create_island_in(&test_data(), &w, "kaputt", &depths, 1.0, CreateOptions::default());
    assert!(
        r.as_ref().err().is_some_and(|e| e.contains("data/worlds/kaputt.json")),
        "Insel einer Welt mit ungültigem Override: {:?}",
        r.err()
    );
}

/// `resolve_world` mischt die Kampfvariante der Kette (Kind gewinnt) in den Wertesatz, die Basis bleibt.
#[test]
fn own_resolve_world_kampfvariante_der_kette() {
    let mut eltern = plain_world("eltern", "");
    eltern.combat.mode = "groups".into();
    let mut kind = plain_world("kind", "eltern");
    kind.combat.group_size = 4;
    let r = resolve(&kind, &lookup_in(&[eltern])).unwrap_or_else(|e| panic!("{e}"));
    let v = r.values.unwrap_or_else(|| panic!("Wertesatz"));
    assert_eq!((v.combat.mode.as_str(), v.combat.group_size), ("groups", 4));
    assert_eq!(default_values().combat, CombatSettings::default(), "Basis unverändert");
}
