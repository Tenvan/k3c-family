//! Seed-Modus (Port von `engine/balance/seedmode.go`, B-634, BS1.1): Bis zur Abnahme der neuen Werte (Projekt BAS) rechnet ein
//! Lauf ohne Seed-Angabe nur den ersten Seed der Datei (`k3c-dev`), danach wieder alle. Ausdrücklich übergebene Seeds
//! (`--seeds N`, `--seed-list`) gehen vor dem Modus. Die Typen `Targets` und `Target` liegen in `k3c-data`; die Methoden
//! von Go hängen hier als Trait daran.

use k3c_data::balance_targets::Targets;

use crate::fehler::Fehler;

/// Nur der erste Seed der Datei.
pub const SEED_MODE_DEV: &str = "dev";
/// Alle Seeds der Datei (Standard, wenn das Feld fehlt).
pub const SEED_MODE_ALL: &str = "all";
/// Ausdrücklich übergebene Seeds (`with_seeds`); nur im Speicher, nie in der Datei.
pub const SEED_MODE_PICK: &str = "auswahl";

/// Der erste Seed der Datei während BAS; ein Test prüft, dass er vorn steht.
pub const DEV_SEED: &str = "k3c-dev";

/// Die Seed-Methoden von `Targets` (Go: `seedList`, `WithSeeds`, `allSeeds`, `seedModeOf`).
pub trait SeedModes: Sized {
    /// Die ersten `n` Seeds der Datei; `n = 0` folgt dem Seed-Modus (dev: nur der erste, sonst alle).
    fn seed_list(&self, n: i64) -> &[String];
    /// Eine Kopie, die genau diese Seeds rechnet (`--seed-list`, `sim_test seedList`); der Modus gilt dann nicht mehr.
    /// Leere oder doppelte Seeds sind ein Fehler.
    fn with_seeds(&self, seeds: &[String]) -> Result<Self, Fehler>;
    /// Eine Kopie ohne Seed-Modus dev: ohne Angabe rechnet sie alle Seeds (Sensitivität, Kurven).
    fn all_seeds(&self) -> Self;
    /// Der Modus eines Laufs über `seeds`: dev, wenn der Modus dev gilt und genau der erste Seed lief, all bei allen Seeds der
    /// Datei, sonst auswahl.
    fn seed_mode_of(&self, seeds: &[String]) -> &'static str;
}

impl SeedModes for Targets {
    fn seed_list(&self, mut n: i64) -> &[String] {
        if n == 0 && self.seed_mode == SEED_MODE_DEV {
            n = 1;
        }
        if n > 0 && (n as usize) < self.seeds.len() {
            return &self.seeds[..n as usize];
        }
        &self.seeds
    }

    fn with_seeds(&self, seeds: &[String]) -> Result<Self, Fehler> {
        check_seeds(seeds)?;
        Ok(Targets {
            seeds: seeds.to_vec(),
            seed_mode: SEED_MODE_PICK.into(),
            ..self.clone()
        })
    }

    fn all_seeds(&self) -> Self {
        if self.seed_mode != SEED_MODE_DEV {
            return self.clone();
        }
        Targets {
            seed_mode: SEED_MODE_ALL.into(),
            ..self.clone()
        }
    }

    fn seed_mode_of(&self, seeds: &[String]) -> &'static str {
        if self.seed_mode == SEED_MODE_PICK {
            SEED_MODE_PICK
        } else if self.seed_mode == SEED_MODE_DEV && seeds.len() == 1 && self.seeds.first() == Some(&seeds[0]) {
            SEED_MODE_DEV
        } else if seeds.len() == self.seeds.len() {
            SEED_MODE_ALL
        } else {
            SEED_MODE_PICK
        }
    }
}

/// Mindestens ein Seed, keiner leer oder doppelt (Go: `validateSeeds` ohne den Modus).
fn check_seeds(seeds: &[String]) -> Result<(), Fehler> {
    if seeds.is_empty() {
        return Err(Fehler::Daten("seeds: mindestens ein Seed nötig".into()));
    }
    for (i, s) in seeds.iter().enumerate() {
        if s.is_empty() || seeds[..i].contains(s) {
            return Err(Fehler::Daten(format!("seeds: {s:?} ist leer oder doppelt")));
        }
    }
    Ok(())
}

/// Die Kopfzeile des Berichts zu Seeds und Modus, z. B. „1 Seed, Seed-Modus dev (k3c-dev)“ (Go: `Summary.seedText`; die
/// Felder `Seeds`, `SeedMode` und `SeedNames` der Zusammenfassung kommen mit RU28).
pub fn seed_text(seeds: i64, seed_mode: &str, seed_names: &[String]) -> String {
    let word = if seeds == 1 { "Seed" } else { "Seeds" };
    let mut out = format!("{seeds} {word}");
    if seed_mode.is_empty() {
        return out;
    }
    out.push_str(", Seed-Modus ");
    out.push_str(seed_mode);
    if !seed_names.is_empty() {
        out.push_str(&format!(" ({})", seed_names.join(", ")));
    }
    out
}
