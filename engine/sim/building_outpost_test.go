package sim

import (
	"math"
	"testing"

	"k3c/engine/level"
)

// Vorposten (WZ14.4, B-468/AC-03, docs/rules/geplante-chunks.md § 8): Plätze jenseits des äußersten Tors laut Layout,
// mit 2 Spielern über fixtureIsland (alle drei Ruinen der Fixture liegen dort).

// edgeWalls setzt die äußersten Mauer-Plätze der Fixture auf Hub ± 124 (alte Linie 5): Die Ruinen der Fixture liegen dahinter,
// die Plätze der Werkstatt und des Lagers innerhalb (die verteilten Zonen reichen sonst bis ±290, B-652).
func edgeWalls(w *World) {
	w.Level.Sites = []level.WallSite{{Side: -1, Kind: "wall", X: w.HubX - 124, Zone: 5}, {Side: 1, Kind: "wall", X: w.HubX + 124, Zone: 5}}
}

// outpostPeasant ist der Bauer der Stufe, tagsüber ohne Gegner.
func outpostPeasant(t *testing.T, w *World) *Troop {
	t.Helper()
	for _, tr := range w.Troops {
		if tr.Kind == "peasant" && !isDangerous(w) {
			return tr
		}
	}
	t.Fatal("Ausgangslage: Bauer fehlt oder Gefahr")
	return nil
}

// endanger setzt einen betäubten Gegner an den Rand der Stufe: Gefahr, ohne dass jemand angegriffen wird.
func endanger(w *World) {
	spawnEnemy(w, "goblin", w.WidthUnits-1).Stun = math.Inf(1)
}

func TestVorpostenLageLautLayout(t *testing.T) {
	_, w := fixtureIsland(t)
	edgeWalls(w)
	left, _ := outermostGate(w, -1)
	right, _ := outermostGate(w, 1)
	for _, s := range w.Sites {
		want := s.X < left || s.X > right
		if outpost(w, s.X) != want {
			t.Errorf("%s@%v: Vorposten %v, äußerste Mauern %v/%v", s.Kind, s.X, outpost(w, s.X), left, right)
		}
		if s.Ruin && !outpost(w, s.X) {
			t.Errorf("Ruine %s@%v liegt nicht jenseits der äußersten Mauer", s.Kind, s.X)
		}
	}
	w.Level.Sites = nil
	if outpost(w, 0) || outpost(w, w.WidthUnits) {
		t.Error("ohne Mauer-Plätze gibt es keine Vorposten")
	}
}

// Reparatur am Vorposten tagsüber; bei Gefahr bleibt sie liegen und der Bauer kehrt zur Burg zurück.
func TestVorpostenReparaturNurOhneGefahr(t *testing.T) {
	_, w := fixtureIsland(t)
	edgeWalls(w)
	p := outpostPeasant(t, w)
	p.Job = repairJob(w, p)
	if p.Job == nil {
		t.Fatal("kein Reparatur-Auftrag")
	}
	s := siteByID(w, p.Job.SiteID)
	if !s.Ruin || !outpost(w, s.X) {
		t.Fatalf("Reparatur-Auftrag %+v, erwartet eine Ruine am Vorposten", p.Job)
	}
	for range 30 {
		stepPeasant(w, p, dt)
	}
	if p.Job == nil || p.Job.Type != "repair" {
		t.Fatalf("tagsüber Reparatur liegen gelassen: %+v", p.Job)
	}
	endanger(w)
	start := math.Abs(p.X - w.HubX)
	for range 30 {
		stepPeasant(w, p, dt)
	}
	if p.Job != nil || s.WorkerID != nil {
		t.Fatalf("bei Gefahr Auftrag %+v, Bauer am Platz %v", p.Job, s.WorkerID)
	}
	if math.Abs(p.X-w.HubX) >= start {
		t.Errorf("Bauer geht nicht zur Burg: %v → %v", start, math.Abs(p.X-w.HubX))
	}
}

