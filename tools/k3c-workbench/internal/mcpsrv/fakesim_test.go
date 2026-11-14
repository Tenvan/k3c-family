package mcpsrv

import (
	"bufio"
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"sync"
	"testing"

	"k3c/tools/k3c-workbench/internal/simexe"
)

// Falsches bin/k3c-sim (RS2.2): Die Test-EXE wird nach <root>/bin/k3c-sim kopiert und spielt dort das Engine-CLI
// (Schnittstelle cmd/k3c-sim/main.go). So prüfen die Tests den Weg Workbench → Kindprozess ohne Engine; die Rechnung
// selbst prüft cmd/k3c-sim (main_test.go, bot_test.go).

const fakeSimEnv = "K3C_FAKE_SIM"

func TestMain(m *testing.M) {
	if os.Getenv(fakeSimEnv) == "1" && strings.HasPrefix(filepath.Base(os.Args[0]), "k3c-sim") {
		os.Exit(fakeSim(os.Args[1:], os.Stdin, os.Stdout, os.Stderr))
	}
	// Die Asset-Tests rufen die CLI-Attrappe; einmal für den Lauf statt t.Setenv je Test, damit sie parallel laufen (B-830).
	if os.Getenv("K3C_PAPER_CLI") == "" {
		stub, _ := filepath.Abs(filepath.Join("..", "..", "..", "paper", "fixtures", "cli-stub.ts"))
		_ = os.Setenv("K3C_PAPER_CLI", stub)
	}
	// Höchstens sechs parallele Tests, wenn -parallel nicht gesetzt ist: Mit GOMAXPROCS warten git-, bun- und
	// Render-Prozesse aufeinander und ein einzelner Test zeigt über 3 s (B-830).
	flag.Parse()
	set := false
	flag.Visit(func(f *flag.Flag) { set = set || f.Name == "test.parallel" })
	if !set {
		_ = flag.Set("test.parallel", "6")
	}
	code := m.Run()
	for _, dir := range []string{paperDir, fakeSimDir} {
		if dir != "" {
			_ = os.RemoveAll(dir)
		}
	}
	os.Exit(code)
}

// installFakeSim legt das falsche k3c-sim in root ab. Die Test-EXE (einige zehn MB) wird einmal je Lauf kopiert und je Test nur verlinkt (Hardlink): Eine neue Kopie je Test
// prüft der Virenscanner beim ersten Start jedes Mal ganz (B-830). Ohne Hardlink (anderes Laufwerk) wird kopiert.
func installFakeSim(t *testing.T, root string) {
	t.Helper()
	exe := filepath.Join(root, simexe.Rel(0))
	if err := os.MkdirAll(filepath.Dir(exe), 0o755); err != nil {
		t.Fatal(err)
	}
	src := fakeSimVorlage(t)
	if err := os.Link(src, exe); err != nil {
		self, err := os.ReadFile(src)
		if err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(exe, self, 0o755); err != nil {
			t.Fatal(err)
		}
	}
	t.Setenv(fakeSimEnv, "1")
}

var (
	fakeSimOnce sync.Once
	fakeSimDir  string // TestMain räumt auf
	fakeSimExe  string
	fakeSimErr  error
)

// fakeSimVorlage kopiert die Test-EXE einmal je Lauf (nicht die laufende EXE verlinken: die ließe sich nicht löschen).
func fakeSimVorlage(t *testing.T) string {
	t.Helper()
	fakeSimOnce.Do(func() {
		self, err := os.ReadFile(os.Args[0])
		if err != nil {
			fakeSimErr = err
			return
		}
		if fakeSimDir, fakeSimErr = os.MkdirTemp("", "mcpsrv-fakesim-"); fakeSimErr != nil {
			return
		}
		fakeSimExe = filepath.Join(fakeSimDir, filepath.Base(simexe.Rel(0)))
		fakeSimErr = os.WriteFile(fakeSimExe, self, 0o755)
	})
	if fakeSimErr != nil {
		t.Fatal(fakeSimErr)
	}
	return fakeSimExe
}

func fakeSim(args []string, stdin io.Reader, stdout, stderr io.Writer) int {
	emit := func(v any) { b, _ := json.Marshal(v); _, _ = fmt.Fprintln(stdout, string(b)) }
	fail := func(code int, msg string) int {
		emit(map[string]string{"event": "error", "message": msg})
		_, _ = fmt.Fprintln(stderr, "❌ k3c-sim: "+msg)
		return code
	}
	switch args[0] {
	case "run", "level":
		var in struct {
			Seed  string
			Ticks int
		}
		_ = json.NewDecoder(stdin).Decode(&in)
		if args[0] == "run" && (in.Ticks < 1 || in.Ticks > 100000) {
			return fail(2, fmt.Sprintf("ticks muss zwischen 1 und 100000 liegen, war %d", in.Ticks))
		}
		emit(map[string]string{"event": "result", "text": fmt.Sprintf("%s · Seed %q · %d Ticks", args[0], in.Seed, in.Ticks)})
	case "balance":
		return fakePlay(args, emit, fail)
	case "bot":
		return fakeBot(stdin, emit)
	default:
		return fail(2, "unbekannter Unterbefehl")
	}
	return 0
}

// fakePlay spielt `balance --play DATEI`: gleicher Endzustand wie die Aufnahme, Version 1 erwartet.
func fakePlay(args []string, emit func(any), fail func(int, string) int) int {
	i := slices.Index(args, "--play")
	if i < 0 || i+1 >= len(args) {
		return fail(2, "--play fehlt")
	}
	raw, err := os.ReadFile(args[i+1])
	if err != nil {
		return fail(1, "replay nicht lesbar")
	}
	var r struct {
		Version        int
		EndHash        string
		CastleFallTick *int
	}
	_ = json.Unmarshal(raw, &r)
	if r.Version != 1 {
		return fail(1, fmt.Sprintf("%s: unbekannte Version %d", args[i+1], r.Version))
	}
	emit(map[string]any{"event": "result", "report": map[string]any{"ticks": 30, "endHash": r.EndHash,
		"castleFallTick": r.CastleFallTick, "warnings": []string{}}})
	return 0
}

// fakeBot antwortet je Anfrage: alle genannten Monarchen nach rechts, zahlen.
func fakeBot(stdin io.Reader, emit func(any)) int {
	sc := bufio.NewScanner(stdin)
	sc.Buffer(make([]byte, 0, 1<<20), 64<<20)
	for sc.Scan() {
		var req struct{ Players []int }
		_ = json.Unmarshal(sc.Bytes(), &req)
		cmds := []simexe.Command{}
		for _, p := range req.Players {
			cmds = append(cmds, simexe.Command{Player: p, MoveX: 1, Pay: true})
		}
		emit(map[string]any{"event": "commands", "commands": cmds})
	}
	return 0
}
