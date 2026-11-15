package simcli

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"sync/atomic"
	"testing"
	"time"

	"k3c/engine/balance"
	"k3c/engine/enginetools"
)

// call führt k3c-sim im Prozess aus: Exit-Code, stdout (JSON-Zeilen), stderr.
func call(t *testing.T, stdin string, args ...string) (int, string, string) {
	t.Helper()
	var out, errOut bytes.Buffer
	code := run(args, env{stdin: strings.NewReader(stdin), stderr: &errOut, out: NewEmitter(&out)})
	return code, out.String(), errOut.String()
}

// resultText liest das Feld text der einzigen result-Zeile.
func resultText(t *testing.T, out string) string {
	t.Helper()
	var ev struct{ Event, Text string }
	if err := json.Unmarshal([]byte(strings.TrimSpace(out)), &ev); err != nil || ev.Event != "result" {
		t.Fatalf("keine result-Zeile: %v %q", err, out)
	}
	return ev.Text
}

// AC-01 (RS2.1): run liefert auf festem Seed denselben Text wie der bisherige Weg (enginetools.Run, MCP sim_run),
// Eingabe von stdin oder aus Datei.
func TestRunGleichSimRun(t *testing.T) {
	inputs := []enginetools.Segment{{FromTick: 0, ToTick: 600, MoveX: 1, Pay: true}, {Player: 1, FromTick: 30, ToTick: 900, MoveX: -1}}
	want, err := enginetools.Run("forest", "k3c-dev", 3000, inputs)
	if err != nil {
		t.Fatal(err)
	}
	in, _ := json.Marshal(runInput{Seed: "k3c-dev", Ticks: 3000, Inputs: inputs})
	code, out, errOut := call(t, string(in), "run")
	if code != ExitOK || resultText(t, out) != want {
		t.Fatalf("stdin: Exit %d %s\n%s\nerwartet\n%s", code, errOut, out, want)
	}
	file := filepath.Join(t.TempDir(), "run.json")
	if err := os.WriteFile(file, in, 0o644); err != nil {
		t.Fatal(err)
	}
	if code, out, _ := call(t, "", "run", "--input", file); code != ExitOK || resultText(t, out) != want {
		t.Fatalf("Datei: Exit %d %s", code, out)
	}
}

// AC-01 (RS2.1): level liefert denselben Text wie level_generate (enginetools.Level).
func TestLevelGleichLevelGenerate(t *testing.T) {
	for _, biome := range enginetools.Biomes {
		want, err := enginetools.Level(biome, "familie-2026")
		if err != nil {
			t.Fatal(err)
		}
		code, out, errOut := call(t, `{"biome":"`+biome+`","seed":"familie-2026"}`, "level")
		if code != ExitOK || resultText(t, out) != want {
			t.Fatalf("%s: Exit %d %s %q", biome, code, errOut, out)
		}
	}
}

// Falsche Aufrufe sind Exit 2 mit error-Zeile, Lauffehler Exit 1.
func TestAufrufFehler(t *testing.T) {
	for _, c := range []struct {
		stdin string
		args  []string
		code  int
	}{
		{"", nil, ExitUsage},
		{"", []string{"fliegen"}, ExitUsage},
		{`{"seed":"1","ticks":10,"tiks":5}`, []string{"run"}, ExitUsage},
		{`kein json`, []string{"level"}, ExitUsage},
		{"", []string{"run", "--input", filepath.Join(t.TempDir(), "fehlt.json")}, ExitUsage},
		{`{"seed":"1","ticks":0}`, []string{"run"}, ExitFailed},
		{`{"biome":"mond","seed":"1"}`, []string{"level"}, ExitFailed},
		{`{"seed":"1","ticks":10} {"seed":"2"}`, []string{"run"}, ExitUsage},
		{`{"targets":{"a":1}}`, []string{"balance", "--input", "-"}, ExitUsage},
		{"", []string{"balance", "--gibtsnicht"}, ExitUsage},
		{"", []string{"balance", "--targets", "--write-baseline"}, ExitUsage},
	} {
		code, out, _ := call(t, c.stdin, c.args...)
		if code != c.code || !strings.Contains(out, `"event":"error"`) {
			t.Errorf("%v %q: Exit %d (erwartet %d), stdout %q", c.args, c.stdin, code, c.code, out)
		}
	}
}

// Das JSON hinter --input wird zu Flags (sortiert), Flags der Befehlszeile stehen danach.
func TestBalanceJSONWirdFlags(t *testing.T) {
	got, err := balanceArgs([]string{"--input", "-", "--days", "3"},
		strings.NewReader(`{"targets":true,"seed-list":"k3c-dev","vary":["a.json:x=1","b.json:y=2"],"seeds":2}`))
	if err != nil {
		t.Fatal(err)
	}
	want := "--seed-list=k3c-dev --seeds=2 --targets=true --vary=a.json:x=1 --vary=b.json:y=2 --days 3"
	if strings.Join(got, " ") != want {
		t.Fatalf("%v\nerwartet %s", got, want)
	}
}

