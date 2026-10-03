package sim

import "math"

// Feedback-Ereignisse (B-139, Beschluss Q08): kurze Felder, damit im Mittel ≤ 200 Byte je Tick und Client reichen.
// Orte `x` in Units; auf einer Insel trägt jedes Ereignis zusätzlich `stage` (Index der Stufe, StepIsland).
//
//	hit           – Schaden wirkt: x, target (player, troop, enemy, castle, site), id (Spieler: Index, sonst ID), damage
//	kill          – Gegner besiegt: kind, x, gold (gestreute Münzen)
//	arrow         – Geschoss abgeschossen: from, to (IDs), x, team (player, enemy)
//	strike        – Nahkampf-Schlag eines Gegners: from, x
//	coinPickup    – Münze aufgehoben: player (Index), x
//	coinGive      – Münze gegeben: player, x, to (site, recruit, mark); fällt sie nur zu Boden, kein Ereignis
//	buildProgress – Bau fortgeschritten: site (ID), kind, x, percent (25, 50, 75; fertig = built)
//	revive        – Monarch steht nach der Wartezeit wieder: player, x
//
// Auf bestehende Ereignisse abgebildet (Auslegung von Q08, bestätigt mit der Freigabe von F3): Tod = playerDown,
// Bau fertig = built, Skill = skillPoint, Nacht naht = dusk, Portal = arrived.

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

// coinGiveEvent meldet eine Münze, die ein Ziel bezahlt (Bauplatz, Landstreicher, Markierung einer Ressource).
func coinGiveEvent(w *World, p *Player, t *payTarget) {
	to := "mark"
	switch {
	case t.site != nil:
		to = "site"
	case t.troop != nil:
		to = "recruit"
	}
	emit(w, "coinGive", Event{"player": p.Index, "x": unitX(p.X), "to": to})
}

// buildProgressEvent meldet das Überschreiten von 25, 50 und 75 % (100 % ist `built`), höchstens eins je Schritt.
func buildProgressEvent(w *World, s *Site, before float64) {
	step := math.Floor(s.BuildProgress * 4)
	if step > math.Floor(before*4) && step >= 1 && step <= 3 {
		emit(w, "buildProgress", Event{"site": s.ID, "kind": s.Kind, "x": unitX(s.X), "percent": int(step) * 25})
	}
}

// maxEventsPerTick ist die Obergrenze K je Tick und Stufe. Vorläufig, 🧑 bestätigt mit der Freigabe bzw. F4 (Benchmark):
// Gemessen über alle Golden-Läufe nach F3.2: größter Tick 19 Ereignisse (forest-nacht, forest-tag), Mittel ≤ 0,1 je Tick.
const maxEventsPerTick = 32

// priorityEvent: Tod und Bau gehen bei Überlauf vor (B-139/AC-03).
func priorityEvent(ev Event) bool {
	switch ev["type"] {
	case "playerDown", "castleFallen", "built", "buildProgress", "destroyed":
		return true
	}
	return false
}

// capEvents begrenzt die Ereignisse eines Ticks auf maxEventsPerTick: zuerst alle mit Priorität, dann die übrigen in
// ihrer Reihenfolge; die Reihenfolge der behaltenen bleibt. EventsDropped zählt die Verworfenen.
func capEvents(w *World) {
	w.EventsDropped = 0
	if len(w.Events) <= maxEventsPerTick {
		return
	}
	keep := make([]bool, len(w.Events))
	room := maxEventsPerTick
	for _, prio := range []bool{true, false} {
		for i, ev := range w.Events {
			if room > 0 && priorityEvent(ev) == prio {
				keep[i], room = true, room-1
			}
		}
	}
	kept := make([]Event, 0, maxEventsPerTick)
	for i, ev := range w.Events {
		if keep[i] {
			kept = append(kept, ev)
		}
	}
	w.EventsDropped = len(w.Events) - len(kept)
	w.Events = kept
}
