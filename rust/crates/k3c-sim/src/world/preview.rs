//! Hilfen für die Vorschau des Welt-Betrachters (Port von `engine/sim/preview.go`; WZ7.8, B-460). Alle arbeiten nur auf der
//! übergebenen Welt, lesen die geladenen Daten und setzen keinen globalen Zustand.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::state::{CycleInfo, Site, World};

use super::base::base;
use super::pending::{cycle_at, cycle_phases, dev_build_site};
use super::{hub_of, vals};

/// Go: `PreviewCycle`: setzt die Tageszeit der Welt direkt: Sim-Phase (day, dusk, night, dawn) und Fortschritt in [0, 1)
/// innerhalb der Phase. Der Zyklus bleibt, `w.time` wird so abgeleitet, dass `cycle_at` dieselbe Phase liefert; es entstehen
/// keine Ereignisse. Eine andere Phase und ein Fortschritt außerhalb von [0, 1) (1 wäre der Beginn der nächsten Phase)
/// ergeben einen Fehler, ebenso `cycle_speed` 0 (der Zyklus steht); die Welt bleibt dann unverändert. `w.time` darf auch
/// rückwärts springen: gedacht für eine frisch erzeugte Vorschau-Welt, nicht für eine laufende Simulation (Fristen wie
/// `craft_done` liegen sonst in der Zukunft).
pub fn preview_cycle(w: &mut World, phase: &str, progress: f64) -> Result<(), String> {
    let day_night = &base()?.day_night;
    let (phases, cycle_len) = cycle_phases(day_night);
    let Some(p) = phases.iter().find(|p| p.name == phase) else {
        return Err(format!("vorschau: unbekannte Phase {phase:?} (day, dusk, night, dawn)"));
    };
    let (start, length) = (p.start, p.length);
    if !(0.0..1.0).contains(&progress) {
        return Err(format!("vorschau: Fortschritt {progress} außerhalb von [0, 1)"));
    }
    let speed = w.cycle_speed;
    if speed <= 0.0 {
        return Err(format!("vorschau: CycleSpeed {speed}, der Zyklus steht"));
    }
    let today = ((w.time * speed) / cycle_len).floor();
    let seconds = today * cycle_len + start + progress * length;
    w.time = seconds / speed;
    for _ in 0..8 {
        // (x/speed)*speed ist nicht immer genau x: bei Phasenbeginn nicht in der vorigen Phase landen
        if cycle_at(day_night, w.time * speed).phase == phase {
            break;
        }
        w.time = w.time.next_up();
    }
    let mut day = today as i64 + 1;
    if phase == "dawn" {
        day += 1; // das Morgengrauen gehört schon zum nächsten Tag (cycle_at)
    }
    w.cycle = CycleInfo {
        phase: phase.to_string(),
        day,
        progress,
        seconds_left: (1.0 - progress) * length,
    };
    Ok(())
}

/// Go: `PreviewHubLevel`: baut die Welt auf Hub-Stufe `level` aus (1 bis Anzahl hub.json › levels). Plätze und Hub-Platz
/// folgen den Regeln der Simulation wie bei `DevBuildAll` (`dev_build_site`: Hub-Stufe des Platzes, Mauerlinie, Tore bleiben).
/// Die Material-Stufe von Mauer und Turm läuft danach je Art reihum in der Reihenfolge der Plätze von 1 bis
/// min(level, Stufenzahl der Art), damit jede erlaubte Stufe im Bild steht und `next_level` (Stufe m erst ab Hub-Stufe m)
/// nie verletzt wird. Bei einem Fehler bleibt die Welt unverändert. Sie baut nur auf, nichts ab: gedacht für eine frisch
/// erzeugte Welt (`create_world`), nicht für eine, die schon höher ausgebaut ist.
pub fn preview_hub_level(w: &mut World, level: i64) -> Result<(), String> {
    let n = hub_of(w).levels.len() as i64;
    if level < 1 || level > n {
        return Err(format!("vorschau: Hub-Stufe {level} außerhalb von 1 bis {n}"));
    }
    w.hub_level = level;
    w.hub_step = 1;
    if let Some(h) = w.hub_site.as_mut() {
        h.upgrade.clear();
        h.upgrade_paid = 0;
        h.build_progress = 1.0;
        h.worker_id = None;
    }
    for si in 0..w.sites.len() {
        dev_build_site(w, si, level);
    }
    let v = vals(w);
    let mut seen: BTreeMap<&str, i64> = BTreeMap::new();
    for s in &mut w.sites {
        let levels = v.buildings.get(&s.kind).map_or(&[][..], |b| b.levels.as_slice());
        if s.state != "built" || levels.is_empty() {
            continue;
        }
        let count = seen.entry(s.kind.as_str()).or_insert(0);
        s.level = *count % level.min(levels.len() as i64) + 1;
        *count += 1;
        let hp = levels[(s.level - 1) as usize].hp;
        s.hp = hp;
        s.max_hp = hp;
    }
    Ok(())
}

/// Go: `SiteLook`: der Zustand eines Platzes in der Vorschau (Asset-Studio): Baustufe, Zustand der Trefferpunkte oder noch
/// nicht gebaut. Nullwerte lassen den Platz, wie `preview_hub_level` ihn hinterlassen hat.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SiteLook {
    /// Baustufe 1 bis Stufenzahl der Art; 0 = unverändert.
    #[serde(rename = "level")]
    pub level: i64,
    /// Trefferpunkte in Prozent der Höchstwerte (1 bis 100); 0 = unverändert.
    #[serde(rename = "hpPercent")]
    pub hp_percent: i64,
    /// Leerer Bauplatz (nicht bezahlt), `level` und `hp_percent` entfallen.
    #[serde(rename = "unbuilt")]
    pub unbuilt: bool,
}

/// Go: `PreviewSetSite`: setzt den Platz `si` der Vorschau-Welt auf `look`. Eine Baustufe oder Trefferpunkte an einem nicht
/// gebauten Platz werden ignoriert; eine Stufe außerhalb der Stufen der Art ist ein Fehler.
pub fn preview_set_site(w: &mut World, si: usize, look: &SiteLook) -> Result<(), String> {
    let v = vals(w);
    let Some(s) = w.sites.get_mut(si) else {
        return Err(format!("vorschau: Platz {si} fehlt"));
    };
    let building = v.buildings.get(&s.kind);
    if look.unbuilt {
        // eine Ruine behält ihre Kennung wie bei destroySite
        *s = Site {
            id: s.id,
            kind: std::mem::take(&mut s.kind),
            x: s.x,
            state: "unpaid".to_string(),
            max_hp: building.map_or(0.0, |b| b.hp),
            ruin: s.ruin,
            ..Site::default()
        };
        return Ok(());
    }
    if s.state != "built" {
        return Ok(());
    }
    let levels = building.map_or(&[][..], |b| b.levels.as_slice());
    if look.level != 0 {
        if look.level < 1 || look.level > levels.len() as i64 {
            return Err(format!(
                "vorschau: Baustufe {} außerhalb von 1 bis {} ({})",
                look.level,
                levels.len(),
                s.kind
            ));
        }
        s.level = look.level;
        let hp = levels[(look.level - 1) as usize].hp;
        s.hp = hp;
        s.max_hp = hp;
    }
    if look.hp_percent != 0 {
        if !(1..=100).contains(&look.hp_percent) {
            return Err(format!(
                "vorschau: Trefferpunkte {} % außerhalb von 1 bis 100 ({})",
                look.hp_percent, s.kind
            ));
        }
        s.hp = s.max_hp * look.hp_percent as f64 / 100.0;
    }
    Ok(())
}
