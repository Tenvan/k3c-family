package room

import (
	"encoding/json"
	"maps"
	"testing"

	"k3c/engine/sim"
)

// Inselwechsel im Raum (B-345, K4.2): Der Raum tauscht die Insel erst bei SwitchReady. Die Daten kennen nur eine Insel,
// deshalb liefert der Test die nächste über nextDef.

func withNextIsland(t *testing.T) {
	t.Helper()
	old := nextDef
	t.Cleanup(func() { nextDef = old })
	def := sim.IslandDef{ID: "testIsland2", Name: "Test-Insel", Depths: []int{0, 1}}
	def.DepthScaling.HP, def.DepthScaling.Damage, def.DepthScaling.Speed = 2, 2, 1
	nextDef = func(*sim.Island) (sim.IslandDef, bool) { return def, true }
}

func TestKampfInselwechselErstBeiSwitchReady(t *testing.T) {
	withNextIsland(t)
	_, r, _, _ := zweiStufen(t)
	old := r.isl
	ticks(r, 60)
	if r.isl != old {
		t.Fatal("Insel getauscht ohne SwitchReady")
	}
}

func TestKampfInselwechselTauschtMitGeraeten(t *testing.T) {
	withNextIsland(t)
	f, r, x, h := zweiStufen(t)
	old, xi, hi := r.isl, len(x.msgs), len(h.msgs)
	old.SwitchReady = true
	ticks(r, 1)
	if r.isl == old || r.isl.Number != 1 || r.isl.SwitchReady {
		t.Fatalf("Insel nicht getauscht: Nummer %d, bereit %v", r.isl.Number, r.isl.SwitchReady)
	}
	for i := range 3 { // Monarchen 0, 1 (Xbox) und 2 (Handy) stehen in der ersten Stufe
		if r.isl.StageOf(i) != 0 || r.monarchs[i].state != Taken {
			t.Fatalf("Monarch %d: Stufe %d, Zustand %v", i, r.isl.StageOf(i), r.monarchs[i].state)
		}
	}
	if x, h := r.devices["xbox"].slots, r.devices["handy"].slots; !maps.Equal(x, map[int]int{0: 0, 1: 1}) || !maps.Equal(h, map[int]int{0: 2}) {
		t.Fatalf("Slots Xbox %v, Handy %v", x, h)
	}
	// Level vor Zustand: Das Gerät beginnt die neue Insel mit einem vollen Zustand (net sendet nach Level immer snap).
	if !beginsWithLevel(x.since(xi), 0) || !beginsWithLevel(h.since(hi), 0) {
		t.Fatalf("Xbox %v, Handy %v", x.since(xi), h.since(hi))
	}
	r.saveNow()
	var s struct{ Island int }
	if err := json.Unmarshal(f.store.data["stufen"], &s); err != nil || s.Island != 1 {
		t.Fatalf("Spielstand: Insel %d, %v", s.Island, err)
	}
}

func TestKampfInselwechselLetzteInselOhneTausch(t *testing.T) {
	_, r, _, _ := zweiStufen(t)
	old := r.isl
	old.SwitchReady = true
	ticks(r, 5)
	if r.isl != old {
		t.Fatal("Insel getauscht, obwohl es keine nächste gibt")
	}
}
