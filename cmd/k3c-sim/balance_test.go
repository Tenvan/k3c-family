package main

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/engine/balance"
)

// Übernommen aus tools/k3c-workbench/internal/balance/cmd (bin/k3c-balance), Fortschritt jetzt als JSON-Zeile.

// Verletzte Ziele sind kein Fehler (Exit 0, BAL2.3): Der Bericht entsteht.
func TestVerletztesZielIstKeinFehler(t *testing.T) {
	if testing.Short() {
		t.Skip("langer Test, läuft ohne -short (B-619)")
	}
	dir := t.TempDir()
	if code, _, errOut := call(t, "", "balance", "--targets", "--seeds", "3", "--report-dir", dir); code != exitOK {
		t.Fatalf("Bericht ohne Baseline: Exit %d %s", code, errOut)
	}
	md, _ := filepath.Glob(filepath.Join(dir, "balance-*.md"))
	if len(md) != 1 {
		t.Fatalf("erwartet einen Markdown-Bericht, gefunden %v", md)
	}
}

// Ein Fehler des Werkzeugs selbst ist Exit 1, ein falscher Aufruf Exit 2.
func TestWerkzeugFehlerIstFehler(t *testing.T) {
	if testing.Short() {
		t.Skip("langer Test, läuft ohne -short (B-619)")
	}
	if code, _, _ := call(t, "", "balance", "--targets", "--seeds", "1", "--write-baseline"); code != exitUsage {
		t.Fatalf("--write-baseline ohne --baseline: Exit %d, erwartet %d", code, exitUsage)
	}
	datei := filepath.Join(t.TempDir(), "datei")
	if err := os.WriteFile(datei, nil, 0o644); err != nil {
		t.Fatal(err)
	}
	if code, _, _ := call(t, "", "balance", "--targets", "--seeds", "1", "--report-dir", datei); code != exitFailed {
		t.Fatalf("Berichtsordner, der eine Datei ist: Exit %d, erwartet %d", code, exitFailed)
	}
}

// WZ28.4: --progress schreibt progress-Zeilen, --vary und die Kennzahlen stehen im Bericht, die Berichtsdateien
// meldet eine report-Zeile; Fehlbedienung ist ein Fehler.
func TestFortschrittVarianteImBericht(t *testing.T) {
	dir := t.TempDir()
	code, out, errOut := call(t, "", "balance", "--targets", "--seeds", "1", "--days", "1", "--grade", "normal", "--progress",
		"--vary", "economy.json:purse.startGold=120", "--report-dir", dir)
	if code != exitOK {
		t.Fatalf("Exit %d: %s", code, errOut)
	}
	if !strings.Contains(out, `{"event":"progress","done":1,"total":1}`+"\n") {
		t.Errorf("Fortschritt fehlt: %q", out)
	}
	requireJSONLines(t, out)
	js, _ := filepath.Glob(filepath.Join(dir, "balance-*.json"))
	if len(js) != 1 || !strings.Contains(out, `"event":"report"`) {
		t.Fatalf("Bericht: %v, stdout %q", js, out)
	}
	raw, _ := os.ReadFile(js[0])
	s, err := balance.ReadSummary(raw)
	if err != nil || len(s.Varianten) != 1 || s.Kennzahlen == nil || len(s.Kennzahlen.Nights) != 1 {
		t.Fatalf("%v %+v", err, s)
	}
	for _, args := range [][]string{
		{"balance", "--targets", "--seeds", "1", "--vary", "economy.json:purse.gibtsnicht=1", "--report-dir", dir},
		{"balance", "--vary", "economy.json:purse.startGold=1"},
		{"balance", "--targets", "--players", "1,2", "--report-dir", dir},
	} {
		if code, _, _ := call(t, "", args...); code == exitOK {
			t.Errorf("%v: Fehler erwartet", args)
		}
	}
}

// WZ28.6: --tune sweep schreibt tuning-<Zeit>.json mit Punkten, Begründung und Fortschritt; Fehlbedienung ist ein Fehler.
func TestTuneSchreibtErgebnis(t *testing.T) {
	dir := t.TempDir()
	code, out, errOut := call(t, "", "balance", "--targets", "--seeds", "1", "--days", "1", "--grade", "normal", "--progress",
		"--tune", "sweep", "--steps", "2", "--param", "economy.json:purse.startGold=8..20", "--report-dir", dir)
	if code != exitOK {
		t.Fatalf("Exit %d: %s", code, errOut)
	}
	if !strings.Contains(out, `{"event":"progress","done":2,"total":2}`+"\n") {
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
		{"balance", "--tune", "sweep", "--param", "economy.json:purse.startGold=1..2"},
		{"balance", "--targets", "--param", "economy.json:purse.startGold=1..2", "--report-dir", dir},
		{"balance", "--targets", "--tune", "raten", "--param", "economy.json:purse.startGold=1..2", "--report-dir", dir},
		{"balance", "--targets", "--tune", "sweep", "--report-dir", dir},
	} {
		if code, _, _ := call(t, "", args...); code == exitOK {
			t.Errorf("%v: Fehler erwartet", args)
		}
	}
}