// Bau am Vorposten nur ohne Gefahr; ein Bauplatz innerhalb der Mauern wird auch bei Gefahr gebaut.
func TestVorpostenBauNurOhneGefahr(t *testing.T) {
	_, w := fixtureIsland(t)
	edgeWalls(w)
	p := outpostPeasant(t, w)
	ruin := ruinOf(t, w, "storage")
	destroySite(w, ruin)
	ruin.State = "waitingWorker"
	if j := buildJob(w, p); j == nil || j.SiteID != ruin.ID {
		t.Fatalf("tagsüber Bau-Auftrag %+v, erwartet die Ruine %d", j, ruin.ID)
	}
	ruin.WorkerID = nil
	endanger(w)
	if j := buildJob(w, p); j != nil {
		t.Fatalf("bei Gefahr Bau am Vorposten angenommen: %+v", j)
	}
	inner := siteAt(w, "storage", 524)
	inner.State = "waitingWorker"
	if j := buildJob(w, p); j == nil || j.SiteID != inner.ID {
		t.Fatalf("bei Gefahr Bau innerhalb der Mauern: %+v, erwartet %d", j, inner.ID)
	}
	if leaveOnDanger(w, &Job{Type: "build", SiteID: inner.ID}) {
		t.Error("Neubau innerhalb der Mauern bleibt bei Gefahr liegen")
	}
	p.Job, ruin.WorkerID, inner.WorkerID = &Job{Type: "build", SiteID: ruin.ID}, intPtr(p.ID), nil
	stepPeasant(w, p, dt)
	if p.Job != nil && p.Job.SiteID == ruin.ID {
		t.Errorf("laufender Bau am Vorposten bei Gefahr nicht abgebrochen: %+v", p.Job)
	}
}

// Waffen holen: an einer Werkstatt jenseits des Tors nur ohne Gefahr, innerhalb der Mauern auch bei Gefahr.
func TestVorpostenWaffenNurOhneGefahr(t *testing.T) {
	_, w := fixtureIsland(t)
	edgeWalls(w)
	p := outpostPeasant(t, w)
	right, _ := outermostGate(w, 1)
	far := emptySite(w, "workshop", right+20)
	far.State, far.Bows = "built", 1
	w.Sites = append(w.Sites, far)
	if j := weaponToFetch(w, p); j == nil || j.Type != "fetchBow" || j.SiteID != far.ID {
		t.Fatalf("tagsüber Bogen holen: %+v, erwartet Werkstatt %d", j, far.ID)
	}
	endanger(w)
	if j := weaponToFetch(w, p); j != nil {
		t.Fatalf("bei Gefahr Waffe am Vorposten geholt: %+v", j)
	}
	if !leaveOnDanger(w, &Job{Type: "fetchSword", SiteID: far.ID}) {
		t.Error("Schwert holen am Vorposten bleibt bei Gefahr nicht liegen")
	}
	inner := siteAt(w, "workshop", 534)
	inner.State, inner.Swords = "built", 1
	if j := weaponToFetch(w, p); j == nil || j.Type != "fetchSword" || j.SiteID != inner.ID {
		t.Fatalf("bei Gefahr Schwert innerhalb der Mauern: %+v, erwartet %d", j, inner.ID)
	}
}

// Dev-Funktion Burgfall: Stufenverlust auch ohne gefallene Burg, die zerstörte Ruine steht wieder.
func TestDevCastleFallStelltRuineWiederHer(t *testing.T) {
	isl, w := fixtureIsland(t)
	s := ruinOf(t, w, "smithy")
	id := s.ID
	destroySite(w, s)
	w.Players[0].Gold, w.Players[1].Gold = 10, 7
	if !DevCastleFall(isl, 1) {
		t.Fatal("Spieler 1 nicht gefunden")
	}
	if s.ID != id || !s.Ruin || s.State != "built" || s.HP != ruinStartHP("smithy") {
		t.Errorf("Ruine nach Dev-Burgfall: %+v", s)
	}
	if w.Players[0].Gold != 5 || w.Players[1].Gold != 3 {
		t.Errorf("Gold nach Dev-Burgfall %d/%d, erwartet 5/3", w.Players[0].Gold, w.Players[1].Gold)
	}
	for _, tr := range w.Troops {
		if tr.Kind != "vagrant" {
			t.Errorf("Truppe %s überlebt den Burgfall", tr.Kind)
		}
	}
	if DevCastleFall(isl, 9) {
		t.Error("unbekannter Spieler akzeptiert")
	}
}
