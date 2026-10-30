package sim

import (
	"math"
	"testing"
)

// B-647 Weg A: Ist ein Monarch beim kämpfenden Endboss, marschieren die Kämpfer der Stufe zu ihm und treffen ihn; ohne
// Monarch in der Nähe bleiben sie auf ihrem Posten.
func TestTruppenBegleitenDenMonarchenZumEndboss(t *testing.T) {
	_, w, lair := endIsland(t, 1)
	troopsAt := func() (archers, warriors []*Troop) {
		for i := range 3 {
			a := &Troop{ID: w.newID(), Kind: "archer", X: w.HubX + 5, AnchorX: w.HubX + 5, HP: troops["archer"].HP, MaxHP: troops["archer"].HP}
			k := &Troop{ID: w.newID(), Kind: "warrior", X: w.HubX - 5, AnchorX: w.HubX - 5, HP: troops["warrior"].HP, MaxHP: troops["warrior"].HP}
			w.Troops = append(w.Troops, a, k)
			archers, warriors = append(archers, a), append(warriors, k)
			_ = i
		}
		return archers, warriors
	}
	archers, warriors := troopsAt()
	// Ohne Monarch am Boss: kein Begleitschutz (der Boss ist gar nicht erschienen).
	if _, ok := escortPost(w, archers[0]); ok {
		t.Fatal("Begleitschutz ohne Endboss")
	}
	p := w.Players[0]
	p.X = lair - 10 // näher als triggerUnits: der Boss erscheint und der Begleitschutz rastet ein
	endSteps(w, 2, true)
	boss := warden(w)
	if boss == nil {
		t.Fatal("Endboss nicht erschienen")
	}
	boss.HP, boss.MaxHP = 1e9, 1e9 // der Test prüft den Marsch, nicht den Sieg
	endSteps(w, 120, true)
	for _, a := range archers {
		if d := math.Abs(a.X - boss.X); d > troops["archer"].Range+1 {
			t.Errorf("Schütze %d steht %.1f vom Boss, Reichweite %.1f", a.ID, d, troops["archer"].Range)
		}
	}
	for _, k := range warriors {
		if d := math.Abs(k.X - boss.X); d > troops["warrior"].Range+2 {
			t.Errorf("Krieger %d steht %.1f vom Boss", k.ID, d)
		}
	}
	if boss.HP >= 1e9 {
		t.Error("kein Truppenschaden am Boss")
	}
	// Der Begleitschutz bleibt, auch wenn der Monarch weggeht, und endet mit dem Tod des Bosses.
	p.X = w.HubX
	endSteps(w, 1, true)
	if _, ok := escortPost(w, archers[0]); !ok {
		t.Error("Begleitschutz hörte mit dem Monarchen auf")
	}
	boss.HP = 0
	if _, ok := escortPost(w, archers[0]); ok {
		t.Error("Begleitschutz trotz totem Boss")
	}
}
