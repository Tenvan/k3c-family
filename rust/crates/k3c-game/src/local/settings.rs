//! Port von `native/k3c-game/local/settings.go`: Die Einstellungen liegen als `_settings.json` im Ordner der Spielstände,
//! im selben Format wie in Go (Feldnamen, Reihenfolge, fehlende Felder behalten den Standard).

use std::path::Path;

use k3c_room::room::Log as _;
use k3c_room::store::Fs;
use serde::{Deserialize, Serialize};

use super::error::{UserError, user_err};
use super::session::Session;
use crate::view::profile::Controls;
use crate::view::settings::{Settings, default_settings};
use crate::view::stress::StressTest;

/// SettingsFile ist die Datei der Einstellungen im Ordner der Spielstände. Der Unterstrich hält sie aus der
/// Spielstand-Liste (`Saves::list` nimmt nur Namen aus a–z, 0–9 und -).
pub const SETTINGS_FILE: &str = "_settings.json";

/// Das Dateiformat der Einstellungen (Go `settingsJSON`); fehlende Felder behalten den Standard.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SettingsJson {
    pub music: Option<f64>,
    pub effects: Option<f64>,
    /// Anzeige-Modus (B-826): "", "rahmenlos" oder "vollbild"
    pub window: Option<String>,
    pub language: Option<String>,
    /// fehlt: Startwert (K3C_RIVER), B-816
    #[serde(skip_serializing_if = "Option::is_none")]
    pub river: Option<String>,
    pub stats: Option<bool>,
    /// Blitze an (B-809); fehlt: an
    pub flashes: Option<bool>,
    /// Anleitung an (B-809); fehlt: an
    pub guide: Option<bool>,
    /// erledigte Hinweise der ersten Nacht (Bitmaske, `Settings::guide_seen`)
    #[serde(rename = "guideSeen")]
    pub guide_seen: Option<i64>,
    /// Eingabe-Profil je Spieler (B-893); fehlt: Standard
    pub controls: Option<Vec<String>>,
    /// Entwicklermodus (Karte Testing, B-896)
    #[serde(rename = "devMode", skip_serializing_if = "Option::is_none")]
    pub dev_mode: Option<bool>,
    /// zuletzt eingestellter Stresstest (B-896)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stress: Option<StressTest>,
}

/// Liest die Einstellungen aus `path`. Fehlt die Datei oder ist sie kaputt, gelten die Standardwerte (eine kaputte Datei
/// meldet der zweite Wert, das Spiel startet trotzdem).
pub fn load_settings(path: &Path) -> (Settings, Option<String>) {
    let mut s = default_settings();
    let raw = match std::fs::read(path) {
        Ok(raw) => raw,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return (s, None),
        Err(e) => return (s, Some(e.to_string())),
    };
    let j: SettingsJson = match serde_json::from_slice(&raw) {
        Ok(j) => j,
        Err(e) => return (s, Some(e.to_string())),
    };
    if let Some(v) = j.music {
        s.music = v;
    }
    if let Some(v) = j.effects {
        s.effects = v;
    }
    if let Some(v) = j.window.clone() {
        s.window = v;
    }
    if let Some(v) = j.language.clone() {
        s.language = v;
    }
    if let Some(v) = j.river.clone() {
        s.river = v;
    }
    if let Some(v) = j.stats {
        s.stats = v;
    }
    apply_guide(&mut s, j);
    (s.normalized(), None)
}

/// Übernimmt Blitze, Anleitung, „Anleitung gesehen“ (B-809), die Eingabe-Profile (B-893), Entwicklermodus und Stresstest
/// (B-896) aus der Datei; fehlende Felder bleiben Standard.
pub fn apply_guide(s: &mut Settings, j: SettingsJson) {
    if let Some(v) = j.flashes {
        s.flashes_off = !v;
    }
    if let Some(v) = j.guide {
        s.guide_off = !v;
    }
    if let Some(v) = j.guide_seen {
        s.guide_seen = v;
    }
    for (slot, p) in s.controls.iter_mut().zip(j.controls.unwrap_or_default()) {
        *slot = p;
    }
    if let Some(v) = j.dev_mode {
        s.dev_mode = v;
    }
    if let Some(v) = j.stress {
        s.stress = v;
    }
}

/// Der Inhalt der Datei für `s`: Go schreibt `river` und `devMode` nur, wenn gesetzt, alles andere immer.
pub fn settings_json(s: &Settings) -> SettingsJson {
    let controls: &Controls = &s.controls;
    SettingsJson {
        music: Some(s.music),
        effects: Some(s.effects),
        window: Some(s.window.clone()),
        language: Some(s.language.clone()),
        river: (!s.river.is_empty()).then(|| s.river.clone()),
        stats: Some(s.stats),
        flashes: Some(!s.flashes_off),
        guide: Some(!s.guide_off),
        guide_seen: Some(s.guide_seen),
        controls: Some(controls.to_vec()),
        dev_mode: s.dev_mode.then_some(true),
        stress: Some(s.stress.clone()),
    }
}

/// Schreibt die Einstellungen nach `path` (erst in eine Temp-Datei, dann umbenennen: nie halb geschrieben, über
/// `k3c_room::store::Fs`). Der Text gleicht dem von Go (`json.MarshalIndent`, 2 Leerzeichen, ganze Zahlen ohne `.0`).
pub fn save_settings(path: &Path, s: &Settings) -> Result<(), String> {
    let pretty = serde_json::to_string_pretty(&settings_json(s)).map_err(|e| e.to_string())?;
    Fs::default()
        .write_atomic(path, go_numbers(&pretty).as_bytes())
        .map_err(|e| e.to_string())
}

/// Go schreibt `5.0` als `5`; serde_json als `5.0`. Zeilen, deren Wert nur eine ganze Zahl mit `.0` ist, bekommen die Form
/// von Go (Schlüssel und Texte bleiben unberührt, der Wert steht nach `": "`).
fn go_numbers(pretty: &str) -> String {
    let fix = |line: &str| -> String {
        let (body, comma) = line.strip_suffix(',').map_or((line, ""), |b| (b, ","));
        let Some((key, value)) = body.rsplit_once("\": ") else {
            return line.to_string();
        };
        match value.strip_suffix(".0") {
            Some(int) if !int.is_empty() && int.trim_start_matches('-').bytes().all(|c| c.is_ascii_digit()) => {
                format!("{key}\": {int}{comma}")
            }
            _ => line.to_string(),
        }
    };
    pretty.lines().map(fix).collect::<Vec<_>>().join("\n")
}

impl Session {
    /// Übernimmt die Einstellungen sofort (ab dem nächsten Frame) und speichert sie neben den Spielständen. Ein Fehler
    /// betrifft nur das Speichern, die Werte gelten trotzdem.
    pub(super) fn apply_settings(&mut self, v: Settings) -> Result<(), UserError> {
        self.settings = v.normalized();
        let path = self.settings_path.display().to_string();
        if let Err(e) = save_settings(&self.settings_path, &self.settings) {
            self.log
                .error("💥 Einstellungen nicht gespeichert", &[("datei", &path), ("err", &e)]);
            return Err(user_err(
                "Einstellungen nicht gespeichert, sie gelten bis zum Beenden",
                Some(e.into()),
            ));
        }
        self.log.info("💾 Einstellungen gespeichert", &[("datei", &path)]);
        Ok(())
    }
}
