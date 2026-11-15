package sim

import (
	"math"
)

// Gruppen (Variante groups, B-932) und Banner-Trupps (Variante general, B-933): Zuordnung der Soldaten und Schützen.

// freeWarriors: Soldaten ohne Gruppe, Schild-Weg und Führerrolle, in Truppen-Reihenfolge.
func freeWarriors(w *World) []*Troop {
	var out []*Troop
	for _, t := range w.Troops {
		if isSoldier(t) && t.Group == 0 && !t.shieldFetch {
			out = append(out, t)
		}
	}
	return out
}

func countFetching(w *World) int {
	n := 0
	for _, t := range w.Troops {
		if t.shieldFetch {
			n++
		}
	}
	return n
}

// leadersOf: die Gruppenführer der Seite side.
func leadersOf(w *World, side float64) []*Troop {
	var out []*Troop
	for _, t := range w.Troops {
		if t.Leader && troopSide(w, t) == side {
			out = append(out, t)
		}
	}
	return out
}

// nearestTroop: aus ts die Truppe, die x am nächsten ist (bei Gleichstand die frühere); nil bei leerer Liste.
func nearestTroop(ts []*Troop, x float64) *Troop {
	var best *Troop
	for _, t := range ts {
		if best == nil || math.Abs(t.X-x) < math.Abs(best.X-x) {
			best = t
		}
	}
	return best
}

// groupCount: Soldaten der Gruppe des Führers l neben ihm.
func groupCount(w *World, l *Troop) int {
	n := 0
	for _, t := range w.Troops {
		if t.Group == l.ID && t != l {
			n++
		}
	}
	return n
}

// guardedLeader: Ein Führer fällt nicht, solange Soldaten seiner Gruppe stehen (kampf.md § 9: Vorschlag keine Wirkung).
func guardedLeader(w *World, t *Troop) bool { return t.Leader && groupCount(w, t) > 0 }

// stepGroups pflegt die Gruppen: Schilde vergeben, Führer ernennen, Gruppen füllen, Einsatz beenden ohne Ziel.
func stepGroups(w *World) {
	for _, t := range w.Troops { // Mitglieder ohne Führer (er ist gefallen) sind wieder frei
		if t.Group != 0 && !t.Leader {
			if l := troopByID(w, t.Group); l == nil || !l.Leader {
				leaveSquad(w, t)
			}
		}
	}
	for w.combat.shields > 0 {
		t := nearestTroop(freeWarriors(w), w.HubX)
		if t == nil {
			break
		}
		t.shieldFetch = true
		w.combat.shields--
	}
	for _, t := range w.Troops {
		switch {
		case t.shieldFetch && math.Abs(t.X-w.HubX) < w.values().Economy.Combat.ShieldArriveUnits:
			t.shieldFetch, t.Leader, t.Group = false, true, t.ID
			emit(w, "shield", Event{"troop": t.ID, "x": unitX(t.X)})
		case t.Leader:
			fillGroup(w, t)
			endSortie(w, t)
		}
	}
}

// fillGroup holt einen freien Soldaten der Seite in die Gruppe des Führers l (einen je Tick), bis groupSize erreicht ist.
func fillGroup(w *World, l *Troop) {
	if groupCount(w, l) >= groupSize(w) {
		return
	}
	var cand []*Troop
	for _, t := range freeWarriors(w) {
		if troopSide(w, t) == troopSide(w, l) {
			cand = append(cand, t)
		}
	}
	if t := nearestTroop(cand, l.X); t != nil {
		t.Group = l.ID
	}
}

// endSortie: Hat die Gruppe kein Ziel mehr voraus, kehrt sie ans Tor zurück.
func endSortie(w *World, l *Troop) {
	if l.sortie == 0 {
		return
	}
	if _, ok := squadTarget(w, l.X, l.sortie, true); !ok {
		l.sortie = 0
		emit(w, "groupReturned", Event{"troop": l.ID, "x": unitX(l.X)})
	}
}

// sendGroups schickt alle Gruppen der Seite los (Auslöser am Tor, kampf.md § 3.1).
func sendGroups(w *World, side float64) {
	n := 0
	for _, l := range leadersOf(w, side) {
		l.sortie = side
		n++
	}
	emit(w, "attackOrdered", Event{"side": int(side), "groups": n})
}

// takeBanner: Der Monarch p nimmt das Banner am Tor der Seite side auf (Variante general).
func takeBanner(w *World, p *Player, side int) {
	p.Banner, p.bannerOut = side, false
	emit(w, "bannerTaken", Event{"player": p.Index, "side": side})
}

// stepBanners: Banner zurückstellen, Trupps der Monarchen mit Banner füllen.
func stepBanners(w *World) {
	for _, p := range w.Players {
		if p.Banner == 0 {
			continue
		}
		side := float64(p.Banner)
		wall := outerWall(w, side)
		if wall == nil || !crowned(p) {
			returnBanner(w, p)
			continue
		}
		if (p.X-wall.X)*side > 0 {
			p.bannerOut = true
		} else if p.bannerOut { // hinter das äußerste Tor zurück
			returnBanner(w, p)
			continue
		}
		fillSquad(w, p, wall.X)
	}
	for _, t := range w.Troops { // Trupp ohne Monarch mit Banner löst sich auf
		if t.Group != 0 {
			if p := playerByID(w, t.Group); p == nil || p.Banner == 0 {
				leaveSquad(w, t)
			}
		}
	}
}

// returnBanner stellt das Banner des Monarchen p am Tor wieder auf; sein Trupp kehrt zurück.
func returnBanner(w *World, p *Player) {
	p.Banner, p.bannerOut = 0, false
	for _, t := range w.Troops {
		if t.Group == p.ID {
			leaveSquad(w, t)
		}
	}
	emit(w, "bannerReturned", Event{"player": p.Index})
}

// fillSquad ergänzt den Trupp des Monarchen p auf Soldaten und Schützen (je Art einen je Tick) vom Tor gateX; ein
// gefallener Kämpfer ist so nach einem Tick ersetzt (kampf.md § 3.2).
func fillSquad(w *World, p *Player, gateX float64) {
	c := w.values().Economy.Combat
	soldiers, archers := 0, 0
	var freeS, freeA []*Troop
	for _, t := range w.Troops {
		switch {
		case !isFighter(w.values().Troops, t.Kind):
		case t.Group == p.ID && isSoldier(t):
			soldiers++
		case t.Group == p.ID:
			archers++
		case t.Group != 0 || troopSide(w, t) != float64(p.Banner):
		case isSoldier(t):
			freeS = append(freeS, t)
		default:
			freeA = append(freeA, t)
		}
	}
	if soldiers < c.TroopSoldiers {
		if t := nearestTroop(freeS, gateX); t != nil {
			t.Group = p.ID
		}
	}
	if archers < c.TroopArchers {
		if t := nearestTroop(freeA, gateX); t != nil {
			t.Group, t.TowerID = p.ID, nil
		}
	}
}
