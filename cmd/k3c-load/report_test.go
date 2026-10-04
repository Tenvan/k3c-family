package main

import (
	"bytes"
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// AC-03: drei Bewertungen und Exit-Code (Ziel 10 ms).
func TestBewertungUndExitCode(t *testing.T) {
	for _, tc := range []struct {
		name string
		p99  []float64
		want string
		exit int
	}{
		{"alle unter Ziel", []float64{8, 9, 9.9}, erreicht, 0},
		{"Spitze über Ziel, Mittel darunter", []float64{8, 9, 10.5}, knapp, 0},
		{"Mittel über Ziel", []float64{10.2, 10.3, 11}, verfehlt, 1},
		{"genau Ziel zählt als nicht erreicht", []float64{10, 10, 10}, verfehlt, 1},
		{"keine Werte", nil, ohneDaten, 2},
	} {
		got := evaluate(tc.p99, 10)
		if got != tc.want || exitCode(got) != tc.exit {
			t.Errorf("%s: %s, Exit %d; erwartet %s, %d", tc.name, got, exitCode(got), tc.want, tc.exit)
		}
	}
	if worst(erreicht, verfehlt, knapp) != verfehlt || worst() != ohneDaten {
		t.Error("worst")
	}
}

// Zeilen je Raum und Phase, CPU nur wo vorhanden; Markdown und JSON enthalten sie.
func TestBerichtJeRaumUndPhase(t *testing.T) {
	cpu := func(v float64) *float64 { return &v }
	runs := []*roomRun{{Name: "test-a", Code: "ABCD", Samples: []sample{
		{AtS: 0, Phase: "day", Tick: 1, TickP99: 0}, // noch nicht getickt: zählt nicht
		{AtS: 5, Phase: "day", Tick: 100, TickP99: 4, CPU: cpu(20)},
		{AtS: 10, Phase: "day", Tick: 200, TickP99: 6, CPU: cpu(40)},
		{AtS: 15, Phase: "night", Tick: 300, TickP99: 12},
	}}}
	rep := buildReport(config{url: "http://x", seed: "s", players: 3, target: 10}, time.Unix(0, 0), runs)
	rows := rep.Rooms[0].Rows
	if len(rows) != 2 || rows[0].Phase != "day" || rows[0].Verdict != erreicht || rows[0].P99Mean != 5 ||
		*rows[0].CPUMean != 30 || *rows[0].CPUPeak != 40 || rows[1].Verdict != verfehlt || rows[1].CPUMean != nil {
		t.Fatalf("Zeilen: %+v", rows)
	}
	if rep.Verdict != verfehlt || exitCode(rep.Verdict) != 1 {
		t.Errorf("Gesamt %s", rep.Verdict)
	}
	checkFiles(t, rep)
}

func checkFiles(t *testing.T, rep report) {
	t.Helper()
	md := rep.markdown()
	if !strings.Contains(md, "| test-a | night | 1 | 12.00 | 12.00 | 12.00 | – | – | verfehlt |") {
		t.Errorf("Markdown:\n%s", md)
	}
	paths, err := rep.write(filepath.Join(t.TempDir(), "sub", "load-x"))
	if err != nil || len(paths) != 2 {
		t.Fatalf("write: %v %v", paths, err)
	}
	data, _ := os.ReadFile(paths[0])
	var back report
	if json.Unmarshal(data, &back) != nil || len(back.Rooms[0].Samples) != 4 || back.Verdict != verfehlt {
		t.Errorf("JSON: %s", data)
	}
}

// Nacht: erst wenn jeder Raum Nacht gesehen hat und wieder bei „day“ ist, ist die Messung zu Ende.
func TestNachtEnde(t *testing.T) {
	p := newPoller(nil, []*roomRun{{Name: "a"}, {Name: "b"}}, true)
	p.seen["a"], p.over["a"] = true, true
	p.seen["b"] = true
	if len(p.over) == len(p.runs) {
		t.Fatal("b ist noch in der Nacht")
	}
	p.over["b"] = true
	if len(p.over) != len(p.runs) {
		t.Fatal("beide vorbei")
	}
}

// Lauf gegen den In-Prozess-Server: Status-Proben mit Tick-Dauer und Phase landen im Bericht, Exit-Code folgt dem Ziel.
func TestLaufSchreibtBericht(t *testing.T) {
	srv, _, _, _ := testServer(t)
	out := filepath.Join(t.TempDir(), "load")
	var stdout, stderr bytes.Buffer
	args := []string{"-url", srv.URL, "-token", testToken, "-rooms", "1", "-players", "2", "-duration", "3s", "-interval", "500ms", "-target-p99", "10000", "-out", out}
	code := run(context.Background(), args, func(string) string { return "" }, &stdout, &stderr)
	if code != 0 {
		t.Fatalf("Exit %d, stderr: %s\nstdout: %s", code, stderr.String(), stdout.String())
	}
	var rep report
	data, err := os.ReadFile(out + ".json")
	if err != nil || json.Unmarshal(data, &rep) != nil || len(rep.Rooms) != 1 || len(rep.Rooms[0].Samples) < 3 {
		t.Fatalf("Bericht: %v %s", err, data)
	}
	s := rep.Rooms[0].Samples[len(rep.Rooms[0].Samples)-1]
	if s.Phase == "" || s.Tick == 0 {
		t.Errorf("letzte Probe ohne Phase oder Tick: %+v", s)
	}
	if _, err := os.Stat(out + ".md"); err != nil || strings.Contains(string(data)+stdout.String(), testToken) {
		t.Errorf("Markdown fehlt oder Token im Bericht: %v", err)
	}
	// Ziel unerreichbar niedrig: verfehlt → Exit 1 (Messwerte stehen trotzdem im Bericht).
	args = append(args[:len(args)-4], "-target-p99", "0.000001", "-out", out+"2")
	if code := run(context.Background(), args, func(string) string { return "" }, &stdout, &stderr); code != 1 {
		t.Errorf("verfehltes Ziel: Exit %d, stderr %s", code, stderr.String())
	}
}
