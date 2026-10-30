package sim

import (
	"fmt"
	"math"
	"slices"
)

// Inselwechsel (B-103, B-637, docs/rules/stufen.md § 1): Der Sieg über den Endboss öffnet den Wechselpunkt am Steg des
// Hafens (Stufe mit dem fertigen Schiff, ship.go), wenn es eine nächste Insel gibt. Stehen alle lebenden, gesteuerten Spieler der Insel dort
// hub.Travel.Seconds lang, ist der Wechsel bereit (SwitchReady); den Austausch der Insel macht der Raum (B-345) mit
// NextIsland.

// isLast: Die Insel ist die letzte ihrer Welt (kein Wechselpunkt, Kampagnensieg möglich).
func (isl *Island) isLast() bool { return isl.Number+1 >= len(isl.world.Islands) }


// stepIslandSwitch: Wechselpunkt öffnen und den gemeinsamen Fortschritt rechnen (nach dem Tick aller Stufen).
func stepIslandSwitch(isl *Island, dt float64) {
	if !isl.EndbossDefeated || isl.isLast() || isl.SwitchReady {
		return
	}
	w, ship := isl.harbor()
	if ship == nil || !shipComplete(w, ship) {
		isl.GateOpen, isl.switchProgress = false, 0 // Schiff verloren (Stufenverlust): der Wechselpunkt schließt
		return
	}
	if !isl.GateOpen {
		isl.GateOpen = true
		emit(w, "islandGateOpen", Event{"island": isl.Number + 1, "x": unitX(ship.X)})
	}
	if !allAtGate(isl, w, ship.X) {
		isl.switchProgress = 0
		return
	}
	isl.switchProgress += dt / hub.Travel.Seconds
	if isl.switchProgress >= 1 {
		isl.SwitchReady = true
		emit(w, "islandSwitch", Event{"island": isl.Number + 1})
	}
}

// allAtGate: Alle lebenden, gesteuerten Spieler der Insel stehen in der Stufe w am Steg bei x; tote und freie
// zählen nicht, einer in einer anderen Stufe lässt den Wechsel warten. Ohne solche Spieler kein Wechsel.
func allAtGate(isl *Island, w *World, x float64) bool {
	n := 0
	for _, s := range isl.Stages {
		for _, p := range s.Players {
			if !isAlive(p) || p.Free {
				continue
			}
			if s != w || math.Abs(p.X-x) > hub.Travel.RangeUnits {
				return false
			}
			n++
		}
	}
	return n > 0
}

// NextDef liefert die Daten der nächsten Insel der Welt; ok ist false auf der letzten.
func (isl *Island) NextDef() (def IslandDef, ok bool) {
	if isl.isLast() {
		return IslandDef{}, false
	}
	return isl.world.Islands[isl.Number+1], true
}

// NextIsland baut die nächste Insel nach def in der Welt der alten Insel: Seed aus der Spiel-ID und dem Inselindex,
// Startvorrat, eigene Hubs, Optionen der alten Insel, Insel-ID aus def. Die Spieler ziehen mit (Index, Gold, Skills und Slots bleiben) und stehen mit voller HP an
// der Burg der ersten Stufe; ihre Skill-Basis ist die Zahl der mitgebrachten Skills.
func NextIsland(cur *Island, def IslandDef) (*Island, error) {
	if want, ok := cur.NextDef(); !ok || want.ID != def.ID {
		return nil, fmt.Errorf("insel: %q ist nicht die nächste Insel der Welt %q", def.ID, cur.world.ID)
	}
	number := cur.Number + 1
	next, err := createIsland(fmt.Sprintf("%s:%d", cur.ID, number), def.Depths, cur.CycleSpeed, 0, def.CreateOptions(), cur.world)
	if err != nil {
		return nil, err
	}
	*next.Stock = hub.IslandStartStock
	next.ID, next.Number, next.islandID, next.Scaling = cur.ID, number, def.ID, def.DepthScaling
	next.Options, next.nextPlayer = cur.Options, cur.nextPlayer
	first := next.Stages[0]
	for _, p := range cur.Players() {
		q := *p
		q.ID = first.newID()
		q.PayCooldown, q.Paying, q.PayKey, q.PayAmount = 0.5, false, nil, 0
		q.Skills, q.Slots, q.Cooldowns = slices.Clone(p.Skills), slices.Clone(p.Slots), nil
		q.SkillBase = len(q.Skills)
		respawn(first, &q)
		first.Players = insertPlayer(first.Players, &q)
	}
	return next, nil
}
