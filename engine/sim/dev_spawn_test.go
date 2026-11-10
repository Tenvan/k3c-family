package sim

import (
	"math"
	"testing"
)

// B-766: Die Dev-Aktion spawn setzt Gegner und Truppen gleichmäßig um den Spieler und lehnt Ungültiges ab.

func TestDevSpawnUnitsGegner(t *testing.T) {
	isl := mustIsland(t, "dev-spawn", []int{0})
	p := AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	before := len(w.Enemies)
	n, err := DevSpawnUnits(isl, p.Index, "enemy", "wolf", 20)
	if err != nil || n != 20 {
		t.Fatalf("spawn: %d, %v", n, err)
	}
	if got := len(w.Enemies) - before; got != 20 {
		t.Fatalf("Gegner %d, erwartet 20", got)
	}
	for _, e := range w.Enemies[before:] {
		if e.Kind != "wolf" || math.Abs(e.X-p.X) > devSpawnSpan/2+1e-9 || e.HP <= 0 {
			t.Fatalf("Gegner %+v (Spieler bei %v)", e, p.X)
		}
	}
}

func TestDevSpawnUnitsTruppen(t *testing.T) {
	isl := mustIsland(t, "dev-spawn-t", []int{0})
	p := AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	before := len(w.Troops)
	if _, err := DevSpawnUnits(isl, p.Index, "troop", "archer", 5); err != nil {
		t.Fatal(err)
	}
	if got := len(w.Troops) - before; got != 5 {
		t.Fatalf("Truppen %d, erwartet 5", got)
	}
	last := w.Troops[len(w.Troops)-1]
	if last.Kind != "archer" || last.HP <= 0 || last.MaxHP != last.HP {
		t.Fatalf("Truppe %+v", last)
	}
	ids := map[int]bool{}
	for _, tr := range w.Troops {
		if ids[tr.ID] {
			t.Fatalf("doppelte ID %d", tr.ID)
		}
		ids[tr.ID] = true
	}
}

func TestDevSpawnUnitsUngueltig(t *testing.T) {
	isl := mustIsland(t, "dev-spawn-bad", []int{0})
	p := AddIslandPlayer(isl, 0)
	for _, c := range []struct {
		name       string
		idx        int
		kind, unit string
		count      int
	}{
		{"Spieler unbekannt", 7, "enemy", "wolf", 1},
		{"Art unbekannt", p.Index, "boss", "wolf", 1},
		{"Sorte unbekannt", p.Index, "enemy", "drache", 1},
		{"Truppe unbekannt", p.Index, "troop", "wolf", 1},
		{"null", p.Index, "enemy", "wolf", 0},
		{"zu viele", p.Index, "enemy", "wolf", DevMaxSpawn + 1},
	} {
		if _, err := DevSpawnUnits(isl, c.idx, c.kind, c.unit, c.count); err == nil {
			t.Errorf("%s angenommen", c.name)
		}
	}
	if len(isl.Stages[0].Enemies) != 0 {
		t.Fatalf("abgelehnte Aktionen setzten %d Gegner", len(isl.Stages[0].Enemies))
	}
}

func TestDevSpawnUnitsEinzelnAmSpieler(t *testing.T) {
	isl := mustIsland(t, "dev-spawn-1", []int{0})
	p := AddIslandPlayer(isl, 0)
	if _, err := DevSpawnUnits(isl, p.Index, "enemy", "greed", 1); err != nil {
		t.Fatal(err)
	}
	es := isl.Stages[0].Enemies
	if e := es[len(es)-1]; e.X != p.X {
		t.Fatalf("Einzelne Figur bei %v, Spieler bei %v", e.X, p.X)
	}
}
