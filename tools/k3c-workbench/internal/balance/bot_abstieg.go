package balance

import (
	"k3c/engine/sim"
)

// Abstieg (BS11.2, B-642): Im Insel-Lauf (Szenario mit World) steigen die Bots über den Tiefen-Eingang am Levelende ab,
// sobald ihre Stufe sicher steht (Default aus B-642): Hub-Stufe mindestens Tiefe + 2 (die letzte Hub-Stufe genügt) und
// die Mauern der Linie 1 stehen auf beiden Seiten. Sie gehen nur am hellen Tag. Jeder Spieler wechselt allein
// (Einzelwechsel, stufen.md § 1); weil die Regel nur von der Stufe abhängt, gehen alle Spieler der Stufe zusammen.
// Fällt der Hub-Ausbau aus (im Wald fehlt Stein: der Hub-Ausbau braucht 25, der Vorrat bleibt bei 20), steigen sie ab dem
// Tag fallbackDay + 2 × Tiefe trotzdem ab (Notausstieg, Abweichung vom Default, B-642 gemeldet); sonst liefe der Tester nie über
// Tiefe 0 hinaus. Auf der neuen Stufe spielt der Bot sein Profil weiter: Hub-Ausbau, Mauern und Bauten gelten je Stufe.

// fallbackDay ist der Tag, ab dem der Notausstieg der Stufe Tiefe 0 gilt; jede Tiefe darunter zwei Tage später.
const fallbackDay = 4

// withDescent legt den Abstieg über ein Profil.
func withDescent(inner Bot, deepest int) Bot {
	return func(w *sim.World, p *sim.Player) sim.PlayerCommand {
		if x, ok := descentX(w, deepest); ok && stageSafe(w) {
			return goTo(p, x, false)
		}
		return inner(w, p)
	}
}

// descentX: der Tiefen-Eingang der Stufe, wenn es eine tiefere Stufe der Insel gibt.
func descentX(w *sim.World, deepest int) (float64, bool) {
	if w.Biome.Depth >= deepest {
		return 0, false
	}
	for _, e := range w.Level.Entities {
		if e.Kind == "exit" {
			return e.X, true
		}
	}
	return 0, false
}

// stageSafe: Hub-Stufe Tiefe + 2 (oder die letzte), Mauern der innersten Linie auf beiden Seiten gebaut, heller Tag.
func stageSafe(w *sim.World) bool {
	if w.Cycle.Phase != "day" {
		return false
	}
	hubOK := sim.EconomyOf(w).HubUpgrade == nil || w.HubLevel >= w.Biome.Depth+2 || w.Cycle.Day >= fallbackDay+2*w.Biome.Depth
	return hubOK && innerWallsBuilt(w)
}

// innerWallsBuilt: je Seite mit Linien steht die Mauer der innersten Linie (Index 1).
func innerWallsBuilt(w *sim.World) bool {
	need := map[int]float64{}
	for _, l := range w.Level.Lines {
		if l.Index == 1 {
			need[l.Side] = l.Wall
		}
	}
	for _, x := range need {
		ok := false
		for _, s := range w.Sites {
			ok = ok || (s.Kind == "wall" && s.X == x && s.State == "built")
		}
		if !ok {
			return false
		}
	}
	return len(need) > 0
}

// deepestOf ist die tiefste Tiefe der Stufen der Insel.
func deepestOf(isl *sim.Island) int {
	deepest := 0
	for _, w := range isl.Stages {
		deepest = max(deepest, w.Biome.Depth)
	}
	return deepest
}
