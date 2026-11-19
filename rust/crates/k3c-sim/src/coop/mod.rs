//! Koop-Aktionen (RU62.3, B-968, docs/rules/koop.md; Entscheidung 013: neue Mechanik nur in Rust, ohne Go-Orakel): Eine
//! Doppel-Truhe öffnet nur, wenn zwei gesteuerte Monarchen gleichzeitig davorstehen, und wirft mehr Gold. Daten:
//! data/coop.json (`k3c-data/coop.rs`).
//!
//! Aktiv ist die Mechanik nur in Welten aus `coop.json › worlds` (nicht `standard`) und nur, solange auf der Insel
//! mindestens zwei Monarchen gesteuert werden (`Player::free` false). Sonst ist jede Truhe eine normale Truhe (Solo
//! verliert nichts). Das Kennzeichen wird aus Insel-Seed, Tiefe und Ort der Truhe berechnet (`k3c-rng`), nie gespeichert
//! und ohne Eingriff in den Level-Generator.
//!
//! - `mod.rs`: Kennzeichen, Einschalten der Insel, Takt-Vorbereitung.
//! - `chest.rs`: Öffnen im Aufheb-Schritt (`economy/players.rs`) und Zustand für die Anzeige (`World::coop`).
use std::sync::Arc;

use k3c_data::Data;
use k3c_rng::Rng;

use crate::island::Island;

pub mod chest;

#[cfg(test)]
mod chest_tests;
#[cfg(test)]
mod mark_tests;
#[cfg(test)]
mod testutil;

/// Kennt die Welt `world_id` Koop-Aktionen (`coop.json › worlds`)?
pub fn world_has_coop(data: &Data, world_id: &str) -> bool {
    data.coop.worlds.iter().any(|w| w == world_id)
}

/// Ist die Truhe an `x` der Insel mit dem Seed `seed` in der Tiefe `stage` eine Doppel-Truhe? Deterministisch, im Mittel
/// jede `every_nth`-te Truhe; `every_nth` <= 1 kennzeichnet jede.
pub fn is_double_chest(seed: &str, stage: i64, x: f64, every_nth: i64) -> bool {
    if every_nth <= 1 {
        return true;
    }
    let pos = (x * 100.0).round() as i64;
    let mut rng = Rng::new(&format!("{seed}|coop|{stage}|{pos}"));
    rng.int(0, every_nth - 1) == 0
}

impl Island {
    /// Schaltet die Koop-Aktionen der Insel ein (nur Welten aus coop.json; Aufruf beim Bauen der Insel).
    pub fn enable_coop(&mut self, data: &Data) {
        let cfg = Arc::new(data.coop.clone());
        for w in &mut self.stages {
            w.coop_cfg = Some(Arc::clone(&cfg));
        }
    }

    /// Je Takt vor den Stufen: Sind mindestens zwei Monarchen der Insel gesteuert (alle Stufen zusammen)? Die Stufen
    /// übernehmen das Ergebnis in `coop_active`; ohne Koop-Regeln bleibt es aus. Keine Zuteilung.
    pub fn prepare_coop(&mut self) {
        let controlled: usize = self
            .stages
            .iter()
            .map(|w| w.players.iter().filter(|p| !p.free).count())
            .sum();
        for w in &mut self.stages {
            w.coop_active = w.coop_cfg.is_some() && controlled >= 2;
        }
    }
}
