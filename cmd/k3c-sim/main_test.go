package main

import (
	"bufio"
	"bytes"
	"encoding/json"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync/atomic"
	"testing"
	"time"

	"k3c/engine/balance"
	"k3c/engine/enginetools"
)

// childEnv startet die Test-EXE als k3c-sim (Abbruch-Test mit echtem Prozess); Argumente als JSON-Feld.
const childEnv = "K3C_SIM_CHILD_ARGS"

func TestMain(m *testing.M) {
	if raw := os.Getenv(childEnv); raw != "" {
		var args []string
		if err := json.Unmarshal([]byte(raw), &args); err != nil {
			os.Exit(exitUsage)
		}
		os.Exit(run(args, env{stdin: os.Stdin, stderr: os.Stderr, out: &emitter{out: os.Stdout}}))
	}
	os.Exit(m.Run())
}

// call führt k3c-sim im Prozess aus: Exit-Code, stdout (JSON-Zeilen), stderr.
func call(t *testing.T, stdin string, args ...string) (int, string, string) {
	t.Helper()
	var out, errOut bytes.Buffer
	code := run(args, env{stdin: strings.NewReader(stdin), stderr: &errOut, out: &emitter{out: &out}})
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
	if code != exitOK || resultText(t, out) != want {
		t.Fatalf("stdin: Exit %d %s\n%s\nerwartet\n%s", code, errOut, out, want)
	}
	file := filepath.Join(t.TempDir(), "run.json")
	if err := os.WriteFile(file, in, 0o644); err != nil {
		t.Fatal(err)
	}
	if code, out, _ := call(t, "", "run", "--input", file); code != exitOK || resultText(t, out) != want {
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
		if code != exitOK || resultText(t, out) != want {
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
		{"", nil, exitUsage},
		{"", []string{"fliegen"}, exitUsage},
		{`{"seed":"1","ticks":10,"tiks":5}`, []string{"run"}, exitUsage},
		{`kein json`, []string{"level"}, exitUsage},
		{"", []string{"run", "--input", filepath.Join(t.TempDir(), "fehlt.json")}, exitUsage},
		{`{"seed":"1","ticks":0}`, []string{"run"}, exitFailed},
		{`{"biome":"mond","seed":"1"}`, []string{"level"}, exitFailed},
		{`{"seed":"1","ticks":10} {"seed":"2"}`, []string{"run"}, exitUsage},
		{`{"targets":{"a":1}}`, []string{"balance", "--input", "-"}, exitUsage},
		{"", []string{"balance", "--gibtsnicht"}, exitUsage},
		{"", []string{"balance", "--targets", "--write-baseline"}, exitUsage},
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
	if code, _, errOut := call(t, in, "balance", "--input", "-"); code != exitOK {
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

// AC-01 (RS2.1): Abbruch per Prozess-Ende. Der Lauf meldet Fortschritt als JSON-Zeilen; nach der ersten Zeile endet
// der Prozess (TerminateProcess wie beim Job-Objekt der Workbench), ein Bericht entsteht nicht.
func TestAbbruchPerProzessEnde(t *testing.T) {
	exe, err := os.Executable()
	if err != nil {
		t.Fatal(err)
	}
	dir := t.TempDir()
	args, _ := json.Marshal([]string{"balance", "--targets", "--seeds", "20", "--days", "1", "--progress", "--report-dir", dir})
	cmd := exec.Command(exe, "-test.run=^$")
	cmd.Env = append(os.Environ(), childEnv+"="+string(args))
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		t.Fatal(err)
	}
	if err := cmd.Start(); err != nil {
		t.Fatal(err)
	}
	line := firstLine(t, stdout)
	var ev progressEvent
	if err := json.Unmarshal([]byte(line), &ev); err != nil || ev.Event != "progress" || ev.Done < 1 || ev.Done >= ev.Total {
		_ = cmd.Process.Kill()
		t.Fatalf("erste Zeile %q: %v", line, err)
	}
	if err := cmd.Process.Kill(); err != nil {
		t.Fatal(err)
	}
	if err := cmd.Wait(); err == nil {
		t.Fatal("Prozess endete nach Kill mit Exit 0")
	}
	if files, _ := filepath.Glob(filepath.Join(dir, "balance-*")); len(files) > 0 {
		t.Fatalf("Bericht trotz Abbruch: %v", files)
	}
}

// Signal-Abbruch: Zeile aborted und Exit 130; ist der Lauf schon fertig, bleibt es bei dessen Ausgabe und Exit-Code.
func TestSignalAbbruch(t *testing.T) {
	var out, errOut bytes.Buffer
	var finished atomic.Bool
	code := -1
	onAbort(&emitter{out: &out}, &errOut, &finished, func(c int) { code = c })
	if code != exitAborted || out.String() != `{"event":"aborted"}`+"\n" {
		t.Fatalf("Exit %d, stdout %q", code, out.String())
	}
	out.Reset()
	code = -1
	finished.Store(true)
	onAbort(&emitter{out: &out}, &errOut, &finished, func(c int) { code = c })
	if code != -1 || out.Len() != 0 {
		t.Fatalf("nach Laufende: Exit %d, stdout %q", code, out.String())
	}
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

// firstLine liest die erste Zeile von r, höchstens eine Minute lang.
func firstLine(t *testing.T, r interface{ Read([]byte) (int, error) }) string {
	t.Helper()
	got := make(chan string, 1)
	go func() {
		sc := bufio.NewScanner(r)
		sc.Scan()
		got <- sc.Text()
	}()
	select {
	case line := <-got:
		return line
	case <-time.After(time.Minute):
		t.Fatal("keine Fortschrittszeile nach einer Minute")
	}
	return ""
}
