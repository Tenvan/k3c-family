//! Seed-Modus (B-634, BS1.1): Im Modus `dev` rechnet ein Lauf ohne Seed-Angabe nur den ersten Seed der Datei
//! (`k3c-dev`), sonst alle. Ausdrücklich übergebene Seeds gehen vor dem Modus.
//!
//! In RU70.14 aus RU70.15 vorgezogen (V11), weil Laden und Zusammenfassung sie brauchen.

use std::collections::HashSet;

use super::summary::Summary;
use super::targets::Targets;
use crate::kitx::go_quoted;

/// Nur der erste Seed der Datei.
pub const SEED_MODE_DEV: &str = "dev";
/// Alle Seeds der Datei (Standard, wenn das Feld fehlt).
pub const SEED_MODE_ALL: &str = "all";
/// Ausdrücklich übergebene Seeds; nur im Speicher, nie in der Datei.
pub(crate) const SEED_MODE_PICK: &str = "auswahl";

/// Der erste Seed der Datei; ein Test prüft, dass er vorn steht.
pub const DEV_SEED: &str = "k3c-dev";

fn valid_seed_mode(m: &str) -> bool {
    m.is_empty() || m == SEED_MODE_DEV || m == SEED_MODE_ALL
}

impl Targets {
    /// Die ersten `n` Seeds der Datei; `n` = 0 folgt dem Seed-Modus (dev: nur der erste, sonst alle).
    pub fn seed_list(&self, n: usize) -> &[String] {
        let n = if n == 0 && self.seed_mode == SEED_MODE_DEV {
            1
        } else {
            n
        };
        if n > 0 && n < self.seeds.len() {
            return &self.seeds[..n];
        }
        &self.seeds
    }

    /// Kopie, die genau diese Seeds rechnet; leere oder doppelte Seeds sind ein Fehler.
    pub fn with_seeds(&self, seeds: Vec<String>) -> Result<Targets, String> {
        let mut out = self.clone();
        out.seeds = seeds;
        SEED_MODE_ALL.clone_into(&mut out.seed_mode);
        out.validate_seeds()?;
        SEED_MODE_PICK.clone_into(&mut out.seed_mode);
        Ok(out)
    }

    pub(crate) fn validate_seeds(&self) -> Result<(), String> {
        if !valid_seed_mode(&self.seed_mode) {
            return Err(format!(
                "seedMode {}: {SEED_MODE_DEV} oder {SEED_MODE_ALL}",
                go_quoted(&self.seed_mode)
            ));
        }
        if self.seeds.is_empty() {
            return Err("seeds: mindestens ein Seed nötig".to_owned());
        }
        let mut seen = HashSet::new();
        for s in &self.seeds {
            if s.is_empty() || !seen.insert(s) {
                return Err(format!("seeds: {} ist leer oder doppelt", go_quoted(s)));
            }
        }
        Ok(())
    }

    /// Modus eines Laufs über `seeds`: dev, wenn der Modus dev gilt und genau der erste Seed lief, all bei allen Seeds
    /// der Datei, sonst auswahl.
    pub(crate) fn seed_mode_of(&self, seeds: &[String]) -> &'static str {
        if self.seed_mode == SEED_MODE_PICK {
            SEED_MODE_PICK
        } else if self.seed_mode == SEED_MODE_DEV && seeds.len() == 1 && seeds.first() == self.seeds.first() {
            SEED_MODE_DEV
        } else if seeds.len() == self.seeds.len() {
            SEED_MODE_ALL
        } else {
            SEED_MODE_PICK
        }
    }
}

impl Summary {
    /// Kopfzeile des Berichts zu Seeds und Modus, z. B. „1 Seed, Seed-Modus dev (k3c-dev)“.
    pub(crate) fn seed_text(&self) -> String {
        let word = if self.seeds == 1 { "Seed" } else { "Seeds" };
        let mut out = format!("{} {word}", self.seeds);
        if self.seed_mode.is_empty() {
            return out;
        }
        out = out + ", Seed-Modus " + &self.seed_mode;
        if !self.seed_names.is_empty() {
            out = format!("{out} ({})", self.seed_names.join(", "));
        }
        out
    }
}
