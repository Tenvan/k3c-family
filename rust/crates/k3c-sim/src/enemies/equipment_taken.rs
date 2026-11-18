//! Gegner tragen Ausrüstung weg (Port von `engine/sim/enemies_equipment_taken.go`, Q68, Q69, B-312): Ein Gegner mit
//! `grabRange` > 0 (data/enemies.json) nimmt den ersten Drop in Reichweite (Reihenfolge von `w.drops`) vom Boden und
//! flieht damit zum Portal; Ereignis `equipmentTaken`. Im Portal ist die Ausrüstung weg, stirbt der Träger, fällt sie an
//! seiner Stelle wieder zu Boden (`drop_carried`).

use serde_json::json;

use crate::state::common::event;
use crate::state::pending::{drop_crown_of, emit, unit_x};
use crate::state::{Enemy, World, WorldValues};
use crate::unit::hammer::HAMMER_KIND;

/// Go: `takeEquipment`: lässt `w.enemies[ei]` einen Drop in Reichweite aufheben; nicht fliehende Gegner ohne Last, keine
/// Bosse.
pub fn take_equipment(w: &mut World, v: &WorldValues, ei: usize) {
    let e = &w.enemies[ei];
    let grab = v.enemies.get(e.kind.as_str()).map_or(0.0, |d| d.grab_range);
    if grab <= 0.0 || e.boss || e.fleeing || e.carried.is_some() || w.portals.is_empty() {
        return;
    }
    let ex = e.x;
    // Hämmer bleiben liegen (B-950)
    let Some(i) = w
        .drops
        .iter()
        .position(|d| d.kind != HAMMER_KIND && (d.x - ex).abs() <= grab)
    else {
        return;
    };
    let drop = w.drops.remove(i);
    let kind = drop.kind.clone(); // Ereignis-Text; einmal je aufgehobenem Drop
    let e = &mut w.enemies[ei];
    e.carried = Some(drop);
    e.fleeing = true;
    emit(w, "equipmentTaken", event(json!({"kind": kind, "x": unit_x(ex)})));
}

/// Go: `dropCarried`: legt die getragene Ausrüstung und Krone eines gefallenen Gegners wieder ab. Der Gegner ist schon
/// aus `w.enemies` genommen (Zustandsmodell, Vorentscheidung 7).
pub fn drop_carried(w: &mut World, e: &mut Enemy) {
    drop_crown_of(e);
    if let Some(mut d) = e.carried.take() {
        d.x = e.x;
        w.drops.push(d);
    }
}
