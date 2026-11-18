//! Tests zu `values.rs` (Port von `values_test.go`).

use super::values::*;
use crate::props::pending::{Economy, HubUpgrade};
use k3c_data::hub::Stock;
use k3c_sim::state::{CycleInfo, Merchant, Travel};
use std::cell::Cell;

fn check(cases: &[(&str, String, &str)]) {
    for (name, got, want) in cases {
        assert_eq!(got, want, "{name}");
    }
}

fn cycle(phase: &str, day: i64, left: f64) -> CycleInfo {
    CycleInfo {
        phase: phase.to_string(),
        day,
        seconds_left: left,
        ..CycleInfo::default()
    }
}

#[test]
fn uhr_und_zeit() {
    let clock = |lang, c: CycleInfo, speed, agg, saved| clock_text(&new_clock_key(lang, &c, speed, agg, saved));
    check(&[
        ("m:ss", format_time(90.0), "1:30"),
        ("aufgerundet", format_time(60.2), "1:01"),
        ("nie negativ", format_time(-3.0), "0:00"),
        (
            "Tag",
            clock("de", cycle("day", 2, 125.0), 1.0, None, false),
            "Tag 2  ·  Dämmerung in 2:05",
        ),
        (
            "Zeitraffer",
            clock("de", cycle("night", 1, 120.0), 2.0, None, false),
            "Nacht 1  ·  Morgen in 1:00",
        ),
        (
            "Aggression und gesichert",
            clock("de", cycle("dusk", 0, 5.0), 1.0, Some(37.8), true),
            "Dämmerung  ·  Nacht in 0:05  ·  Aggression 37%  ·  gesichert",
        ),
        (
            "englisch",
            clock("en", cycle("dawn", 3, 61.0), 1.0, None, false),
            "Day 3  ·  Sunrise in 1:01",
        ),
    ]);
}

/// Gold nur im HUD (goldOnlyInHud.test.ts): die Spielerkarte nennt Gold und Maximum, gefallen die Sekunden.
#[test]
fn gold_nur_auf_der_karte() {
    let k = GoldKey {
        lang: "de".into(),
        gold: 12,
        max: 30,
    };
    check(&[("Gold/Maximum", gold_text(&k), "12/30")]);
}

#[test]
fn gold_flash() {
    let mut g = GoldFlash::default();
    assert!(!g.step(5, 0.0), "der erste Wert blinkt nicht");
    assert!(g.step(6, 1.0) && g.step(6, 1.79), "eine Änderung hebt den Topf hervor");
    assert!(!g.step(6, 1.8), "nach 0,8 s ist es vorbei");
    g.step(7, 2.0);
    assert!(
        g.step(8, 2.5) && g.step(8, 3.29),
        "eine neue Änderung startet die Zeit neu"
    );
}

fn stock_line(lang: &str, primary: &str, stock: [i64; 5], max: i64, points: i64) -> String {
    shared_text(&StockKey {
        lang: lang.into(),
        primary: primary.into(),
        stock,
        max,
        points,
    })
}

#[test]
fn vorrat_zeile() {
    let k = stock_line;
    check(&[
        (
            "nur mit Vorrat",
            k("de", "wood", [0, 40, 0, 0, 0], 300, 0),
            "Holz 0/300  ·  Stein 40/300",
        ),
        ("voll", k("de", "", [300, 0, 0, 0, 0], 300, 0), "Holz 300/300 voll"),
        (
            "fünf in Reihenfolge",
            k("de", "", [1, 2, 3, 7, 5], 600, 0),
            "Holz 1/600  ·  Stein 2/600  ·  Kupfer 3/600  ·  Eisen 7/600  ·  Kristall 5/600",
        ),
        (
            "Primär mit 0",
            k("de", "iron", [0, 0, 4, 0, 0], 300, 0),
            "Kupfer 4/300  ·  Eisen 0/300",
        ),
        ("ohne Maximum", k("de", "", [12, 0, 0, 0, 0], 0, 0), "Holz 12"),
        (
            "Skill-Punkte",
            k("de", "", [12, 0, 0, 0, 0], 0, 2),
            "Holz 12  ·  Skill-Punkte 2",
        ),
        ("englisch", k("en", "", [0, 0, 0, 300, 0], 300, 0), "Iron 300/300 full"),
        ("leer", k("de", "", [0; 5], 0, 0), ""),
    ]);
}

#[test]
fn stock_of_liest_in_fester_reihenfolge() {
    assert_eq!(stock_of(None), [0; 5]);
    let s = Stock {
        wood: 1,
        stone: 2,
        copper: 3,
        iron: 4,
        crystal: 5,
    };
    assert_eq!(stock_of(Some(&s)), [1, 2, 3, 4, 5]);
}

#[test]
fn new_hub_key_liest_wirtschaft_und_haendler() {
    let e = Economy {
        hub_level: 2,
        hub_step: 1,
        fighters: 4,
        troop_limit: 9,
        hub_upgrade: Some(HubUpgrade {
            gold: 40,
            paid: 15,
            steps: 3,
            material: Stock {
                stone: 100,
                ..Stock::default()
            },
            ..HubUpgrade::default()
        }),
        ..Economy::default()
    };
    let m = Merchant {
        resource: "wood".into(),
        leaves: 7,
        buy_paid: 2,
        ..Merchant::default()
    };
    let k = new_hub_key("de", &e, Some(&m));
    assert!(k.has && k.compact && k.merchant);
    assert_eq!((k.level, k.step, k.steps, k.gold, k.paid), (2, 1, 3, 40, 15));
    assert_eq!((k.material, k.fighters, k.limit), ([0, 100, 0, 0, 0], 4, 9));
    assert_eq!((k.m_resource.as_str(), k.m_leaves, k.m_paid), ("wood", 7, 2));
    let ohne = new_hub_key("de", &Economy::default(), None);
    assert!(!ohne.has && !ohne.merchant);
}

