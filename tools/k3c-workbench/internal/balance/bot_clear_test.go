package balance

import (
	"testing"

	"k3c/engine/sim"
)

// BS8.1/AC-01 (B-636): Der Bot räumt den Freiraum der Baumschule, wenn kein Mauer- oder Turm-Platz mehr gesperrt ist, und
// kauft sie danach über die Bauliste.
func TestBotRodetBaumschule(t *testing.T) {
	isl, err := sim.CreateIsland("1", []int{0}, 1, sim.CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	w := isl.Stages[0]
	p := sim.AddIslandPlayer(isl, 0)
	p.Gold = 100
	w.HubLevel = 5
	var nursery *sim.Site
	for _, s := range w.Sites {
		switch s.Kind {
		case "wall", "tower":
			s.State = "built" // nur die Baumschule bleibt gesperrt
		case "nursery":
			if nursery == nil && s.State == "unpaid" && len(sim.SiteBlockers(w, s)) > 0 {
				nursery = s
			}
		}
	}
	if nursery == nil {
		t.Fatal("kein gesperrter Baumschul-Platz im Wald-Level")
	}
	if got := blockedSite(w); got == nil || got.Kind != "nursery" {
		t.Fatalf("blockedSite %v, erwartet eine Baumschule", got)
	}
	n := clearNode(w, p)
	if n == nil || !sim.SiteNeedsClearing(w, nursery) {
		t.Fatalf("clearNode %v, erst roden %v", n, sim.SiteNeedsClearing(w, nursery))
	}
}
