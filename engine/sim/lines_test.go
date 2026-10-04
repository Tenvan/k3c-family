package sim

import (
	"fmt"
	"testing"
)

// W0.3 (B-206/AC-03, AC-04, Sprint W0 AC-03, AC-06): Linien-Plätze in der Sim. Positionen kommen aus dem Layout der
// Stufe, Hub-Stufen aus data/hub.json › wallLines.

// lineSite ist der Platz kind der Linie k auf Seite side.
func lineSite(t *testing.T, w *World, kind string, side, k int) *Site {
	t.Helper()
	for _, s := range w.Sites {
		if l, ok := siteLine(w, s); ok && s.Kind == kind && l.Side == side && l.Index == k {
			return s
		}
	}
	t.Fatalf("kein Platz %s Linie %d Seite %d", kind, k, side)
	return nil
}

func built(s *Site) { s.State, s.HP = "built", s.MaxHP }

// wantPayable prüft sitePayable für kind der Linie k auf Seite side.
func wantPayable(t *testing.T, w *World, kind string, side, k int, want bool) {
	t.Helper()
	if got := sitePayable(w, lineSite(t, w, kind, side, k)); got != want {
		t.Errorf("Hub-Stufe %d: %s Linie %d Seite %+d bezahlbar = %v, erwartet %v", w.HubLevel, kind, k, side, got, want)
	}
}

// (a), (b): Linie k ab Hub-Stufe k nach gebauter Mauer k−1 derselben Seite, je Seite.
func TestLineFreischaltungJeSeite(t *testing.T) {
	w := quietWorld(t)
	if w.HubLevel != 1 {
		t.Fatalf("Start-Hub-Stufe %d, erwartet 1", w.HubLevel)
	}
	for _, kind := range []string{"wall", "tower"} {
		wantPayable(t, w, kind, -1, 1, true)
		wantPayable(t, w, kind, 1, 1, true)
		wantPayable(t, w, kind, -1, 2, false)
	}
	built(lineSite(t, w, "wall", -1, 1))
	wantPayable(t, w, "wall", -1, 2, false) // Hub-Stufe 1
	w.HubLevel = 2
	for _, kind := range []string{"wall", "tower"} {
		wantPayable(t, w, kind, -1, 2, true)
		wantPayable(t, w, kind, 1, 2, false) // rechte Linie 1 fehlt
	}
	built(lineSite(t, w, "wall", -1, 2))
	built(lineSite(t, w, "wall", 1, 1))
	built(lineSite(t, w, "wall", 1, 2))
	for _, side := range []int{-1, 1} {
		wantPayable(t, w, "wall", side, 3, false) // Hub-Stufe 3 fehlt
		wantPayable(t, w, "tower", side, 3, false)
	}
	w.HubLevel = 3
	wantPayable(t, w, "wall", -1, 3, true)
}

// Q58: Eine zerstörte Mauer k−1 lässt die gebaute Linie k gelten, nur Linie k+1 wartet auf die Reparatur.
func TestLineZerstoerteMauerSperrtNurNeueLinien(t *testing.T) {
	w := quietWorld(t)
	w.HubLevel = 3
	wall1, wall2, tower2 := lineSite(t, w, "wall", -1, 1), lineSite(t, w, "wall", -1, 2), lineSite(t, w, "tower", -1, 2)
	built(wall1)
	built(wall2)
	built(tower2)
	wantPayable(t, w, "wall", -1, 3, true)
	destroySite(w, wall1)
	if wall2.State != "built" || tower2.State != "built" {
		t.Fatalf("Linie 2 nach Zerstörung der Mauer 1: Mauer %s, Turm %s", wall2.State, tower2.State)
	}
	wantPayable(t, w, "wall", -1, 1, true) // Reparatur
	wantPayable(t, w, "wall", -1, 3, false)
	wantPayable(t, w, "tower", -1, 3, false)
	wantPayable(t, w, "gate", -1, 2, true) // Linie 2 bleibt gültig, auch ihr Tor
	built(wall1)
	wantPayable(t, w, "wall", -1, 3, true)
}

// (c): Tor ab Hub-Stufe max(gateHubLevel, k), nur an der äußersten gebauten Mauer; ein gebautes inneres Tor bleibt.
func TestLineTorNurAnDerAeusserstenLinie(t *testing.T) {
	w := quietWorld(t)
	gateLevel := hub.WallLines.GateHubLevel
	w.HubLevel = gateLevel - 1
	built(lineSite(t, w, "wall", -1, 1))
	wantPayable(t, w, "gate", -1, 1, false)
	w.HubLevel = gateLevel
	wantPayable(t, w, "gate", -1, 1, true)
	wantPayable(t, w, "gate", -1, 2, false) // Mauer 2 fehlt
	wantPayable(t, w, "gate", 1, 1, false)  // rechte Mauer 1 fehlt
	gate1 := lineSite(t, w, "gate", -1, 1)
	built(gate1)
	built(lineSite(t, w, "wall", -1, 2))
	if gate1.State != "built" {
		t.Fatalf("inneres Tor: %s", gate1.State)
	}
	wantPayable(t, w, "gate", -1, 2, true)
	built(lineSite(t, w, "wall", 1, 1))
	built(lineSite(t, w, "wall", 1, 2))
	wantPayable(t, w, "gate", 1, 1, false) // nicht mehr die äußerste
	wantPayable(t, w, "gate", 1, 2, true)
	w.HubLevel = 3
	built(lineSite(t, w, "wall", 1, 3))
	w.HubLevel = 2
	wantPayable(t, w, "gate", 1, 3, false) // Hub-Stufe max(2, 3) fehlt
	wantPayable(t, w, "gate", 1, 2, false)
}

