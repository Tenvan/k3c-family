//! Bausteine der Seiten (ohne Win32): kurze Konstruktoren für Controls und Nachrichten.
#![allow(dead_code)] // bis RU56.10: Bausteine, die noch nicht jede Seite nutzt

use k3c_devproto::{DevCommand, Entry, Msg};

use crate::layout::{Control, Group, Kind, PLAYER_KEY, Values};

/// MAX_PLAYERS ist die Höchstzahl der Spieler (Auswahl des Zielspielers; das Spiel lehnt fehlende ab).
pub const MAX_PLAYERS: usize = 4;

/// act ist ein `dev`-Befehl mit nur der Aktion.
pub fn act(action: &str) -> DevCommand {
    DevCommand {
        action: action.to_owned(),
        ..DevCommand::default()
    }
}

/// dev macht aus dem Befehl die Nachricht (Spieler setzt die Seite, Kennung das Fenster).
pub fn dev(c: DevCommand) -> Msg {
    Msg::Dev {
        id: 0,
        player: 0,
        action: c,
    }
}

/// ui ist die Bedienung einer Spieltaste.
pub fn ui(key: &str, value: Option<i64>) -> Msg {
    Msg::Ui {
        id: 0,
        key: key.to_owned(),
        value,
    }
}

/// button ist eine Schaltfläche mit fester Nachricht.
pub fn button(key: &str, text: &str, msg: Msg) -> Control {
    Control::new(key, Kind::Button, text).sends(msg)
}

/// label ist eine Beschriftung.
pub fn label(key: &str, text: &str) -> Control {
    Control::new(key, Kind::Label, text)
}

/// select ist eine Auswahl; ohne Einträge steht ein Strich da (der Katalog fehlt noch).
pub fn select(key: &str, text: &str, options: Vec<String>) -> Control {
    let options = if options.is_empty() {
        vec!["–".to_owned()]
    } else {
        options
    };
    Control::new(key, Kind::Select, text).with_options(options)
}

/// input ist ein Eingabefeld mit Vorgabe.
pub fn input(key: &str, default: &str) -> Control {
    Control::new(key, Kind::Input, default)
}

/// group ist eine Gruppe mit Titel.
pub fn group(title: &str, controls: Vec<Control>) -> Group {
    Group {
        title: title.to_owned(),
        controls,
    }
}

/// player_select ist die Auswahl des Zielspielers; ihr Index gilt für jeden `dev`-Befehl der Seite.
pub fn player_select() -> Control {
    let options = (1..=MAX_PLAYERS).map(|n| format!("Spieler {n}")).collect();
    Control::new(PLAYER_KEY, Kind::Select, "Zielspieler").with_options(options)
}

/// index liest den Index der Auswahl `key` (`None` ohne Wahl).
pub fn index(v: &Values, key: &str) -> Option<usize> {
    v.get(&format!("{key}.index"))?.parse::<usize>().ok()
}

/// number liest die Zahl des Eingabefelds `key`.
pub fn number(v: &Values, key: &str) -> Option<i64> {
    v.get(key)?.trim().parse::<i64>().ok()
}

/// labels sind die Anzeigenamen der Einträge (ohne Namen die Kennung).
pub fn labels(entries: &[Entry]) -> Vec<String> {
    entries
        .iter()
        .map(|e| {
            if e.label.is_empty() {
                e.id.clone()
            } else {
                e.label.clone()
            }
        })
        .collect()
}

/// ids sind die Kennungen der Einträge (gleiche Reihenfolge wie `labels`).
pub fn ids(entries: &[Entry]) -> Vec<String> {
    entries.iter().map(|e| e.id.clone()).collect()
}

/// phase_label nennt die Tageszeit deutsch (Unbekanntes bleibt stehen).
pub fn phase_label(id: &str) -> String {
    match id {
        "day" => "Tag",
        "dusk" => "Abend",
        "night" => "Nacht",
        "dawn" => "Morgen",
        other => other,
    }
    .to_owned()
}

/// grade_label nennt den Grad deutsch (Unbekanntes bleibt stehen).
pub fn grade_label(id: &str) -> String {
    match id {
        "easy" => "Leicht",
        "normal" => "Normal",
        "hard" => "Hart",
        "ultra" => "Ultra",
        other => other,
    }
    .to_owned()
}

#[cfg(test)]
/// find sucht das Control mit dem Schlüssel in den Gruppen (für Tests der Seiten).
pub fn find<'a>(groups: &'a [Group], key: &str) -> Option<&'a Control> {
    groups.iter().flat_map(|g| g.controls.iter()).find(|c| c.key == key)
}
