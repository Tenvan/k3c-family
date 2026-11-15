package sim

import (
	"math"
	"slices"
	"testing"
)

// enemiesAfter tickt die Insel seconds lang und liefert die größte Zahl gleichzeitiger Gegner (ohne Wachen).
func enemiesAfter(isl *Island, cmds []PlayerCommand, seconds float64) int {
	most := 0
	for tm := 0.0; tm < seconds; tm += dt {
		StepIsland(isl, cmds, dt)
		n := 0
		for _, e := range isl.Stages[0].Enemies {
			if !e.Guard {
				n++
			}
		}
		most = max(most, n)
	}
	return most
}

// B-920/AC-01: Innerhalb der ersten Nacht entstehen Gegner (islandSpeed 60: Nacht nach etwa 8 s), auch ohne Dev-Aktion; eine Dev-Welle
// am ersten Tag stellt sie schon vorher, sodass der Stresstest den Zyklus nicht abwarten muss.
func TestStressFirstNightSpawnsEnemies(t *testing.T) {
	isl := mustIsland(t, "stress-nacht", []int{0})
	AddIslandPlayer(isl, 0)
	if n := enemiesAfter(isl, []PlayerCommand{{}}, 5); n != 0 {
		t.Fatalf("%d Gegner vor der Nacht", n)
	}
	if n := enemiesAfter(isl, []PlayerCommand{{}}, 25); n == 0 || isl.Stages[0].Wave == 0 {
		t.Fatalf("Gegner in der ersten Nacht %d, Welle %d", n, isl.Stages[0].Wave)
	}

	dev := mustIsland(t, "stress-dev-welle", []int{0})
	p := AddIslandPlayer(dev, 0)
	if !DevStartWave(dev, p.Index) {
		t.Fatal("Dev-Welle abgelehnt")
	}
	if n := enemiesAfter(dev, []PlayerCommand{{}}, 10); n == 0 {
		t.Fatal("keine Gegner nach der Dev-Welle am ersten Tag")
	}
}

// pureRun setzt im reinen Modus 7 Truppen und 12 Gegner an die Burg, tickt über die erste Nacht und liefert die X der
// gesetzten Gegner und die Zahl der Truppen am Ende.
func pureRun(t *testing.T) (xs []float64, troops int) {
	t.Helper()
	isl := mustIsland(t, "stress-rein", []int{0})
	p := AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	DevSetPure(isl, true)
	if _, err := DevSpawnUnits(isl, p.Index, "troop", "archer", 7); err != nil {
		t.Fatal(err)
	}
	if n, err := DevSpawnAtCastle(isl, p.Index, "enemy", "goblin", 12); err != nil || n != 12 {
		t.Fatalf("spawnCastle %d, %v", n, err)
	}
	for _, e := range w.Enemies {
		if math.Abs(e.X-w.HubX) > w.values().Hub.CastleRadiusUnits+1e-9 {
			t.Fatalf("Gegner bei %v, Burg bei %v", e.X, w.HubX)
		}
		xs = append(xs, e.X)
	}
	for range 40 * 30 { // über die erste Nacht hinaus (islandSpeed 60: Nacht nach etwa 8 s)
		StepIsland(isl, []PlayerCommand{{}}, dt)
	}
	if w.Wave != 0 || len(w.SpawnQueue) != 0 {
		t.Fatalf("Welle %d, wartend %d im reinen Modus", w.Wave, len(w.SpawnQueue))
	}
	for _, tr := range w.Troops {
		if tr.Kind == "vagrant" {
			t.Fatal("Landstreicher im reinen Modus")
		}
		troops++
	}
	return xs, troops
}

// B-921/AC-01 bis AC-03: Im reinen Stressmodus kommt nichts von selbst (keine Welle in der Nacht, keine Landstreicher);
// gesetzte Gegner stehen an der Burg, gesetzte Truppen zählen; zwei Läufe mit gleichem Seed sind gleich.
func TestPureStressMode(t *testing.T) {
	x1, t1 := pureRun(t)
	x2, t2 := pureRun(t)
	if len(x1) != 12 || t1 < 7 || t1 != t2 || !slices.Equal(x1, x2) {
		t.Fatalf("Gegner %d/%d, Truppen %d/%d", len(x1), len(x2), t1, t2)
	}
}
