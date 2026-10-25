package sim

import (
	"math"
	"testing"

	"k3c/engine/level"
)

// W12.1 (B-470/AC-01, AC-03): vier Phasen Tag, Dämmerung, Nacht, Morgengrauen (Q65); das Ereignis dawn fällt bei
// Beginn des Morgengrauens, der Übergang zum Tag löst nichts aus.

// forestCycle ist der Wald nach Q65 mit 6/2/4/2 min, unabhängig von späteren Werten in data/.
var forestCycle = level.Cycle{Type: "dayNight", DayMinutes: 6, TwilightMinutes: 2, NightMinutes: 4, DawnMinutes: 2}

func TestCycleAtVierPhasenGrenzenUndMitte(t *testing.T) {
	for _, c := range []struct {
		seconds  float64
		phase    string
		day      int
		progress float64
		left     float64
	}{
		{0, "day", 1, 0, 360}, {180, "day", 1, 0.5, 180},
		{360, "dusk", 1, 0, 120}, {420, "dusk", 1, 0.5, 60},
		{480, "night", 1, 0, 240}, {600, "night", 1, 0.5, 120},
		{720, "dawn", 2, 0, 120}, {780, "dawn", 2, 0.5, 60},
		{840, "day", 2, 0, 360}, {840 + 720, "dawn", 3, 0, 120},
	} {
		got := cycleAt(forestCycle, c.seconds)
		if got.Phase != c.phase || got.Day != c.day || math.Abs(got.Progress-c.progress) > 1e-9 ||
			math.Abs(got.SecondsLeft-c.left) > 1e-9 {
			t.Errorf("%v s: %+v, erwartet %s Tag %d Fortschritt %v Rest %v", c.seconds, got, c.phase, c.day, c.progress, c.left)
		}
	}
	if got := cycleAt(forestCycle, 719.999); got.Phase != "night" || got.Day != 1 {
		t.Errorf("kurz vor dem Morgengrauen: %+v", got)
	}
}

// Über zwei Zyklen: dawn genau einmal je Zyklus, im ersten Schritt des Morgengrauens, samt Gold je lebendem Spieler.
func TestDawnBeiBeginnDesMorgengrauens(t *testing.T) {
	w := newWorld(0, "dawn", Options{CycleSpeed: 60})
	w.Troops, w.Camps, w.Pickups = []*Troop{}, []*Camp{}, []*Pickup{}
	AddPlayer(w)
	AddPlayer(w)
	_, length := cyclePhases(globalDayNight)
	var dawns, nights []int
	for w.Time*w.CycleSpeed < 2*length+1 {
		gold := totalGold(w)
		before := w.Cycle
		Step(w, nil, dt)
		for _, e := range w.Events {
			switch e["type"] {
			case "night":
				nights = append(nights, w.Cycle.Day)
			case "dawn":
				dawns = append(dawns, w.Cycle.Day)
				if before.Phase != "night" || w.Cycle.Phase != "dawn" || e["day"] != w.Cycle.Day {
					t.Fatalf("dawn bei %+v → %+v, Ereignis %v", before, w.Cycle, e)
				}
				if got := totalGold(w) - gold; got != 2*economy.DawnGoldPerPlayer {
					t.Errorf("Tag %d: %d Gold, erwartet %d", w.Cycle.Day, got, 2*economy.DawnGoldPerPlayer)
				}
			}
		}
	}
	if len(dawns) != 2 || dawns[0] != 2 || dawns[1] != 3 || len(nights) != 2 {
		t.Fatalf("dawn an Tagen %v, Nächte %v, erwartet je zwei (dawn Tag 2 und 3)", dawns, nights)
	}
	if w.Cycle.Phase != "day" || w.Cycle.Day != 3 {
		t.Fatalf("nach zwei Zyklen %+v, erwartet Tag 3", w.Cycle)
	}
}

// totalGold ist das Gold der Spieler plus die Münzen am Boden.
func totalGold(w *World) int {
	n := len(w.Coins)
	for _, p := range w.Players {
		n += p.Gold
	}
	return n
}
