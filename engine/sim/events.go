package sim

import "math"

// Feedback-Ereignisse (B-139, Beschluss Q08): kurze Felder, damit im Mittel ≤ 200 Byte je Tick und Client reichen.
// Orte `x` in Units. Kampf (F3.1):
//
//	hit    – Schaden wirkt: x, target (player, troop, enemy, castle, site), id (Spieler: Index, sonst ID), damage
//	kill   – Gegner besiegt: kind, x, gold (gestreute Münzen)
//	arrow  – Geschoss abgeschossen: from, to (IDs), x, team (player, enemy)
//	strike – Nahkampf-Schlag eines Gegners: from, x

// emit hängt ein Ereignis an. Es liest nur und verbraucht kein rng, damit sich der Spielverlauf nicht ändert.
func emit(w *World, typ string, fields Event) {
	fields["type"] = typ
	w.Events = append(w.Events, fields)
}

// unitX rundet einen Ort auf eine Nachkommastelle (kürzer im Protokoll; nur für Ereignisse, nie für den Zustand).
func unitX(x float64) float64 { return math.Round(x*10) / 10 }

// hitEvent meldet wirksamen Schaden an einem Ziel.
func hitEvent(w *World, target string, id int, x, damage float64) {
	emit(w, "hit", Event{"x": unitX(x), "target": target, "id": id, "damage": damage})
}

// arrowEvent meldet ein neues Geschoss.
func arrowEvent(w *World, p *Projectile, from int) {
	emit(w, "arrow", Event{"from": from, "to": p.TargetID, "x": unitX(p.X), "team": p.Team})
}
