//! Spielstand der Insel (Go: `island_save_test.go`, SP12.3): Rundlauf, Stände der Version 1 und Fehlerfälle.
//!
//! Die Go-Tests bauen die Insel mit `must_island` (`create_world_with`, RU14): `island_save_round_trip`,
//! `parse_island_save_from_version1`, `version1_players_move_into_stored_stage`, `island_save_monarch_an_der_burg` tragen
//! `ignore`. Zwei Go-Tests laufen schon, mit dem Spielstand der Fixture `testdata/saves/v9/familie.json` statt eines
//! frisch gebauten (`ToSave` braucht eine gebaute Insel): `parse_island_save_rejects_bad_input` und
//! `version2_is_not_read_by_campaign_parser`. Dazu `save_v9_rundlauf` (AC-03) und Tests ohne Go-Gegenstück.

use std::path::PathBuf;

use k3c_data::Data;
use k3c_data::monarch::SkillData;
use k3c_det::golden;
use serde_json::{Value, json};

use super::save::island_from_v1;
use super::testutil::*;
use super::{
    ISLAND_SAVE_VERSION, IslandDef, IslandSave, SaveTables, WorldDef, add_island_player, default_options,
    parse_island_save,
};
use crate::state::save::parse_save;
use crate::state::{Fehler, PlayerCommand, SkillTable, WorldValues};

type Mutate = fn(&mut Value);

