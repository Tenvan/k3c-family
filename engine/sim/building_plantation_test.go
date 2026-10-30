package sim

import (
	"math"
	"testing"
)

// W2.1 (B-114/AC-03, AC-04, Sprint W2 AC-02): Plantage an beiden Baumschule-Plätzen, Werte aus data/buildings.json.

func farms(w *World) []*Site {
	var out []*Site
	for _, s := range w.Sites {
		if s.Kind == "nursery" {
			out = append(out, s)
		}
	}
	return out
}

// plantedAt zählt die Plantage-Bäume im Streifen der Baumschule.
func plantedAt(w *World, nursery *Site) int {
	p, n := buildings["nursery"].Plantation, 0
	for _, node := range w.Nodes {
		if node.Kind == "tree" && node.Marked && math.Abs(node.X-nursery.X) <= p.StripUnits {
			n++
		}
	}
	return n
}

func TestPlantageWaechstJePlatz(t *testing.T) {
	w := quietWorld(t)
	w.CycleSpeed = 1e-6
	p := buildings["nursery"].Plantation
	fs := farms(w)
	if len(fs) != 2 {
		t.Fatalf("%d Baumschule-Plätze, erwartet 2 (je Seite)", len(fs))
	}
	level := len(w.Nodes)
	run(w, p.GrowSeconds+1)
	if len(w.Nodes) != level {
		t.Fatal("Bäume ohne gebaute Baumschule")
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
			t.Errorf("Baumschule %v: %d Bäume nach %v s, erwartet %d", f.X, got, p.GrowSeconds, p.Slots)
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
	p := buildings["nursery"].Plantation
	nursery := farms(w)[0]
	built(nursery)
	w.Nodes = []*ResourceNode{} // ohne Level-Bäume im Streifen
	run(w, p.GrowSeconds+dt)
	pl := AddPlayer(w)
	pl.X = plotX(nursery, p, 0)
	if target := findPayTarget(w, pl); target != nil && target.node != nil {
		t.Errorf("Plantage-Baum ist Zahlziel: %+v", target.node)
	}
	gold := pl.Gold
	run(w, 300) // der Platz liegt weit draußen im Wald, der Bauer muss hinlaufen
	if w.Stock.Wood == 0 || pl.Gold != gold {
		t.Errorf("Holz %d, Gold %d → %d", w.Stock.Wood, gold, pl.Gold)
	}
	if plantedAt(w, nursery) == 0 {
		t.Error("abgebaute Plätze wachsen nicht nach")
	}
}

// B-636 (AC-04): Eine Baumschule ist erst nach dem Roden bezahlbar; vorher meldet der Server „erst roden“.
func TestBaumschuleErstNachDemRoden(t *testing.T) {
	w := dayWorld(t)
	w.HubLevel = 1
	nursery := farms(w)[0]
	tree := &ResourceNode{ID: w.newID(), Kind: "tree", X: nursery.X + 1}
	w.Nodes = []*ResourceNode{tree}
	if sitePayable(w, nursery) || !SiteNeedsClearing(w, nursery) {
		t.Fatalf("mit Baum im Freiraum: bezahlbar %v, erst roden %v", sitePayable(w, nursery), SiteNeedsClearing(w, nursery))
	}
	tree.gone = true
	if !sitePayable(w, nursery) || SiteNeedsClearing(w, nursery) {
		t.Fatalf("nach dem Roden: bezahlbar %v, erst roden %v", sitePayable(w, nursery), SiteNeedsClearing(w, nursery))
	}
}
