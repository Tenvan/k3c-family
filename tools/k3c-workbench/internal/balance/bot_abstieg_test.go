package balance

import "testing"

// BS11.2/AC-03 (B-642/AC-03): Ein kurzer Insel-Lauf mit coop2 erreicht mindestens Tiefe 1 und die Bots spielen dort weiter;
// ohne World steigt kein Bot ab (alte Läufe bleiben gleich).
func TestBotsSteigenAb(t *testing.T) {
	r := mustRun(t, Matrix{Seeds: []string{"k3c-dev"}, Players: []int{2}, Bots: []string{"coop2"}, Depths: []int{0}, Days: fallbackDay, World: "standard"})
	res := r.Results[0]
	if !res.Valid {
		t.Fatal(res.Error)
	}
	m := res.Metrics
	if m.ReachedDepth < 1 || m.Stages[1].ArrivalDay == nil {
		t.Fatalf("Tiefe %d erreicht, Stufe 1: %+v", m.ReachedDepth, m.Stages[1])
	}
	if d := *m.Stages[1].ArrivalDay; d > fallbackDay {
		t.Errorf("Ankunft in der Höhle erst Tag %d", d)
	}
}

func TestOhneWorldKeinAbstieg(t *testing.T) {
	r := mustRun(t, Matrix{Seeds: []string{"k3c-dev"}, Players: []int{2}, Bots: []string{"coop2"}, Depths: []int{0}, Days: 1})
	if m := r.Results[0].Metrics; m.ReachedDepth != 0 || len(m.Stages) != 0 {
		t.Errorf("ohne World: Tiefe %d, Stufen %v", m.ReachedDepth, m.Stages)
	}
}
