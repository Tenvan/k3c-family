package sim

import "testing"

// Einzelwechsel (SP12.2): ein Spieler wechselt allein, die anderen bleiben stehen.

func exitX(t *testing.T, w *World) float64 {
	t.Helper()
	for _, e := range w.Level.Entities {
		if e.Kind == "exit" {
			return e.X
		}
	}
	t.Fatal("Stufe hat keinen Ausgang")
	return 0
}

func builtSite(t *testing.T, w *World, kind string) *Site {
	t.Helper()
	for _, s := range w.Sites {
		if s.Kind == kind {
			s.State = "built"
			return s
		}
	}
	t.Fatalf("kein Bauplatz %s", kind)
	return nil
}

// standFor lässt die Insel `seconds` ticken (ohne Eingaben) und hält die Spieler dabei an ihrem Platz.
func standFor(isl *Island, seconds float64, hold map[*Player]float64) {
	cmds := make([]PlayerCommand, isl.nextPlayer)
	for tm := 0.0; tm < seconds; tm += dt {
		for p, x := range hold {
			p.X, p.VX = x, 0
		}
		StepIsland(isl, cmds, dt)
	}
}

func TestIslandSinglePlayerTravelsViaExit(t *testing.T) {
	isl := mustIsland(t, "reise-a", []int{0, 1, 2})
	a, b, c := AddIslandPlayer(isl, 0), AddIslandPlayer(isl, 0), AddIslandPlayer(isl, 1)
	a.Gold, b.Gold = 55, 33
	bx, cx := b.X, c.X
	standFor(isl, 2.5, map[*Player]float64{a: exitX(t, isl.Stages[0])})

	if st := isl.StageOf(0); st != 1 {
		t.Fatalf("Spieler 0 muss in der Höhle (Stufe 1) sein, ist in %d", st)
	}
	if isl.StageOf(1) != 0 || isl.StageOf(2) != 1 {
		t.Errorf("andere Spieler dürfen nicht wechseln: %d, %d", isl.StageOf(1), isl.StageOf(2))
	}
	if b.X != bx || c.X != cx {
		t.Errorf("andere Spieler bleiben stehen: %v→%v, %v→%v", bx, b.X, cx, c.X)
	}
	moved := isl.Stages[1].Players[0]
	if moved.Index != 0 || moved.Gold != 55 {
		t.Errorf("Index und Gold bleiben: %+v", *moved)
	}
	if len(isl.Stages[1].Players) != 2 || isl.Stages[1].Players[1].Index != 2 {
		t.Errorf("Reihenfolge in der Zielstufe nach Index: %+v", isl.Stages[1].Players)
	}
	if got := isl.Players(); len(got) != 3 || got[0].Index != 0 || got[1].Index != 1 || got[2].Index != 2 {
		t.Errorf("Players() nach Index sortiert erwartet")
	}
}

func TestIslandTravelViaStairs(t *testing.T) {
	isl := mustIsland(t, "reise-b", []int{0, 1})
	p := AddIslandPlayer(isl, 0)
	down := builtSite(t, isl.Stages[0], "stairsDown")
	standFor(isl, 2.5, map[*Player]float64{p: down.X})
	if isl.StageOf(0) != 1 {
		t.Fatalf("Treppe runter: Spieler muss in Stufe 1 sein, ist in %d", isl.StageOf(0))
	}
	up := builtSite(t, isl.Stages[1], "stairsUp")
	standFor(isl, 2.5, map[*Player]float64{isl.Stages[1].Players[0]: up.X})
	if isl.StageOf(0) != 0 {
		t.Fatalf("Treppe hoch: Spieler muss zurück in Stufe 0 sein, ist in %d", isl.StageOf(0))
	}
}

func TestIslandTravelNeedsBuiltStairs(t *testing.T) {
	isl := mustIsland(t, "reise-c", []int{0, 1})
	p := AddIslandPlayer(isl, 0)
	var down *Site
	for _, s := range isl.Stages[0].Sites {
		if s.Kind == "stairsDown" {
			down = s
		}
	}
	standFor(isl, 3, map[*Player]float64{p: down.X})
	if isl.StageOf(0) != 0 {
		t.Error("ungebaute Treppe darf nicht reisen lassen")
	}
}

func TestIslandTravelAbortsWhenLeavingPoint(t *testing.T) {
	isl := mustIsland(t, "reise-d", []int{0, 1})
	p := AddIslandPlayer(isl, 0)
	x := exitX(t, isl.Stages[0])
	standFor(isl, 1.2, map[*Player]float64{p: x})
	standFor(isl, 0.5, map[*Player]float64{p: x - 20}) // weg vom Punkt: Fortschritt zurück auf 0
	standFor(isl, 1.2, map[*Player]float64{p: x})
	if isl.StageOf(0) != 0 {
		t.Error("nach Verlassen des Punkts muss die Zeit neu laufen")
	}
	standFor(isl, 1.2, map[*Player]float64{p: x})
	if isl.StageOf(0) != 1 {
		t.Error("nach 2 s am Stück muss der Wechsel erfolgen")
	}
}

func TestIslandDeadAndFreePlayersDoNotTravel(t *testing.T) {
	isl := mustIsland(t, "reise-e", []int{0, 1})
	dead, free := AddIslandPlayer(isl, 0), AddIslandPlayer(isl, 0)
	dead.HP, dead.RespawnIn = 0, 999
	free.Free = true
	x := exitX(t, isl.Stages[0])
	standFor(isl, 3, map[*Player]float64{dead: x, free: x})
	if isl.StageOf(0) != 0 || isl.StageOf(1) != 0 {
		t.Error("tote und freie Spieler wechseln nicht")
	}
}

func TestIslandTravelWithoutTargetStageStays(t *testing.T) {
	isl := mustIsland(t, "reise-f", []int{0, 1}) // Tiefe 2 gibt es global, aber nicht in dieser Insel
	p := AddIslandPlayer(isl, 1)
	standFor(isl, 3, map[*Player]float64{p: exitX(t, isl.Stages[1])})
	if isl.StageOf(0) != 1 {
		t.Error("ohne Zielstufe bleibt der Spieler stehen")
	}
}

func TestIslandTravelDeterministic(t *testing.T) {
	run := func() []string {
		isl := mustIsland(t, "reise-g", []int{0, 1, 2})
		a, b := AddIslandPlayer(isl, 0), AddIslandPlayer(isl, 1)
		standFor(isl, 2.5, map[*Player]float64{a: exitX(t, isl.Stages[0])})
		standFor(isl, 5, map[*Player]float64{b: exitX(t, isl.Stages[1])})
		return islandJSON(t, isl)
	}
	x, y := run(), run()
	for i := range x {
		if x[i] != y[i] {
			t.Errorf("Stufe %d: Reise ist nicht deterministisch", i)
		}
	}
}
