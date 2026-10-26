package main

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/tools/k3c-workbench/internal/balance"
)

// WZ28.6: --tune sweep schreibt tuning-<Zeit>.json mit Punkten, Begründung und Fortschritt; Fehlbedienung ist ein Fehler.
func TestTuneSchreibtErgebnis(t *testing.T) {
	dir := t.TempDir()
	out := captureStdout(t, func() error {
		return run([]string{"--targets", "--seeds", "1", "--days", "1", "--grade", "normal", "--progress", "--tune", "sweep", "--steps", "2",
			"--param", "economy.json:purse.startGold=8..20", "--report-dir", dir})
	})
	if !strings.Contains(out, "PROGRESS 2/2\n") {
		t.Errorf("Fortschritt: %q", out)
	}
	files, _ := filepath.Glob(filepath.Join(dir, "tuning-*.json"))
	if len(files) != 1 {
		t.Fatalf("ergebnis: %v", files)
	}
	raw, _ := os.ReadFile(files[0])
	var tn balance.Tuning
	if err := json.Unmarshal(raw, &tn); err != nil || tn.Mode != "sweep" || len(tn.Points) != 2 || tn.Reason == "" ||
		len(tn.Params) != 1 || tn.Params[0].Base != 14 {
		t.Fatalf("%v %+v", err, tn)
	}
	for _, args := range [][]string{
		{"--tune", "sweep", "--param", "economy.json:purse.startGold=1..2"},
		{"--targets", "--param", "economy.json:purse.startGold=1..2", "--report-dir", dir},
		{"--targets", "--tune", "raten", "--param", "economy.json:purse.startGold=1..2", "--report-dir", dir},
		{"--targets", "--tune", "sweep", "--report-dir", dir},
	} {
		if err := run(args); err == nil {
			t.Errorf("%v: Fehler erwartet", args)
		}
	}
}
