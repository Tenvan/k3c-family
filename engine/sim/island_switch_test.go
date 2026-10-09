package sim

import (
	"math"
	"slices"
	"testing"
	"testing/fstest"
)

// Inselwechsel (B-103, K2.3a).

// withTestIslands hängt für einen Test eine zweite Insel an die Daten (Tiefen 0 und 1, eigene Tabelle).
func withTestIslands(t *testing.T) IslandDef {
	t.Helper()
	old := islandDefs
	two := IslandDef{ID: "testIsland2", Name: "Test-Insel", Depths: []int{0, 1}}
	two.DepthScaling.HP, two.DepthScaling.Damage, two.DepthScaling.Speed = 2, 2, 1
	islandDefs = append(slices.Clone(old), two)
	t.Cleanup(func() { islandDefs = old })
	return two
}

// gateIsland: Insel mit Wald und Höhle, n Spieler am Wechselpunkt in der Höhle (tiefste Stufe), Endboss besiegt.
func gateIsland(t *testing.T, n int) (*Island, []*Player) {
	t.Helper()
	isl := mustIsland(t, "wechsel", []int{0, 1})
	var ps []*Player
	for range n {
		p := AddIslandPlayer(isl, 1)
		p.X = isl.Stages[1].HubX
		ps = append(ps, p)
	}
	isl.EndbossDefeated = true
	return isl, ps
}

// stepSeconds rechnet die Insel so viele Sekunden und sammelt die Ereignisse aller Stufen.
func stepSeconds(isl *Island, seconds float64) []Event {
	var all []Event
	for t := 0.0; t < seconds; t += dt {
		StepIsland(isl, nil, dt)
		for _, w := range isl.Stages {
			all = append(all, w.Events...)
		}
	}
	return all
}

func TestInselwechselErstNachEndboss(t *testing.T) {
	withTestIslands(t)
	isl, _ := gateIsland(t, 2)
	isl.EndbossDefeated = false
	if ev := stepSeconds(isl, 3); count(ev, "islandGateOpen") != 0 || isl.SwitchReady {
		t.Fatal("Wechselpunkt vor dem Endboss")
	}
	isl.EndbossDefeated = true
	ev := stepSeconds(isl, 3)
	if count(ev, "islandGateOpen") != 1 || count(ev, "islandSwitch") != 1 || !isl.SwitchReady {
		t.Errorf("%d× islandGateOpen, %d× islandSwitch, bereit %v", count(ev, "islandGateOpen"), count(ev, "islandSwitch"), isl.SwitchReady)
	}
	if def, ok := isl.NextDef(); !ok || def.ID != "testIsland2" {
		t.Errorf("nächste Insel %q, ok %v", def.ID, ok)
	}
	if count(ev, "victory") != 0 {
		t.Error("victory vor der letzten Insel")
	}
}

func TestInselwechselLetzteInselOhneWechsel(t *testing.T) {
	isl, _ := gateIsland(t, 2)
	ev := stepSeconds(isl, 3)
	if count(ev, "islandGateOpen") != 0 || isl.SwitchReady || count(ev, "victory") != 1 {
		t.Errorf("letzte Insel: %d× islandGateOpen, bereit %v, %d× victory", count(ev, "islandGateOpen"), isl.SwitchReady, count(ev, "victory"))
	}
}

func TestInselwechselAlleLebendenAmPunkt(t *testing.T) {
	cases := []struct {
		name  string
		n     int
		setup func(isl *Island, ps []*Player)
		ready bool
	}{
		{"2 Spieler", 2, func(*Island, []*Player) {}, true},
		{"4 Spieler", 4, func(*Island, []*Player) {}, true},
		{"einer fehlt", 4, func(_ *Island, ps []*Player) { ps[3].X += 20 }, false},
		{"einer tot", 2, func(_ *Island, ps []*Player) { ps[1].X += 20; ps[1].RespawnIn = 100 }, true},
		{"einer frei", 2, func(_ *Island, ps []*Player) { ps[1].X += 20; ps[1].Free = true }, true},
		{"einer in anderer Stufe", 2, func(isl *Island, _ []*Player) { AddIslandPlayer(isl, 0) }, false},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			withTestIslands(t)
			isl, ps := gateIsland(t, c.n)
			c.setup(isl, ps)
			stepSeconds(isl, hub.Travel.Seconds+0.5)
			if isl.SwitchReady != c.ready {
				t.Errorf("bereit %v, erwartet %v", isl.SwitchReady, c.ready)
			}
		})
	}
}

func TestInselwechselAbbruchBeimWeggehen(t *testing.T) {
	withTestIslands(t)
	isl, ps := gateIsland(t, 2)
	gate := ps[0].X
	stepSeconds(isl, 0.75*hub.Travel.Seconds)
	ps[0].X = gate + 20
	stepSeconds(isl, dt)
	ps[0].X = gate
	stepSeconds(isl, 0.75*hub.Travel.Seconds)
	if isl.SwitchReady {
		t.Fatal("Fortschritt läuft nach dem Weggehen weiter")
	}
	stepSeconds(isl, 0.5*hub.Travel.Seconds)
	if !isl.SwitchReady {
		t.Error("nach vollem Fortschritt nicht bereit")
	}
}

// switchedIsland: Insel 1 (Grad hard, Ziel gold) mit zwei Spielern, Gold und Skills, Wechsel zur Test-Insel.
func switchedIsland(t *testing.T) (cur, next *Island) {
	t.Helper()
	def := withTestIslands(t)
	cur, ps := gateIsland(t, 2)
	if err := SetOptions(cur, IslandOptions{Grade: "hard", Goal: "gold", Defeat: "resources"}, false); err != nil {
		t.Fatal(err)
	}
	ps[0].Gold, ps[1].Gold = 33, 44
	ps[0].Skills, ps[0].Slots = []string{"a", "b"}, []string{"a"}
	ps[1].HP, ps[1].RespawnIn = 0, 3
	next, err := NextIsland(cur, def)
	if err != nil {
		t.Fatal(err)
	}
	return cur, next
}