/// Go: `readFixture`: `testdata/saves/v<version>/familie.json`.
pub(super) fn fixture(version: i64) -> String {
    let path: PathBuf = golden::testdata().join(format!("saves/v{version}/familie.json"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// Die Tabellen zum Lesen eines Stands der Version 8 oder 9 (kein `migrate_world`, daher keine Standard-Welt nötig).
pub(super) struct Env {
    pub(super) data: Data,
    pub(super) defaults: WorldValues,
    pub(super) world: WorldDef,
}

impl Env {
    pub(super) fn new() -> Self {
        let data = test_data();
        // Der Basis-Wertesatz mit Skill-Katalog (Go: `defaultValues()`, `skillTable`: nach ID sortiert, mit Stelle je ID).
        let list: Vec<_> = data
            .monarch
            .skills
            .iter()
            .map(|(id, s)| SkillData {
                id: id.clone(),
                ..s.clone()
            })
            .collect();
        let index = list.iter().enumerate().map(|(i, s)| (s.id.clone(), i)).collect();
        let mut defaults = crate::enemies::testutil::test_values();
        defaults.skills = SkillTable { list, index };
        Env {
            data,
            defaults,
            world: WorldDef::default(),
        }
    }

    pub(super) fn tables(&self) -> SaveTables<'_> {
        SaveTables {
            data: &self.data,
            defaults: &self.defaults,
            standard_world: &self.world,
        }
    }

    pub(super) fn parse(&self, raw: &str) -> Result<IslandSave, Fehler> {
        parse_island_save(raw, &self.tables())
    }

    /// Wie `new`, mit der Standard-Welt aus einer Insel `island1` (Go: `standardWorld`): Stände bis Version 7 kommen dorthin
    /// (`migrate_world`).
    pub(super) fn standard() -> Self {
        let mut env = Env::new();
        env.world = world_of(&["island1"]);
        env
    }
}

/// Eine Welt `standard` mit Inseln dieser IDs, von Hand (Go: eine `WorldDef` aus der Welt-Datei, RU10.5).
pub(super) fn world_of(ids: &[&str]) -> WorldDef {
    WorldDef {
        id: "standard".into(),
        islands: ids
            .iter()
            .map(|id| IslandDef {
                id: (*id).into(),
                ..IslandDef::default()
            })
            .collect(),
        ..WorldDef::default()
    }
}

/// AC-03: Der Stand der Version 12 (RU62.10; Fixture v12, aus v11 mit Versionsnummer) wird gelesen und ergibt geschrieben denselben JSON-Baum (jede Zahl mit denselben Bits;
/// `null` und leere Liste gelten als gleich). Lese- und Schreibweg des `IslandSave`-Werts, ohne `create_world_with`.
#[test]
fn save_v9_rundlauf() {
    let env = Env::new();
    let raw = fixture(12);
    let s = env.parse(&raw).expect("Version 12 lesen");
    assert_eq!(s.version, ISLAND_SAVE_VERSION);
    assert_eq!(
        (s.world.as_str(), s.island_id.as_str(), s.stages.len()),
        ("standard", "island1", 2)
    );
    let want: Value = serde_json::from_str(&raw).expect("Fixture ist JSON");
    let d = golden::diff("saves/v12/familie.json", &want, &golden::tree(&s));
    assert!(d.is_empty(), "{d}");
    // Zweiter Durchgang über den Text: Lesen des geschriebenen Stands ergibt denselben Wert.
    let again = env
        .parse(&serde_json::to_string(&s).expect("schreiben"))
        .expect("erneut lesen");
    assert_eq!(again, s);
}

/// Die Version 8 (ohne geräumte Lager) wird wie die 9 gelesen; die Version steht danach auf 9.
#[test]
fn save_v8_wird_als_v9_gelesen() {
    let env = Env::new();
    let s = env.parse(&fixture(8)).expect("Version 8 lesen");
    assert_eq!(s.version, ISLAND_SAVE_VERSION);
    assert!(
        !s.world.is_empty() && !s.island_id.is_empty(),
        "Version 8 nennt Welt und Insel"
    );
}

/// Go: `TestParseIslandSaveRejectsBadInput`, mit der Fixture der Version 9 als Ausgangsstand.
#[test]
fn parse_island_save_rejects_bad_input() {
    let env = Env::new();
    let m: Value = serde_json::from_str(&fixture(9)).expect("Fixture ist JSON");
    let cases: [(&str, Mutate); 5] = [
        ("Version", |m| m["version"] = json!(99)),
        ("Pflichtfeld", |m| {
            m.as_object_mut().expect("Objekt").remove("stock");
        }),
        ("Spieler ohne Stufe", |m| {
            m["players"] = json!([{"index": 0, "gold": 1, "depth": 2}]);
        }),
        ("doppelter Index", |m| {
            m["players"] = json!([{"index": 0, "gold": 1, "depth": 0}, {"index": 0, "gold": 1, "depth": 0}]);
        }),
        ("unbekannte Tiefe", |m| m["stages"][0]["depth"] = json!(9)),
    ];
    for (name, mutate) in cases {
        let mut cp = m.clone();
        mutate(&mut cp);
        let err = env.parse(&cp.to_string()).expect_err(name);
        assert!(err.to_string().starts_with("spielstand: "), "{name}: {err}");
    }
    assert!(env.parse("kein json").is_err(), "kein JSON: Fehler erwartet");
}

/// Go: `TestVersion2IsNotReadByCampaignParser`: `parse_save` liest nur Version 1 und lehnt einen Insel-Stand ab.
#[test]
fn version2_is_not_read_by_campaign_parser() {
    let env = Env::new();
    let err = parse_save(&fixture(9), &env.data.biomes, &env.defaults).expect_err("Version 9");
    assert!(err.to_string().starts_with("spielstand: "), "{err}"); // Go prüft erst die Pflichtfelder, dann die Version
}

/// Die Fehlertexte der Prüfungen beim Lesen, Zeichen für Zeichen wie in Go.
#[test]
fn own_fehlertexte_wie_in_go() {
    let env = Env::new();
    let m: Value = serde_json::from_str(&fixture(9)).expect("Fixture ist JSON");
    let err = |f: fn(&mut Value)| {
        let mut cp = m.clone();
        f(&mut cp);
        env.parse(&cp.to_string()).expect_err("Fehler erwartet").to_string()
    };
    assert_eq!(
        err(|m| m["version"] = json!(99)),
        "spielstand: Version 99, erwartet 1, 2, 3, 4, 5, 6, 7, 8 oder 12"
    );
    assert_eq!(
        err(|m| {
            m.as_object_mut().expect("Objekt").remove("stock");
        }),
        "spielstand: Feld stock fehlt"
    );
    assert_eq!(
        err(|m| m["stages"][0]["depth"] = json!(9)),
        "spielstand: unbekannte oder doppelte Tiefe 9"
    );
    assert_eq!(
        err(|m| m["players"] = json!([{"index": 3, "gold": 1, "depth": 7}])),
        "spielstand: Spieler 3 ungültig"
    );
    assert_eq!(
        err(|m| m["defeatedBosses"] = json!(["drache"])),
        "spielstand: besiegter Boss \"drache\" unbekannt oder doppelt"
    );
    assert_eq!(
        err(|m| m["defeatedBosses"] = json!(["goblinLeader", "goblinLeader"])),
        "spielstand: besiegter Boss \"goblinLeader\" unbekannt oder doppelt"
    );
    assert_eq!(
        err(|m| m["goldCollected"] = json!(-1)),
        "spielstand: Münzzähler -1 negativ"
    );
    assert_eq!(
        err(|m| m["players"][0]["skillBase"] = json!(1)),
        "spielstand: Spieler 0: Skill-Basis 1 bei 0 Skills"
    );
    assert_eq!(
        err(|m| m["merchant"]["hp"] = json!(0)),
        "spielstand: Händler-Besuche 3 oder Händler-HP ungültig"
    );
    assert_eq!(err(|m| m["skillPool"] = json!(-1)), "spielstand: Pool -1 negativ");
    assert_eq!(
        err(|m| m["options"]["grade"] = json!("x")),
        "insel: unbekannter Wert \"x\" für Grad"
    );
    assert_eq!(
        err(|m| m["players"][0]["skills"] = json!(["gibtesnicht"])),
        "spielstand: Spieler 0 hat 1 Skills, Pool 0"
    );
}

/// Felder, die im Stand fehlen, behalten den Standard; bei `options` je Feld (Go liest in den vorbelegten Wert).
#[test]
fn own_options_fehlen_im_stand_bleiben_standard() {
    let env = Env::new();
    let def = default_options(&env.data.difficulty);
    let mut m: Value = serde_json::from_str(&fixture(9)).expect("Fixture ist JSON");
    assert_eq!(env.parse(&m.to_string()).expect("lesen").options.defeat, "stage");
    m.as_object_mut().expect("Objekt").remove("options");
    assert_eq!(env.parse(&m.to_string()).expect("ohne options").options, def);
    m["options"] = json!({"grade": "hard"});
    let o = env.parse(&m.to_string()).expect("teilweise options").options;
    assert_eq!(
        (o.grade.as_str(), o.goal.as_str(), o.defeat.as_str()),
        ("hard", "endboss", "resources")
    );
    m["options"] = Value::Null;
    assert_eq!(env.parse(&m.to_string()).expect("options null").options, def);
}

/// `island_from_v1`: Die Hubs werden Stufen (fehlende Tiefen 0..2 kommen leer dazu), die Vorräte zählen zusammen, alle
/// Spieler stehen in der gespeicherten Tiefe. Der Stand ist die Fixture der Version 1; das Lesen selbst (`parse_save`,
/// `migrate_world`) gehört zu RU8 und RU10.3 (dort `save_v8_tests.rs`).
#[test]
fn own_island_from_v1_ueberfuehrt_die_campaign() {
    let env = Env::new();
    let old = parse_save(&fixture(1), &env.data.biomes, &env.defaults).expect("Version 1 lesen");
    let s = island_from_v1(&old, default_options(&env.data.difficulty));
    assert_eq!(
        (s.version, s.campaign_id.as_str(), s.seed.as_str()),
        (12, old.campaign_id.as_str(), old.seed.as_str())
    );
    let depths: Vec<i64> = s.stages.iter().map(|h| h.depth).collect();
    assert!(
        depths.windows(2).all(|w| w[0] < w[1]) && (0..=2).all(|d| depths.contains(&d)) && depths.contains(&old.depth),
        "Stufen nach Tiefe, mindestens 0..2 und die gespeicherte: {depths:?}"
    );
    let sum = |f: fn(&k3c_data::hub::Stock) -> i64| old.hubs.iter().map(|h| f(&h.stock)).sum::<i64>();
    assert_eq!(
        (s.stock.wood, s.stock.stone, s.stock.copper),
        (sum(|k| k.wood), sum(|k| k.stone), sum(|k| k.copper)),
        "Vorräte der Hubs zählen zusammen"
    );
    assert_eq!(s.skill_pool, old.skill_points);
    assert_eq!(s.players.len(), old.players.len());
    for (i, (p, o)) in s.players.iter().zip(&old.players).enumerate() {
        assert_eq!((p.index, p.gold, p.depth), (i as i64, o.gold, old.depth), "Spieler {i}");
    }
    assert_eq!(s.options, default_options(&env.data.difficulty));
}

#[test]
fn island_save_round_trip() {
    let data = test_data();
    let mut isl = must_island(&data, "stand-a", &[0, 1, 2]);
    let a = add_island_player(&mut isl, 0);
    let b = add_island_player(&mut isl, 2);
    let c = add_island_player(&mut isl, 0);
    isl.stages[0].players[a].gold = 61;
    isl.stages[2].players[b].gold = 22;
    isl.stages[0].players[c].gold = 5;
    crate::enemies::testutil::build_site(&mut isl.stages[0], "wall");
    isl.skill_pool += 3; // Pool der Insel, alle Stufen spiegeln ihn (S1.4)
    isl.stock.wood += 40;
    isl.stock.copper += 9;
    run_island(
        &mut isl,
        &data,
        &[
            PlayerCommand {
                move_x: 1.0,
                ..PlayerCommand::default()
            },
            PlayerCommand::default(),
            PlayerCommand::default(),
        ],
        20.0,
    );

    let first = save_json(&isl, "2026-10-02T10:00:00Z");
    let loaded = load_island(&data, &first);
    let second = save_json(&loaded, "2026-10-02T10:00:00Z");
    assert_eq!(first, second, "Speichern, Laden, Speichern ergibt etwas anderes");
    let (want, got) = (isl.players(), loaded.players());
    for (p, q) in want.iter().zip(&got) {
        assert!(
            q.index == p.index && q.gold == p.gold,
            "Spieler {}: Index/Gold weichen ab",
            p.index
        );
    }
    assert!(
        loaded.stock.wood == isl.stock.wood && loaded.stock.copper == 9 && loaded.skill_pool == isl.skill_pool,
        "Vorrat oder Pool weichen ab: {:?}, Pool {} ({})",
        loaded.stock,
        loaded.skill_pool,
        isl.skill_pool
    );
    assert_eq!(loaded.stages[0].skill_points, loaded.skill_pool);
}

#[test]
fn parse_island_save_from_version1() {
    let data = test_data();
    let raw = fixture(1);
    let isl = load_island(&data, &raw);
    let depths: Vec<i64> = isl.stages.iter().map(|w| w.biome.depth).collect();
    assert_eq!(depths, [0, 1, 2], "drei Stufen (Tiefe 0, 1, 2) erwartet");
    let old = parse_save(&raw, &data.biomes, &crate::world::vals(&isl.stages[0])).expect("Version 1");
    assert_eq!(
        (isl.id.as_str(), isl.seed.as_str()),
        (old.campaign_id.as_str(), old.seed.as_str())
    );
    assert_eq!(
        isl.stock.wood,
        old.hubs.iter().map(|h| h.stock.wood).sum::<i64>(),
        "Vorräte der Hubs zählen zusammen"
    );
}

#[test]
fn version1_players_move_into_stored_stage() {
    let data = test_data();
    let raw = fixture(1);
    let isl = load_island(&data, &raw);
    let old = parse_save(&raw, &data.biomes, &crate::world::vals(&isl.stages[0])).expect("Version 1");
    let got = isl.players();
    assert_eq!(got.len(), old.players.len());
    for (q, o) in got.iter().zip(&old.players) {
        assert_eq!(q.gold, o.gold, "Gold der Spieler stimmt nicht");
        let si = isl.stage_of(q.index).expect("Stufe des Spielers");
        assert_eq!(
            isl.stages[si].biome.depth, old.depth,
            "Spieler steht in der gespeicherten Tiefe"
        );
    }
}

/// B-814/AC-02: Die Stelle des Monarchen steht nicht im Spielstand; nach dem Laden steht er wie beim Beitritt an der Burg
/// seiner Stufe, nicht an der gespeicherten Stelle (gewollt, `from_island_save_in`).
#[test]
fn island_save_monarch_an_der_burg() {
    let data = test_data();
    let mut isl = must_island(&data, "stand-burg", &[0, 1]);
    let (a, b) = (add_island_player(&mut isl, 0), add_island_player(&mut isl, 1));
    let (home_a, home_b) = (isl.stages[0].players[a].x, isl.stages[1].players[b].x);
    isl.stages[0].players[a].x = isl.stages[0].hub_x + 40.0;
    isl.stages[1].players[b].x = isl.stages[1].hub_x - 25.0;
    let loaded = load_island(&data, &save_json(&isl, "x"));
    for (i, home) in [home_a, home_b].into_iter().enumerate() {
        let w = &loaded.stages[i];
        let q = &w.players[0];
        assert!(
            q.index == i as i64 && home.is_finite() && q.x == w.hub_x + (2 * i as i64 - 1) as f64 * 3.0,
            "Spieler {i} nach dem Laden bei {}, erwartet die Burg bei {home}",
            q.x
        );
    }
}
