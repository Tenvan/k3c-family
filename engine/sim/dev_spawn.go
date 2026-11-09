package sim

import (
	"math"
	"slices"
)

// Dev-Aktion spawn (B-766, RX1): setzt Gegner oder Truppen in die Stufe eines Spielers, für den Stresstest des echten Clients.
// Nur für den Dev-Mode des Raums (engine/room/dev_combat.go), keine Spielregel. Die Figuren stehen gleichmäßig verteilt um den
// Spieler (kein rng, damit der Strom des Spiels unberührt bleibt) und handeln danach wie alle anderen.

// DevMaxSpawn ist die größte Zahl je Aufruf; Aufrufe lassen sich wiederholen, die Obergrenze schützt das Tick-Budget.
const DevMaxSpawn = 200

// devSpawnSpan ist die Breite (Units), auf die sich die gesetzten Figuren verteilen, um den Spieler zentriert.
const devSpawnSpan = 40.0

// DevSpawnUnits stellt count Figuren der Art kind ("enemy" oder "troop") und der Sorte unit (Schlüssel aus data/enemies.json
// bzw. data/troops.json) in die Stufe des Spielers index. Liefert die Zahl der Figuren; Fehler: unbekannter Spieler, Art oder
// Sorte oder count außerhalb von 1…DevMaxSpawn.
func DevSpawnUnits(isl *Island, index int, kind, unit string, count int) (int, error) {
	stage := isl.StageOf(index)
	if stage < 0 || count < 1 || count > DevMaxSpawn {
		return 0, errDevInvalid
	}
	w := isl.Stages[stage]
	i := slices.IndexFunc(w.Players, func(p *Player) bool { return p.Index == index })
	if i < 0 {
		return 0, errDevInvalid
	}
	switch kind {
	case "enemy":
		if _, ok := w.values().Enemies[unit]; !ok {
			return 0, errDevInvalid
		}
	case "troop":
		if _, ok := w.values().Troops[unit]; !ok {
			return 0, errDevInvalid
		}
	default:
		return 0, errDevInvalid
	}
	for n := 0; n < count; n++ {
		x := math.Min(math.Max(devSpawnX(w.Players[i].X, n, count), 0), w.WidthUnits)
		if kind == "enemy" {
			spawnEnemy(w, unit, x)
			continue
		}
		hp := w.values().Troops[unit].HP
		w.Troops = append(w.Troops, &Troop{ID: w.newID(), Kind: unit, X: x, HP: hp, MaxHP: hp, AnchorX: w.HubX, TargetX: x})
	}
	return count, nil
}

// devSpawnX verteilt Figur n von count gleichmäßig auf devSpawnSpan Units um center.
func devSpawnX(center float64, n, count int) float64 {
	if count == 1 {
		return center
	}
	return center + (float64(n)/float64(count-1)-0.5)*devSpawnSpan
}
