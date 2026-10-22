package net

import (
	"encoding/json"
	"testing"

	"k3c/engine/level"
	"k3c/engine/sim"
)

// BD2.2 (B-377): Die Rückmeldungs-Ereignisse der Simulation haben die Schlüssel ihres Beispiels
// testdata/protocol/s2c-snapshot-rueckmeldung.json (Monarch-strike mit und ohne Treffer, castFailed).

// feedbackEvents erzeugt in einer frischen Welt einen Tick mit cmd und liefert die Ereignisse des Monarchen als JSON-Objekte.
func feedbackEvents(t *testing.T, typ string, setup func(w *sim.World, p *sim.Player), cmd sim.PlayerCommand) []map[string]any {
	t.Helper()
	b, err := level.LoadBiome("forest")
	if err != nil {
		t.Fatal(err)
	}
	w, err := sim.CreateWorld(b, "rueckmeldung", sim.Options{CycleSpeed: 1})
	if err != nil {
		t.Fatal(err)
	}
	p := sim.AddPlayer(w)
	w.Enemies = nil
	setup(w, p)
	sim.Step(w, []sim.PlayerCommand{cmd}, 1.0/30)
	var out []map[string]any
	for _, ev := range w.Events {
		raw, err := json.Marshal(ev)
		if err != nil {
			t.Fatal(err)
		}
		var m map[string]any
		if err := json.Unmarshal(raw, &m); err != nil {
			t.Fatal(err)
		}
		if m["type"] == typ && m["from"] == float64(p.ID) {
			out = append(out, m)
		}
	}
	return out
}

func TestRueckmeldungEreignisseWieBeispiel(t *testing.T) {
	examples := map[string]map[string]any{}
	for _, ev := range want(t, "snapshot-rueckmeldung")["s"].(map[string]any)["events"].([]any) {
		m := ev.(map[string]any)
		key := m["type"].(string)
		if hit, ok := m["hit"].(bool); ok {
			key += map[bool]string{true: ":hit", false: ":miss"}[hit]
		}
		examples[key] = m
	}
	nothing := func(*sim.World, *sim.Player) {}
	target := func(w *sim.World, p *sim.Player) {
		w.Enemies = append(w.Enemies, &sim.Enemy{ID: 9001, Kind: "goblin", X: p.X + 1, HP: 1e6, MaxHP: 1e6})
	}
	skill := func(w *sim.World, p *sim.Player) { p.Slots, p.Skills = []string{"fireball"}, []string{"fireball"} }
	for _, c := range []struct {
		key, typ string
		setup    func(*sim.World, *sim.Player)
		cmd      sim.PlayerCommand
	}{
		{"strike:miss", "strike", nothing, sim.PlayerCommand{Attack: true}},
		{"strike:hit", "strike", target, sim.PlayerCommand{Attack: true}},
		{"castFailed", "castFailed", skill, sim.PlayerCommand{Skill: 1}},
	} {
		want, ok := examples[c.key]
		if !ok {
			t.Fatalf("%s: fehlt im Beispiel", c.key)
		}
		evs := feedbackEvents(t, c.typ, c.setup, c.cmd)
		if len(evs) != 1 {
			t.Fatalf("%s: %d Ereignisse aus der Simulation, erwartet 1", c.key, len(evs))
		}
		if d := sameKeys(c.key, want, evs[0], false); d != "" {
			t.Errorf("%s", d)
		}
		if hit, ok := want["hit"].(bool); ok && evs[0]["hit"] != hit {
			t.Errorf("%s: hit %v, erwartet %v", c.key, evs[0]["hit"], hit)
		}
	}
}
