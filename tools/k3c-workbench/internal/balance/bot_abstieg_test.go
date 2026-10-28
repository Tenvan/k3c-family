package balance

import "testing"

// BS11.2/AC-03 (B-642/AC-03): Der Bot steigt erst ab, wenn der Miniboss seiner Stufe besiegt ist, und nur mit ungeradem
// Index, wenn mehrere Monarchen spielen. Den ganzen Abstieg bis zur tiefsten Stufe sichert der Stufen-Test
// `task balance:fullgame STUFE=2` (Build-Tag fullgame, nicht in check:diff).
func TestBotsSteigenErstNachDemMiniboss(t *testing.T) {
	isl, err := newIsland(Scenario{Seed: "k3c-dev", Players: 2, Days: 1, World: "standard"})
	if err != nil {
		t.Fatal(err)
	}
	if id := miniBossID(0); id != "goblinLeader" {
		t.Fatalf("Miniboss des Waldes %q", id)
	}
	if miniDefeated(isl, 0) {
		t.Error("Miniboss noch nicht besiegt, Abstieg trotzdem frei")
	}
	isl.DefeatedBosses = append(isl.DefeatedBosses, "goblinLeader")
	if !miniDefeated(isl, 0) {
		t.Error("Miniboss besiegt, Abstieg gesperrt")
	}
	p0, p1 := isl.Stages[0].Players[0], isl.Stages[0].Players[1]
	w := isl.Stages[0]
	if mayDescend(isl, w, p0) || !mayDescend(isl, w, p1) {
		t.Errorf("bei zwei Monarchen steigt zuerst nur der mit ungeradem Index ab: %d %d", p0.Index, p1.Index)
	}
	if miniBossID(99) != "" || !miniDefeated(isl, 99) {
		t.Error("Stufe ohne Miniboss muss frei sein")
	}
}

func TestOhneWorldKeinAbstieg(t *testing.T) {
	r := mustRun(t, Matrix{Seeds: []string{"k3c-dev"}, Players: []int{2}, Bots: []string{"coop2"}, Depths: []int{0}, Days: 1})
	if m := r.Results[0].Metrics; m.ReachedDepth != 0 || len(m.Stages) != 0 {
		t.Errorf("ohne World: Tiefe %d, Stufen %v", m.ReachedDepth, m.Stages)
	}
}
