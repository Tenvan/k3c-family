//! Tag/Nacht-Zyklus (Port von `engine/sim/world_cycle.go`, Q65, docs/rules/wirtschaft.md § 3): Tag → Dämmerung → Nacht →
//! Morgengrauen → nächster Tag. Unter Tage läuft der globale Zyklus der Oberwelt weiter, Wellen kommen dort über den
//! Aggressionspool (`add_pool`).
//!
//! Go: `globalDayNight` ist eine Paket-Variable; hier kommt der Zyklus aus `base()` (die eingebetteten Daten). Die Rechnung
//! braucht nur `floor` und `%`, keine `k3c_det`-Funktionen (Leitfaden § 5).

use k3c_data::biome::Cycle;
use serde_json::json;

use super::base::base;
use super::pending::{merchant_dawn, night_events_end, night_events_start, tavern_dawn};
use super::vals;
use crate::economy::pay_dawn_income;
use crate::enemies::send_enemies_home;
use crate::enemies::waves::start_wave;
use crate::state::common::event;
use crate::state::{CycleInfo, IslandRef, World};
use crate::unit::hammer::spare_hammer;

/// Go: `cyclePhase`: eine Phase des Zyklus mit Beginn und Länge in Zyklus-Sekunden.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CyclePhase {
    pub name: &'static str,
    pub start: f64,
    pub length: f64,
}

/// Go: `cyclePhases`: die vier Phasen in ihrer Reihenfolge (day, dusk, night, dawn) und die Länge eines ganzen Zyklus.
pub fn cycle_phases(c: &Cycle) -> ([CyclePhase; 4], f64) {
    let (day, dusk) = (c.day_minutes * 60.0, c.twilight_minutes * 60.0);
    let (night, dawn) = (c.night_minutes * 60.0, c.dawn_minutes * 60.0);
    (
        [
            CyclePhase {
                name: "day",
                start: 0.0,
                length: day,
            },
            CyclePhase {
                name: "dusk",
                start: day,
                length: dusk,
            },
            CyclePhase {
                name: "night",
                start: day + dusk,
                length: night,
            },
            CyclePhase {
                name: "dawn",
                start: day + dusk + night,
                length: dawn,
            },
        ],
        day + dusk + night + dawn,
    )
}

/// Go: `globalDayNight`: der Zyklus der Oberwelt aus den eingebetteten Daten; `None`, wenn sie nicht lesbar sind (wegen der
/// Prüfung in `k3c-data` nicht zu erwarten).
fn global_cycle() -> Option<&'static Cycle> {
    base().ok().map(|b| &b.day_night)
}

/// Go: `SecondsToNight`: die Spielsekunden bis zum nächsten Nachtbeginn der Welt `w` (0 in der Nacht; Anzeige des nativen
/// Debug-Overlays, B-889). Ohne lesbare Daten 0 wie in der Nacht.
pub fn seconds_to_night(w: &World) -> f64 {
    if w.cycle.phase == "night" || w.cycle_speed <= 0.0 {
        return 0.0;
    }
    let Some(global) = global_cycle() else {
        return 0.0;
    };
    let (phases, length) = cycle_phases(global);
    let t = (w.time * w.cycle_speed) % length; // wie math.Mod
    let mut start = phases[2].start;
    if t >= start {
        start += length;
    }
    (start - t) / w.cycle_speed
}

/// Go: `cycleAt`: Phase, Tag und Fortschritt zur Zykluszeit `seconds` (Sekunden mal Zyklusgeschwindigkeit). Day 1 = erster
/// Tag; die Nacht gehört zum Tag davor, das Morgengrauen schon zum neuen Tag.
pub fn cycle_at(c: &Cycle, seconds: f64) -> CycleInfo {
    let (phases, length) = cycle_phases(c);
    let n = (seconds / length).floor();
    let t = seconds - n * length;
    for p in &phases[..3] {
        if t < p.start + p.length {
            return CycleInfo {
                phase: p.name.to_string(),
                day: n as i64 + 1,
                progress: (t - p.start) / p.length,
                seconds_left: p.start + p.length - t,
            };
        }
    }
    let dawn = &phases[3];
    CycleInfo {
        phase: dawn.name.to_string(),
        day: n as i64 + 2,
        progress: (t - dawn.start) / dawn.length,
        seconds_left: length - t,
    }
}

