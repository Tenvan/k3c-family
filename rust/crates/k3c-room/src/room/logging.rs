//! Logging des Raummodells: Lebenszyklus der Räume und Geräte (Info), langsame Ticks (Warn, höchstens einmal je
//! `SLOW_EVERY_SECS`) und eine Takt-Statistik je Raum (`Manager::log_stats` in `run.rs`). Port von `logging.go`; Geräte-IDs
//! erscheinen nur gekürzt (`short`). Das Log kommt ohne `slog` aus (`log.rs`).
use super::env::Env;
use super::log::Level;
use super::types::{Room, TICK_HZ};

/// Ein Tick, der länger dauert, überzieht den Takt (Go `slowTick`, 1/`TICK_HZ` s in Nanosekunden).
pub const SLOW_TICK_NANOS: i64 = 1_000_000_000 / TICK_HZ;
/// Mindestabstand zweier Meldungen langsamer Ticks in Sekunden (Go `slowEvery`).
pub const SLOW_EVERY_SECS: i64 = 10;

impl Room {
    /// Merkt langsame Ticks vor und meldet sie gesammelt; die Uhr kommt aus `env`.
    pub fn note_tick(&mut self, env: &mut Env, took_nanos: i64) {
        if took_nanos <= SLOW_TICK_NANOS {
            return;
        }
        self.slow_count += 1;
        let now = env.now;
        let recent = self
            .slow_logged
            .is_some_and(|l| now.since_nanos(l) < i128::from(SLOW_EVERY_SECS) * 1_000_000_000);
        if recent {
            return;
        }
        let ms = |n: i64| n as f64 / 1_000_000.0;
        let (took, budget) = (ms(took_nanos), ms(SLOW_TICK_NANOS));
        let (since, tick, devices, factor) = (self.slow_count, self.tick, self.connected(), self.scale());
        self.log(
            env,
            Level::Warn,
            "🐢 Tick zu langsam",
            &[
                ("ms", &took),
                ("budgetMs", &budget),
                ("seitLetzterMeldung", &since),
                ("tick", &tick),
                ("geraete", &devices),
                ("faktor", &factor),
            ],
        );
        self.slow_logged = Some(now);
        self.slow_count = 0;
    }
}
