package sim

import (
	"math"
	"testing"
)

// W1.1 (B-112/AC-01, AC-03, Sprint W1 AC-01): Hub-Ausbau an der Burg und Sperre nach Hub-Stufe. Werte aus data/.

// dayWorld ist eine ruhige Welt, in der der Tag nicht endet, mit einem Bauern an der Burg.
func dayWorld(t *testing.T) *World {
	t.Helper()
	w := quietWorld(t)
	w.CycleSpeed = 1e-6
	peasant := spawnVagrant(w, w.HubX, w.HubX)
	peasant.Kind, peasant.HP, peasant.MaxHP = "peasant", troops["peasant"].HP, troops["peasant"].HP
	return w
}

// materialOnly sind die Materialkosten ohne Gold als Vorrat.
func materialOnly(c Cost) Stock {
	return Stock{Wood: c.Wood, Stone: c.Stone, Copper: c.Copper, Iron: c.Iron, Crystal: c.Crystal}
}

// payAt lässt die Spieler an x zahlen, bis done gilt oder seconds um sind; false bei Zeitablauf.
func payAt(w *World, players []*Player, x float64, seconds float64, done func() bool) bool {
	cmds := make([]PlayerCommand, len(w.Players))
	for _, p := range players {
		cmds[p.Index].Pay = true
	}
	for tm := 0.0; tm < seconds; tm += dt {
		for _, p := range players {
			p.X = x
		}
		Step(w, cmds, dt)
		if done() {
			return true
		}
	}
	return false
}

// runUntil rechnet ohne Eingaben, bis done gilt; die Rückgabe sind die simulierten Sekunden, -1 bei Zeitablauf.
func runUntil(w *World, seconds float64, done func() bool) float64 {
	for tm := 0.0; tm < seconds; tm += dt {
		Step(w, nil, dt)
		if done() {
			return tm + dt
		}
	}
	return -1
}

func TestHubAusbauStufe2Bis5(t *testing.T) {
	w := dayWorld(t)
	p := AddPlayer(w)
	const spare = 7
	for n := 2; n <= len(hub.Levels); n++ {
		lv := hub.Levels[n-1]
		*w.Stock = materialOnly(lv.Cost)
		w.Stock.Wood += spare
		p.Gold = lv.Cost.Gold
		seconds := 1.2*float64(lv.Cost.Gold)*economy.PayIntervalSeconds + 5 // Takt auf ganze Ticks gerundet
		if !payAt(w, []*Player{p}, w.Castle.X, seconds, func() bool { return w.hubSite.Upgrade == "waitingWorker" }) {
			t.Fatalf("Stufe %d: Ausbau nicht bezahlt: %+v, Vorrat %+v", n, w.hubSite, *w.Stock)
		}
		if p.Gold != 0 || *w.Stock != (Stock{Wood: spare}) {
			t.Errorf("Stufe %d: Gold übrig %d, Vorrat %+v; erwartet genau %+v abgebucht", n, p.Gold, *w.Stock, lv.Cost)
		}
		w.Troops[0].X = w.Castle.X // ohne Weg: gemessen wird nur die Bauzeit
		took := runUntil(w, 2*lv.BuildSeconds, func() bool { return w.HubLevel == n })
		if took < lv.BuildSeconds-dt || took > lv.BuildSeconds+0.5 {
			t.Errorf("Stufe %d: Bau dauerte %.2f s, laut Daten %v s", n, took, lv.BuildSeconds)
		}
	}
	if w.hubSite.State != "built" || upgradePayable(w, w.hubSite) {
		t.Errorf("Hub-Stufe %d: weiter ausbaubar %+v", w.HubLevel, w.hubSite)
	}
}

