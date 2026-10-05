package sim

import (
	"math"
	"testing"
)

// W2.1 (B-114/AC-03, AC-04, Sprint W2 AC-02): Plantage an beiden Farm-Plätzen, Werte aus data/buildings.json.

func farms(w *World) []*Site {
	var out []*Site
	for _, s := range w.Sites {
		if s.Kind == "farm" {
			out = append(out, s)
		}
	}
	return out
}

// plantedAt zählt die Plantage-Bäume im Streifen der Farm.
func plantedAt(w *World, farm *Site) int {
	p, n := buildings["farm"].Plantation, 0
	for _, node := range w.Nodes {
		if node.Kind == "tree" && node.Marked && math.Abs(node.X-farm.X) <= p.StripUnits {
			n++
		}
	}
	return n
}

func TestPlantageWaechstJePlatz(t *testing.T) {
	w := quietWorld(t)
	w.CycleSpeed = 1e-6
	p := buildings["farm"].Plantation
	fs := farms(w)
	if len(fs) != 2 {
		t.Fatalf("%d Farm-Plätze, erwartet 2 (je Seite)", len(fs))
	}
	level := len(w.Nodes)
	run(w, p.GrowSeconds+1)
	if len(w.Nodes) != level {
		t.Fatal("Bäume ohne gebaute Farm")
	}
	for _, f := range fs {
		built(f)
	}
	run(w, p.GrowSeconds-1)
	if plantedAt(w, fs[0]) != 0 {
		t.Fatal("Baum vor der Wachstumszeit")
	}
	run(w, 1+dt)
	for _, f := range fs {
		if got := plantedAt(w, f); got != p.Slots {
			t.Errorf("Farm %v: %d Bäume nach %v s, erwartet %d", f.X, got, p.GrowSeconds, p.Slots)
		}
	}
	run(w, 10*p.GrowSeconds)
	if got := len(w.Nodes) - level; got != 2*p.Slots {
		t.Errorf("%d Plantage-Bäume nach langer Zeit, höchstens %d", got, 2*p.Slots)
	}
}

// Plantage-Bäume sind kein Zahlziel, ein Bauer holt sie ohne Münze; der Platz wächst danach neu.
func TestPlantageOhneMarkierung(t *testing.T) {
	w := dayWorld(t)
	p := buildings["farm"].Plantation
	farm := farms(w)[0]
	built(farm)
	w.Nodes = []*ResourceNode{} // ohne Level-Bäume im Streifen
	run(w, p.GrowSeconds+dt)
	pl := AddPlayer(w)
	pl.X = plotX(farm, p, 0)
	if target := findPayTarget(w, pl); target != nil && target.node != nil {
		t.Errorf("Plantage-Baum ist Zahlziel: %+v", target.node)
	}
	gold := pl.Gold
	run(w, 60)
	if w.Stock.Wood == 0 || pl.Gold != gold {
		t.Errorf("Holz %d, Gold %d → %d", w.Stock.Wood, gold, pl.Gold)
	}
	if plantedAt(w, farm) == 0 {
		t.Error("abgebaute Plätze wachsen nicht nach")
	}
}
