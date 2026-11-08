package balance

import (
	"encoding/json"
	"strings"
	"testing"

	"k3c/engine/sim"
)

// recorded spielt einen sparsamen Lauf (Seed 1, 2 Spieler, 1 Tag) und liefert seine Aufnahme als Datei.
func recorded(t *testing.T) (Result, []byte) {
	t.Helper()
	res := runOne(Scenario{Seed: "1", Players: 2, Bot: "saver", Depth: 0, Days: 1}, 0)
	if !res.Valid || res.Replay == nil {
		t.Fatalf("Lauf ungültig: %s", res.Error)
	}
	b, err := res.Replay.JSON()
	if err != nil {
		t.Fatal(err)
	}
	return res, b
}

func mustRead(t *testing.T, b []byte) Replay {
	t.Helper()
	r, err := ReadReplay(b)
	if err != nil {
		t.Fatal(err)
	}
	return r
}

// BAL1/AC-03: Aufnahme eines Bot-Laufs und Wiedergabe aus der Datei → gleicher Endzustand-Hash und Burgfall-Tick.
func TestAufnahmeUndWiedergabeGleicherHash(t *testing.T) {
	res, b := recorded(t)
	pb, err := Play(mustRead(t, b))
	if err != nil {
		t.Fatal(err)
	}
	if len(pb.Warnings) > 0 {
		t.Errorf("unerwartete Warnungen: %v", pb.Warnings)
	}
	if pb.EndHash == "" || pb.EndHash != res.Replay.EndHash {
		t.Errorf("Endzustand-Hash: Wiedergabe %q, Aufnahme %q", pb.EndHash, res.Replay.EndHash)
	}
	if pb.Ticks != res.Ticks || deref(pb.CastleFallTick) != deref(res.Metrics.CastleFallTick) {
		t.Errorf("Ticks/Burgfall: Wiedergabe %d/%v, Aufnahme %d/%v", pb.Ticks, deref(pb.CastleFallTick), res.Ticks, deref(res.Metrics.CastleFallTick))
	}
}

func deref(p *int) any {
	if p == nil {
		return nil
	}
	return *p
}

// Die Lauflängen ergeben bei der Wiedergabe exakt dieselben Befehle je Tick.
func TestLauflaengenExakt(t *testing.T) {
	ticks := [][]sim.PlayerCommand{
		{{MoveX: 1}, {}}, {{MoveX: 1}, {Pay: true}}, {{}, {Pay: true}}, {{Sprint: true, MoveX: -1}, {}},
	}
	var r Replay
	for _, cmds := range ticks {
		r.record(cmds)
	}
	for i := range 2 {
		var got []sim.PlayerCommand
		for _, s := range r.Inputs[i] {
			for range s.N {
				got = append(got, s.PlayerCommand)
			}
		}
		for tick, c := range got {
			if c != ticks[tick][i] {
				t.Errorf("Spieler %d Tick %d: %+v, erwartet %+v", i, tick, c, ticks[tick][i])
			}
		}
		if len(got) != len(ticks) || r.Ticks != len(ticks) {
			t.Errorf("Spieler %d: %d Ticks, erwartet %d", i, len(got), len(ticks))
		}
	}
	if len(r.Inputs[0]) != 3 || len(r.Inputs[1]) != 3 {
		t.Errorf("Lauflängen nicht zusammengefasst: %+v", r.Inputs)
	}
}

// BAL1/AC-04: Die Datei enthält Version, Seed, Parameter, Datenstand-Hash und Eingaben.
func TestDateiEnthaeltSeedParameterHashEingaben(t *testing.T) {
	_, b := recorded(t)
	var f map[string]any
	if err := json.Unmarshal(b, &f); err != nil {
		t.Fatal(err)
	}
	want := map[string]any{"version": float64(ReplayVersion), "seed": "1", "players": float64(2), "bot": "saver",
		"depth": float64(0), "days": float64(1), "dataHash": DataHash()}
	for k, v := range want {
		if f[k] != v {
			t.Errorf("%s = %v, erwartet %v", k, f[k], v)
		}
	}
	if in, _ := f["inputs"].([]any); len(in) != 2 {
		t.Errorf("inputs: %v, erwartet je Spieler eine Liste", f["inputs"])
	}
	if len(DataHash()) != 64 {
		t.Errorf("Datenstand-Hash %q ist kein SHA-256", DataHash())
	}
}

// BAL1/AC-05: unbekannte Version → Fehler mit Versionsangabe; kaputtes JSON → Fehler mit Zeilennummer.
func TestUnbekannteVersionUndKaputteDatei(t *testing.T) {
	_, b := recorded(t)
	_, err := ReadReplay([]byte(strings.Replace(string(b), `"version": 1`, `"version": 99`, 1)))
	if err == nil || !strings.Contains(err.Error(), "Version 99") {
		t.Errorf("Version 99: Fehler %v, erwartet Versionsangabe", err)
	}
	_, err = ReadReplay([]byte("{\n  \"version\": 1,\n  \"seed\": 1\n}"))
	if err == nil || !strings.Contains(err.Error(), "Zeile 3") {
		t.Errorf("Typfehler: %v, erwartet Zeile 3", err)
	}
	_, err = ReadReplay([]byte("{\n  \"version\": 1\n  \"seed\": \"1\"\n}"))
	if err == nil || !strings.Contains(err.Error(), "Zeile 3") {
		t.Errorf("Syntaxfehler: %v, erwartet Zeile 3", err)
	}
}

// BAL1/AC-05: anderer Datenstand-Hash → Warnung, die Wiedergabe läuft trotzdem durch.
func TestAndererDatenstandWarnt(t *testing.T) {
	res, b := recorded(t)
	r := mustRead(t, b)
	r.DataHash = "anders"
	pb, err := Play(r)
	if err != nil {
		t.Fatal(err)
	}
	if len(pb.Warnings) != 1 || !strings.Contains(pb.Warnings[0], "Datenstand") {
		t.Errorf("Warnungen %v, erwartet eine zum Datenstand", pb.Warnings)
	}
	if pb.EndHash != res.Replay.EndHash {
		t.Error("Wiedergabe mit Warnung lief nicht bis zum Ende")
	}
}

// Riesige Tick-Zahl aus einer Datei → Fehler statt endloser Wiedergabe.
func TestZuvieleTicksAbgelehnt(t *testing.T) {
	_, err := ReadReplay([]byte(`{"version":1,"players":1,"ticks":2000000000,"inputs":[[{"n":2000000000}]]}`))
	if err == nil || !strings.Contains(err.Error(), "höchstens") {
		t.Errorf("Fehler %v, erwartet Tick-Obergrenze", err)
	}
}
