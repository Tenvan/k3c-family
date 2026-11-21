//! Dev-Bau (Port von `DevBuildAll` und `devBuildSite`, `engine/sim/dev.go`): jede Stufe der Insel bis zu einer
//! Hub-Stufe ausbauen. Gos Abschnitt vor `DevBuildAll`.

use crate::island::Island;
use crate::state::World;
use crate::state::links::values;
use crate::state::site_view::site_hub_level_of;
use crate::world::hub_of;
use crate::world::lines::level_site;
use crate::world::ship::{harbor, ship_base_stages};

/// Go: `DevBuildAll`: baut jede Stufe der Insel auf `level` aus: Hub-Stufe, gebaute Plätze mit Ausbau-Stufen (Mauer, Turm)
/// und alle noch freien Plätze, deren Hub-Stufe und Linie bis `level` reicht (Tore bleiben, sie hängen an der äußersten
/// Mauer). Laufende Ausbauten und Aufträge entfallen. `false`: `level` außerhalb 1 bis Anzahl `hub.json › levels`.
pub fn dev_build_all(isl: &mut Island, level: i64) -> bool {
    let Some(first) = isl.stages.first() else {
        return false;
    };
    if level < 1 || level > hub_of(first).levels.len() as i64 {
        return false;
    }
    for w in &mut isl.stages {
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
    }
    true
}

/// Go: `devBuildSite`: baut Platz `si` als fertig, soweit `level` reicht; ein gebauter Platz bekommt nur die Ausbau-Stufe.
pub fn dev_build_site(w: &mut World, si: usize, level: i64) {
    let s = &w.sites[si];
    if level_site(w, s).is_some_and(|l| l.zone > level) {
        return;
    }
    let v = values(w);
    let building = v.buildings.get(&s.kind);
    let mut hp = building.map_or(0.0, |b| b.hp);
    if s.state != "built" && (site_hub_level_of(w, s) > level || hp == 0.0) {
        return;
    }
    let levels = building.map_or(&[][..], |b| b.levels.as_slice());
    let mut new_level = None;
    if !levels.is_empty() {
        let l = level.min(levels.len() as i64);
        hp = levels[(l - 1) as usize].hp;
        new_level = Some(l);
    }
    let s = &mut w.sites[si];
    if let Some(l) = new_level {
        s.level = l;
    }
    let cur = if s.state == "built" && s.hp < hp { s.hp } else { hp }; // beschädigt (auch eine Ruine) bleibt beschädigt
    s.state = "built".to_string();
    s.build_progress = 1.0;
    s.hp = cur;
    s.max_hp = hp;
    s.worker_id = None;
    s.upgrade.clear();
    s.upgrade_paid = 0;
}

/// RU36.3 (B-1157): macht die Insel abfahrbereit: Schiff am Hafen fertig (Kiel, Rumpf, Segel) und Endboss als besiegt
/// vermerkt, damit der Wechselpunkt am Steg öffnet (`step_island_switch`) und die Seekarte aufgeht. `false`: kein Hafen
/// oder keine nächste Insel (letzte Insel der Welt: dort gibt es keine Abfahrt, sondern den Sieg).
pub fn dev_ready_to_sail(isl: &mut Island) -> bool {
    let Some((wi, si)) = harbor(isl) else {
        return false;
    };
    if isl.is_last() {
        return false;
    }
    let w = &mut isl.stages[wi];
    let stages = ship_base_stages(w);
    dev_build_site(w, si, stages);
    isl.endboss_defeated = true;
    isl.endboss = None;
    true
}
