package net

import (
	"encoding/json"
	"os"
	"reflect"
	"testing"

	"k3c/engine/level"
	"k3c/engine/sim"
)

// apply wendet ein Delta an, so wie es ein Client tut (docs/protocol.md › Zustand und Delta).
func apply(prev, d map[string]any) map[string]any {
	next := map[string]any{}
	for k, v := range prev {
		next[k] = v
	}
	next["events"] = []any{}
	for k, v := range d {
		if !idLists[k] {
			next[k] = v
			continue
		}
		ch := v.(map[string]any)
		del := map[any]bool{}
		for _, id := range ch["del"].([]any) {
			del[id] = true
		}
		set := map[any]any{}
		for _, e := range ch["set"].([]any) {
			set[e.(map[string]any)["id"]] = e
		}
		list := []any{}
		for _, e := range asList(prev[k]) {
			id := e.(map[string]any)["id"]
			if del[id] {
				continue
			}
			if n, ok := set[id]; ok {
				e = n
				delete(set, id)
			}
			list = append(list, e)
		}
		for _, e := range ch["set"].([]any) { // neue Einträge in ihrer Reihenfolge hinten an
			if _, ok := set[e.(map[string]any)["id"]]; ok {
				list = append(list, e)
			}
		}
		next[k] = list
	}
	return next
}

// roundtrip wie über die Leitung: JSON hin und zurück.
func wire(t *testing.T, v any) map[string]any {
	t.Helper()
	raw, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	var m map[string]any
	if err := json.Unmarshal(raw, &m); err != nil {
		t.Fatal(err)
	}
	return m
}

// replayWorld spielt die Eingaben eines Golden-Laufs ab und ruft each für jeden Tick ab from auf.
func replayWorld(t *testing.T, file string, from int, each func(tick int, w *sim.World)) {
	t.Helper()
	raw, err := os.ReadFile("../../testdata/golden/" + file)
	var run struct {
		Biome, Seed string
		CycleSpeed  float64
		Players     int
		Inputs      []struct {
			Ticks    int
			Commands []sim.PlayerCommand
		}
	}
	if err == nil {
		err = json.Unmarshal(raw, &run)
	}
	b, berr := level.LoadBiome(run.Biome)
	if err != nil || berr != nil {
		t.Fatal(err, berr)
	}
	w, err := sim.CreateWorld(b, run.Seed, sim.Options{CycleSpeed: run.CycleSpeed})
	if err != nil {
		t.Fatal(err)
	}
	for range run.Players {
		sim.AddPlayer(w)
	}
	tick := 0
	for _, seg := range run.Inputs {
		for range seg.Ticks {
			sim.Step(w, seg.Commands, 1.0/30)
			if tick++; tick >= from {
				each(tick, w)
			}
		}
	}
}

// Belagerung der Höhle: Gegner, Pfeile, Münzen, tote Truppen und der Fall der Burg.
func TestDeltaErgibtJedenVollenZustand(t *testing.T) {
	var client, prev map[string]any
	changes := map[string]int{}
	replayWorld(t, "sim-cave-belagerung.json", 2400, func(tick int, w *sim.World) {
		cur := stateOf(w, 0, false)
		if prev == nil {
			client, prev = wire(t, cur), cur
			return
		}
		d := deltaOf(prev, cur)
		for k := range d {
			changes[k]++
		}
		client = apply(client, wire(t, d))
		if want := wire(t, cur); !reflect.DeepEqual(client, want) {
			for k := range want {
				if !reflect.DeepEqual(client[k], want[k]) {
					t.Fatalf("Tick %d: Feld %s weicht nach dem Delta ab", tick, k)
				}
			}
			t.Fatalf("Tick %d: Zustand weicht ab", tick)
		}
		prev = cur
	})
	for _, k := range []string{"players", "coins", "troops", "enemies", "projectiles", "events", "cycle", "castle"} {
		if changes[k] == 0 {
			t.Errorf("Feld %s hat sich im Lauf nie geändert, der Test deckt es nicht ab", k)
		}
	}
}

func TestDeltaOhneAenderungIstLeer(t *testing.T) {
	c := sim.CreateCampaign("leer", "l", 1)
	s := stateOf(c.CurrentWorld(), 0, false)
	if d := deltaOf(s, stateOf(c.CurrentWorld(), 0, false)); len(d) != 0 {
		t.Fatalf("Delta ohne Änderung: %v", d)
	}
}
