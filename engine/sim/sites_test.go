package sim

import (
	"math"
	"testing"
)

// W3 (B-116/AC-02, Sprint W3 AC-02): Kosten, HP, Bauzeit, Hub-Stufe und Platz der Gebäude kommen aus den Daten.

// buildNew bezahlt einen unbezahlten Platz mit den Spielern (Gold gleich verteilt, Material genau im Vorrat) und lässt
// einen Bauern bauen. Rückgabe: Bauzeit in Sekunden, -1 bei Zeitablauf.
func buildNew(t *testing.T, w *World, players []*Player, s *Site) float64 {
	t.Helper()
	cost := buildings[s.Kind].Cost
	*w.Stock = materialOnly(cost)
	share := cost.Gold / len(players)
	for i, p := range players {
		p.Gold = share
		if i == 0 {
			p.Gold += cost.Gold - share*len(players)
		}
	}
	seconds := 1.2*float64(cost.Gold)*economy.PayIntervalSeconds + 5
	if !payAt(w, players, s.X, seconds, func() bool { return s.State == "waitingWorker" }) {
		t.Fatalf("%s: nicht bezahlt: %+v, Vorrat %+v", s.Kind, s, *w.Stock)
	}
	for _, p := range players {
		if p.Gold != 0 {
			t.Errorf("%s: Spieler %d hat %d Gold übrig", s.Kind, p.Index, p.Gold)
		}
	}
	if *w.Stock != (Stock{}) {
		t.Errorf("%s: Vorrat %+v, erwartet genau %+v abgebucht", s.Kind, *w.Stock, cost)
	}
	for _, tr := range w.Troops {
		if tr.Kind == "peasant" {
			tr.X = s.X
		}
	}
	return runUntil(w, 2*buildings[s.Kind].BuildSeconds+1, func() bool { return s.State == "built" })
}

// wantBuiltFromData prüft HP und Bauzeit eines frisch gebauten Platzes gegen data/buildings.json.
func wantBuiltFromData(t *testing.T, s *Site, took float64) {
	t.Helper()
	b := buildings[s.Kind]
	if s.HP != b.HP || s.MaxHP != b.HP {
		t.Errorf("%s: HP %v/%v, laut Daten %v", s.Kind, s.HP, s.MaxHP, b.HP)
	}
	if took < b.BuildSeconds-dt || took > b.BuildSeconds+0.5 {
		t.Errorf("%s: Bau dauerte %.2f s, laut Daten %v s", s.Kind, took, b.BuildSeconds)
	}
}

// hubSiteOf ist der Platz kind und sein Eintrag in data/hub.json › sites.
func hubSiteOf(t *testing.T, w *World, kind string) (*Site, HubSite) {
	t.Helper()
	for _, h := range hub.Sites {
		if h.Kind != kind {
			continue
		}
		for _, s := range w.Sites {
			if s.Kind == kind && math.Abs(s.X-(w.HubX+h.OffsetUnits)) < 1e-9 {
				return s, h
			}
		}
	}
	t.Fatalf("kein Hub-Platz %s laut data/hub.json", kind)
	return nil, HubSite{}
}

// wantHubBuildFromData: Platz und Hub-Stufe aus hub.json, darunter nicht bezahlbar, Kosten, HP und Bauzeit aus
// buildings.json; mit zwei Spielern bezahlt.
func wantHubBuildFromData(t *testing.T, kind string) *World {
	t.Helper()
	w := dayWorld(t)
	players := []*Player{AddPlayer(w), AddPlayer(w)}
	s, h := hubSiteOf(t, w, kind)
	if siteHubLevel(w, s) != h.HubLevel {
		t.Errorf("%s: Hub-Stufe %d, laut Daten %d", kind, siteHubLevel(w, s), h.HubLevel)
	}
	w.HubLevel = h.HubLevel - 1
	if sitePayable(w, s) {
		t.Errorf("%s: bei Hub-Stufe %d bezahlbar", kind, w.HubLevel)
	}
	w.HubLevel = h.HubLevel
	wantBuiltFromData(t, s, buildNew(t, w, players, s))
	return w
}

func TestHubGebaeudeAusDenDaten(t *testing.T) {
	for _, kind := range []string{"barracks", "tavern", "healer", "smithy", "armory"} {
		wantHubBuildFromData(t, kind)
	}
}

// Tor: Platz der Linie aus dem Layout, ab gateHubLevel, Kosten, HP und Bauzeit aus buildings.json.
func TestTorAusDenDaten(t *testing.T) {
	w := dayWorld(t)
	players := []*Player{AddPlayer(w), AddPlayer(w)}
	built(lineSite(t, w, "wall", -1, 1))
	gate := lineSite(t, w, "gate", -1, 1)
	if l, _ := siteLine(w, gate); gate.X != l.Gate {
		t.Errorf("Tor bei %v, laut Layout %v", gate.X, l.Gate)
	}
	w.HubLevel = hub.WallLines.GateHubLevel - 1
	if sitePayable(w, gate) {
		t.Errorf("Tor bei Hub-Stufe %d bezahlbar", w.HubLevel)
	}
	w.HubLevel = hub.WallLines.GateHubLevel
	wantBuiltFromData(t, gate, buildNew(t, w, players, gate))
}

// W3.2 (B-116/AC-03, Sprint W3 AC-03): Schmiede und Rüstkammer lassen sich bauen (oben, ab ihrer Hub-Stufe) und
// zerstören; ihre Wirkung prüft upgrades_test.go (W4.3b, B-313).
func TestSchmiedeUndRuestkammerZerstoerbar(t *testing.T) {
	for _, kind := range []string{"smithy", "armory"} {
		w := wantHubBuildFromData(t, kind)
		s, _ := hubSiteOf(t, w, kind)
		destroySite(w, s)
		if s.State != "unpaid" || s.HP != 0 || len(w.Events) == 0 || w.Events[len(w.Events)-1]["type"] != "destroyed" {
			t.Errorf("%s: nach der Zerstörung %+v, Ereignisse %v", kind, s, w.Events)
		}
	}
}
