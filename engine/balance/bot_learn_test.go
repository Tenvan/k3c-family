package balance

import (
	"slices"
	"testing"

	"k3c/engine/sim"
)

// B-647 Weg B: Ein Bot mit freien Skill-Punkten lernt in der Reihenfolge der Tank-Linie, nie mehr als er Punkte hat.
func TestBotLerntSkillsInReihenfolge(t *testing.T) {
	isl, err := newIsland(Scenario{Seed: "1", Players: 1, Days: 1, World: "standard"})
	if err != nil {
		t.Fatal(err)
	}
	p := isl.Stages[0].Players[0]
	botLearn(isl)
	if len(p.Skills) != 0 {
		t.Fatalf("ohne Punkte gelernt: %v", p.Skills)
	}
	isl.SkillPool = 3
	for _, s := range isl.Stages {
		s.SkillPoints = isl.SkillPool
	}
	botLearn(isl)
	want := []string{"taunt", "shieldBash", "regeneration"}
	if !slices.Equal(p.Skills, want) {
		t.Errorf("gelernt %v, erwartet %v", p.Skills, want)
	}
	if sim.AvailablePoints(isl.Stages[0], p) != 0 {
		t.Errorf("Punkte übrig: %d", sim.AvailablePoints(isl.Stages[0], p))
	}
	botLearn(isl) // ein zweiter Aufruf ändert nichts
	if len(p.Skills) != 3 {
		t.Errorf("zweiter Aufruf lernte weiter: %v", p.Skills)
	}
}
