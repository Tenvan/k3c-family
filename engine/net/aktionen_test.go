package net

import (
	"encoding/json"
	"maps"
	"slices"
	"strings"
	"testing"

	"k3c/engine/sim"
)

// W11.3 (B-410): actions trägt train (Beruf) und trade (Händler) hinter attack, skill und learn.

// aktionenInsel: Werkstatt gebaut, Händler mit Holz, 30 Holz im Vorrat, zwei Spieler mit Gold.
func aktionenInsel(t *testing.T) *sim.World {
	t.Helper()
	isl, err := sim.CreateIsland("w11-aktionen", []int{0}, 1)
	if err != nil {
		t.Fatal(err)
	}
	sim.AddIslandPlayer(isl, 0)
	sim.AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	for _, s := range w.Sites {
		if s.Kind == "workshop" {
			s.State = "built"
		}
	}
	w.Merchant = &sim.Merchant{Resource: "wood", Leaves: 3}
	isl.Stock.Wood = 30
	for _, p := range w.Players {
		p.Gold = 12
	}
	return w
}

// payEntryAt sucht im Hub die erste Stelle, an der p den Eintrag action (mit side bzw. kind = sub) bekommt.
func payEntryAt(t *testing.T, w *sim.World, p *sim.Player, action, sub string) map[string]any {
	t.Helper()
	for x := w.HubX - 50; x <= w.HubX+50; x += 0.25 {
		p.X = x
		for _, e := range actionsOf(w, p) {
			if e["action"] == action && (e["side"] == sub || e["kind"] == sub) {
				return e
			}
		}
	}
	t.Fatalf("kein %s/%s im Hub", action, sub)
	return nil
}

// payFields sind die Felder je Eintrag laut B-410 › Anforderungen.
var payFields = map[string][]string{
	"train": {"action", "gold", "kind", "paid"},
	"trade": {"action", "gold", "material", "resource", "side"},
}

func checkFields(t *testing.T, at string, e map[string]any) {
	t.Helper()
	keys := slices.Sorted(maps.Keys(e))
	if want := payFields[e["action"].(string)]; !slices.Equal(keys, want) {
		t.Errorf("%s: Felder %v, erwartet %v", at, keys, want)
	}
}

// AC-01: je ein Eintrag mit genau den Feldern aus B-410, höchstens einer je Spieler, hinter attack, skill, learn.
func TestAktionenTrainUndTrade(t *testing.T) {
	w := aktionenInsel(t)
	p := w.Players[0]
	for _, id := range []string{"train/miner", "trade/buy", "trade/sell"} {
		action, sub, _ := strings.Cut(id, "/")
		checkFields(t, id, payEntryAt(t, w, p, action, sub))
		checkOrder(t, id, asJSON(t, actionsOf(w, p)).([]any))
	}
	payEntryAt(t, w, p, "trade", "sell")
	w.Stock.Wood = 0 // Vorrat zu klein: der Eintrag fehlt
	for _, e := range actionsOf(w, p) {
		if e["action"] == "trade" {
			t.Errorf("trade ohne Vorrat: %v", e)
		}
	}
}

// AC-03: Das Beispiel s2c-snapshot-wirtschaft.json enthält train und trade mit den Feldern aus B-410.
func TestAktionenBeispiel(t *testing.T) {
	seen := map[string]bool{}
	for _, p := range want(t, "snapshot-wirtschaft")["s"].(map[string]any)["players"].([]any) {
		for _, raw := range p.(map[string]any)["actions"].([]any) {
			e := raw.(map[string]any)
			if a := e["action"].(string); payFields[a] != nil {
				checkFields(t, "Beispiel "+a, e)
				seen[a] = true
			}
		}
	}
	if !seen["train"] || !seen["trade"] {
		t.Errorf("Beispiel: %v, erwartet train und trade", seen)
	}
}

// checkOrder: Einträge in der Reihenfolge attack, skill, learn, train, trade, höchstens ein train oder trade.
func checkOrder(t *testing.T, at string, list []any) {
	t.Helper()
	rank := map[string]int{"attack": 0, "skill": 1, "learn": 2, "train": 3, "trade": 4}
	last, pays := -1, 0
	for _, raw := range list {
		r := rank[raw.(map[string]any)["action"].(string)]
		if r < last {
			t.Errorf("%s: Reihenfolge %v", at, list)
		}
		if r >= 3 {
			pays++
		}
		last = r
	}
	if pays != 1 {
		t.Errorf("%s: %d Zahl-Einträge in %v, erwartet 1", at, pays, list)
	}
}

// AC-02: Zwei Spieler an verschiedenen Orten bekommen im Zustand verschiedene Listen; der Zustand ändert die Welt nicht.
func TestAktionenZweiSpielerZweiListen(t *testing.T) {
	w := aktionenInsel(t)
	p0, p1 := w.Players[0], w.Players[1]
	payEntryAt(t, w, p0, "train", "miner")
	payEntryAt(t, w, p1, "trade", "sell")
	before, _ := json.Marshal(w)
	s := asJSON(t, stateOf(w, 0, false)).(map[string]any)
	if after, _ := json.Marshal(w); string(before) != string(after) {
		t.Fatal("stateOf hat die Welt verändert")
	}
	got := map[float64]string{}
	for _, raw := range s["players"].([]any) {
		pl := raw.(map[string]any)
		for _, e := range pl["actions"].([]any) {
			if a := e.(map[string]any)["action"].(string); a == "train" || a == "trade" {
				got[pl["index"].(float64)] = a
			}
		}
	}
	if got[float64(p0.Index)] != "train" || got[float64(p1.Index)] != "trade" {
		t.Errorf("Zahl-Einträge je Spieler: %v", got)
	}
}
