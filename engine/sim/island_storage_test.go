package sim

import (
	"encoding/json"
	"strings"
	"testing"
)

// Tests SP13.2: fünf Materialien, Lager-Maximum (300 je Hub + 300 je Lager) und Lager-Bauplatz nur in Inseln.

func storageSite(w *World) *Site {
	for _, s := range w.Sites {
		if s.Kind == "storage" {
			return s
		}
	}
	return nil
}

func TestIslandStorageCapacity(t *testing.T) {
	for stages := 1; stages <= 3; stages++ {
		isl := mustIsland(t, "lager-a", []int{0, 1, 2}[:stages])
		for built := 0; built <= min(2, stages); built++ {
			for i, w := range isl.Stages {
				storageSite(w).State = map[bool]string{true: "built", false: "unpaid"}[i < built]
			}
			got, ok := capacity(isl.Stages[0])
			if want := 300*stages + 300*built; !ok || got != want {
				t.Errorf("%d Stufen, %d Lager: Kapazität %d, erwartet %d", stages, built, got, want)
			}
		}
	}
}

func TestIslandStorageAddCapped(t *testing.T) {
	isl := mustIsland(t, "lager-b", []int{0, 1})
	w := isl.Stages[0]
	if taken, _ := addStockCapped(w, "wood", 250); taken != 250 {
		t.Fatalf("aufgenommen %d, erwartet 250", taken)
	}
	if taken, _ := addStockCapped(isl.Stages[1], "wood", 500); taken != 350 || isl.Stock.Wood != 600 {
		t.Fatalf("aufgenommen %d, Holz %d, erwartet 350 und 600", taken, isl.Stock.Wood)
	}
	if taken, _ := addStockCapped(w, "wood", 5); taken != 0 {
		t.Fatalf("voller Vorrat nahm %d auf", taken)
	}
	// Zerstörung: Überschuss bleibt, aufgenommen wird erst wieder unter dem Maximum.
	storageSite(w).State = "built"
	addStock(w, "wood", 100)
	destroySite(w, storageSite(w))
	if isl.Stock.Wood != 700 {
		t.Fatalf("Überschuss gelöscht: Holz %d", isl.Stock.Wood)
	}
	if taken, _ := addStockCapped(w, "wood", 5); taken != 0 {
		t.Fatalf("über Kapazität nahm %d auf", taken)
	}
	isl.Stock.Wood -= 150
	if taken, _ := addStockCapped(w, "wood", 100); taken != 50 {
		t.Fatalf("aufgenommen %d, erwartet 50", taken)
	}
	if _, ok := addStockCapped(w, "gold", 1); ok {
		t.Fatal("gold ist kein Baumaterial")
	}
}

func TestIslandStorageFiveMaterials(t *testing.T) {
	isl := mustIsland(t, "lager-c", []int{0})
	w := isl.Stages[0]
	for _, r := range []string{"wood", "stone", "copper", "iron", "crystal"} {
		addStock(w, r, 10)
	}
	cost := Cost{Wood: 1, Stone: 2, Copper: 3, Iron: 4, Crystal: 5}
	if !canAfford(*w.Stock, cost) {
		t.Fatal("alle fünf sollten reichen")
	}
	spend(w.Stock, cost)
	if want := (Stock{Wood: 9, Stone: 8, Copper: 7, Iron: 6, Crystal: 5}); *w.Stock != want {
		t.Fatalf("Vorrat %+v", *w.Stock)
	}
	if canAfford(*w.Stock, Cost{Crystal: 6}) {
		t.Fatal("Kristall 6 darf nicht reichen")
	}
	// Spielstand: Rundlauf mit fünf Materialien.
	raw, err := json.Marshal(isl.ToSave("t"))
	if err != nil {
		t.Fatal(err)
	}
	s, err := ParseIslandSave(raw)
	if err != nil {
		t.Fatal(err)
	}
	back, err := FromIslandSave(s, islandSpeed)
	if err != nil || *back.Stock != *isl.Stock {
		t.Fatalf("Rundlauf: %v, %+v", err, back.Stock)
	}
	// Alter Stand ohne Eisen und Kristall lädt unverändert.
	old := strings.Replace(string(raw), `,"iron":6,"crystal":5`, "", 1)
	if old == string(raw) {
		t.Fatal("Eisen/Kristall nicht im Stand")
	}
	s, err = ParseIslandSave([]byte(old))
	if err != nil || s.Stock != (Stock{Wood: 9, Stone: 8, Copper: 7}) {
		t.Fatalf("alter Stand: %v, %+v", err, s.Stock)
	}
}

func TestIslandStorageSiteOnlyInIslands(t *testing.T) {
	isl := mustIsland(t, "lager-d", []int{0, 1})
	for _, w := range isl.Stages {
		if storageSite(w) == nil {
			t.Fatal("Insel-Stufe ohne Lager-Bauplatz")
		}
	}
	camp, err := CreateWorld(biomeForDepth(0), "lager-d", Options{CycleSpeed: islandSpeed})
	if err != nil {
		t.Fatal(err)
	}
	if storageSite(camp) != nil {
		t.Fatal("Campaign-Welt hat einen Lager-Bauplatz")
	}
	if _, ok := capacity(camp); ok {
		t.Fatal("Campaign-Welt hat ein Maximum")
	}
	if !strings.Contains(string(mustJSON(t, camp.Stock)), `"copper":0}`) {
		t.Fatal("Eisen/Kristall müssen ohne Wert wegfallen (omitempty)")
	}
}

func mustJSON(t *testing.T, v any) []byte {
	t.Helper()
	b, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	return b
}

func TestIslandStorageDeterministic(t *testing.T) {
	run := func() string {
		isl := mustIsland(t, "lager-e", []int{0, 1})
		AddIslandPlayer(isl, 0)
		addStock(isl.Stages[0], "iron", 400)
		storageSite(isl.Stages[1]).State = "built"
		runIsland(isl, []PlayerCommand{{}}, 20)
		return strings.Join(islandJSON(t, isl), "|")
	}
	a, b := run(), run()
	if a != b {
		t.Fatal("Insel mit Lager ist nicht deterministisch")
	}
}
