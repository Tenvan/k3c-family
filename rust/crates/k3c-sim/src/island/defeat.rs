//! Niederlage-Modi der Raum-Optionen (Port von `engine/sim/island_defeat.go`, B-102, docs/rules/stufen.md § 4, kampf.md § 6):
//! Fällt die Burg einer Stufe, wirkt der Modus der Insel (`Options.defeat`), das Spiel endet dadurch nie. Welten ohne Insel
//! (Campaign, Golden-Läufe) behalten den Stufenverlust. Das Spielende kommt nur durch Kronenraub (crown.go).
//!
//! Die Insel steht in Rust nicht in der Stufe (Zustandsmodell, Entscheidung 4): Die Funktionen bekommen den `IslandRef` der
//! Stufe. Der Vorrat liegt während ihres Schritts in der Stufe (Entscheidung 5), Gold der Spieler anderer Stufen erreicht
//! sie nicht: `IslandRef::halve_other_gold` lässt die Insel es in `put_stage_ref` halbieren, am selben Punkt der Reihenfolge
//! wie in Go (keine Stufe rechnet dazwischen).

use k3c_data::hub::Stock;

use crate::state::pending::{castle_losses, emit};
use crate::state::{Event, IslandRef, World};

use super::Island;

/// Go: `defeatLosses` wählt die Verluste eines Burgfalls nach dem Niederlage-Modus.
pub fn defeat_losses(w: &mut World, isl: Option<&mut IslandRef>) {
    match isl {
        None => castle_losses(w, None),
        Some(i) if i.defeat_resources => resource_losses_in_stage(w, i),
        Some(i) => castle_losses(w, Some(&*i)), // stage
    }
}

/// Go: `resourceLosses` (Insel ruht zwischen den Schritten): Gold aller Spieler der Insel und der Vorrat je −50 %; Bauten
/// und Truppen bleiben.
pub fn resource_losses(isl: &mut Island) {
    halve_stock(&mut isl.stock);
    for w in &mut isl.stages {
        w.players.iter_mut().for_each(|p| p.gold /= 2);
    }
}

/// Go: `resourceLosses` während des Schritts der Stufe `w`: Der Vorrat der Insel liegt in `w.stock`, `w.players` sind die
/// Spieler dieser Stufe; die der anderen Stufen halbiert `Island::put_stage_ref`.
pub fn resource_losses_in_stage(w: &mut World, isl: &mut IslandRef) {
    halve_stock(&mut w.stock);
    w.players.iter_mut().for_each(|p| p.gold /= 2);
    isl.halve_other_gold = true;
}

/// Go: `gameOver`: Kronenraub, komplett verloren. Die Insel endet (`Island::over`), `step_island` rechnet danach nichts
/// mehr, damit Spieler in allen Stufen dasselbe Ende sehen. Dass der letzte Spielstand unverändert bleibt, sichert der
/// Raum (B-344).
pub fn game_over(w: &mut World, isl: Option<&mut IslandRef>) {
    if let Some(i) = isl {
        i.over = true;
    }
    let mut ev = Event::new();
    ev.insert("cause".to_string(), "crown".into());
    emit(w, "gameOver", ev);
}

/// Go: `halveStock` halbiert jeden Posten ganzzahlig (Rundung zur Null).
pub fn halve_stock(s: &mut Stock) {
    *s = Stock {
        wood: s.wood / 2,
        stone: s.stone / 2,
        copper: s.copper / 2,
        iron: s.iron / 2,
        crystal: s.crystal / 2,
    };
}
