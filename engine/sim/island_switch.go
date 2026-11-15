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

// isLast: Die Insel ist die letzte ihrer Welt: keine Ziele mehr (kein Wechselpunkt, Kampagnensieg möglich, B-961).
func (isl *Island) isLast() bool { return len(isl.targetIndexes()) == 0 }

// stepIslandSwitch: Wechselpunkt öffnen und den gemeinsamen Fortschritt rechnen (nach dem Tick aller Stufen).
func stepIslandSwitch(isl *Island, dt float64) {
	if !isl.EndbossDefeated || isl.isLast() || isl.SwitchReady {
		return
	}
	isl.applyChoices()
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
	isl.switchProgress += dt / isl.values().Hub.Travel.Seconds
	if isl.switchProgress >= 1 {
		isl.SwitchReady = true
		emit(w, "islandSwitch", Event{"island": isl.Number + 1, "target": isl.ChosenTarget()})
	}
}

// allAtGate: Alle lebenden, gesteuerten Spieler der Insel stehen in der Stufe w am Steg bei x; tote und freie
// zählen nicht, einer in einer anderen Stufe lässt den Wechsel warten. Ohne solche Spieler kein Wechsel.
func allAtGate(isl *Island, w *World, x float64) bool {
	n := 0
	for _, s := range isl.Stages {
		for _, p := range s.Players {
			if !crowned(p) || p.Free {
				continue
			}
			if s != w || math.Abs(p.X-x) > w.values().Hub.Travel.RangeUnits {
				return false
			}
			n++
		}
	}
	return n > 0
}

// NextDef liefert die Daten des gewählten Ziels (Wahl, sonst erstes Ziel); ok ist false auf der letzten Insel.
func (isl *Island) NextDef() (def IslandDef, ok bool) {
	i, ok := isl.chosenIndex()
	if !ok {
		return IslandDef{}, false
	}
	return isl.world.Islands[i], true
}

// NextIsland baut die Insel def (das gewählte Ziel, NextDef) in der Welt der alten Insel: Seed aus der Spiel-ID und dem
// Listenplatz des Ziels, Startvorrat, eigene Hubs, Optionen der alten Insel, Insel-ID aus def. Die abgeschlossenen
// Inseln ziehen mit (die alte kommt dazu). Die Spieler ziehen mit (Index, Gold, Skills und Slots bleiben) und stehen mit
// Krone an der Burg der ersten Stufe; ihre Skill-Basis ist die Zahl der mitgebrachten Skills.
func NextIsland(cur *Island, def IslandDef) (*Island, error) {
	number, ok := cur.chosenIndex()
	if !ok || cur.world.Islands[number].ID != def.ID {
		return nil, fmt.Errorf("insel: %q ist nicht das Ziel des Wechsels in Welt %q", def.ID, cur.world.ID)
	}
	next, err := createIsland(fmt.Sprintf("%s:%d", cur.ID, number), def.Depths, cur.CycleSpeed, 0, def.CreateOptions(), cur.world, def)
	if err != nil {
		return nil, err
	}
	next.Completed = slices.Clone(cur.Completed)
	if id := cur.world.Islands[cur.Number].ID; cur.EndbossDefeated && !slices.Contains(next.Completed, id) {
		next.Completed = append(next.Completed, id)
	}
	*next.Stock = next.values().Hub.IslandStartStock
	next.ID, next.Number, next.islandID, next.Scaling = cur.ID, number, def.ID, def.DepthScaling
	next.Options, next.nextPlayer = cur.Options, cur.nextPlayer
	first := next.Stages[0]
	for _, p := range cur.Players() {
		q := *p
		q.ID = first.newID()
		q.PayCooldown, q.Paying, q.PayKey, q.PayAmount = 0.5, false, nil, 0
		q.Skills, q.Slots, q.Cooldowns = slices.Clone(p.Skills), slices.Clone(p.Slots), nil
		q.SkillBase = len(q.Skills)
		q.Crown = nil
		toHub(first, &q)
		first.Players = insertPlayer(first.Players, &q)
	}
	return next, nil
}
