package net

import (
	"encoding/json"
	"os"
	"testing"

	"k3c/engine/level"
	"k3c/engine/room"
	"k3c/engine/sim"
)

// WZ14.5 (B-468/AC-05): sites[].ruin steht in snap und delta nur an Ruinen der Planung (testdata/planned/forest-ruin.json),
// die Vorschau liefert sie mit.

// ruinPlan ist die Planung der Fixture testdata/planned/forest-ruin.json.
func ruinPlan(t *testing.T) []level.PlannedChunk {
	t.Helper()
	raw, err := os.ReadFile("../../testdata/planned/forest-ruin.json")
	if err != nil {
		t.Fatal(err)
	}
	var f struct{ Planned []level.PlannedChunk }
	if err := json.Unmarshal(raw, &f); err != nil {
		t.Fatal(err)
	}
	return f.Planned
}

// ruinFlags liefert je Platz (kind@x) das Feld ruin aus einer Liste sites des Zustands; fehlt es, steht false da.
func ruinFlags(t *testing.T, sites any) map[string]bool {
	t.Helper()
	out := map[string]bool{}
	for _, e := range sites.([]any) {
		s := e.(map[string]any)
		key := room.PreviewSiteKey(s["kind"].(string), s["x"].(float64))
		r, has := s["ruin"]
		if has && r != true {
			t.Fatalf("%s: ruin %v, erwartet true oder fehlend", key, r)
		}
		out[key] = has
	}
	return out
}

// wantRuins prüft, dass genau die Ruinen des Levels ruin tragen.
func wantRuins(t *testing.T, what string, flags map[string]bool, ruins []level.Ruin) {
	t.Helper()
	n := 0
	for _, on := range flags {
		if on {
			n++
		}
	}
	for _, r := range ruins {
		if !flags[room.PreviewSiteKey(r.Kind, r.X)] {
			t.Errorf("%s: Ruine %s@%v ohne ruin", what, r.Kind, r.X)
		}
	}
	if n != len(ruins) {
		t.Errorf("%s: %d Plätze mit ruin, erwartet %d", what, n, len(ruins))
	}
}

// roundTrip ist der Zustand so, wie ihn ein Gerät liest (JSON hin und zurück).
func roundTrip(t *testing.T, v any) map[string]any {
	t.Helper()
	raw, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	var out map[string]any
	if err := json.Unmarshal(raw, &out); err != nil {
		t.Fatal(err)
	}
	return out
}

func TestRuineInSnapUndDelta(t *testing.T) {
	isl, err := sim.CreateIsland("ruine-1", []int{0}, 1, sim.CreateOptions{Planned: map[int][]level.PlannedChunk{0: ruinPlan(t)}})
	if err != nil {
		t.Fatal(err)
	}
	sim.AddIslandPlayer(isl, 0)
	sim.AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	if len(w.Level.Ruins) != 3 {
		t.Fatalf("Ruinen im Level: %+v", w.Level.Ruins)
	}
	prev := roundTrip(t, stateOf(w, 0, false))
	wantRuins(t, "snap", ruinFlags(t, prev["sites"]), w.Level.Ruins)

	var ruin, plain *sim.Site
	for _, s := range w.Sites {
		switch {
		case s.Ruin && ruin == nil:
			ruin = s
		case !s.Ruin && plain == nil:
			plain = s
		}
	}
	ruin.HP--
	plain.PaidGold++
	d := deltaOf(prev, roundTrip(t, stateOf(w, 0, false)))
	set := ruinFlags(t, d["sites"].(map[string]any)["set"])
	if len(set) != 2 || !set[room.PreviewSiteKey(ruin.Kind, ruin.X)] || set[room.PreviewSiteKey(plain.Kind, plain.X)] {
		t.Errorf("delta sites.set: %v, erwartet die Ruine mit und den Bauplatz ohne ruin", set)
	}
}

func TestRuineInDerVorschau(t *testing.T) {
	o := room.PreviewOptions{Seed: "ruine-1", Source: room.PreviewSource{Biome: "forest"}, Planned: ruinPlan(t)}
	res, err := Preview(o)
	if err != nil {
		t.Fatal(err)
	}
	wantRuins(t, "Vorschau", ruinFlags(t, roundTrip(t, res.State)["sites"]), mustRuins(t, o))
	o.Planned = nil
	if res, err = Preview(o); err != nil {
		t.Fatal(err)
	}
	wantRuins(t, "Vorschau ohne Planung", ruinFlags(t, roundTrip(t, res.State)["sites"]), nil)
}

// mustRuins sind die Ruinen des Vorschau-Levels.
func mustRuins(t *testing.T, o room.PreviewOptions) []level.Ruin {
	t.Helper()
	w, err := room.BuildPreview(o)
	if err != nil {
		t.Fatal(err)
	}
	if len(w.Level.Ruins) == 0 {
		t.Fatal("Vorschau ohne Ruinen")
	}
	return w.Level.Ruins
}

// Der Dev-Befehl castleFall (c2s-dev-castlefall.json) wird im Dev-Mode angenommen; ohne slot ist er bad_request.
func TestDevBurgfallUeberProtokoll(t *testing.T) {
	srv, m := devServer(t)
	x := hello(t, srv, "xbox")
	create(x, "dev", 0)
	r := m.Room(x.entered()["room"].(string))
	x.send(devExamples(t)[4])
	x.send(map[string]any{"t": "input", "seq": 3, "p": []map[string]any{{"slot": 0}}})
	waitFor(t, func() bool { r.Tick(); return x.expect("delta", "seats")["ack"] == float64(3) })
	x.send(map[string]any{"t": "dev", "action": "castleFall"})
	x.expectError("bad_request")
}
