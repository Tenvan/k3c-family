package net

import (
	"reflect"
	"slices"
	"testing"

	"k3c/engine/sim"
)

// K4.1 (B-154/AC-01, AC-03): Boss mit Phase und Warnkreis, Nacht-Event mit Restzeit und Inselwechsel im Zustand.
// Die Felder setzt die Sim (K4.1a, state_mirror.go); stateOf reicht sie durch, das Delta nennt nur die Änderung.

// kampfWelt ist Stufe 0 einer Insel mit 4 Spielern und einem Endboss in Phase 2 (Flächenschlag in 3,4 s).
func kampfWelt(t *testing.T) *sim.World {
	t.Helper()
	isl, err := sim.CreateIsland("k4-kampf", []int{4}, 1)
	if err != nil {
		t.Fatal(err)
	}
	for range 4 {
		sim.AddIslandPlayer(isl, 0)
	}
	w := isl.Stages[0]
	w.Events = []sim.Event{}
	w.Enemies = []*sim.Enemy{{ID: 41, Kind: "crystalHeartWarden", Boss: true, X: 388.4, HP: 4200, MaxHP: 6000,
		AoeIn: 3.4, Phase: 2, Warn: &sim.WarnZone{X: 388.4, R: 5, In: 3.4}}}
	return w
}

func keysOf(m map[string]any) []string {
	var k []string
	for key := range m {
		k = append(k, key)
	}
	slices.Sort(k)
	return k
}

func TestKampfZustand(t *testing.T) {
	w := kampfWelt(t)
	s := asJSON(t, stateOf(w, 0, false)).(map[string]any)
	boss := s["enemies"].([]any)[0].(map[string]any)
	warn, _ := boss["warn"].(map[string]any)
	if boss["hp"] != 4200.0 || boss["maxHp"] != 6000.0 || boss["phase"] != 2.0 || warn["x"] != 388.4 || warn["r"] != 5.0 || warn["in"] != 3.4 {
		t.Errorf("Boss: %v", boss)
	}
	if _, ok := s["nightEvents"]; ok {
		t.Error("nightEvents ohne Event")
	}
	if _, ok := s["islandSwitch"]; ok {
		t.Error("islandSwitch vor dem Endboss")
	}

	w.NightEvents = []sim.ActiveEvent{{ID: "fullMoon", SecondsLeft: 95.5}}
	w.IslandSwitch = &sim.SwitchState{Open: true, Progress: 0.4}
	s = asJSON(t, stateOf(w, 0, false)).(map[string]any)
	if !reflect.DeepEqual(s["nightEvents"], []any{map[string]any{"id": "fullMoon", "secondsLeft": 95.5}}) ||
		!reflect.DeepEqual(s["islandSwitch"], map[string]any{"open": true, "progress": 0.4, "ready": false}) {
		t.Errorf("Event/Inselwechsel: %v %v", s["nightEvents"], s["islandSwitch"])
	}
}

func TestKampfDelta(t *testing.T) {
	w := kampfWelt(t)
	w.NightEvents = []sim.ActiveEvent{{ID: "fullMoon", SecondsLeft: 95.5}}
	prev := stateOf(w, 0, false)
	w.Enemies[0].HP, w.Enemies[0].Warn.In = 4100, 3.3
	w.NightEvents[0].SecondsLeft = 95.4
	d := asJSON(t, deltaOf(prev, stateOf(w, 0, false))).(map[string]any)
	if !reflect.DeepEqual(keysOf(d), []string{"enemies", "nightEvents"}) {
		t.Fatalf("Delta-Felder %v", keysOf(d))
	}
	set := d["enemies"].(map[string]any)["set"].([]any)
	if len(set) != 1 || set[0].(map[string]any)["hp"] != 4100.0 {
		t.Errorf("enemies.set: %v", set)
	}
	if !reflect.DeepEqual(d["nightEvents"], []any{map[string]any{"id": "fullMoon", "secondsLeft": 95.4}}) {
		t.Errorf("nightEvents: %v", d["nightEvents"])
	}

	// Event vorbei und Wechselpunkt offen: nightEvents fehlt (unset), islandSwitch kommt ganz.
	prev = stateOf(w, 0, false)
	w.NightEvents, w.IslandSwitch = nil, &sim.SwitchState{Open: true, Progress: 0.1}
	d = asJSON(t, deltaOf(prev, stateOf(w, 0, false))).(map[string]any)
	if !reflect.DeepEqual(d["unset"], []any{"nightEvents"}) || d["islandSwitch"] == nil {
		t.Errorf("unset/islandSwitch: %v %v", d["unset"], d["islandSwitch"])
	}
}

// Die Beispiele s2c-snapshot-boss.json und -event.json haben die Schlüssel des Zustands mit den Kampf-Feldern.
func TestKampfBeispiele(t *testing.T) {
	w := kampfWelt(t)
	w.NightEvents = []sim.ActiveEvent{{ID: "fullMoon", SecondsLeft: 95.5}}
	w.IslandSwitch = &sim.SwitchState{Open: true, Progress: 0.4}
	got := asJSON(t, stateMsg{T: "snap", S: stateOf(w, 0, false)}).(map[string]any)["s"].(map[string]any)
	boss := want(t, "snapshot-boss")["s"].(map[string]any)
	if d := sameKeys("boss.enemies", boss["enemies"], got["enemies"], true); d != "" {
		t.Error(d)
	}
	ev := want(t, "snapshot-event")["s"].(map[string]any)
	for _, k := range []string{"nightEvents", "islandSwitch"} {
		if d := sameKeys(k, ev[k], got[k], true); d != "" {
			t.Error(d)
		}
	}
}
