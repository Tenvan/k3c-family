package sim

import "testing"

// B-631/AC-01: Das Tor hat Ausbaustufen wie die Mauer (Stufe n ab Hub-Stufe n) und ist ab Hub-Stufe 1 bezahlbar.

// Die fünf Tor-Stufen laut Spec: Material der Stufe, Gold, HP, Bauzeit.
func TestTorStufenInDenDaten(t *testing.T) {
	want := []LevelData{
		{Cost: Cost{Wood: 30, Gold: 3}, HP: 250, BuildSeconds: 6},
		{Cost: Cost{Stone: 45, Gold: 4}, HP: 500, BuildSeconds: 9},
		{Cost: Cost{Copper: 60, Gold: 6}, HP: 850, BuildSeconds: 12},
		{Cost: Cost{Iron: 75, Gold: 10}, HP: 1350, BuildSeconds: 16},
		{Cost: Cost{Crystal: 90, Gold: 15}, HP: 2100, BuildSeconds: 20},
	}
	got := buildings["gate"].Levels
	if len(got) != len(want) {
		t.Fatalf("%d Tor-Stufen, erwartet %d", len(got), len(want))
	}
	for i, lv := range want {
		if g := got[i]; g.Cost != lv.Cost || g.HP != lv.HP || g.BuildSeconds != lv.BuildSeconds {
			t.Errorf("Tor-Stufe %d: %+v, erwartet %+v", i+1, g, lv)
		}
	}
	if hub.WallLines.GateHubLevel != 1 {
		t.Errorf("gateHubLevel %d, erwartet 1", hub.WallLines.GateHubLevel)
	}
}

// Zwei Spieler bauen das Tor auf Hub-Stufe 1 und bauen es auf Hub-Stufe 2 am selben Platz aus; ohne Hub-Stufe 2
// nimmt es keine Münzen für den Ausbau.
func TestTorAusbauAbHubStufe(t *testing.T) {
	w := dayWorld(t)
	players := []*Player{AddPlayer(w), AddPlayer(w)}
	w.HubLevel = 1
	built(lineSite(t, w, "wall", -1, 1))
	gate := lineSite(t, w, "gate", -1, 1)
	wantBuiltFromData(t, gate, buildNew(t, w, players, gate))
	if nextLevel(w, gate) != nil || sitePayable(w, gate) {
		t.Fatalf("Tor auf Hub-Stufe 1 ausbaubar: %+v", nextLevel(w, gate))
	}
	w.HubLevel = 2
	if next := nextLevel(w, gate); next == nil || next.Cost != (Cost{Stone: 45, Gold: 4}) {
		t.Fatalf("Tor-Stufe 2 auf Hub-Stufe 2: %+v", next)
	}
	wantLevel(t, gate, 2, gate.X, upgradeAt(t, w, players, gate))
}
