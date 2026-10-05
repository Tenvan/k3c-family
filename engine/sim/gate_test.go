package sim

import (
	"fmt"
	"testing"
)

// W3.1 (B-116/AC-01, Sprint W3 AC-01): Das Tor hält Gegner auf und ist ihr Angriffsziel wie eine Mauer (Q27),
// Spieler und Truppen passieren; Tore innerer Linien wirken weiter (Q54).

// enemyBefore lässt einen Skelett-Gegner bei x loslaufen und rechnet seconds Sekunden.
func enemyBefore(w *World, x, seconds float64) *Enemy {
	e := spawnEnemy(w, "skeleton", x)
	run(w, seconds)
	return e
}

// sturdy: gebaut und so fest, dass er während der Messung hält.
func sturdy(s *Site) *Site {
	built(s)
	s.MaxHP, s.HP = 1e6, 1e6
	return s
}

func TestTorHaeltGegnerAufBisZurZerstoerung(t *testing.T) {
	w := dayWorld(t)
	AddPlayer(w)
	AddPlayer(w)
	wall := sturdy(lineSite(t, w, "wall", -1, 1))
	gate := sturdy(lineSite(t, w, "gate", -1, 1))
	e := enemyBefore(w, gate.X-8, 8)
	if e.X >= gate.X || gate.HP >= gate.MaxHP {
		t.Fatalf("Gegner bei %v, Tor bei %v mit HP %v: nicht aufgehalten oder nicht angegriffen", e.X, gate.X, gate.HP)
	}
	destroySite(w, gate)
	run(w, 8)
	if e.X <= gate.X || e.X >= wall.X || wall.HP >= wall.MaxHP {
		t.Errorf("nach der Zerstörung: Gegner bei %v, erwartet vor der Mauer %v (Tor %v)", e.X, wall.X, gate.X)
	}
}

func TestEigenePassierenDasTor(t *testing.T) {
	w := dayWorld(t)
	p1, p2 := AddPlayer(w), AddPlayer(w)
	built(lineSite(t, w, "wall", -1, 1))
	gate := lineSite(t, w, "gate", -1, 1)
	built(gate)
	for _, p := range []*Player{p1, p2} {
		p.X = gate.X + 3
	}
	cmds := []PlayerCommand{{MoveX: -1}, {MoveX: -1}}
	for tm := 0.0; tm < 3; tm += dt {
		Step(w, cmds, dt)
	}
	peasant := w.Troops[0]
	peasant.X = gate.X + 3
	for tm := 0.0; tm < 5 && !walkTo(peasant, gate.X-3, dt); tm += dt {
	}
	for _, x := range []float64{p1.X, p2.X, peasant.X} {
		if x >= gate.X-2 {
			t.Errorf("Eigene bei %v, nicht durch das Tor bei %v", x, gate.X)
		}
	}
}

// Linie 2 außen gebaut: Gegner bleibt am äußersten Tor, dann an der Mauer der Linie, dann am inneren Tor (Q54);
// Schützen bleiben hinter der äußersten Mauer (Q27).
func TestTorAeussersteLinieDannInnere(t *testing.T) {
	w := dayWorld(t)
	w.HubLevel = 2
	sites := map[string]*Site{}
	for _, k := range []int{1, 2} {
		for _, kind := range []string{"wall", "gate"} {
			sites[fmt.Sprint(kind, k)] = sturdy(lineSite(t, w, kind, -1, k))
		}
	}
	if outerWall(w, -1) != sites["wall2"] {
		t.Errorf("Posten der Schützen an %v, erwartet Mauer 2 bei %v", outerWall(w, -1).X, sites["wall2"].X)
	}
	e := enemyBefore(w, sites["gate2"].X-8, 8)
	for _, next := range []string{"gate2", "wall2", "gate1"} {
		s := sites[next]
		if e.X >= s.X || s.HP >= s.MaxHP {
			t.Fatalf("Gegner bei %v: nicht vor %s bei %v aufgehalten (HP %v)", e.X, next, s.X, s.HP)
		}
		destroySite(w, s)
		run(w, 15)
	}
}
