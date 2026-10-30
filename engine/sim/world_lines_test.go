package sim

import (
	"fmt"
	"testing"

	"k3c/engine/level"
)

// B-652 (BS12.2): Verteilte Mauer- und Turm-Plätze in der Sim. Positionen kommen aus `Layout.Sites` der Stufe (Zone n ab
// Hub-Stufe n), bezahlbar erst im Freiraum von Bäumen und Felsen; ein Tor-Platz existiert nicht mehr.

// lineSite ist der erste Platz kind der Zone k auf Seite side (Zone = frühere Linie).
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

// wantPayable prüft sitePayable für kind der Zone k auf Seite side.
func wantPayable(t *testing.T, w *World, kind string, side, k int, want bool) {
	t.Helper()
	if got := sitePayable(w, lineSite(t, w, kind, side, k)); got != want {
		t.Errorf("Hub-Stufe %d: %s Linie %d Seite %+d bezahlbar = %v, erwartet %v", w.HubLevel, kind, k, side, got, want)
	}
}

// clearNodes räumt alle Bäume und Felsen der Welt (Freiraum-Regel), damit ein Test nur die Zonen prüft.
func clearNodes(w *World) { w.Nodes = []*ResourceNode{} }

// Zone n ab Hub-Stufe n, je Seite; innere Mauern müssen nicht gebaut sein.
func TestZoneFreischaltungJeSeite(t *testing.T) {
	w := quietWorld(t)
	clearNodes(w)
	if w.HubLevel != 1 {
		t.Fatalf("Start-Hub-Stufe %d, erwartet 1", w.HubLevel)
	}
	for _, kind := range []string{"wall", "tower"} {
		wantPayable(t, w, kind, -1, 1, true)
		wantPayable(t, w, kind, 1, 1, true)
		wantPayable(t, w, kind, -1, 2, false)
		wantPayable(t, w, kind, 1, 3, false)
	}
	w.HubLevel = 3
	for _, kind := range []string{"wall", "tower"} {
		for side := -1; side <= 1; side += 2 {
			wantPayable(t, w, kind, side, 3, true) // ohne gebaute Mauern der Zonen 1 und 2
			wantPayable(t, w, kind, side, 4, false)
		}
	}
}

// Freiraum: Ein Baum oder Fels im Radius sperrt den Platz, nach dem Abbau (gone/entfernt) ist er bezahlbar.
func TestFreiraumSperrtPlatzBisZumAbbau(t *testing.T) {
	w := quietWorld(t)
	clearNodes(w)
	site := lineSite(t, w, "wall", -1, 1)
	radius := level.DefaultSitePlacement().ClearRadiusUnits
	tree := &ResourceNode{ID: w.newID(), Kind: "tree", X: site.X + radius - 1}
	w.Nodes = append(w.Nodes, tree)
	wantPayable(t, w, "wall", -1, 1, false)
	tree.X = site.X + radius + 1
	wantPayable(t, w, "wall", -1, 1, true)
	rock := &ResourceNode{ID: w.newID(), Kind: "rock", X: site.X}
	w.Nodes = append(w.Nodes, rock)
	wantPayable(t, w, "wall", -1, 1, false)
	rock.gone = true
	wantPayable(t, w, "wall", -1, 1, true)
	w.Nodes = append(w.Nodes, &ResourceNode{ID: w.newID(), Kind: "bush", X: site.X}) // Büsche sperren nicht
	wantPayable(t, w, "wall", -1, 1, true)
}

// Das Tor ist die äußerste gebaute Mauer je Seite: Es wandert nach innen, wenn sie zerstört wird, und nach außen, wenn weiter außen gebaut wird.
func TestTorIstAeusserstGebauteMauer(t *testing.T) {
	w := quietWorld(t)
	inner, outer := lineSite(t, w, "wall", 1, 1), lineSite(t, w, "wall", 1, 2)
	if outerWall(w, 1) != nil {
		t.Fatal("ohne gebaute Mauer gibt es kein Tor")
	}
	built(inner)
	if outerWall(w, 1) != inner {
		t.Fatalf("Tor %v, erwartet die innere Mauer", outerWall(w, 1))
	}
	built(outer)
	if outerWall(w, 1) != outer {
		t.Fatalf("Tor %v, erwartet die äußere Mauer", outerWall(w, 1))
	}
	destroySite(w, outer)
	if outerWall(w, 1) != inner {
		t.Fatalf("Tor nach Zerstörung %v, erwartet wieder die innere Mauer", outerWall(w, 1))
	}
	for _, s := range w.Sites {
		if s.Kind == "gate" {
			t.Fatalf("Tor-Platz %+v: ein eigenes Tor gibt es nicht mehr", s)
		}
	}
}

// (d) Q54: Ein innerer Turm trägt weiter Schützen, Gegner bleiben an der äußersten gebauten Mauer stehen.
func TestZoneInnererTurmUndAeussersteMauer(t *testing.T) {
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
	clearNodes(w)
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

// (e) AC-06: Zwei Spieler zahlen gleichzeitig an Mauer und Turm derselben Zone bzw. an beiden Seiten.
func TestZoneZweiSpielerZahlenGleichzeitig(t *testing.T) {
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

// Jede Stufe einer Insel hat Mauern und Türme ihres eigenen Layouts, fest nach X angelegt.
func TestZonenInselJedeStufeEigeneSites(t *testing.T) {
	isl := mustIsland(t, "linien", []int{0, 1})
	for i, w := range isl.Stages {
		sites := 0
		for j, s := range w.Sites {
			if _, ok := levelSite(w, s); ok {
				sites++
			}
			if j > 0 && s.X < w.Sites[j-1].X && s.Kind != "storage" && s.Kind != shipKind {
				t.Errorf("Stufe %d: Platz %s@%v vor %s@%v nicht nach X", i, s.Kind, s.X, w.Sites[j-1].Kind, w.Sites[j-1].X)
			}
		}
		if sites != len(w.Level.Sites) || sites == 0 {
			t.Errorf("Stufe %d: %d verteilte Plätze, erwartet %d", i, sites, len(w.Level.Sites))
		}
	}
	a, b := isl.Stages[0], isl.Stages[1]
	if fmt.Sprint(a.Level.Sites) == fmt.Sprint(b.Level.Sites) {
		t.Error("beide Stufen haben dieselben Plätze")
	}
}