/// Go: `stepCycle`: rückt den Zyklus der Stufe um `dt` weiter, löst die Übergänge aus (Dämmerung, Nacht mit Welle,
/// Morgengrauen mit Einkommen) und füllt unter Tage den Aggressionspool. Ohne lesbare Daten bleibt der Zyklus stehen.
pub fn step_cycle(w: &mut World, mut isl: Option<&mut IslandRef>, dt: f64) {
    let Some(global) = global_cycle() else {
        return;
    };
    let before = std::mem::take(&mut w.cycle.phase);
    w.cycle = cycle_at(global, w.time * w.cycle_speed);
    let changed = before != w.cycle.phase;
    let day_night = w.biome.cycle.r#type == "dayNight";
    if changed && w.cycle.phase == "dusk" {
        w.events.push(event(json!({"type": "dusk"})));
    }
    if changed && w.cycle.phase == "night" {
        w.events.push(event(json!({"type": "night", "day": w.cycle.day})));
        night_events_start(w); // Vollmond, Blutmond (events_moon.go)
        if day_night {
            start_wave(w, isl.as_deref_mut());
        }
    }
    if changed && w.cycle.phase == "dawn" {
        // der Übergang Morgengrauen → Tag löst nichts aus (Q65)
        dawn_break(w, isl.as_deref_mut(), day_night);
    }
    if w.aggression.is_some() && w.biome.cycle.r#type == "aggressionPool" {
        add_pool(w, w.biome.cycle.percent_per_minute / 60.0 * dt * w.cycle_speed);
        if w.aggression.is_some_and(|a| a >= 100.0) {
            w.aggression = Some(0.0);
            start_wave(w, isl);
        }
    }
}

/// Der Beginn des Morgengrauens in `step_cycle`: Ereignis `dawn`, Gegner fliehen, Morgengold, Ersatz-Hammer, Wirt, Händler.
fn dawn_break(w: &mut World, isl: Option<&mut IslandRef>, day_night: bool) {
    w.events.push(event(json!({"type": "dawn", "day": w.cycle.day})));
    if day_night {
        send_enemies_home(w);
    }
    let v = vals(w);
    pay_dawn_income(w, &v, isl.as_deref());
    spare_hammer(w, &v);
    tavern_dawn(w);
    merchant_dawn(w, isl);
    night_events_end(w);
}

/// Go: `minWaveGapSeconds`: der Mindestabstand zweier Wellen unter Tage (4 min Spielzeit, zielkorridore.md § 1, B-646).
const MIN_WAVE_GAP_SECONDS: f64 = 240.0;

/// Go: `poolFrozen`: Der Aggressionspool wächst nicht, solange Gegner einer Welle leben oder noch kommen und innerhalb des
/// Mindestabstands nach der letzten Welle. Ohne das löst jede getötete Welle über +5 % je Kill sofort die nächste aus
/// (Wellen-Lawine, B-646).
fn pool_frozen(w: &World) -> bool {
    if w.wave > 0 && w.time - w.last_wave_at < MIN_WAVE_GAP_SECONDS {
        return true;
    }
    if !w.spawn_queue.is_empty() {
        return true;
    }
    w.enemies.iter().any(|e| e.hp > 0.0 && !e.guard)
}

/// Go: `addPool`: erhöht den Aggressionspool um `amount` (höchstens auf 100), sofern es ihn gibt und er nicht eingefroren
/// ist.
pub fn add_pool(w: &mut World, amount: f64) {
    if w.biome.cycle.r#type != "aggressionPool" || pool_frozen(w) {
        return;
    }
    if let Some(a) = w.aggression.as_mut() {
        *a = (*a + amount).min(100.0);
    }
}
