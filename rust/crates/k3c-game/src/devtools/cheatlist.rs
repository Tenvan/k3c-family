//! Port von `native/k3c-game/devtools/cheatlist.go`.
//!
//! Einträge des Cheat-Menüs (F9, B-888) in den Gruppen des alten Dialogs (src/scenes/debug/debugActions.ts): Vorrat,
//! Ausbau, Zeit, Grad, Kampf. Jeder Eintrag ist eine Dev-Aktion des Raums (engine/room/dev.go); Bosse kommen aus
//! data/bosses.json. Reine Daten, ohne Zeichnen.

use std::sync::OnceLock;

use k3c_data::Source;
use serde::Deserialize;

use crate::view::devkeys::{DevAction, TIMESCALES};

/// CheatEntry ist ein Eintrag: Beschriftung und Aktion; `toggle` nicht leer schaltet einen Schalter (`on` wird beim
/// Auslösen gesetzt, die Beschriftung zeigt den Stand).
#[derive(Clone, Debug, PartialEq)]
pub struct CheatEntry {
    pub label: String,
    pub act: DevAction,
    pub toggle: &'static str,
}

/// CheatGroup ist eine Gruppe (Reiter) des Menüs.
#[derive(Clone, Debug, PartialEq)]
pub struct CheatGroup {
    pub name: &'static str,
    pub entries: Vec<CheatEntry>,
}

// Werte der Einträge (wie der alte Dialog).
const GOLD_AMOUNTS: [i64; 3] = [10, 100, 500];
const MATERIAL_AMOUNT: i64 = 50;
const UPGRADE_LEVELS: [i64; 5] = [1, 2, 3, 4, 5];
pub const RESOURCES: [(&str, &str); 5] = [
    ("wood", "Holz"),
    ("stone", "Stein"),
    ("copper", "Kupfer"),
    ("iron", "Eisen"),
    ("crystal", "Kristall"),
];
pub const PHASES: [(&str, &str); 4] = [
    ("day", "Tag"),
    ("dusk", "Abend"),
    ("night", "Nacht"),
    ("dawn", "Morgen"),
];
// grades ohne dev: der Grad dev geht nur mit Dev-Mode des Raums, den das native Spiel nicht setzt (local/dev.go).
pub const GRADES: [(&str, &str); 4] = [
    ("easy", "Leicht"),
    ("normal", "Normal"),
    ("hard", "Hart"),
    ("ultra", "Ultra"),
];
pub const EVENTS: [(&str, &str); 3] = [
    ("fullMoon", "Vollmond"),
    ("bloodMoon", "Blutmond"),
    ("merchantRaid", "Händler-Überfall"),
];

/// cheat_groups ist das Menü (beim ersten Zugriff gebaut).
pub fn cheat_groups() -> &'static [CheatGroup] {
    static G: OnceLock<Vec<CheatGroup>> = OnceLock::new();
    G.get_or_init(|| build_groups(&load_bosses()))
}

fn act(action: &str) -> DevAction {
    DevAction {
        action: action.to_string(),
        ..Default::default()
    }
}

fn entry(label: impl Into<String>, act: DevAction) -> CheatEntry {
    CheatEntry {
        label: label.into(),
        act,
        toggle: "",
    }
}

fn switch(label: &str, action: &'static str) -> CheatEntry {
    CheatEntry {
        label: label.to_string(),
        act: act(action),
        toggle: action,
    }
}

/// build_groups baut die Gruppen; `bosses` sind Id und Name der Bosse.
pub fn build_groups(bosses: &[(String, String)]) -> Vec<CheatGroup> {
    let group = |name, entries| CheatGroup { name, entries };
    vec![
        group("Vorrat", stock_entries()),
        group("Ausbau", build_entries()),
        group("Zeit", time_entries()),
        group("Grad", grade_entries()),
        group("Kampf", fight_entries(bosses)),
    ]
}

