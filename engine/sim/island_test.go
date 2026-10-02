package sim

import (
	"encoding/json"
	"testing"
)

// Insel-Tests (SP12.1): mehrere Stufen ticken gemeinsam, ein Vorrat je Insel, deterministisch.

const islandSpeed = 60 // schneller Zyklus: Tag, Dämmerung und Nacht in Sekunden statt Minuten

func mustIsland(t *testing.T, seed string, depths []int) *Island {
	t.Helper()
	isl, err := CreateIsland(seed, depths, islandSpeed)
	if err != nil {
		t.Fatal(err)
	}
	return isl
}

// runIsland tickt die Insel `seconds` lang und liefert alle Ereignisse je Stufe.
func runIsland(isl *Island, cmds []PlayerCommand, seconds float64) [][]Event {
	events := make([][]Event, len(isl.Stages))
	for tm := 0.0; tm < seconds; tm += dt {
		StepIsland(isl, cmds, dt)
		for i, w := range isl.Stages {
			events[i] = append(events[i], w.Events...)
		}
	}
	return events
}

func count(events []Event, typ string) int {
	n := 0
	for _, e := range events {
		if e["type"] == typ {
			n++
		}
	}
	return n
}

func islandJSON(t *testing.T, isl *Island) []string {
	t.Helper()
	out := make([]string, len(isl.Stages))
	for i, w := range isl.Stages {
		b, err := json.Marshal(w)
		if err != nil {
			t.Fatal(err)
		}
		out[i] = string(b)
	}
	return out
}

func TestIslandStagesTickTogether(t *testing.T) {
	isl := mustIsland(t, "insel-a", []int{0, 1})
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 1)
	events := runIsland(isl, []PlayerCommand{{}, {}}, 30)

	if isl.Stages[0].Time != isl.Stages[1].Time || isl.Stages[0].Time < 29 {
		t.Fatalf("Zeit der Stufen weicht ab: %v und %v", isl.Stages[0].Time, isl.Stages[1].Time)
	}
	if count(events[0], "night") == 0 || count(events[0], "wave") == 0 {
		t.Errorf("Wald: Nacht und Welle erwartet, Ereignisse: night=%d wave=%d", count(events[0], "night"), count(events[0], "wave"))
	}
	if isl.Stages[0].Aggression != nil {
		t.Error("Wald hat keinen Aggressionspool")
	}
	if isl.Stages[1].Aggression == nil || (*isl.Stages[1].Aggression == 0 && count(events[1], "wave") == 0) {
		t.Error("Höhle: Aggressionspool muss laufen (Anstieg oder Welle)")
	}
	if len(isl.Stages[0].Players) != 1 || len(isl.Stages[1].Players) != 1 {
		t.Errorf("je Stufe ein Spieler erwartet: %d und %d", len(isl.Stages[0].Players), len(isl.Stages[1].Players))
	}
}

func TestIslandStageWithoutPlayerKeepsRunning(t *testing.T) {
	isl := mustIsland(t, "insel-b", []int{0, 1})
	AddIslandPlayer(isl, 0)
	events := runIsland(isl, []PlayerCommand{{}}, 20)
	if isl.Stages[1].Time < 19 {
		t.Fatalf("Stufe ohne Spieler steht still: %v", isl.Stages[1].Time)
	}
	if count(events[1], "wave")+int(*isl.Stages[1].Aggression) == 0 {
		t.Error("Stufe ohne Spieler: Aggressionspool muss weiterlaufen")
	}
}

func TestIslandSharedStock(t *testing.T) {
	isl := mustIsland(t, "insel-c", []int{0, 1, 2})
	for i := range isl.Stages {
		if isl.Stages[i].Stock != isl.Stock {
			t.Fatalf("Stufe %d hat einen eigenen Vorrat", i)
		}
	}
	addStock(isl.Stages[0], "wood", 30)
	addStock(isl.Stages[2], "stone", 12)
	if isl.Stages[1].Stock.Wood != 30 || isl.Stages[1].Stock.Stone != 12 {
		t.Errorf("Vorrat nicht gemeinsam: %+v", *isl.Stages[1].Stock)
	}
	spend(isl.Stages[1].Stock, Cost{Wood: 10})
	if isl.Stages[0].Stock.Wood != 20 {
		t.Errorf("Ausgabe in Stufe 1 muss Stufe 0 sehen: %d", isl.Stages[0].Stock.Wood)
	}
	// Gold gehört dem Spieler, nicht der Insel.
	a, b := AddIslandPlayer(isl, 0), AddIslandPlayer(isl, 1)
	a.Gold = 7
	if b.Gold != economy.Purse.StartGold {
		t.Errorf("Gold ist nicht je Spieler: %d", b.Gold)
	}
}

func TestIslandPlayerIndexIsUnique(t *testing.T) {
	isl := mustIsland(t, "insel-d", []int{0, 1, 2})
	stages := []int{0, 1, 0, 2}
	seen := map[int]bool{}
	for i, st := range stages {
		p := AddIslandPlayer(isl, st)
		if p.Index != i || seen[p.Index] {
			t.Fatalf("Spieler %d hat Index %d", i, p.Index)
		}
		seen[p.Index] = true
		if len(isl.Stages[st].Players) == 0 || isl.Stages[st].Players[len(isl.Stages[st].Players)-1] != p {
			t.Errorf("Spieler %d nicht in Stufe %d", i, st)
		}
	}
}

func TestIslandDeterministic(t *testing.T) {
	run := func(seed string) []string {
		isl := mustIsland(t, seed, []int{0, 1, 2})
		for _, st := range []int{0, 1, 2, 0} {
			AddIslandPlayer(isl, st)
		}
		cmds := []PlayerCommand{{MoveX: 1, Sprint: true}, {MoveX: -1}, {Pay: true}, {}}
		runIsland(isl, cmds, 40)
		return islandJSON(t, isl)
	}
	a, b := run("insel-e"), run("insel-e")
	for i := range a {
		if a[i] != b[i] {
			t.Errorf("Stufe %d: gleicher Seed und gleiche Eingaben ergeben verschiedene Zustände", i)
		}
	}
	if c := run("insel-f"); c[0] == a[0] {
		t.Error("anderer Seed muss einen anderen Zustand ergeben")
	}
}

func TestCreateIslandRejectsBadInput(t *testing.T) {
	if _, err := CreateIsland("x", nil, 1); err == nil {
		t.Error("ohne Stufen muss ein Fehler kommen")
	}
	if _, err := CreateIsland("x", []int{0, 9}, 1); err == nil {
		t.Error("unbekannte Tiefe muss ein Fehler sein")
	}
}