// (d) Q54: Ein innerer Turm trägt weiter Schützen, Gegner bleiben an der äußersten gebauten Mauer stehen.
func TestLineInnererTurmUndAeussersteMauer(t *testing.T) {
	w := quietWorld(t)
	w.HubLevel = 2
	inner := lineSite(t, w, "tower", -1, 1)
	built(inner)
	built(lineSite(t, w, "wall", -1, 1))
	built(lineSite(t, w, "tower", -1, 2))
	outer := lineSite(t, w, "wall", -1, 2)
	built(outer)
	archer := spawnVagrant(w, inner.X, inner.X)
	makeArcher(w, archer)
	archer.AnchorX = inner.X
	if got := freeTower(w, archer); got != inner {
		t.Fatalf("Schütze am inneren Turm bekommt %+v", got)
	}
	e := spawnEnemy(w, "goblin", outer.X-10)
	run(w, 6)
	if archer.TowerID == nil || *archer.TowerID != inner.ID || !isOnTower(w, archer) {
		t.Fatalf("Schütze nicht auf dem inneren Turm: %+v", archer)
	}
	if e.HP > 0 && (e.X >= outer.X || outer.HP >= outer.MaxHP) {
		t.Fatalf("Gegner bei %v, äußere Mauer bei %v mit %v/%v", e.X, outer.X, outer.HP, outer.MaxHP)
	}
}

// payResult: Zustand und bezahltes Gold zweier Plätze und das ausgegebene Gold der Spieler, die dort stehen.
type payResult struct {
	stateA, stateB string
	paidA, paidB   int
	spentA, spentB int
}

// payTogether lässt zwei Spieler gleichzeitig an a und b zahlen; swap tauscht, welcher Spieler wo steht.
func payTogether(t *testing.T, pick func(*World) (*Site, *Site), swap bool) payResult {
	t.Helper()
	w := quietWorld(t)
	p0, p1 := AddPlayer(w), AddPlayer(w)
	a, b := pick(w)
	pa, pb := p0, p1
	if swap {
		pa, pb = p1, p0
	}
	pa.X, pb.X = a.X, b.X
	const gold = 50
	p0.Gold, p1.Gold = gold, gold
	cmds := []PlayerCommand{{Pay: true}, {Pay: true}}
	for range 300 {
		Step(w, cmds, dt)
		pa.X, pb.X = a.X, b.X
	}
	return payResult{a.State, b.State, a.PaidGold, b.PaidGold, gold - pa.Gold, gold - pb.Gold}
}

// (e) AC-06: Zwei Spieler zahlen gleichzeitig an Mauer und Turm derselben Linie bzw. an beiden Seiten.
func TestLineZweiSpielerZahlenGleichzeitig(t *testing.T) {
	cases := map[string]func(*World) (*Site, *Site){
		"Mauer und Turm links": func(w *World) (*Site, *Site) {
			return lineSite(t, w, "wall", -1, 1), lineSite(t, w, "tower", -1, 1)
		},
		"Mauer links und rechts": func(w *World) (*Site, *Site) {
			return lineSite(t, w, "wall", -1, 1), lineSite(t, w, "wall", 1, 1)
		},
	}
	for name, pick := range cases {
		got, swapped := payTogether(t, pick, false), payTogether(t, pick, true)
		if got != swapped {
			t.Errorf("%s: Ergebnis hängt von der Spieler-Reihenfolge ab: %+v gegen %+v", name, got, swapped)
		}
		if got.stateA == "unpaid" || got.stateB == "unpaid" || got.spentA == 0 || got.spentB == 0 {
			t.Errorf("%s: nicht beide Zahlungen gezählt: %+v", name, got)
		}
	}
}

// (f): Jede Stufe einer Insel hat Mauer, Turm und Tor jeder Linie ihres eigenen Layouts, fest nach X angelegt.
func TestLineInselJedeStufeEigeneLinien(t *testing.T) {
	isl := mustIsland(t, "linien", []int{0, 1})
	for i, w := range isl.Stages {
		lineSites := 0
		for j, s := range w.Sites {
			if _, ok := siteLine(w, s); ok {
				lineSites++
			}
			if j > 0 && s.X < w.Sites[j-1].X && s.Kind != "storage" {
				t.Errorf("Stufe %d: Platz %s@%v vor %s@%v nicht nach X", i, s.Kind, s.X, w.Sites[j-1].Kind, w.Sites[j-1].X)
			}
		}
		if want := 3 * len(w.Level.Lines); lineSites != want || len(w.Level.Lines) != 2*len(hub.WallLines.WallUnits) {
			t.Errorf("Stufe %d: %d Linien-Plätze, erwartet %d", i, lineSites, want)
		}
	}
	a, b := isl.Stages[0], isl.Stages[1]
	if fmt.Sprint(a.Level.Lines) == fmt.Sprint(b.Level.Lines) {
		t.Error("beide Stufen haben dieselben Linien")
	}
	a.HubLevel, b.HubLevel = 2, 2
	built(lineSite(t, b, "wall", -1, 1))
	wantPayable(t, b, "wall", -1, 2, true)
	wantPayable(t, a, "wall", -1, 2, false) // Linie 1 der anderen Stufe zählt nicht
}
