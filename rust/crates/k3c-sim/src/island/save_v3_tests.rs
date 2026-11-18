//! Spielstand Version 2 und 3 (Go: `island_save_v3_test.go`, S1.4, B-022): Pool, Verteilung und Slots im Stand, Migration aus 1
//! und 2, Fehlerfälle. Die Go-Tests, die eine Insel bauen oder laden (`create_world_with`, `values_of`: RU14; Skill-Hilfen:
//! RU18), tragen `ignore`. `island_save_v3_fehler` liest nur (`parse_island_save`) und läuft; `own_*` prüft die Lese-Seite der
//! übrigen und `restore_pool` ohne Insel-Bau.

use serde_json::{Value, json};

use super::save_tests::{Env, fixture};
use super::save_v3::{parse_island_v2, restore_pool};
use super::testutil::*;
use super::{Island, IslandSave, add_island_player, step_island};
use crate::state::World;
use crate::state::save::HubSave;

fn fixture_map(version: i64) -> Value {
    serde_json::from_str(&fixture(version)).expect("Fixture ist JSON")
}

/// Das Fixture v2 mit drei Stufen und den Zählern 0, 2, 1 (Go: `v2 mit drei Stufen`).
fn v2_mit_drei_stufen() -> Value {
    let mut v2 = fixture_map(2);
    let mut third = v2["stages"][1].clone();
    third["depth"] = json!(2);
    v2["stages"].as_array_mut().expect("Stufen").push(third);
    v2["stageSkillPoints"] = json!([0, 2, 1]);
    v2
}

/// Go: `TestIslandSaveV3Fehler`: Ungültige Verteilungen ergeben einen Fehler (nur Lesen, die Quelldatei bleibt ein Text).
#[test]
fn island_save_v3_fehler() {
    let env = Env::new();
    type Mutate = fn(&mut Value);
    let cases: [(&str, Mutate); 7] = [
        ("unbekannter Skill", |m| {
            m["players"][0]["skills"] = json!(["taunt", "shadowStrike"])
        }),
        ("doppelter Skill", |m| {
            m["players"][1]["skills"] = json!(["fireball", "fireball"])
        }),
        ("mehr Skills als Pool", |m| m["skillPool"] = json!(2)),
        ("Gating verletzt", |m| {
            m["players"][0]["skills"] = json!(["taunt", "armorAura"])
        }),
        ("Slot nicht gelernt", |m| {
            m["players"][1]["slots"] = json!(["fireball", "heal"])
        }),
        ("Passiv im Slot", |m| m["players"][0]["slots"] = json!(["armorAura"])),
        ("Pool fehlt", |m| {
            m.as_object_mut().expect("Objekt").remove("skillPool");
        }),
    ];
    for (name, mutate) in cases {
        let mut m = fixture_map(3);
        mutate(&mut m);
        let err = env.parse(&m.to_string()).expect_err(name);
        assert!(err.to_string().starts_with("spielstand: "), "{name}: {err}");
    }
}

/// Go: `TestIslandSaveV3Migration` (Lese-Seite): v1 und v2 lesen mit leerer Verteilung, der Pool folgt der Migrationsregel
/// (Summe der Zähler je Stufe).
#[test]
fn own_migration_v1_v2_liest_pool_und_leere_verteilung() {
    let env = Env::standard();
    let v2 = v2_mit_drei_stufen().to_string();
    for (name, raw, pool) in [
        ("v1", fixture(1), 0),
        ("v2", fixture(2), 2),
        ("v2 mit drei Stufen", v2, 3),
    ] {
        let s = env.parse(&raw).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!((s.version, s.skill_pool), (9, pool), "{name}: Version und Pool");
        for p in &s.players {
            assert!(
                p.skills.is_empty() && p.slots.is_empty(),
                "{name}: Spieler {} hat Verteilung",
                p.index
            );
        }
        assert_eq!(
            (s.world.as_str(), s.island_id.as_str()),
            ("standard", "island1"),
            "{name}: Standard-Welt"
        );
    }
}

/// `parse_island_v2` selbst: Fehlertexte wie in Go, das Pool-Feld des Stands bleibt bestehen, die Zähler kommen dazu.
#[test]
fn own_parse_island_v2_fehler_und_summe() {
    let env = Env::new();
    let t = env.tables();
    let mut m = fixture_map(2);
    assert_eq!(parse_island_v2(&m.to_string(), &t).expect("v2").skill_pool, 2);
    m["stageSkillPoints"] = json!([1]);
    assert_eq!(
        parse_island_v2(&m.to_string(), &t).expect_err("Zähler").to_string(),
        "spielstand: 1 Skill-Punkt-Zähler für 2 Stufen"
    );
    m.as_object_mut().expect("Objekt").remove("stageSkillPoints");
    assert_eq!(
        parse_island_v2(&m.to_string(), &t).expect_err("fehlt").to_string(),
        "spielstand: Feld stageSkillPoints fehlt"
    );
    m["stageSkillPoints"] = json!(["a", "b"]);
    assert!(parse_island_v2(&m.to_string(), &t).is_err(), "Zähler vom falschen Typ");
}

/// Go: `TestIslandSaveV3FixtureLaedt` (Lese-Seite): Das Fixture v3 liest mit Pool und Verteilung.
#[test]
fn own_fixture_v3_liest_pool_und_verteilung() {
    let s = Env::standard().parse(&fixture(3)).expect("v3");
    assert_eq!((s.skill_pool, s.players.len()), (5, 2));
    assert_eq!(s.players[0].skills, ["taunt", "shieldBash", "armorAura"]);
    assert_eq!(s.players[1].slots, ["fireball", "iceWall"]);
}

