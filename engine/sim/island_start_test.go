package sim

import "testing"

// Tests H1 (B-177): Startvorrat Holz einer neuen Insel.

func TestIslandStartStock(t *testing.T) {
	a := newIsland(t, "start-a", []int{0, 1, 2})
	if *a.Stock != (Stock{Wood: 100}) {
		t.Fatalf("Startvorrat %+v, erwartet nur 100 Holz", *a.Stock)
	}
	b := newIsland(t, "start-a", []int{0, 1, 2})
	if *a.Stock != *b.Stock {
		t.Fatalf("gleicher Seed, anderer Vorrat: %+v und %+v", *a.Stock, *b.Stock)
	}
}

func TestIslandStartStockNotAddedOnLoad(t *testing.T) {
	isl := newIsland(t, "start-b", []int{0, 1})
	*isl.Stock = Stock{Wood: 7, Stone: 3}
	loaded, err := FromIslandSave(isl.ToSave("2026-10-03T00:00:00.000Z"), islandSpeed)
	if err != nil {
		t.Fatal(err)
	}
	if *loaded.Stock != (Stock{Wood: 7, Stone: 3}) {
		t.Fatalf("geladener Vorrat %+v, erwartet Holz 7 und Stein 3 ohne Startvorrat", *loaded.Stock)
	}
}

func TestIslandStartStockWithinStorageLimit(t *testing.T) {
	s := hub.IslandStartStock
	for name, v := range map[string]int{"wood": s.Wood, "stone": s.Stone, "copper": s.Copper, "iron": s.Iron, "crystal": s.Crystal} {
		if v < 0 || v > economy.Storage.BasePerHub {
			t.Errorf("islandStartStock.%s = %d, erlaubt 0 bis %d (Lager-Maximum je Hub)", name, v, economy.Storage.BasePerHub)
		}
	}
}

// Beide Mauern und ein Turm sind ohne vorheriges Holzfällen bezahlbar: Gold ist gezahlt, das Material liegt im Vorrat.
func TestIslandStartBuildsWallsAndTowerWithoutChopping(t *testing.T) {
	isl := newIsland(t, "start-c", []int{0, 1, 2})
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	var want []*Site
	walls, towers := 0, 0
	for _, s := range w.Sites {
		switch {
		case s.Kind == "wall" && walls < 2:
			walls++
			want = append(want, s)
		case s.Kind == "tower" && towers < 1:
			towers++
			want = append(want, s)
		}
	}
	if len(want) != 3 {
		t.Fatalf("Bauplätze gefunden: %d, erwartet 2 Mauern und 1 Turm", len(want))
	}
	cost := 0
	for _, s := range want {
		s.PaidGold, s.State = buildings[s.Kind].Cost.Gold, "waitingMaterial"
		cost += buildings[s.Kind].Cost.Wood
	}
	StepIsland(isl, make([]PlayerCommand, 2), 1.0/30)
	for _, s := range want {
		if s.State != "waitingWorker" {
			t.Errorf("%s: Zustand %s, erwartet waitingWorker (Material bezahlt)", s.Kind, s.State)
		}
	}
	if isl.Stock.Wood != 100-cost {
		t.Errorf("Holz %d, erwartet %d", isl.Stock.Wood, 100-cost)
	}
}
