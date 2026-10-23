package net

import (
	"testing"

	"k3c/engine/sim"
)

// B-563/AC-01: Die Aktionsliste nennt `lair`, solange ein lebender Spieler am Bau des wartenden Endbosses ist; fern,
// gefallen oder nach dem Sieg fehlt der Eintrag.
func TestAktionslisteLair(t *testing.T) {
	isl, err := sim.CreateIsland("endboss", []int{4}, 1, sim.CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	p := sim.AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	lairX := -1.0
	for _, e := range w.Level.Entities {
		if e.Kind == "lair" {
			lairX = e.X
		}
	}
	if lairX < 0 {
		t.Fatal("Kristallhöhle ohne Bau")
	}
	hasLair := func() bool {
		for _, a := range actionsOf(w, p) {
			if a["action"] == "lair" {
				return true
			}
		}
		return false
	}
	p.X = lairX - 20
	if !hasLair() {
		t.Error("am Bau: Eintrag fehlt")
	}
	p.X = lairX - 200
	if hasLair() {
		t.Error("fern vom Bau: Eintrag vorhanden")
	}
	p.X, p.RespawnIn = lairX, 4
	if hasLair() {
		t.Error("gefallen: Eintrag vorhanden")
	}
	p.RespawnIn, isl.EndbossDefeated = 0, true
	if hasLair() {
		t.Error("Endboss besiegt: Eintrag vorhanden")
	}
}
