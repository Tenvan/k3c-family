package sim

import "testing"

// W2.2 (B-115/AC-03, Sprint W2 AC-03): Die Insel führt fünf Stufen; Eingang und Treppen verbinden benachbarte.
func TestInselFuenfStufen(t *testing.T) {
	isl := mustIsland(t, "fuenf", []int{0, 1, 2, 3, 4})
	if len(isl.Stages) != 5 {
		t.Fatalf("%d Stufen", len(isl.Stages))
	}
	for i, w := range isl.Stages {
		wantStairs(t, w, i > 0, i < 4)
		for _, pt := range travelPoints(w) {
			if pt.toDepth != i-1 && pt.toDepth != i+1 {
				t.Errorf("Tiefe %d: Reisepunkt nach %d", i, pt.toDepth)
			}
		}
		// Oberste: Eingang und Treppe runter; tiefste: nur Treppe hoch; dazwischen alle drei.
		if n, want := len(travelPoints(w)), map[int]int{0: 2, 4: 1}[i]; (want > 0 && n != want) || (want == 0 && n != 3) {
			t.Errorf("Tiefe %d: %d Reisepunkte", i, n)
		}
	}
}

// wantStairs prüft die Treppen-Plätze und den Tiefen-Eingang einer Stufe und baut die Treppen.
func wantStairs(t *testing.T, w *World, up, down bool) {
	t.Helper()
	has, exits := map[string]bool{}, 0
	for _, s := range w.Sites {
		has[s.Kind] = true
		if s.Kind == "stairsUp" || s.Kind == "stairsDown" {
			built(s)
		}
	}
	for _, e := range w.Level.Entities {
		if e.Kind == "exit" {
			exits++
		}
	}
	if up != has["stairsUp"] || down != has["stairsDown"] || exits != 1 {
		t.Errorf("Tiefe %d: stairsUp %v, stairsDown %v, %d Eingänge", w.Biome.Depth, has["stairsUp"], has["stairsDown"], exits)
	}
}

// Ein Spieler geht über den Eingang von Tiefe 3 nach 4 und über die Treppe zurück.
func TestInselWechselTiefe3Und4(t *testing.T) {
	isl := mustIsland(t, "tief", []int{0, 1, 2, 3, 4})
	p := AddIslandPlayer(isl, 3)
	a, b := isl.Stages[3], isl.Stages[4]
	for _, e := range a.Level.Entities {
		if e.Kind == "exit" {
			p.X = e.X
		}
	}
	runIsland(isl, []PlayerCommand{{}}, hub.Travel.Seconds+0.5)
	if isl.StageOf(p.Index) != 4 {
		t.Fatalf("nicht in Tiefe 4, Stufe %d", isl.StageOf(p.Index))
	}
	var up *Site
	for _, s := range b.Sites {
		if s.Kind == "stairsUp" {
			up = s
		}
	}
	built(up)
	b.Players[0].X = up.X
	runIsland(isl, []PlayerCommand{{}}, hub.Travel.Seconds+0.5)
	if isl.StageOf(p.Index) != 3 || len(a.Players) != 1 {
		t.Fatalf("nicht zurück in Tiefe 3, Stufe %d", isl.StageOf(p.Index))
	}
}