/// `restore_pool`: Pool in alle Stufen, Truhen-Zähler aus den geöffneten Truhen aller Stufen (`chest@…`).
#[test]
fn own_restore_pool_spiegelt_pool_und_zaehlt_truhen() {
    let mut isl = Island {
        stages: vec![World::default(), World::default()],
        chests_opened: 99,
        ..Island::default()
    };
    let hub = |taken: &[&str]| HubSave {
        pickups_taken: taken.iter().map(|k| (*k).to_string()).collect(),
        ..HubSave::default()
    };
    let s = IslandSave {
        skill_pool: 4,
        stages: vec![hub(&["chest@12", "tree@3"]), hub(&["chest@40", "chest@41"])],
        ..IslandSave::default()
    };
    restore_pool(&mut isl, &s);
    assert_eq!((isl.skill_pool, isl.chests_opened), (4, 3));
    assert!(isl.stages.iter().all(|w| w.skill_points == 4));
}

/// Go: `TestIslandSaveV3Rundlauf`: Pool, Verteilung und Slots zweier Spieler bleiben erhalten; Speichern, Laden, Speichern
/// ist gleich.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_save_v3_rundlauf() {
    let data = test_data();
    let mut isl = must_island(&data, "stand-v3", &[0, 1]);
    let (a, b) = (add_island_player(&mut isl, 0), add_island_player(&mut isl, 1));
    add_pool_points(&mut isl.stages[0], 4);
    must_learn(&mut isl.stages[0], a, &["taunt", "shieldBash", "armorAura"]);
    must_learn(&mut isl.stages[1], b, &["heal", "fireball"]);
    isl.stages[1].players[b].slots[0] = String::new(); // freier Slot bleibt frei
    let first = save_json(&isl, "2026-10-04T10:00:00Z");
    let loaded = load_island(&data, &first);
    assert_eq!(
        first,
        save_json(&loaded, "2026-10-04T10:00:00Z"),
        "Speichern, Laden, Speichern"
    );
    assert_eq!((loaded.skill_pool, loaded.stages[1].skill_points), (4, 4));
    assert_eq!(available_points(&loaded.stages[1], 0), 2, "frei nach dem Laden");
}

/// Go: `TestIslandSaveV3Migration`: v1 und v2 laden mit leerer Verteilung, der Pool folgt der Migrationsregel.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_save_v3_migration() {
    let data = test_data();
    for (name, raw, pool) in [
        ("v1", fixture(1), 0),
        ("v2", fixture(2), 2),
        ("v2 mit drei Stufen", v2_mit_drei_stufen().to_string(), 3),
    ] {
        let isl = load_island(&data, &raw);
        assert_eq!(
            (isl.skill_pool, isl.stages[0].skill_points),
            (pool, pool),
            "{name}: Pool"
        );
        for w in &isl.stages {
            for p in &w.players {
                assert!(p.skills.is_empty() && p.slots.is_empty(), "{name}: Spieler {}", p.index);
            }
        }
    }
}

/// Go: `TestIslandSaveV3FixtureLaedt`: Das Fixture v3 lädt mit Pool und Verteilung.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_save_v3_fixture_laedt() {
    let isl = load_island(&test_data(), &fixture(3));
    let players: Vec<_> = isl.stages.iter().flat_map(|w| &w.players).collect();
    assert_eq!((isl.skill_pool, players.len()), (5, 2));
    assert_eq!(players[0].skills, ["taunt", "shieldBash", "armorAura"]);
    assert_eq!(players[1].slots, ["fireball", "iceWall"]);
}

/// Go: `TestIslandSaveV3TruhenZaehler`: Der Truhen-Zähler folgt aus den geöffneten Truhen des Stands: nach zwei Truhen gibt
/// die dritte den Punkt.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_save_v3_truhen_zaehler() {
    let data = test_data();
    let mut s = Env::standard().parse(&fixture(3)).expect("v3");
    let base = must_island(&data, &s.seed, &[0, 1]);
    let mut opened = 0;
    for (i, w) in base.stages.iter().enumerate() {
        for pk in w.pickups.iter().filter(|pk| pk.kind == "chest") {
            if opened < 2 {
                s.stages[i].pickups_taken.push(format!("{}@{}", pk.kind, pk.x)); // Go: `key(kind, x)`
                opened += 1;
            }
        }
    }
    assert_eq!(opened, 2, "nur {opened} Truhen in den Stufen");
    let mut isl = load_island(&data, &serde_json::to_string(&s).expect("schreiben"));
    assert_eq!(isl.chests_opened, 2, "Truhen-Zähler");
    let (id, x) = (isl.stages[0].new_id(), isl.stages[0].players[0].x);
    isl.stages[0].pickups.push(crate::state::Pickup {
        id,
        kind: "chest".into(),
        x,
    });
    step_island(
        &mut isl,
        &[Default::default(), Default::default()],
        &data.difficulty,
        DT,
    );
    assert_eq!(
        (isl.chests_opened, isl.skill_pool, isl.stages[1].skill_points),
        (3, 6, 6),
        "3. Truhe nach dem Laden"
    );
}

/// Go: `TestIslandSaveV3SpaeterBeitritt`: Wer nach dem Laden beitritt, hat alle Pool-Punkte für sich (Preset plus frei).
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn island_save_v3_spaeter_beitritt() {
    let data = test_data();
    let mut isl = load_island(&data, &fixture(3));
    let pi = add_island_player(&mut isl, 0);
    let p = &isl.stages[0].players[pi];
    assert_eq!(p.index, 2);
    assert_eq!(
        p.skills.len() as i64 + available_points(&isl.stages[0], pi),
        isl.skill_pool
    );
}
