//! Begleitschutz für den Endboss-Kampf (Port von `engine/sim/unit_escort.go`, B-647 Weg A): Der Endboss steht an seinem
//! Bau (speed 0), weit vom Hub, und kein Kämpfer erreichte ihn je; nur der Monarch konnte ihn schlagen. Solange der
//! Endboss kämpft und ein lebender Monarch näher als `lair.escortUnits` am Boss ist, verlassen die Kämpfer der Stufe
//! (Schützen, Krieger und ihre Elite) ihre Posten und marschieren zu ihm, die Schützen bis in ihre Reichweite, die Krieger
//! in den Nahkampf. Der Begleitschutz rastet ein, weil ein Monarch dem Boss sonst allein entgegenrennt und fällt, bevor
//! die Truppen ankommen; er endet mit dem Tod des Bosses. Kein rng.
//!
//! `escortCheck` und die Prüfung in `escortPost` lesen den Endboss-Kampf der Insel (`boss_endboss.go`, RU15.2); die
//! Platzhalter `escort_check` und `escort_post` in `state/pending.rs` warten darauf. Hier steht, was ohne ihn rechnet:
//! der Ort, an dem der Kämpfer den Boss angreift.

use crate::state::{Troop, World, WorldValues};

/// Der Ort, an dem der Kämpfer `t` den Endboss bei `boss_x` angreift (Ende von Go `escortPost`, nach den Prüfungen
/// „Kampf läuft, Boss lebt, Begleitschutz eingerastet“): die Truppen kommen von der Seite des Hubs.
pub fn escort_post_at(w: &World, v: &WorldValues, t: &Troop, boss_x: f64) -> f64 {
    let side = if w.hub_x < boss_x { -1.0 } else { 1.0 };
    let range = v.troops.get(t.kind.as_str()).map_or(0.0, |d| d.range);
    boss_x + side * (range * 0.8).max(1.0)
}