fn stock_entries() -> Vec<CheatEntry> {
    let mut v = Vec::new();
    for n in GOLD_AMOUNTS {
        let a = DevAction {
            action: "gold".to_string(),
            amount: n,
            ..Default::default()
        };
        v.push(entry(format!("Gold +{n}"), a));
    }
    for (id, name) in RESOURCES {
        let a = DevAction {
            action: "material".to_string(),
            resource: id.to_string(),
            amount: MATERIAL_AMOUNT,
            ..Default::default()
        };
        v.push(entry(format!("{name} +{MATERIAL_AMOUNT}"), a));
    }
    v.push(entry("Lager-Maximum +100", act("maxStock")));
    v.push(entry("Alle Ressourcen füllen", act("fillStock")));
    v.push(switch("Baukosten 1", "cheapBuild"));
    v
}

fn build_entries() -> Vec<CheatEntry> {
    let mut v = Vec::new();
    for n in UPGRADE_LEVELS {
        let a = DevAction {
            action: "buildAll".to_string(),
            level: n,
            ..Default::default()
        };
        v.push(entry(format!("Alles ausbauen: Stufe {n}"), a));
    }
    v.push(entry("Abfahrbereit (Schiff, Endboss)", act("readyToSail")));
    v.push(entry("Burgfall", act("castleFall")));
    v
}

fn time_entries() -> Vec<CheatEntry> {
    let mut v = vec![entry("Nächste Tageszeit", act("nextPhase"))];
    for (id, name) in PHASES {
        let a = DevAction {
            action: "phase".to_string(),
            phase: id.to_string(),
            ..Default::default()
        };
        v.push(entry(format!("Zu: {name}"), a));
    }
    for f in TIMESCALES {
        let a = DevAction {
            action: "timescale".to_string(),
            factor: f,
            ..Default::default()
        };
        v.push(entry(format!("Zeit {f}×"), a));
    }
    v.push(switch("Zeitraffer endet mit der Tageszeit", "timescaleAuto"));
    v
}

fn grade_entries() -> Vec<CheatEntry> {
    GRADES
        .iter()
        .map(|(id, name)| {
            let a = DevAction {
                action: "grade".to_string(),
                grade: (*id).to_string(),
                ..Default::default()
            };
            entry(format!("Grad {name}"), a)
        })
        .collect()
}

fn fight_entries(bosses: &[(String, String)]) -> Vec<CheatEntry> {
    let mut v = vec![entry("Nacht sofort", act("night")), entry("Welle jetzt", act("wave"))];
    for (id, name) in bosses {
        let a = DevAction {
            action: "boss".to_string(),
            boss: id.clone(),
            ..Default::default()
        };
        v.push(entry(format!("Boss {name}"), a));
    }
    v.push(entry("Boss: nächste Phase", act("bossPhase")));
    for (id, name) in EVENTS {
        let a = DevAction {
            action: "event".to_string(),
            event: id.to_string(),
            ..Default::default()
        };
        v.push(entry(format!("Event {name}"), a));
    }
    v
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct BossesFile {
    #[serde(rename = "Bosses", alias = "bosses")]
    bosses: Vec<BossRef>,
    #[serde(rename = "endBosses")]
    end_bosses: Vec<BossRef>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct BossRef {
    #[serde(rename = "ID", alias = "id")]
    id: String,
    #[serde(rename = "Name", alias = "name")]
    name: String,
}

/// bosses sind Id und Name der Mini- und Endbosse (für das Cheat-Fenster, RU56).
pub fn bosses() -> Vec<(String, String)> {
    load_bosses()
}

/// load_bosses liest Id und Name der Mini- und Endbosse aus data/bosses.json (Fehler: keine Bosse, Log).
fn load_bosses() -> Vec<(String, String)> {
    let res = Source::from_env()
        .read("bosses.json")
        .map_err(|e| e.to_string())
        .and_then(|raw| serde_json::from_str::<BossesFile>(&raw).map_err(|e| e.to_string()));
    match res {
        Ok(v) => v
            .bosses
            .into_iter()
            .chain(v.end_bosses)
            .map(|b| (b.id, b.name))
            .collect(),
        Err(e) => {
            k3c_platform::log::warn("💥 Bosse für das Cheat-Menü nicht lesbar", &[("err", &e)]);
            Vec::new()
        }
    }
}
