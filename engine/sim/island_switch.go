package sim

import (
	"fmt"
	"math"
	"slices"
)

// Inselwechsel (B-103, docs/rules/stufen.md § 1): Der Sieg über den Endboss öffnet den Wechselpunkt neben der Burg der
// tiefsten Stufe, wenn es eine nächste Insel gibt. Stehen alle lebenden, gesteuerten Spieler der Insel dort
// hub.Travel.Seconds lang, ist der Wechsel bereit (SwitchReady); den Austausch der Insel macht der Raum (B-345) mit
// NextIsland.

// isLast: Die Insel ist die letzte der Daten (kein Wechselpunkt, Kampagnensieg möglich).
func (isl *Island) isLast() bool { return isl.Number+1 >= len(islandDefs) }

// gateStage ist die tiefste Stufe der Insel; dort liegt der Wechselpunkt (an der Burg).
func (isl *Island) gateStage() *World {
	return slices.MaxFunc(isl.Stages, func(a, b *World) int { return a.Biome.Depth - b.Biome.Depth })
}

// stepIslandSwitch: Wechselpunkt öffnen und den gemeinsamen Fortschritt rechnen (nach dem Tick aller Stufen).
func stepIslandSwitch(isl *Island, dt float64) {
	if !isl.EndbossDefeated || isl.isLast() || isl.SwitchReady {
		return
	}
	w := isl.gateStage()
	if !isl.GateOpen {
		isl.GateOpen = true
		emit(w, "islandGateOpen", Event{"island": isl.Number + 1, "x": unitX(w.HubX)})
	}
	if !allAtGate(isl, w) {
		isl.switchProgress = 0
		return
	}
	isl.switchProgress += dt / hub.Travel.Seconds
	if isl.switchProgress >= 1 {
		isl.SwitchReady = true
		emit(w, "islandSwitch", Event{"island": isl.Number + 1})
	}
}

// allAtGate: Alle lebenden, gesteuerten Spieler der Insel stehen in der Stufe w am Wechselpunkt; tote und freie
// zählen nicht, einer in einer anderen Stufe lässt den Wechsel warten. Ohne solche Spieler kein Wechsel.
func allAtGate(isl *Island, w *World) bool {
	n := 0
	for _, s := range isl.Stages {
		for _, p := range s.Players {
			if !isAlive(p) || p.Free {
				continue
			}
			if s != w || math.Abs(p.X-w.HubX) > hub.Travel.RangeUnits {
				return false
			}
			n++
		}
	}
	return n > 0
}

// NextDef liefert die Daten der nächsten Insel; ok ist false auf der letzten.
func (isl *Island) NextDef() (def IslandDef, ok bool) {
	if isl.isLast() {
		return IslandDef{}, false
	}
	return islandDefs[isl.Number+1], true
}

// NextIsland baut die nächste Insel nach def: Seed aus der Spiel-ID und dem Inselindex, Startvorrat, eigene Hubs,
// Optionen der alten Insel. Die Spieler ziehen mit (Index, Gold, Skills und Slots bleiben) und stehen mit voller HP an
// der Burg der ersten Stufe; ihre Skill-Basis ist die Zahl der mitgebrachten Skills.
func NextIsland(cur *Island, def IslandDef) (*Island, error) {
	number := cur.Number + 1
	next, err := createIsland(fmt.Sprintf("%s:%d", cur.ID, number), def.Depths, cur.CycleSpeed, 0)
	if err != nil {
		return nil, err
	}
	*next.Stock = hub.IslandStartStock
	next.ID, next.Number, next.Scaling, next.Options, next.nextPlayer = cur.ID, number, def.DepthScaling, cur.Options, cur.nextPlayer
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
