package balance

import (
	"bytes"
	"encoding/json"
	"flag"
	"os"
	"path/filepath"
	"testing"
)

// write schreibt die Vergleichsdateien für den Rust-Port neu (RU28.1, Muster engine/internal/detvec).
var write = flag.Bool("write", false, "Vergleichsdateien unter testdata/rust/balance neu schreiben")

const rustRefDir = "../../testdata/rust/balance"

// rustRefMatrices sind die festen kurzen Läufe, die der Rust-Port (`k3c-balance`, `rustref_tests.rs`) nachrechnet:
// ein Tag, kein Balancing-Lauf (B-1014).
var rustRefMatrices = map[string]Matrix{
	"matrix-saver-1.json":      {Seeds: []string{"1"}, Players: []int{1}, Bots: []string{"saver"}, Depths: []int{0}, Days: 1},
	"matrix-wirtschaft-2.json": {Seeds: []string{"1"}, Players: []int{2}, Bots: []string{"wirtschaft"}, Depths: []int{0}, Days: 1},
}

// TestRustRef schreibt mit -write die Berichte der festen Matrizen; ohne -write vergleicht er einen neuen Lauf mit den Dateien.
func TestRustRef(t *testing.T) {
	for name, m := range rustRefMatrices {
		t.Run(name, func(t *testing.T) {
			got, err := mustRun(t, m).JSON()
			if err != nil {
				t.Fatal(err)
			}
			path := filepath.Join(rustRefDir, name)
			if *write {
				if err := os.WriteFile(path, got, 0o644); err != nil {
					t.Fatal(err)
				}
				return
			}
			want, err := os.ReadFile(path)
			if err != nil {
				t.Fatalf("%v (schreiben mit -write)", err)
			}
			if string(want) != string(got) {
				t.Fatalf("%s weicht von einem neuen Lauf ab (neu schreiben mit -write, nur bei gewollter Änderung)", name)
			}
		})
	}
}

// TestRustRefSummary schreibt mit -write die Zusammenfassung aus dem echten Bericht simtest-run-7 (kein Sim-Lauf): JSON
// nach ReadSummary und Markdown ohne Auswertung; ohne -write vergleicht er mit den Dateien (RU28.3).
func TestRustRefSummary(t *testing.T) {
	raw, err := os.ReadFile(filepath.Join(rustRefDir, "insights-run7.json"))
	if err != nil {
		t.Fatal(err)
	}
	s, err := ReadSummary(raw)
	if err != nil {
		t.Fatal(err)
	}
	js, err := s.JSON()
	if err != nil {
		t.Fatal(err)
	}
	for name, got := range map[string]string{"summary-run7.json": string(js), "summary-run7.md": s.MarkdownWith(nil, nil)} {
		path := filepath.Join(rustRefDir, name)
		if *write {
			if err := os.WriteFile(path, []byte(got), 0o644); err != nil {
				t.Fatal(err)
			}
			continue
		}
		want, err := os.ReadFile(path)
		if err != nil {
			t.Fatalf("%v (schreiben mit -write)", err)
		}
		if string(want) != got {
			t.Fatalf("%s weicht ab (neu schreiben mit -write, nur bei gewollter Änderung)", name)
		}
	}
}

// TestRustRefInsights schreibt mit -write die Auswertung (BuildInsights) aus dem echten Bericht simtest-run-7 (kein
// Sim-Lauf), einmal ohne und einmal mit Baseline (Trend, Vergleich, Delta); ohne -write vergleicht er mit den Dateien (RU31.1).
// Generated bleibt leer, die Ortszeit wäre je Rechner verschieden.
func TestRustRefInsights(t *testing.T) {
	raw, err := os.ReadFile(filepath.Join(rustRefDir, "insights-run7.json"))
	if err != nil {
		t.Fatal(err)
	}
	cur, err := ReadSummary(raw)
	if err != nil {
		t.Fatal(err)
	}
	base, err := ReadSummary(raw)
	if err != nil {
		t.Fatal(err)
	}
	for i := range base.Targets {
		if base.Targets[i].Measure == "goldAtDawnPerPlayer" {
			base.Targets[i].Value, base.Targets[i].Status, base.Targets[i].Pass = 22, StatusOK, true
		}
	}
	h, err := DefaultHints()
	if err != nil {
		t.Fatal(err)
	}
	cases := map[string]Insights{
		"insights-run7-out.json":   BuildInsights(cur, nil, h, Meta{Run: "simtest-run-7"}),
		"insights-run7-trend.json": BuildInsights(cur, &base, h, Meta{Run: "simtest-run-7", Baseline: "b.json"}),
	}
	for name, in := range cases {
		got, err := json.MarshalIndent(in, "", "  ")
		if err != nil {
			t.Fatal(err)
		}
		got = append(got, '\n')
		path := filepath.Join(rustRefDir, name)
		if *write {
			if err := os.WriteFile(path, got, 0o644); err != nil {
				t.Fatal(err)
			}
			continue
		}
		want, err := os.ReadFile(path)
		if err != nil {
			t.Fatalf("%v (schreiben mit -write)", err)
		}
		if !bytes.Equal(bytes.ReplaceAll(want, []byte("\r\n"), []byte("\n")), got) { // Windows-Checkout mit CRLF
			t.Fatalf("%s weicht ab (neu schreiben mit -write, nur bei gewollter Änderung)", name)
		}
	}
}
