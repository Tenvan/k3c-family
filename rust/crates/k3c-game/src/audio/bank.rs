//! Port von `native/k3c-game/audio/bank.go`: die Klang-Bank.
//!
//! Abweichungen von Go: Die Bank hält statt dekodierter PCM-Bytes je Slot ein `SoundId` aus `AudioOut::load`; das
//! Dekodieren macht die Plattform. Ausschnitte eines Atlas kann `k3c_platform::AudioOut` nicht laden oder spielen
//! (nur ganze Dateien, `backend/sound.rs::decode` ist nicht öffentlich). Bis zur Entscheidung von 🧑 (Haltepunkt
//! RU17.5: Plattform um `load_range` erweitern oder Einzeldateien) liest `load_atlas` die Beschreibung, meldet den
//! Atlas aber als nicht ladbar; alle Effekte laufen über den Rückfall von Go (Einzeldateien, „Atlas nicht ladbar“).
//! Jede Warnung landet zusätzlich in `notes` (Tests prüfen „genau einmal im Log“ ohne Log-Umleitung).

use std::collections::BTreeMap;
use std::fmt::Display;
use std::path::Path;
use std::time::Instant;

use k3c_platform::log::{self, Level};
use k3c_platform::{AudioOut, SoundId};

use super::atlas::{Atlas, parse_atlas};
use super::register::Register;

/// Pfade relativ zur Repo-Wurzel (Aufruf über `task native:game`, wie art.DefaultConfig).
pub const DEFAULT_PUBLIC_DIR: &str = "public";
pub const ATLAS_JSON: &str = "audio/atlas.json";

/// bank hält die Effekte geladen (je Slot ein Klang der Plattform): den Atlas einmal und die Slots ohne Atlas-Eintrag
/// als Einzeldatei (Rückfall wie audioCore.ts › preloadEffects). Nach dem Laden nur gelesen.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Bank {
    pub atlas: Atlas,
    /// geladener Atlas; bleibt `None`, solange die Plattform keine Ausschnitte kann (Haltepunkt RU17.5)
    pub pcm: Option<SoundId>,
    /// Slot → geladene Einzeldatei
    pub singles: BTreeMap<String, SoundId>,
    /// Warnungen beim Laden (je fehlende oder kaputte Datei genau eine)
    pub notes: Vec<String>,
}

impl Bank {
    /// clip liefert den Ton eines Effekt-Slots: die Einzeldatei; `None` = stumm. (Go schneidet zuerst den Atlas aus,
    /// das kann die Plattform noch nicht, siehe Kopf.)
    pub fn clip(&self, slot: &str) -> Option<SoundId> {
        self.singles.get(slot).copied()
    }
}

/// clip einer vielleicht noch nicht geladenen Bank (Go: Methode auf `nil`): ohne Bank stumm.
pub fn clip(b: Option<&Bank>, slot: &str) -> Option<SoundId> {
    b.and_then(|b| b.clip(slot))
}

/// Schreibt eine Warnung ins Log und merkt sie in `notes`.
pub fn warn(notes: &mut Vec<String>, msg: &str, fields: &[(&str, &dyn Display)]) {
    log::warn(msg, fields);
    notes.push(log::line(Level::Warn, msg, fields));
}

/// load_bank lädt Atlas und Einzeldateien aus dir (public/) über `out`. Fehlt etwas, bleibt der Slot stumm; jede
/// fehlende oder kaputte Datei steht genau einmal im Log. Die Reihenfolge der Slots ist sortiert (gleiche Logs je Lauf).
/// Die Ladezeit steht im Log (`ms`, Vorentscheidung RU17.5: synchron, ein Thread erst über 100 ms).
pub fn load_bank(dir: &Path, reg: &Register, out: &mut dyn AudioOut) -> Bank {
    let t0 = Instant::now();
    let mut b = Bank::default();
    if let Err(e) = b.load_atlas(dir) {
        warn(
            &mut b.notes,
            "❌ Ton-Atlas nicht ladbar, Effekte als Einzeldateien",
            &[("err", &e)],
        );
    }
    // BTreeMap: Schlüssel schon sortiert (Go: slices.Sort)
    let slots: Vec<&String> = reg
        .effects
        .keys()
        .filter(|slot| !b.atlas.sprites.contains_key(*slot) || b.pcm.is_none())
        .collect();
    for slot in slots {
        match load_file(&dir.join(&reg.effects[slot]), out) {
            Ok(id) => {
                b.singles.insert(slot.clone(), id);
            }
            Err(e) => warn(
                &mut b.notes,
                "❌ Ton-Datei nicht ladbar, Slot stumm",
                &[("slot", slot), ("err", &e)],
            ),
        }
    }
    let ms = t0.elapsed().as_millis();
    log::info(
        "📂 Ton geladen",
        &[
            ("atlas", &b.atlas.sprites.len()),
            ("einzeln", &b.singles.len()),
            ("ms", &ms),
        ],
    );
    b
}

impl Bank {
    /// load_atlas liest atlas.json und wählt die erste vorhandene Datei in der Format-Reihenfolge. Laden kann es sie
    /// noch nicht (Haltepunkt RU17.5): Es meldet den Grund als Fehler, die Bank fällt auf Einzeldateien zurück.
    fn load_atlas(&mut self, dir: &Path) -> Result<(), String> {
        let path = dir.join(ATLAS_JSON);
        let raw = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let a = parse_atlas(&raw).map_err(|e| format!("{}: {e}", path.display()))?;
        let base = path.parent().unwrap_or(dir);
        let Some(name) = a.pick(|n| base.join(n).exists()) else {
            return Err(format!("{}: keine Ton-Datei des Atlas vorhanden", path.display()));
        };
        Err(format!(
            "{}: Ausschnitte kann k3c-platform noch nicht laden (Haltepunkt RU17.5)",
            base.join(name).display()
        ))
    }
}

/// load_file liest eine ganze Datei (ogg, mp3, wav) und lädt sie über die Plattform.
pub fn load_file(path: &Path, out: &mut dyn AudioOut) -> Result<SoundId, String> {
    let raw = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    out.load(&raw).map_err(|e| format!("{}: {e}", path.display()))
}