func TestInselwechselNeueInsel(t *testing.T) {
	cur, next := switchedIsland(t)
	if next.Number != 1 || next.ID != cur.ID || next.Seed == cur.Seed || next.Options != cur.Options || len(next.Stages) != 2 {
		t.Fatalf("Insel %d, ID %q, Seed %q, Optionen %+v, %d Stufen", next.Number, next.ID, next.Seed, next.Options, len(next.Stages))
	}
	if *next.Stock != hub.IslandStartStock {
		t.Errorf("Vorrat %+v, erwartet Startvorrat", *next.Stock)
	}
	for _, w := range next.Stages {
		if builtSites(w) != 0 || w.island != next {
			t.Errorf("Tiefe %d: %d gebaute Plätze, eigene Insel %v", w.Biome.Depth, builtSites(w), w.island == next)
		}
	}
}

func TestInselwechselSpielerZiehenMit(t *testing.T) {
	_, next := switchedIsland(t)
	first := next.Stages[0]
	if len(first.Players) != 2 || len(next.Stages[1].Players) != 0 {
		t.Fatalf("Spieler in Stufe 0: %d, in Stufe 1: %d", len(first.Players), len(next.Stages[1].Players))
	}
	for i, p := range first.Players {
		if p.Index != i || p.HP != p.MaxHP || !isAlive(p) || math.Abs(p.X-first.HubX) > 3 {
			t.Errorf("Spieler %d: Index %d, HP %v/%v, x %v (Burg %v)", i, p.Index, p.HP, p.MaxHP, p.X, first.HubX)
		}
	}
	p0 := first.Players[0]
	if p0.Gold != 33 || first.Players[1].Gold != 44 || !slices.Equal(p0.Skills, []string{"a", "b"}) || !slices.Equal(p0.Slots, []string{"a"}) {
		t.Errorf("Gold %d/%d, Skills %v, Slots %v", p0.Gold, first.Players[1].Gold, p0.Skills, p0.Slots)
	}
}

func TestInselwechselSkillBasis(t *testing.T) {
	_, next := switchedIsland(t)
	first := next.Stages[0]
	p0 := first.Players[0]
	if AvailablePoints(first, p0) != 0 {
		t.Errorf("mitgebrachte Skills zählen gegen den neuen Pool: %d Punkte", AvailablePoints(first, p0))
	}
	addPoolPoints(first, 2)
	if AvailablePoints(first, p0) != 2 {
		t.Errorf("verfügbar %d, erwartet 2 aus dem Pool der neuen Insel", AvailablePoints(first, p0))
	}
	if p := AddIslandPlayer(next, 0); p.Index != 2 {
		t.Errorf("neuer Spieler mit Index %d, erwartet 2", p.Index)
	}
}

func TestInselwechselTabelleDerInsel(t *testing.T) {
	cur, next := switchedIsland(t)
	old, neu := spawnEnemy(cur.Stages[1], "goblin", 0), spawnEnemy(next.Stages[1], "goblin", 0)
	base := enemyData["goblin"].HP
	if old.MaxHP != math.Round(base*1.5) || neu.MaxHP != math.Round(base*2) {
		t.Errorf("Goblin in Tiefe 1: Insel 1 %v, Test-Insel %v (Basis %v)", old.MaxHP, neu.MaxHP, base)
	}
}

func TestInselwechselDaten(t *testing.T) {
	d := islandDefs[0]
	if !slices.Equal(d.Depths, []int{0, 1, 2, 3, 4}) || endBoss(4) == nil {
		t.Fatalf("Insel 1: Tiefen %v, Endboss in Tiefe 4 %v", d.Depths, endBoss(4) != nil)
	}
	for _, depth := range d.Depths {
		if miniBoss(depth) == nil {
			t.Errorf("Insel 1: kein Miniboss in Tiefe %d", depth)
		}
	}
	bad := map[string]string{
		"ohne Endboss": `{"islands":[{"id":"a","depths":[0,1],"depthScaling":{"hp":1,"damage":1,"speed":1}}]}`,
		"doppelt":      `{"islands":[{"id":"a","depths":[0,1,2,3,4],"depthScaling":{"hp":1,"damage":1,"speed":1}},{"id":"a","depths":[0,1,2,3,4],"depthScaling":{"hp":1,"damage":1,"speed":1}}]}`,
		"Faktor 0":     `{"islands":[{"id":"a","depths":[0,1,2,3,4],"depthScaling":{"hp":0,"damage":1,"speed":1}}]}`,
		"Tiefe fehlt":  `{"islands":[{"id":"a","depths":[4,9],"depthScaling":{"hp":1,"damage":1,"speed":1}}]}`,
		"leer":         `{"islands":[]}`,
	}
	for name, raw := range bad {
		var dst []IslandDef
		if err := loadIslands(fstest.MapFS{"islands.json": {Data: []byte(raw)}}, &dst); err == nil || dst != nil {
			t.Errorf("%s: kein Fehler beim Laden", name)
		}
	}
}

func TestInselwechselDeterministisch(t *testing.T) {
	_, a := switchedIsland(t)
	_, b := switchedIsland(t)
	stepSeconds(a, 5)
	stepSeconds(b, 5)
	if !slices.Equal(islandJSON(t, a), islandJSON(t, b)) {
		t.Error("gleicher Seed, andere neue Insel")
	}
}
