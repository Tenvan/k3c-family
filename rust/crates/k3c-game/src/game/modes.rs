//! Port von `native/k3c-game/game/modes.go`: Schalter der Abnahme-Modi (NG9): Ablauf ab dem Hauptmenü (--flow),
//! Aufnahme zu festen Takten (--at), Messmodus (--measure) und die Liste der Grafik-Ordner (--assets).

use super::flags::{Kind, Spec, spec};
use super::scene::{Args, MAX_TICK};
use crate::view::settings::{RIVER_SHADER, RIVER_TILES};

/// Measure sind die Schalter des Messmodus (`measure.go`, RU30): Fenster in Echtzeit, Aufwärmen, dann Messen, Bericht
/// als JSON.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Measure {
    /// Messdauer in Sekunden; 0 = kein Messmodus
    pub seconds: f64,
    /// Sekunden Aufwärmen vor der Messung
    pub warmup: f64,
    /// Pfad des Berichts; leer = Standardausgabe
    pub report: String,
    /// VSync aus (Bildrate unbegrenzt)
    pub no_vsync: bool,
    /// Datei für die Messung je Bild (--measure-frames); leer = keine
    pub frames: String,
    /// Last-Szene „ausbau,truppen,gegner“ (--measure-load); leer = keine
    pub load: String,
    /// --pacing: nur noch angenommen (alte Aufrufe), wirkt nicht; es gibt nur noch die Zeitsammlung `k3c_platform::Clock`
    pub pacing: String,
}

// Prüf-Schalter für Fehler-, Lade- und Leistungsanzeige (NG14): --grafik (anderer Grafik-Ordner, z. B. ein leerer, um
// die Fehlermeldung zu sehen), --river und --stats (wie F3 und F2, gespeichert in den Einstellungen des Ordners
// --saves), --stall (nur mit --shots: dauerhaft die Ladeanzeige).

/// Die Schalter der Abnahme-Modi (Go: `addModeFlags`).
pub const MODE_FLAGS: &[Spec] = &[
    spec("flow", Kind::Bool, "false"),
    spec("at", Kind::Str, ""),
    spec("measure", Kind::Float, "0"),
    spec("warmup", Kind::Float, "5"),
    spec("report", Kind::Str, ""),
    spec("novsync", Kind::Bool, "false"),
    spec("measure-frames", Kind::Str, ""),
    spec("measure-load", Kind::Str, ""),
    spec("pacing", Kind::Str, ""),
    spec("assets", Kind::Bool, "false"),
    spec("grafik", Kind::Str, ""),
    spec("river", Kind::Str, ""),
    spec("stats", Kind::Bool, "false"),
    spec("stall", Kind::Bool, "false"),
    spec("devwin", Kind::Bool, "false"),
];

impl Args {
    /// Liest „t,t,…“ (streng aufsteigend, ab 0): der erste Takt ist der Vorlauf, die Zahl der Takte die der Bilder.
    pub(super) fn apply_at(&mut self, s: &str) -> Result<(), String> {
        if s.is_empty() {
            return Ok(());
        }
        for part in s.split(',') {
            let ok = part
                .parse::<i64>()
                .ok()
                .filter(|&t| (0..=MAX_TICK).contains(&t) && self.at.last().is_none_or(|&last| t > last));
            let Some(t) = ok else {
                return Err(format!(
                    "🚫 --at {s:?}: Takte 0 bis {MAX_TICK}, streng aufsteigend, durch Komma"
                ));
            };
            self.at.push(t);
        }
        self.scene.tick = self.at[0];
        self.count = self.at.len() as i64;
        Ok(())
    }

    /// Prüft, dass sich die Modi vertragen.
    pub(super) fn check_modes(&self) -> Result<(), String> {
        let m = &self.measure;
        let err = |s: &str| Err(s.to_string());
        if self.flow && (self.scene.direct() || !self.scene.menu.is_empty()) {
            err("🚫 --flow startet im Hauptmenü: ohne --seed, --load und --menu")
        } else if !self.at.is_empty() && self.shots.is_empty() {
            err("🚫 --at nur mit --shots")
        } else if m.seconds < 0.0 || m.warmup < 0.0 {
            err("🚫 --measure und --warmup ab 0 Sekunden")
        } else if m.seconds > 0.0 && !self.shots.is_empty() {
            err("🚫 --measure und --shots schließen sich aus")
        } else if !self.river.is_empty() && self.river != RIVER_SHADER && self.river != RIVER_TILES {
            Err(format!(
                "🚫 --river {:?}: {RIVER_SHADER} oder {RIVER_TILES}",
                self.river
            ))
        } else if self.stall && self.shots.is_empty() {
            err("🚫 --stall nur mit --shots")
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
#[path = "modes_tests.rs"]
mod modes_tests;
