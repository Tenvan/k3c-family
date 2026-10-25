package sim

import (
	"math"
	"testing"
)

// W3.1 (B-116/AC-01, Sprint W3 AC-01): Die gebaute Taverne liefert bei jedem dawn Landstreicher bis zum Maximum
// (Q30), Werte aus data/buildings.json › tavern.vagrants.

// toDawn rechnet bis zum nächsten Tagesanbruch (Beginn des Morgengrauens).
func toDawn(t *testing.T, w *World) {
	t.Helper()
	phases, length := cyclePhases(globalDayNight)
	now := w.Time * w.CycleSpeed
	target := math.Floor(now/length)*length + phases[3].start
	if target <= now+2*dt*w.CycleSpeed {
		target += length
	}
	w.Time = target/w.CycleSpeed - 2*dt
	w.Cycle = cycleAt(globalDayNight, w.Time*w.CycleSpeed)
	for range 10 {
		Step(w, nil, dt)
		for _, e := range w.Events {
			if e["type"] == "dawn" {
				return
			}
		}
	}
	t.Fatal("kein dawn")
}

func TestTaverneLandstreicherJeDawnBisMax(t *testing.T) {
	w := quietWorld(t)
	AddPlayer(w)
	AddPlayer(w)
	v := buildings["tavern"].Vagrants
	tavern, _ := hubSiteOf(t, w, "tavern")
	toDawn(t, w)
	if n := vagrantsAt(w, tavern.X); n != 0 {
		t.Fatalf("ungebaut: %d Landstreicher an der Taverne", n)
	}
	built(tavern)
	for day := 1; day <= v.Max+1; day++ {
		toDawn(t, w)
		if want := min(day*v.PerDawn, v.Max); vagrantsAt(w, tavern.X) != want {
			t.Fatalf("Tag %d: %d Landstreicher, erwartet %d", day, vagrantsAt(w, tavern.X), want)
		}
	}
	for _, tr := range w.Troops {
		if tr.AnchorX == tavern.X && math.Abs(tr.X-tavern.X) > v.WanderUnits {
			t.Errorf("Landstreicher bei %v, außerhalb von %v ± %v", tr.X, tavern.X, v.WanderUnits)
		}
	}
	destroySite(w, tavern)
	w.Troops = []*Troop{}
	toDawn(t, w)
	if n := vagrantsAt(w, tavern.X); n != 0 {
		t.Errorf("zerstört: %d Landstreicher an der Taverne", n)
	}
}