// AC-01 (RS2.1): Die Matrix (1 Seed) schreibt mit --out dieselben Bytes wie balance.Run; Eingabe als JSON.
func TestBalanceMatrixGleich(t *testing.T) {
	m := balance.Matrix{Seeds: []string{"k3c-dev"}, Bots: []string{"saver"}, Players: []int{1}, Depths: []int{0}, Days: 1}
	report, err := balance.Run(m)
	if err != nil {
		t.Fatal(err)
	}
	want, _ := report.JSON()
	file := filepath.Join(t.TempDir(), "matrix.json")
	in := `{"seed-list":"k3c-dev","bots":"saver","players":"1","depths":"0","days":1,"out":` + jsonString(file) + `}`
	if code, _, errOut := call(t, in, "balance", "--input", "-"); code != ExitOK {
		t.Fatalf("Exit %d %s", code, errOut)
	}
	got, _ := os.ReadFile(file)
	if !bytes.Equal(got, want) {
		t.Fatalf("Bericht weicht ab (%d gegen %d Bytes)", len(got), len(want))
	}
}

func jsonString(s string) string {
	b, _ := json.Marshal(s)
	return string(b)
}

// requireJSONLines: Jede stdout-Zeile ist ein JSON-Objekt mit "event" (keine fremden Ausgaben).
func requireJSONLines(t *testing.T, out string) {
	t.Helper()
	for _, line := range strings.Split(strings.TrimSuffix(out, "\n"), "\n") {
		var ev struct{ Event string }
		if err := json.Unmarshal([]byte(line), &ev); err != nil || ev.Event == "" {
			t.Errorf("keine JSON-Zeile mit event: %q", line)
		}
	}
}

// panicReader ist eine Eingabe, die beim Lesen abstürzt.
type panicReader struct{}

func (panicReader) Read([]byte) (int, error) { panic("boom") }

// B-924: Eine Panik in der Engine wird zum Fehler (Exit 1, error-Zeile), nie zum Absturz des Aufrufers.
func TestPanikWirdFehler(t *testing.T) {
	var out, errOut bytes.Buffer
	code := Exec(context.Background(), []string{"level"}, panicReader{}, &errOut, NewEmitter(&out), nil)
	if code != ExitFailed || !strings.Contains(out.String(), `"event":"error"`) || !strings.Contains(out.String(), "Absturz in der Engine: boom") {
		t.Fatalf("Exit %d, stdout %q", code, out.String())
	}
	err := Stream(context.Background(), []string{"level"}, []byte(`{"seed":"1"}`), nil, func(Event) { panic("im Rückruf") })
	if err == nil || !strings.Contains(err.Error(), "Absturz in der Engine: im Rückruf") {
		t.Fatalf("Stream: %v", err)
	}
}

// B-924: Call liefert die result-Zeile, Fehler der Engine kommen mit ihrer Meldung, ein beendeter Context bricht ab.
func TestCallErgebnisFehlerUndAbbruch(t *testing.T) {
	ev, err := Call(context.Background(), []string{"level"}, []byte(`{"seed":"familie-2026"}`), nil)
	want, _ := enginetools.Level("forest", "familie-2026")
	if err != nil || ev.Text != want {
		t.Fatalf("level: %v %q", err, ev.Text)
	}
	if _, err := Call(context.Background(), []string{"level"}, []byte(`{"biome":"mond","seed":"1"}`), nil); err == nil || !strings.Contains(err.Error(), "mond") {
		t.Errorf("Fehler: %v", err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	if _, err := Call(ctx, []string{"level"}, []byte(`{"seed":"1"}`), nil); !errors.Is(err, context.Canceled) {
		t.Errorf("Abbruch: %v", err)
	}
}

// B-924: Ein Abbruch beendet balance --targets nach dem nächsten fertigen Lauf; es entsteht kein Bericht.
func TestAbbruchBeendetBalanceTargets(t *testing.T) {
	dir := t.TempDir()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	args := []string{"balance", "--targets", "--seeds", "20", "--days", "1", "--progress", "--report-dir", dir}
	var events atomic.Int32
	err := Stream(ctx, args, nil, nil, func(ev Event) {
		if ev.Event == "progress" && events.Add(1) == 1 {
			cancel()
		}
	})
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("Stream: %v", err)
	}
	time.Sleep(time.Second) // laufende Läufe enden, ohne den Prozess abzureißen (runAll fängt die Panik)
	if files, _ := filepath.Glob(filepath.Join(dir, "balance-*")); len(files) > 0 {
		t.Fatalf("Bericht trotz Abbruch: %v", files)
	}
}

// B-924: Nach einem Abbruch schreiben Matrix- und Sensitivitätsläufe keinen Bericht; Stream kehrt erst nach dem Ende der
// Rechnung zurück (der Aufrufer gibt den Platz danach frei).
func TestAbbruchSchreibtKeinenBericht(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	dir := t.TempDir()
	out := filepath.Join(dir, "matrix.json")
	for name, args := range map[string][]string{
		"matrix":      {"balance", "--seeds", "2", "--days", "1", "--out", out},
		"sensitivity": {"balance", "--curves", "--report-dir", dir},
	} {
		if err := Stream(ctx, args, nil, nil, func(Event) {}); !errors.Is(err, context.Canceled) {
			t.Fatalf("%s: %v", name, err)
		}
	}
	if files, _ := filepath.Glob(filepath.Join(dir, "*")); len(files) > 0 {
		t.Fatalf("Bericht trotz Abbruch: %v", files)
	}
}
