//! Einwurf an der Hütte des Einsiedlers (RU62.10, B-972): Der Monarch wirft mit der Zahlen-Taste `price` Münzen auf einmal
//! ein. Zu wenig Münzen, schon bezahlt (auch im selben Takt von einem anderen Monarchen) oder Hub-Stufe noch nicht erreicht:
//! abgelehnt, nichts wird abgezogen. Wer zahlt, ist gleich; die Hütte bleibt für beide Monarchen gleich. Der Bau beginnt in
//! der Insel direkt nach den Stufen (`step.rs`), weil das Bauwerk in der Stufe der Tiefe 0 steht.
use serde_json::json;

use crate::events::emit;
use crate::state::World;
use crate::state::common::event;
use crate::state::hermit_view::HermitPhase;

/// Der Zahl-Teil an der Hütte für den lebenden Monarchen `pi`. `pay` ist die gehaltene Taste, `pressed` ihr erster Takt (nur
/// dann zählt der Einwurf und kommt die Ablehnung als Ereignis, einmal je Druck). Liefert `true`, wenn der Monarch in
/// Reichweite der Hütte steht: dann wirft die normale Zahlung keine Münze (sie fiele sonst auf den Boden). Ohne Hütte immer
/// `false`.
pub fn step_hut(w: &mut World, pi: usize, pay: bool, pressed: bool) -> bool {
    let Some(h) = w.hermit.as_ref() else {
        return false;
    };
    let Some(hut) = h.hut.as_ref() else {
        return false;
    };
    let (hx, range, price) = (hut.x, hut.range, h.price);
    let (done, pending, open) = (h.phase != HermitPhase::Hut, h.pending, h.open);
    let p = &w.players[pi];
    if (p.x - hx).abs() > range {
        return false;
    }
    // Nur der Tastendruck an der Hütte zählt: wer die Taste gehalten hineinläuft, zahlt nicht ungewollt. Ist der Einsiedler
    // schon bezahlt, ist die Hütte leer: nur der Druck wird abgelehnt, sonst bleibt sie kein totes Zahlziel.
    if !pay || !pressed {
        return !done || pending;
    }
    let (index, x, gold) = (p.index, p.x, p.gold);
    let reason = if done || pending {
        Some("paid")
    } else if !open {
        Some("hubLevel")
    } else if gold < price {
        Some("coins")
    } else {
        None
    };
    if let Some(reason) = reason {
        emit(
            w,
            "hermitRejected",
            event(json!({"player": index, "x": x, "reason": reason})),
        );
        return true;
    }
    w.players[pi].gold -= price;
    let hermit = w.hermit.as_mut().map(|h| {
        h.pending = true;
        h.open = false;
        h.hermit.clone()
    });
    emit(
        w,
        "hermitPaid",
        event(json!({"player": index, "x": x, "hermit": hermit.unwrap_or_default(), "price": price})),
    );
    true
}
