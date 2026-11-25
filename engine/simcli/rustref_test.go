package simcli

import (
	"bytes"
	"encoding/json"
	"flag"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"k3c/data"
)

// Vergleichsdateien für das Rust-Spiel (RU33.1, Muster engine/internal/detvec): Je Fall steht Aufruf, Eingabe, Exit-Code und
// die JSON-Zeilen von k3c-sim unter testdata/rust/runs/<fall>.json. rust/crates/k3c-game/tests/runs_ref.rs ruft die Läufe
// des Spiels im Prozess auf und vergleicht Zeile für Zeile. Neu schreiben nur bei gewollter Änderung der Schnittstelle:
// go test ./engine/simcli/ -run RustRef -write

var writeRustRef = flag.Bool("write", false, "Vergleichsdateien unter testdata/rust/runs neu schreiben")

const rustRefDir = "../../testdata/rust/runs"

// rustRefCase ist eine Vergleichsdatei.
type rustRefCase struct {
	Pre   []string `json:"pre,omitempty"` // Vorlauf (RU40.1): schreibt Dateien nach {tmp}; seine Ausgabe zählt nicht
	Args  []string `json:"args"`
	Stdin string   `json:"stdin"`
	Code  int      `json:"code"`
	Out   []string `json:"out"`
}

// rustRefCases sind die Aufrufe; kurze Testläufe mit festen Eingaben (Sim-Sperre), kein Balancing.
func rustRefCases(t *testing.T) map[string]rustRefCase {
	t.Helper()
	return map[string]rustRefCase{
		"world-level-standard":  {Args: []string{"world"}, Stdin: `{"op":"level","world":"standard","island":"island1","stage":0,"seed":"k3c"}`},
		"world-values-standard": {Args: []string{"world"}, Stdin: `{"op":"values","world":"standard"}`},
		"world-check-standard":  {Args: []string{"world"}, Stdin: worldCheckInput(t)},
		"world-values-nil":      {Args: []string{"world"}, Stdin: `{"op":"values","world":"nil"}`},
		"world-check-nil":       {Args: []string{"world"}, Stdin: worldDraftInput(t, "nil")},
		"world-unbekannte-op":   {Args: []string{"world"}, Stdin: `{"op":"fly"}`},
		"hub-plan-hub7":         {Args: []string{"hub"}, Stdin: hubPlanInput(t)},
		"hub-unbekannte-op":     {Args: []string{"hub"}, Stdin: `{"op":"fly"}`},
		"run-wald-300":          {Args: []string{"run"}, Stdin: `{"biome":"forest","seed":"0","ticks":300}`},
		"run-zwei-spieler": {Args: []string{"run"}, Stdin: `{"biome":"forest","seed":"k3c-dev","ticks":900,` +
			`"inputs":[{"fromTick":0,"toTick":600,"moveX":1,"pay":true},{"player":1,"fromTick":30,"toTick":900,"moveX":-1}]}`},
		"run-zu-viele-ticks":      {Args: []string{"run"}, Stdin: `{"biome":"forest","seed":"0","ticks":100001}`},
		"level-forest-0":          {Args: []string{"level"}, Stdin: `{"biome":"forest","seed":"0"}`},
		"level-cave-familie":      {Args: []string{"level"}, Stdin: `{"biome":"cave","seed":"familie-2026"}`},
		"aufruf-ohne-unterbefehl": {Args: []string{}},
		"aufruf-unbekannt":        {Args: []string{"bogus"}},
		"run-unbekanntes-feld":    {Args: []string{"run"}, Stdin: `{"biome":"forest","seed":"0","ticks":10,"tempo":3}`},
		// RU40.1: balance, kurze Läufe mit festen Eingaben (ein Seed, ein Tag, kein Balancing-Lauf)
		"balance-matrix-klein": {Args: []string{"balance", "--input", "-"},
			Stdin: `{"seed-list":"k3c-dev","bots":"saver","players":"1","depths":"0","days":1}`},
		"balance-json-flags": {Args: []string{"balance", "--input", "-", "--days", "1"},
			Stdin: `{"seeds":1,"players":"1","bots":["saver"],"depths":"0","days":3,"gold-threshold":30000}`},
		"balance-play": {Pre: []string{"balance", "--seed-list", "k3c-dev", "--players", "1", "--bots", "saver", "--depths", "0",
			"--days", "1", "--out", "{tmp}/matrix.json", "--replay-dir", "{tmp}"},
			Args: []string{"balance", "--play", "{tmp}/k3c-dev-p1-saver-d0.replay.json"}},
		"balance-flag-falsch":       {Args: []string{"balance", "--gibtsnicht"}},
		"balance-vary-ohne-targets": {Args: []string{"balance", "--vary", "economy.json:purse.startGold=1"}},
	}
}

