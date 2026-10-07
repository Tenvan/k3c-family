package sim

import (
	"strings"
	"testing"
)

// Niederlage-Modi (B-102, K2.2b, docs/rules/stufen.md § 4).

// defeatIsland: Insel mit Wald und Höhle, je ein Spieler (Gold 40 bzw. 30), Vorrat 20 Holz / 10 Stein, im Wald ein
// gebauter Turm und ein Bogenschütze.
func defeatIsland(t *testing.T, defeat string) (*Island, *World) {
	t.Helper()
	isl := mustIsland(t, "niederlage", []int{0, 1})
	if err := SetOptions(isl, IslandOptions{Grade: "normal", Goal: "endboss", Defeat: defeat}, false); err != nil {
		t.Fatal(err)
	}
	AddIslandPlayer(isl, 0).Gold = 40
	AddIslandPlayer(isl, 1).Gold = 30
	*isl.Stock = Stock{Wood: 20, Stone: 10}
	w := isl.Stages[0]
	buildSite(t, w, "tower")
	w.Troops = append(w.Troops, &Troop{ID: w.newID(), Kind: "archer", X: w.HubX, HP: 10, MaxHP: 10})
	return isl, w
}

func builtSites(w *World) int {
	n := 0
	for _, s := range w.Sites {
		if s.State == "built" {
			n++
		}
	}
	return n
}

func vagrants(w *World) int {
	n := 0
	for _, t := range w.Troops {
		if t.Kind == "vagrant" {
			n++
		}
	}
	return n
}

func TestNiederlageGoldMaterial(t *testing.T) {
	isl, w := defeatIsland(t, "resources")
	sites, troops := builtSites(w), len(w.Troops)
	w.Castle.HP = 0
	castleFallen(w)
	if g0, g1 := isl.Stages[0].Players[0].Gold, isl.Stages[1].Players[0].Gold; g0 != 20 || g1 != 15 {
		t.Errorf("Gold %d/%d, erwartet 20/15 (alle Spieler der Insel −50 %%)", g0, g1)
	}
	if *isl.Stock != (Stock{Wood: 10, Stone: 5}) {
		t.Errorf("Vorrat %+v, erwartet −50 %%", *isl.Stock)
	}
	if builtSites(w) != sites || len(w.Troops) != troops || w.Castle.HP != w.Castle.MaxHP || isl.Over {
		t.Errorf("Bauten %d→%d, Truppen %d→%d, Burg %v, Over %v", sites, builtSites(w), troops, len(w.Troops), w.Castle.HP, isl.Over)
	}
}

func TestNiederlageStufenverlust(t *testing.T) {
	isl, w := defeatIsland(t, "stage")
	w.Castle.HP = 0
	castleFallen(w)
	if g0, g1 := isl.Stages[0].Players[0].Gold, isl.Stages[1].Players[0].Gold; g0 != 20 || g1 != 30 {
		t.Errorf("Gold %d/%d, erwartet 20/30 (nur Spieler der Stufe)", g0, g1)
	}
	if *isl.Stock != (Stock{Wood: 10, Stone: 5}) {
		t.Errorf("Vorrat %+v, erwartet −50 %%", *isl.Stock)
	}
	if builtSites(w) != 0 || len(w.Troops) != vagrants(w) || w.Castle.HP != w.Castle.MaxHP || isl.Over {
		t.Errorf("Hub nicht zurückgesetzt: %d Bauten, %d Truppen, Burg %v, Over %v", builtSites(w), len(w.Troops), w.Castle.HP, isl.Over)
	}
}

func islandState(t *testing.T, isl *Island) string {
	t.Helper()
	return strings.Join(islandJSON(t, isl), "\n")
}

func TestNiederlageKomplettVerloren(t *testing.T) {
	isl, w := defeatIsland(t, "lost")
	w.Castle.HP = 0
	StepIsland(isl, nil, dt)
	if !isl.Over || count(w.Events, "gameOver") != 1 {
		t.Fatalf("Over %v, %d× gameOver", isl.Over, count(w.Events, "gameOver"))
	}
	StepIsland(isl, nil, dt)
	before := islandState(t, isl)
	move := []PlayerCommand{{MoveX: 1}, {MoveX: 1, Pay: true}} // auch der Spieler in der Höhle bewegt sich nicht
	for range 90 {
		StepIsland(isl, move, dt)
	}
	if after := islandState(t, isl); after != before {
		t.Error("nach dem Game Over ändert sich der Zustand")
	}
	for i, s := range isl.Stages {
		if len(s.Events) != 0 {
			t.Errorf("Stufe %d: Ereignisse nach dem Game Over: %v", i, s.Events)
		}
	}
}

func TestNiederlageStandardJeGrad(t *testing.T) {
	want := map[string]string{"dev": "resources", "easy": "resources", "normal": "stage", "hard": "stage", "ultra": "lost"}
	for _, g := range gradeNames {
		if got := DefaultOptionsFor(g).Defeat; got != want[g] {
			t.Errorf("Grad %s: Standard %q, erwartet %q", g, got, want[g])
		}
	}
	isl, w := defeatIsland(t, "stage")
	o := DefaultOptionsFor("ultra")
	o.Defeat = "stage" // Überschreiben je Raum
	if err := SetOptions(isl, o, false); err != nil {
		t.Fatal(err)
	}
	w.Castle.HP = 0
	castleFallen(w)
	if isl.Over || builtSites(w) != 0 {
		t.Errorf("Ultra mit Stufenverlust: Over %v, %d Bauten", isl.Over, builtSites(w))
	}
}

func TestNiederlageOhneInselStufenverlust(t *testing.T) {
	w := quietWorld(t)
	buildSite(t, w, "tower")
	w.Troops = append(w.Troops, &Troop{ID: w.newID(), Kind: "archer", X: w.HubX, HP: 10, MaxHP: 10})
	addPlayerAt(w, 0).Gold = 40
	w.Castle.HP = 0
	castleFallen(w)
	if builtSites(w) != 0 || len(w.Troops) != 0 || w.Players[0].Gold != 20 {
		t.Errorf("ohne Insel: %d Bauten, %d Truppen, Gold %d", builtSites(w), len(w.Troops), w.Players[0].Gold)
	}
}

func TestNiederlageDeterministisch(t *testing.T) {
	var runs [2]string
	for i := range runs {
		isl, w := defeatIsland(t, "lost")
		for range 30 {
			StepIsland(isl, nil, dt)
		}
		w.Castle.HP = 0
		for range 30 {
			StepIsland(isl, nil, dt)
		}
		runs[i] = islandState(t, isl)
	}
	if runs[0] != runs[1] {
		t.Error("gleicher Seed, anderer Zustand")
	}
}
