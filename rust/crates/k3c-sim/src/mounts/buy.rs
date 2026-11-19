//! Kauf des Welt-Reittiers am Gatter (RU62.8, B-970): Der Monarch wirft mit der Zahlen-Taste `price` Münzen auf einmal ein
//! und reitet danach das Welt-Reittier. Zu wenig Münzen oder schon gekauft: abgelehnt, nichts wird abgezogen. Das Gatter
//! bleibt für den anderen Monarchen offen; im selben Takt kaufen beide in der Reihenfolge der Spieler-Indizes.
use serde_json::json;

use crate::events::emit;
use crate::state::World;
use crate::state::common::event;

/// Der Zahl-Teil am Gatter für den lebenden Monarchen `pi`. `pay` ist die gehaltene Taste, `pressed` ihr erster Takt (nur
/// dann kommt die Ablehnung als Ereignis, einmal je Druck). Liefert `true`, wenn der Monarch in Reichweite des Gatters
/// steht: dann wirft die normale Zahlung keine Münze (sie fiele sonst auf den Boden). Ohne Gatter immer `false`.
pub fn step_gate(w: &mut World, pi: usize, pay: bool, pressed: bool) -> bool {
    let Some(g) = w.mount_gate.as_ref() else {
        return false;
    };
    let (gx, range, price) = (g.x, g.range, g.price);
    let p = &w.players[pi];
    if (p.x - gx).abs() > range {
        return false;
    }
    // Nur der Tastendruck am Gatter zählt: wer die Taste gehalten hineinläuft, kauft nicht ungewollt.
    if !pay || !pressed {
        return true;
    }
    let (index, x) = (p.index, p.x);
    if p.mount == g.mount {
        if pressed {
            emit(
                w,
                "mountRejected",
                event(json!({"player": index, "x": x, "reason": "owned"})),
            );
        }
    } else if p.gold < price {
        if pressed {
            emit(
                w,
                "mountRejected",
                event(json!({"player": index, "x": x, "reason": "coins"})),
            );
        }
    } else {
        let id = g.mount.clone();
        let p = &mut w.players[pi];
        p.gold -= price;
        p.mount.clone_from(&id);
        emit(
            w,
            "mountBought",
            event(json!({"player": index, "x": x, "mount": id, "price": price})),
        );
    }
    true
}
