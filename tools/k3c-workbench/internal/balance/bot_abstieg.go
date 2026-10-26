package balance

import (
	"k3c/engine/sim"
)

// Abstieg (BS11.2, B-642, Regel nach B-643): Im Insel-Lauf steigen die Bots über den Tiefen-Eingang am Levelende ab, sobald
// ihre Stufe sicher steht: die Mauern der Linie 1 stehen auf beiden Seiten, es ist heller Tag und mindestens Tag
// minDescentDay. Der Hub-Ausbau gehört nicht zur Regel: Den Stein für Hub-Stufe 2 liefert die Höhle (Adern, Regel
// materialien-gebaeude.md § 1), der Weg nach unten kommt also vor dem Ausbau. Jeder Spieler wechselt allein
// (Einzelwechsel, stufen.md § 1); weil die Regel nur von der Stufe abhängt, gehen alle Spieler der Stufe zusammen.
// Auf der neuen Stufe spielt der Bot sein Profil weiter: Mauern und Bauten gelten je Stufe.

// minDescentDay ist der früheste Tag des Abstiegs (Tag 1 gehört dem Aufbau der ersten Mauern).
const minDescentDay = 2

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

// stageSafe: Mauern der innersten Linie auf beiden Seiten gebaut, heller Tag, ab Tag minDescentDay.
func stageSafe(w *sim.World) bool {
	return w.Cycle.Phase == "day" && w.Cycle.Day >= minDescentDay && innerWallsBuilt(w)
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
