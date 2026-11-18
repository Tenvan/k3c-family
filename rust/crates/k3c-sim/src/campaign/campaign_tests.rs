//! Tests der Kampagne (Go: `campaign_test.go`). Die Golden-Läufe (`TestGoldenKampagne`, `TestGoldenKampagneAusbau`) rechnen
//! die aufgezeichneten Eingaben Tick für Tick und vergleichen Welt und Spielstand mit den Golden (RU22, G3); sie brauchen
//! den ganzen Schritt (RU18, RU19). Die übrigen Tests bauen eine Kampagne und warten auf Zyklus (RU14.2), Plätze und Spieler
//! (RU18) und `step` (RU19). `own_*` prüft, was ohne eine erzeugte Welt läuft.

use std::sync::Arc;

use k3c_det::golden;
use serde::Deserialize;
use serde_json::Value;

use super::{Campaign, new_world};
use crate::enemies::testutil::test_values;
use crate::state::pending::travel_points;
use crate::state::save::{SaveGame, key, parse_save};
use crate::state::{PlayerCommand, WorldValues};
use crate::world::base::base;
use crate::world::testutil::DT;
use crate::world::values_room::default_values;
use crate::world::{Options, step};

const CAMPAIGN_PATH: &str = "golden/campaign-abstieg.json.gz";
const CAMPAIGN_AUSBAU_PATH: &str = "golden/campaign-ausbau.json.gz";

/// Go: `campaignFile`, `testdata/golden/campaign-abstieg.json.gz` (gzip, GD1).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CampaignFile {
    name: String,
    seed: String,
    cycle_speed: f64,
    players: usize,
    dt: f64,
    ticks: i64,
    snapshot_every: i64,
    save_at: i64,
    saved_at: String,
    save: Value,
    inputs: Vec<Segment>,
    snapshots: Vec<Snapshot>,
}

