package sim

import "testing"

// B-139/AC-04: Auf der Insel entsteht ein Ereignis nur in seiner Stufe und trägt deren Index als `stage`.
func TestEventsInselStufe(t *testing.T) {
	isl := mustIsland(t, "ereignis-insel", []int{0, 1})
	a, b := AddIslandPlayer(isl, 0), AddIslandPlayer(isl, 1)
	a.Gold, b.Gold = 0, 0
	cave := isl.Stages[1]
	cave.Coins = append(cave.Coins, &Coin{ID: cave.newID(), X: b.X})
	StepIsland(isl, make([]PlayerCommand, 2), dt)
	var inCave Event
	for _, ev := range cave.Events {
		if ev["type"] == "coinPickup" {
			inCave = ev
		}
	}
	if inCave == nil || inCave["stage"] != 1 || inCave["player"] != b.Index {
		t.Fatalf("Ereignis in Stufe 1: %v", cave.Events)
	}
	for _, ev := range isl.Stages[0].Events {
		if ev["type"] == "coinPickup" {
			t.Fatalf("Ereignis der Höhle auch in Stufe 0: %v", ev)
		}
	}
}

// Portal (arrived) nach einem Einzelwechsel: in der Zielstufe, mit Spielerindex und `stage` der Zielstufe.
func TestEventsPortalArrived(t *testing.T) {
	isl := mustIsland(t, "ereignis-portal", []int{0, 1})
	a := AddIslandPlayer(isl, 0)
	var arrived Event
	cmds := make([]PlayerCommand, 1)
	for tm := 0.0; tm < 3 && arrived == nil; tm += dt {
		a.X, a.VX = exitX(t, isl.Stages[0]), 0
		StepIsland(isl, cmds, dt)
		for _, ev := range isl.Stages[1].Events {
			if ev["type"] == "arrived" {
				arrived = ev
			}
		}
	}
	if arrived == nil || arrived["player"] != 0 || arrived["stage"] != 1 || arrived["depth"] != 1 {
		t.Fatalf("arrived: %v", arrived)
	}
}
