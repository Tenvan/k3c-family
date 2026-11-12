package props

import (
	"math"
	"testing"

	"k3c/engine/sim"
)

// Fälle aus worldRenderer.ts › updateNode: setBar(progress, sichtbar ab progress > 0), setScale(1, 1 - progress * 0.3).
func TestFellShrinkWieImBrowser(t *testing.T) {
	for progress, want := range map[float64]float64{0: 1, 0.5: 0.85, 1: 0.7} {
		if got := fellShrink(progress); math.Abs(got-want) > 1e-9 {
			t.Errorf("Fortschritt %v: Maßstab %v, erwartet %v", progress, got, want)
		}
	}
}

// plantationWorld: Welt mit gebauter Baumschule und einem Baum auf Platz 2 (Fortschritt progress).
func plantationWorld(t *testing.T, progress float64) (*sim.World, float64) {
	t.Helper()
	w := island(t)
	pl := buildings["nursery"].Plantation
	farm := &sim.Site{ID: 9999, Kind: "nursery", State: "built", X: w.Castle.X + 5}
	w.Sites = append(w.Sites, farm)
	x := plotX(farm.X, 2, pl.Slots, pl.StripUnits)
	w.Nodes = append(w.Nodes, &sim.ResourceNode{ID: 9998, Kind: "tree", X: x, Progress: progress})
	return w, x
}

func TestPlantageBaumSchrumpftMitBalken(t *testing.T) {
	needGrafik(t)
	w, x := plantationWorld(t, 0.5)
	p := newPlanner(newFake())
	p.plan(w, nil, camera(x), 0)
	var tree *item
	for i, it := range p.body {
		if it.shape == shapeSprite && it.look == plantTreeLook && it.x == x {
			tree = &p.body[i]
		}
	}
	if tree == nil || math.Abs(tree.sy-0.85) > 1e-9 {
		t.Fatalf("niedrige Pappel mit Maßstab 0,85 erwartet: %+v", tree)
	}
	var fill *item
	for i, it := range p.top {
		if it.shape == shapeRect && it.clr == gold && it.x < x {
			fill = &p.top[i]
		}
	}
	if fill == nil || math.Abs(fill.w-px(25)) > 1e-9 {
		t.Errorf("Balken zur Hälfte gefüllt (50 px · 0,5) erwartet: %+v", fill)
	}
}

func TestPlantageBaumUngefaelltOhneBalken(t *testing.T) {
	needGrafik(t)
	w, x := plantationWorld(t, 0)
	p := newPlanner(newFake())
	p.plan(w, nil, camera(x), 0)
	for _, it := range p.top {
		if it.shape == shapeRect && it.clr == gold {
			t.Errorf("Balken ohne Fortschritt: %+v", it)
		}
	}
	for _, it := range p.body {
		if it.look == plantTreeLook && it.sy != 1 {
			t.Errorf("ungefällter Baum mit Maßstab %v", it.sy)
		}
	}
}
