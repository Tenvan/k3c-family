package sim

import "math"

// Feedback-Ereignisse (B-139, Beschluss Q08): kurze Felder, damit im Mittel ≤ 200 Byte je Tick und Client reichen.
// Orte `x` in Units; auf einer Insel trägt jedes Ereignis zusätzlich `stage` (Index der Stufe, StepIsland).
//
//	hit           – Schaden wirkt: x, target (player, troop, enemy, castle, site), id (Spieler: Index, sonst ID), damage
//	kill          – Gegner besiegt: kind, x, gold (gestreute Münzen); je Tod, mit Priorität (`enemyKilled` aus B-128)
//	arrow         – Geschoss abgeschossen: from, to (IDs), x, team (player, enemy); auch der Splitter-Wurf (Splash: je
//	                getroffenem Ziel ein `hit`). Flächen der Flammenspur melden nur ihre `hit` (boss_abilities.go)
//	strike        – Nahkampf-Schlag eines Gegners: from, x
//	coinPickup    – Münze aufgehoben: player (Index), x
//	coinGive      – Münze gegeben: player, x, to (site, recruit, mark, offer, merchant); fällt sie nur zu Boden, kein Ereignis
//	buildProgress – Bau fortgeschritten: site (ID), kind, x, percent (25, 50, 75; fertig = built)
//	revive        – Monarch steht nach der Wartezeit wieder: player, x
//	revived       – Monarch von einem Mitspieler wiederbelebt (A halten, revive.go; Q62): player, x
//	trained       – Bauer ausgebildet (professions.go): kind (miner, builder, craftsman), x
//	merchantArrived – Händler kommt (merchant.go): resource, x (Zahlziel Kaufen); merchantLeft – Händler reist ab
//	disarmed      – Bürger verliert seine Ausrüstung (Verlust-Kaskade, warrior.go; Q67, Q69): kind (Figur oder
//	                Beruf), x, cause (Gegnerart wie playerDown); ohne Priorität, nicht beim Burgfall
//	equipmentTaken – Gegner trägt Ausrüstung weg (Q68, Q69); nur angelegt, das Aufheben durch Gegner baut K1
//	traded        – Tausch am Händler: player, resource, amount (+ gekauft, − verkauft), gold (− bezahlt, + erhalten)
//	bossSpawned   – Boss erscheint (boss.go): boss (ID aus data/bosses.json), x
//	bossDefeated  – Boss besiegt, Belohnung gegeben (boss.go): boss, x; statt `kill`
//	bossPhase     – Endboss wechselt in die nächste Phase, je Wechsel einmal (boss_endboss.go): boss, phase (ab 2)
//	victory       – Ziel der Insel erreicht, genau einmal je Insel, in Stufe 0 (victory.go): goal (Variante), day
//	gameOver      – Burg gefallen im Niederlage-Modus „Komplett verloren“ (defeat.go): ohne Felder außer stage; danach
//	                ruht die Insel (Island.Over), nach `castleFallen` im selben Tick
//	islandGateOpen – Endboss besiegt, Wechselpunkt zur nächsten Insel offen, in der tiefsten Stufe (island_switch.go):
//	                island (Index der nächsten Insel), x (Burg)
//	islandSwitch  – alle lebenden Spieler stehen am Wechselpunkt, der Raum tauscht die Insel: island
//	eventStarted  – Event der Nacht beginnt, je Nacht und Event einmal, in Stufe 0 (events_moon.go): event (fullMoon,
//	                bloodMoon), day
//	eventEnded    – Event der vergangenen Nacht endet bei Tagesanbruch (events_moon.go): event, day (die Nacht)
//
//	playerDown    – Monarch fällt: player, cause (Gegnerart aus data/enemies.json bei Nahkampf und Geschoss,
//	                sonst "other"; B-182)
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
	case t.sword != nil:
		to = "offer"
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
// `kill` hat Priorität (K1.2, B-128/AC-04): Jeder Tod eines Gegners wird gemeldet, auch wenn Flächenschlag oder Schwarm
// viele Treffer im selben Tick erzeugen.
const maxEventsPerTick = 32

// priorityEvent: Tod und Bau gehen bei Überlauf vor (B-139/AC-03).
func priorityEvent(ev Event) bool {
	switch ev["type"] {
	case "playerDown", "castleFallen", "built", "buildProgress", "destroyed", "kill":
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
