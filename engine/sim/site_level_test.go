package sim

import "testing"

// W1.2 (B-112/AC-02, Sprint W1 AC-02): Mauer und Turm werden am selben Platz auf Stufe 2 bis 5 ausgebaut. Werte aus
// data/buildings.json › levels.

// upgradeAt bezahlt mit den Spielern den Ausbau von s auf die nächste Stufe (Gold je Spieler gleich verteilt, Material
// genau im Vorrat) und lässt einen Bauern ohne Weg bauen. Rückgabe: Bauzeit in Sekunden, -1 bei Zeitablauf.
func upgradeAt(t *testing.T, w *World, players []*Player, s *Site) float64 {
	t.Helper()
	lv := nextLevel(w, s)
	if lv == nil {
		t.Fatalf("%s@%v Stufe %d nicht ausbaubar", s.Kind, s.X, levelOf(w, s))
	}
	*w.Stock = materialOnly(lv.Cost)
	share := lv.Cost.Gold / len(players)
	for i, p := range players {
		p.Gold = share
		if i == 0 {
			p.Gold += lv.Cost.Gold - share*len(players)
		}
	}
	seconds := 1.2*float64(lv.Cost.Gold)*economy.PayIntervalSeconds + 5
	if !payAt(w, players, s.X, seconds, func() bool { return s.Upgrade == "waitingWorker" }) {
		t.Fatalf("%s: Ausbau nicht bezahlt: %+v, Vorrat %+v", s.Kind, s, *w.Stock)
	}
	for _, p := range players {
		if p.Gold != 0 {
			t.Errorf("%s: Spieler %d hat %d Gold übrig", s.Kind, p.Index, p.Gold)
		}
	}
	if *w.Stock != (Stock{}) {
		t.Errorf("%s: Vorrat %+v, erwartet genau %+v abgebucht", s.Kind, *w.Stock, lv.Cost)
	}
	w.Troops[0].X = s.X
	target := levelOf(w, s) + 1
	return runUntil(w, 2*lv.BuildSeconds, func() bool {
		if s.State != "built" {
			t.Fatalf("%s: alte Stufe während des Ausbaus nicht mehr gebaut: %+v", s.Kind, s)
		}
		return levelOf(w, s) == target
	})
}

// wantLevel prüft Stufe, HP, Position und Bauzeit nach einem Ausbau laut Daten.
func wantLevel(t *testing.T, s *Site, level int, x, took float64) {
	t.Helper()
	lv := buildings[s.Kind].Levels[level-1]
	if s.Level != level || s.HP != lv.HP || s.MaxHP != lv.HP || s.X != x {
		t.Errorf("%s: Stufe %d, HP %v/%v, X %v; erwartet Stufe %d, HP %v, X %v", s.Kind, s.Level, s.HP, s.MaxHP, s.X, level, lv.HP, x)
	}
	if took < lv.BuildSeconds-dt || took > lv.BuildSeconds+0.5 {
		t.Errorf("%s Stufe %d: Bau dauerte %.2f s, laut Daten %v s", s.Kind, level, took, lv.BuildSeconds)
	}
}

// Mauer der Linie 1 von Stufe 1 bis 5 am selben Platz; Linie 2 muss dafür nicht gebaut sein.
func TestMauerAusbauAmSelbenPlatz(t *testing.T) {
	w := dayWorld(t)
	p := AddPlayer(w)
	wall := lineSite(t, w, "wall", -1, 1)
	built(wall)
	x := wall.X
	for level := 2; level <= len(buildings["wall"].Levels); level++ {
		w.HubLevel = level
		wantLevel(t, wall, level, x, upgradeAt(t, w, []*Player{p}, wall))
	}
	if outer := lineSite(t, w, "wall", -1, 2); outer.State != "unpaid" {
		t.Errorf("Linie 2 ist %s, erwartet unpaid", outer.State)
	}
	if upgradePayable(w, wall) {
		t.Error("Mauer Stufe 5 weiter ausbaubar")
	}
}

func TestMauerStufe3BrauchtHubStufe3(t *testing.T) {
	w := dayWorld(t)
	wall := lineSite(t, w, "wall", 1, 1)
	built(wall)
	wall.Level = 2
	w.HubLevel = 2
	if sitePayable(w, wall) {
		t.Error("Mauer Stufe 3 bei Hub-Stufe 2 bezahlbar")
	}
	w.HubLevel = 3
	if !sitePayable(w, wall) {
		t.Error("Mauer Stufe 3 bei Hub-Stufe 3 nicht bezahlbar")
	}
}

// Turm-Ausbau am selben Platz (Q45); der Bogenschütze bleibt oben, bis der Zaubertum ihn absteigen lässt (Q31,
// spell_tower_test.go).
func TestTurmAusbauSchuetzeBleibt(t *testing.T) {
	w := dayWorld(t)
	p := AddPlayer(w)
	tower := lineSite(t, w, "tower", 1, 1)
	built(tower)
	archer := spawnVagrant(w, tower.X, tower.X)
	makeArcher(w, archer)
	archer.AnchorX, archer.TowerID = tower.X, intPtr(tower.ID)
	x := tower.X
	for level := 2; level <= len(buildings["tower"].Levels); level++ {
		w.HubLevel = level
		wantLevel(t, tower, level, x, upgradeAt(t, w, []*Player{p}, tower))
		if spell := level >= buildings["tower"].Spell.FromLevel; spell != (archer.TowerID == nil) {
			t.Fatalf("Stufe %d: Schütze auf dem Turm = %v, erwartet %v: %+v", level, archer.TowerID != nil, !spell, archer)
		}
	}
}

func TestMauerAusbauZweiSpieler(t *testing.T) {
	w := dayWorld(t)
	p0, p1 := AddPlayer(w), AddPlayer(w)
	wall := lineSite(t, w, "wall", -1, 1)
	built(wall)
	w.HubLevel = 2
	wantLevel(t, wall, 2, wall.X, upgradeAt(t, w, []*Player{p0, p1}, wall))
}

// Stufe 1 der levels sind die bisherigen Werte (der Client liest hp, cost, buildSeconds).
func TestStufe1GleichGrundwerte(t *testing.T) {
	for _, kind := range []string{"wall", "tower"} {
		b := buildings[kind]
		if len(b.Levels) != 5 || b.Levels[0].HP != b.HP || b.Levels[0].Cost != b.Cost || b.Levels[0].BuildSeconds != b.BuildSeconds {
			t.Errorf("%s: %d Stufen, Stufe 1 %+v, Grundwerte HP %v Kosten %+v Bauzeit %v", kind, len(b.Levels), b.Levels[0], b.HP, b.Cost, b.BuildSeconds)
		}
	}
	if len(hub.Levels) != 5 {
		t.Errorf("%d Hub-Stufen, erwartet 5", len(hub.Levels))
	}
}
