package sim

import "testing"

// B-217/AC-01: `built` trägt das x des Bauplatzes.
func TestEventBuiltMitOrt(t *testing.T) {
	w := quietWorld(t)
	wall := leftWall(t, w)
	wall.State, wall.HP, wall.BuildProgress = "waitingWorker", 0, 0
	w.Troops = []*Troop{{ID: 960, Kind: "peasant", X: wall.X, AnchorX: wall.X, HP: 1, MaxHP: 1, Job: &Job{Type: "build", SiteID: wall.ID}}}
	wall.WorkerID = intPtr(960)
	built := eventsOf(w, "built", 30*60, nil)
	if len(built) == 0 {
		t.Fatal("kein built")
	}
	if ev := built[0]; ev["kind"] != "wall" || ev["x"] != unitX(wall.X) {
		t.Fatalf("built: %v, Mauer bei %v", ev, unitX(wall.X))
	}
}

// B-217/AC-01: `crownFallen` trägt das x des Spielers.
func TestEventPlayerDownMitOrt(t *testing.T) {
	w := quietWorld(t)
	AddPlayer(w)
	p1 := AddPlayer(w)
	p1.X = w.HubX + 7.25
	kill(w, p1)
	down := firstEvent(t, eventsOfType(w, "crownFallen"))
	if down["player"] != 1 || down["x"] != unitX(p1.X) {
		t.Fatalf("crownFallen: %v", down)
	}
}

// B-260/AC-01: Der Schutzplatz hängt am Index der Truppe, nicht an ihrer Entity-ID. Eine zusätzliche Entity vor den
// Truppen (z. B. ein neuer Bauplatz) verschiebt IDs, aber keine Schutzplätze.
func TestSchutzplatzOhneEntityID(t *testing.T) {
	w := quietWorld(t)
	w.Troops = nil
	var before []float64
	for range 6 {
		p := spawnVagrant(w, w.HubX, w.HubX)
		p.Kind = "peasant"
	}
	for _, tr := range w.Troops {
		before = append(before, shelterX(w, tr))
	}
	for _, tr := range w.Troops {
		tr.ID += 3 // wie drei zusätzliche Bauplätze vor den Truppen
	}
	seen := map[float64]bool{}
	for i, tr := range w.Troops {
		if got := shelterX(w, tr); got != before[i] {
			t.Fatalf("Truppe %d: Schutzplatz %v statt %v nach ID-Verschiebung", i, got, before[i])
		}
		seen[before[i]] = true
	}
	if len(seen) != 5 {
		t.Fatalf("Schutzplätze nicht über 5 Plätze gestreut: %v", before)
	}
}