#[test]
fn hub_und_buerger() {
    let up = HubKey {
        lang: "de".into(),
        level: 2,
        has: true,
        gold: 40,
        paid: 15,
        material: [0, 100, 0, 20, 0],
        ..HubKey::default()
    };
    let step = HubKey {
        lang: "de".into(),
        level: 1,
        has: true,
        step: 2,
        steps: 3,
        gold: 25,
        material: [0, 50, 20, 0, 0],
        ..HubKey::default()
    };
    let long = HubKey { compact: false, ..up };
    let short = HubKey {
        compact: true,
        ..step.clone()
    };
    let step = HubKey { compact: false, ..step };
    let de = |compact| HubKey {
        lang: "de".into(),
        compact,
        ..HubKey::default()
    };
    check(&[
        (
            "lang",
            hub_text(&long),
            "Hub-Stufe 2 → 3: 15/40 Gold, 100 Stein, 20 Eisen",
        ),
        (
            "Schritt lang",
            hub_text(&step),
            "Hub-Stufe 1 → 2, Ausbau 2/3: 0/25 Gold, 50 Stein, 20 Kupfer",
        ),
        (
            "Schritt kurz",
            hub_text(&short),
            "Hub 1→2 2/3: 0/25 Gold, 50 Stein, 20 Kupfer",
        ),
        ("höchste Stufe", hub_text(&HubKey { level: 5, ..de(true) }), "Hub 5"),
        ("ohne Stufe", hub_text(&de(false)), ""),
    ]);
}

#[test]
fn limit_haendler_beruf() {
    let line = HubKey {
        lang: "de".into(),
        level: 5,
        compact: true,
        fighters: 3,
        limit: 10,
        merchant: true,
        m_resource: "wood".into(),
        ..HubKey::default()
    };
    check(&[
        ("Limit lang", limit_text("de", 12, 20, false), "Kämpfer 12 von 20"),
        ("Limit kurz", limit_text("de", 0, 10, true), "Limit 0/10"),
        (
            "Limit voll",
            limit_text("de", 20, 20, false),
            "Kämpfer 20 von 20: Limit erreicht",
        ),
        ("Limit voll kurz", limit_text("de", 20, 20, true), "Limit 20/20 voll"),
        (
            "Händler",
            merchant_text("de", "stone", 7, 0, false),
            "Händler: Stein · bis Tag 7",
        ),
        (
            "Händler Kauf",
            merchant_text("de", "iron", 7, 3, false),
            "Händler: Eisen · bis Tag 7 · Kauf 3 Gold",
        ),
        (
            "Händler kurz",
            merchant_text("de", "stone", 7, 3, true),
            "Händler Stein",
        ),
        ("Zeile", citizens_text(&line), "Hub 5  ·  Limit 3/10  ·  Händler Holz"),
        ("Beruf", profession_text("de", "miner", false), "Bergmann"),
        ("Beruf englisch", profession_text("en", "builder", false), "Builder"),
        ("Beruf unbekannt", profession_text("de", "wizard", true), "Beruf"),
        ("kein Beruf", profession_text("de", "", true), ""),
    ]);
}

#[test]
fn kampf_reise_ort() {
    let wave = |n| {
        wave_text(&WaveKey {
            lang: "de".into(),
            wave: 2,
            n,
        })
    };
    let travel = Travel {
        to_depth: 1,
        via: "stairsDown".into(),
        progress: 0.31,
        ..Travel::default()
    };
    let place = |lang: &str, id: &str, name: &str, player| {
        place_text(&PlaceKey {
            lang: lang.into(),
            id: id.into(),
            name: name.into(),
            player,
        })
    };
    check(&[
        ("Welle", wave(7), "Welle 2: 7 Gegner"),
        ("keine Gegner", wave(0), ""),
        (
            "Reise",
            travel_text(&new_travel_key("de", Some(&travel))),
            "Abstieg in Tiefe 1  ■■■■□□□□□□",
        ),
        ("keine Reise", travel_text(&new_travel_key("de", None)), ""),
        ("Ort", place("de", "forest", "Wald", 0), "P1 · Oberwelt (Wald)"),
        ("Ort unbekanntes Biom", place("en", "neu", "Neuland", 1), "P2 · Neuland"),
    ]);
}

thread_local! {
    static BUILDS: Cell<u32> = const { Cell::new(0) };
}

fn counting_build(k: &WaveKey) -> String {
    BUILDS.with(|n| n.set(n.get() + 1));
    wave_text(k)
}

#[test]
fn memo_baut_nur_bei_neuem_schluessel() {
    let mut m: Memo<WaveKey> = Memo::new();
    let key = |n| WaveKey {
        lang: "de".into(),
        wave: 1,
        n,
    };
    let a = m.get(key(3), counting_build).to_string();
    let b = m.get(key(3), counting_build).to_string();
    assert!(
        a == b && BUILDS.with(Cell::get) == 1,
        "gleicher Schlüssel baut nicht neu"
    );
    m.get(key(4), counting_build);
    assert_eq!(BUILDS.with(Cell::get), 2, "neuer Schlüssel baut neu");
}
