package balance

import (
	"testing"

	"k3c/engine/sim"
)

// Ein Bot ohne Krone läuft zur liegenden Krone, ein Träger zum Besitzer (B-931), sonst gilt das Profil.
func TestBotHebtKroneAuf(t *testing.T) {
	isl, err := newIsland(Scenario{Seed: "1", Players: 2, Days: 1, World: "standard"})
	if err != nil {
		t.Fatal(err)
	}
	w := isl.Stages[0]
	p0, p1 := w.Players[0], w.Players[1]
	p0.X, p1.X = w.HubX, w.HubX+20
	idle := sim.PlayerCommand{Skill: 2}
	if got := retrieveCrown(w, p0, idle); got != idle {
		t.Fatalf("mit Krone gilt das Profil: %+v", got)
	}
	p0.Crown = &sim.Crown{Owner: 0, State: "ground", X: w.HubX - 10}
	if got := retrieveCrown(w, p0, idle); got.MoveX >= 0 {
		t.Fatalf("Besitzer läuft nicht zur Krone: %+v", got)
	}
	if got := retrieveCrown(w, p1, idle); got.MoveX >= 0 {
		t.Fatalf("Mitspieler läuft nicht zur fremden Krone: %+v", got)
	}
	p0.Crown.State, p0.Crown.Carrier = "player", p1.ID
	if got := retrieveCrown(w, p1, idle); got.MoveX >= 0 {
		t.Fatalf("Träger läuft nicht zum Besitzer: %+v", got)
	}
}

// healMonarchs lässt eine vom Gegner getragene Krone dort.
func TestHelpHealMonarchsKroneBeimGegner(t *testing.T) {
	isl, err := newIsland(Scenario{Seed: "1", Players: 2, Days: 1, World: "standard"})
	if err != nil {
		t.Fatal(err)
	}
	p0, p1 := isl.Stages[0].Players[0], isl.Stages[0].Players[1]
	p0.Crown, p1.Crown = &sim.Crown{State: "enemy"}, &sim.Crown{State: "ground"}
	healMonarchs(isl)
	if p0.Crown == nil || p1.Crown != nil {
		t.Fatalf("Gegner-Krone %v, liegende Krone %v", p0.Crown, p1.Crown)
	}
}