// worldCheckInput ist der Entwurf der Standard-Welt mit einem passenden Override (wie TestWorldCheck).
func worldCheckInput(t *testing.T) string {
	t.Helper()
	raw, err := os.ReadFile("../../data/worlds/standard.json")
	if err != nil {
		t.Fatal(err)
	}
	text := strings.TrimRight(string(raw), "\r\n }") + `, "overrides": {"economy":{"purse":{"startGold":20}}}}`
	in, _ := json.Marshal(map[string]any{"op": "check", "world": "standard", "draft": json.RawMessage(text)})
	return string(in)
}

// worldDraftInput ist die Welt-Datei id als unveränderter Entwurf (prüft auch ihre Nachfolger).
func worldDraftInput(t *testing.T, id string) string {
	t.Helper()
	raw, err := os.ReadFile("../../data/worlds/" + id + ".json")
	if err != nil {
		t.Fatal(err)
	}
	in, _ := json.Marshal(map[string]any{"op": "check", "world": id, "draft": json.RawMessage(raw)})
	return string(in)
}

// hubPlanInput ist die Eingabe `plan` mit testdata/hub/hub7.json.
func hubPlanInput(t *testing.T) string {
	t.Helper()
	raw, err := os.ReadFile("../../testdata/hub/hub7.json")
	if err != nil {
		t.Fatal(err)
	}
	in, _ := json.Marshal(map[string]any{"op": "plan", "raw": string(raw)})
	return string(in)
}

// runRustRef führt einen Fall mit k3c-sim aus.
func runRustRef(t *testing.T, c rustRefCase) rustRefCase {
	t.Helper()
	var out, errOut bytes.Buffer
	tmp := t.TempDir()
	sub := func(args []string) []string {
		out := make([]string, len(args))
		for i, a := range args {
			out[i] = strings.ReplaceAll(a, "{tmp}", filepath.ToSlash(tmp))
		}
		return out
	}
	if len(c.Pre) > 0 {
		var pre bytes.Buffer
		if code := run(sub(c.Pre), env{stdin: strings.NewReader(""), stderr: &errOut, out: NewEmitter(&pre)}); code != ExitOK {
			t.Fatalf("Vorlauf %v: Exit %d %s", c.Pre, code, errOut.String())
		}
	}
	c.Code = run(sub(c.Args), env{stdin: strings.NewReader(c.Stdin), stderr: &errOut, out: NewEmitter(&out), files: os.DirFS("../../data").(data.FS)})
	c.Out = []string{}
	for _, l := range strings.Split(strings.TrimSuffix(out.String(), "\n"), "\n") {
		if l != "" {
			c.Out = append(c.Out, l)
		}
	}
	return c
}

// TestRustRef schreibt mit -write die Vergleichsdateien; ohne -write prüft er, dass sie zum Lauf passen.
func TestRustRef(t *testing.T) {
	for name, c := range rustRefCases(t) {
		got := runRustRef(t, c)
		path := filepath.Join(rustRefDir, name+".json")
		if *writeRustRef {
			raw, err := json.MarshalIndent(got, "", "  ")
			if err != nil {
				t.Fatal(err)
			}
			if err := os.MkdirAll(rustRefDir, 0o755); err != nil {
				t.Fatal(err)
			}
			if err := os.WriteFile(path, append(raw, '\n'), 0o644); err != nil {
				t.Fatal(err)
			}
			continue
		}
		raw, err := os.ReadFile(path)
		if err != nil {
			t.Fatalf("%s: %v (neu schreiben mit -write)", name, err)
		}
		var want rustRefCase
		if err := json.Unmarshal(raw, &want); err != nil {
			t.Fatalf("%s: %v", name, err)
		}
		a, _ := json.Marshal(got)
		b, _ := json.Marshal(want)
		if !bytes.Equal(a, b) {
			t.Fatalf("%s: Lauf weicht von der Datei ab (neu schreiben mit -write)\n%s\nerwartet\n%s", name, a, b)
		}
	}
}