func TestHubAusbauWartetAufMaterial(t *testing.T) {
	w := dayWorld(t)
	p := AddPlayer(w)
	lv := hub.Levels[1]
	p.Gold = lv.Cost.Gold
	payAt(w, []*Player{p}, w.Castle.X, 30, func() bool { return w.hubSite.Upgrade != "" })
	run(w, 5)
	if w.hubSite.Upgrade != "waitingMaterial" || w.HubLevel != 1 {
		t.Fatalf("ohne Material: %+v, Hub-Stufe %d", w.hubSite, w.HubLevel)
	}
	*w.Stock = materialOnly(lv.Cost)
	if runUntil(w, 2*lv.BuildSeconds+1, func() bool { return w.HubLevel == 2 }) < 0 {
		t.Fatalf("mit Material nicht ausgebaut: %+v", w.hubSite)
	}
}

// Während einer Welle bezahlt: Der Bauer baut erst nach der Gefahr.
func TestHubAusbauNachDerGefahr(t *testing.T) {
	w := dayWorld(t)
	p := AddPlayer(w)
	lv := hub.Levels[1]
	*w.Stock = materialOnly(lv.Cost)
	p.Gold = lv.Cost.Gold
	e := spawnEnemy(w, "goblin", w.WidthUnits-1)
	e.Stun = math.Inf(1)
	if !payAt(w, []*Player{p}, w.Castle.X, 30, func() bool { return w.hubSite.Upgrade == "waitingWorker" }) {
		t.Fatalf("bei Gefahr nicht bezahlbar: %+v", w.hubSite)
	}
	run(w, 2*lv.BuildSeconds)
	if w.HubLevel != 1 || w.hubSite.BuildProgress != 0 {
		t.Fatalf("bei Gefahr gebaut: Hub-Stufe %d, Fortschritt %v", w.HubLevel, w.hubSite.BuildProgress)
	}
	w.Enemies = []*Enemy{}
	if runUntil(w, 2*lv.BuildSeconds+1, func() bool { return w.HubLevel == 2 }) < 0 {
		t.Fatal("nach der Gefahr nicht ausgebaut")
	}
}

func TestHubAusbauZweiSpielerZahlenGemeinsam(t *testing.T) {
	w := dayWorld(t)
	p0, p1 := AddPlayer(w), AddPlayer(w)
	lv := hub.Levels[1]
	half := lv.Cost.Gold / 2
	p0.Gold, p1.Gold = half, lv.Cost.Gold-half
	if !payAt(w, []*Player{p0, p1}, w.Castle.X, 30, func() bool { return w.hubSite.Upgrade != "" }) {
		t.Fatalf("gemeinsam nicht bezahlt: %+v", w.hubSite)
	}
	if p0.Gold != 0 || p1.Gold != 0 || w.hubSite.UpgradePaid != lv.Cost.Gold {
		t.Errorf("Gold übrig %d/%d, bezahlt %d von %d", p0.Gold, p1.Gold, w.hubSite.UpgradePaid, lv.Cost.Gold)
	}
}

// Ein Hub-Platz ist erst ab seiner Hub-Stufe aus data/hub.json bezahlbar (Treppe, Kaserne ab 2, Schmiede ab 3 …).
func TestHubPlatzErstAbHubStufe(t *testing.T) {
	isl := mustIsland(t, "sperre", []int{0, 1})
	for _, w := range isl.Stages {
		for level := 1; level <= len(hub.Levels); level++ {
			w.HubLevel = level
			for _, s := range w.Sites {
				if _, onLine := siteLine(w, s); onLine {
					continue
				}
				if want := level >= siteHubLevel(w, s); sitePayable(w, s) != want {
					t.Errorf("Tiefe %d, Hub-Stufe %d: %s bezahlbar = %v", w.Biome.Depth, level, s.Kind, !want)
				}
			}
		}
	}
	w := isl.Stages[0]
	want := map[string]int{"stairsDown": 2, "storage": 2, "barracks": 2, "tavern": 2, "smithy": 3, "armory": 4, "workshop": 1}
	for kind, level := range want {
		for _, s := range w.Sites {
			if s.Kind == kind && siteHubLevel(w, s) != level {
				t.Errorf("%s: Hub-Stufe %d, erwartet %d", kind, siteHubLevel(w, s), level)
			}
		}
	}
}