#[derive(Debug, Deserialize)]
struct Segment {
    ticks: i64,
    commands: Vec<PlayerCommand>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Snapshot {
    tick: i64,
    depth: i64,
    unlocked_depth: i64,
    world: Value,
}

fn load_campaign_at(path: &str) -> CampaignFile {
    let f: CampaignFile = serde_json::from_value(golden::read(golden::testdata().join(path)))
        .unwrap_or_else(|e| panic!("{path} unbrauchbar: {e}"));
    assert!(
        !f.snapshots.is_empty() && f.save_at != 0 && !f.save.is_null(),
        "{path} unbrauchbar"
    );
    f
}

/// Go: `goldenPurse`: der Beutel der Zeit vor der Münzskala (100 Gold, BS3): Die aufgezeichneten Eingaben brauchen so viel
/// Gold, und die Läufe prüfen die Simulation, nicht die Werte der Wirtschaft.
fn golden_purse() -> Arc<WorldValues> {
    let mut v = (*default_values()).clone();
    v.economy.purse.start_gold = 100;
    v.economy.purse.max_gold = 100;
    Arc::new(v)
}

fn compare_campaign(f: &CampaignFile, snap: usize, c: &Campaign) {
    let s = &f.snapshots[snap];
    assert!(
        s.depth == c.depth && s.unlocked_depth == c.unlocked_depth,
        "Tick {}: Tiefe {}/{}, erwartet {}/{}",
        s.tick,
        c.depth,
        c.unlocked_depth,
        s.depth,
        s.unlocked_depth
    );
    let got = golden::tree(c.current_world().expect("Stufe"));
    let d = golden::diff("world", &s.world, &got);
    assert!(d.is_empty(), "{} Tick {}: {d}", f.name, s.tick);
}

/// Go: `saveAndLoad`: vergleicht den Spielstand mit TS und lädt ihn wie der Golden-Lauf.
fn save_and_load(f: &CampaignFile, c: &Campaign, vals: &Arc<WorldValues>) -> Campaign {
    let got = golden::tree(&c.to_save(&f.saved_at).expect("Spielstand"));
    let d = golden::diff("save", &golden::tree(&f.save), &got);
    assert!(d.is_empty(), "{} Tick {}: {d}", f.name, f.save_at);
    let biomes = &base().expect("Daten").data.biomes;
    let s = parse_save(&f.save.to_string(), biomes, &default_values()).expect("Spielstand lesen");
    let mut loaded = Campaign::from_save_with(&s, f.cycle_speed, Some(Arc::clone(vals))).expect("Kampagne");
    for _ in 0..f.players {
        loaded.join_player();
    }
    loaded
}

/// Go: `runGoldenKampagne`: spielt die Datei ab und liefert den gespeicherten Stand.
fn run_golden_kampagne(path: &str, want_depth: i64) -> Value {
    let f = load_campaign_at(path);
    let vals = golden_purse();
    let mut c = Campaign::create_with(&f.seed, "golden", f.cycle_speed, Some(Arc::clone(&vals))).expect("Kampagne");
    for _ in 0..f.players {
        c.join_player();
    }
    compare_campaign(&f, 0, &c);
    let (mut tick, mut snap) = (0, 0);
    for seg in &f.inputs {
        for _ in 0..seg.ticks {
            let w = c.current_world_mut().expect("Stufe");
            step(w, &seg.commands, None, f.dt);
            tick += 1;
            let to_depth = w.travel.as_ref().filter(|t| t.progress >= 1.0).map(|t| t.to_depth);
            if let Some(depth) = to_depth {
                c.travel(depth).expect("Stufenwechsel");
            }
            if tick == f.save_at {
                c = save_and_load(&f, &c, &vals);
            }
            if tick % f.snapshot_every == 0 {
                snap += 1;
                compare_campaign(&f, snap, &c);
            }
        }
    }
    let ok = tick == f.ticks && snap + 1 == f.snapshots.len() && c.depth == want_depth;
    assert!(
        ok,
        "{tick} Ticks, {} Snapshots, Tiefe {}; Datei hat {} Ticks",
        snap + 1,
        c.depth,
        f.ticks
    );
    f.save
}

#[test]
fn golden_kampagne() {
    run_golden_kampagne(CAMPAIGN_PATH, 2);
}

#[test]
fn golden_kampagne_ausbau() {
    // B-075: Spielstand mit ausgebautem Hub (Bauern angeworben, Bäume markiert und gefällt, Truhe geöffnet, Werkstatt gebaut,
    // Bogen bezahlt, Fels markiert), danach speichern, laden, weiterlaufen.
    let raw = run_golden_kampagne(CAMPAIGN_AUSBAU_PATH, 0);
    let save: SaveGame = serde_json::from_value(raw).expect("Spielstand");
    let h = &save.hubs[0];
    let built = h.sites.iter().filter(|s| s.state != "unpaid").count(); // gebaut oder bezahlt und im Bau
    let archers = h.troops.iter().filter(|t| t.kind == "archer").count();
    let leer = h.nodes_gone.is_empty() || h.nodes_marked.is_empty() || h.pickups_taken.is_empty();
    assert!(
        !(leer || built == 0 || archers == 0),
        "Stand zu leer: gebaut {built}, Bogenschützen {archers}"
    );
}

#[test]
fn spielstand_rundreise() {
    let f = load_campaign_at(CAMPAIGN_PATH);
    let biomes = &base().expect("Daten").data.biomes;
    let s = parse_save(&f.save.to_string(), biomes, &default_values()).expect("Spielstand lesen");
    let back = Campaign::from_save(&s, 1.0)
        .expect("Kampagne")
        .to_save(&s.saved_at)
        .expect("Spielstand");
    let d = golden::diff("save", &golden::tree(&f.save), &golden::tree(&back));
    assert!(d.is_empty(), "to_save(from_save(save)) weicht ab: {d}");
}

#[test]
fn spielstand_andere_version() {
    let f = load_campaign_at(CAMPAIGN_PATH);
    let biomes = &base().expect("Daten").data.biomes;
    let defaults = test_values();
    let mut m = f.save.as_object().expect("Spielstand").clone();
    m.insert("version".into(), 2.into());
    let v2 = Value::Object(m.clone()).to_string();
    assert!(parse_save(&v2, biomes, &defaults).is_err(), "Version 2 wurde geladen");
    m.remove("hubs");
    m.insert("version".into(), 1.into());
    let broken = Value::Object(m).to_string();
    assert!(
        parse_save(&broken, biomes, &defaults).is_err(),
        "Spielstand ohne hubs wurde geladen"
    );
}

#[test]
fn key_wie_to_fixed() {
    // Erwartet wie `x.toFixed(2)` in JavaScript.
    for (x, want) in [
        (1.125, "1.13"),
        (0.375, "0.38"),
        (2.5, "2.50"),
        (1.005, "1.00"),
        (12.344, "12.34"),
        (7.0, "7.00"),
    ] {
        assert_eq!(key("tree", x), format!("tree@{want}"), "key({x})");
    }
}

/// Go: `freeMonarchCampaign` (B-059): drei Monarchen, Nr. 1 ist frei und steht fern; 0 und 2 stehen am Tiefen-Eingang.
fn free_monarch_campaign() -> (Campaign, f64) {
    let mut c = Campaign::create("frei", "b059", 1.0).expect("Kampagne");
    for _ in 0..3 {
        c.join_player();
    }
    let w = c.current_world_mut().expect("Stufe");
    let pts = travel_points(w);
    assert!(
        !pts.is_empty() && pts[0].via == "exit",
        "kein Tiefen-Eingang in Tiefe 0"
    );
    w.players[1].free = true;
    w.players[1].x = w.hub_x;
    (c, pts[0].x)
}

/// Go: `walkTravel`: die Spieler `players` stehen bei `x`; liefert den Wechsel, falls er gestartet ist.
fn walk_travel(c: &mut Campaign, x: f64, players: &[usize]) -> Option<crate::state::Travel> {
    let w = c.current_world_mut().expect("Stufe");
    for _ in 0..200 {
        for &i in players {
            w.players[i].x = x;
        }
        step(w, &[], None, DT);
        if w.travel.as_ref().is_some_and(|t| t.progress >= 1.0) {
            break;
        }
    }
    w.travel.clone()
}

#[test]
fn freier_monarch_blockiert_nicht() {
    let (mut c, exit) = free_monarch_campaign();
    let tr = walk_travel(&mut c, exit, &[0, 2]);
    assert!(
        tr.as_ref().is_some_and(|t| t.progress >= 1.0),
        "Wechsel nicht gestartet: {tr:?}"
    );
    c.travel(1).expect("Stufenwechsel");
    let cave = c.current_world().expect("Höhle");
    assert!(
        cave.players.len() == 3 && cave.players[1].free,
        "freier Monarch reist nicht mit: {} Spieler",
        cave.players.len()
    );
    for p in &cave.players {
        let d = p.x - cave.hub_x;
        assert!(
            (-5.0..=5.0).contains(&d),
            "Spieler {} steht nicht an der Burg (x={})",
            p.index,
            p.x
        );
    }
}

#[test]
fn ohne_gesteuerten_monarchen_kein_wechsel() {
    let (mut c, exit) = free_monarch_campaign();
    for p in &mut c.current_world_mut().expect("Stufe").players {
        p.free = true;
    }
    let tr = walk_travel(&mut c, exit, &[0, 1, 2]);
    assert!(tr.is_none(), "Wechsel ohne gesteuerten Monarchen: {tr:?}");
}

#[test]
fn own_new_world_unbekannte_tiefe_ist_ein_fehler() {
    // Go: `panic("Kein Biom für Tiefe %d")`; Rust meldet es als Fehler (Leitplanke § 11).
    let err = new_world(99, "x", Options::default()).expect_err("Tiefe 99");
    assert_eq!(err, "Kein Biom für Tiefe 99");
}

#[test]
fn own_campaign_default_ist_leer() {
    let c = Campaign::default();
    assert!(c.current_world().is_none(), "ohne Stufe keine aktuelle Welt");
    assert!(c.vals.is_none() && c.depth == 0 && c.unlocked_depth == 0);
}
