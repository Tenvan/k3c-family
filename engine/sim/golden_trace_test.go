package sim

import (
	"bufio"
	"encoding/json"
	"flag"
	"os"
	"path/filepath"
	"testing"

	"k3c/engine/internal/golden"
)

// Go-Spur für den Abgleich der Rust-Sim (Sprint RU22): Mit `go test ./engine/sim -run TestGoldenTrace -trace` schreibt
// jeder Golden-Lauf je Schritt eine Zeile {"tick":N,"world":<Zustand>} nach .work/trace/sim-<name>.jsonl (Zeile 0 =
// Startzustand). Das Rust-Werkzeug (rust/crates/k3c-sim/tests/common/step_trace.rs) nennt damit den ersten abweichenden
// Schritt statt nur den ersten abweichenden Snapshot. Ohne -trace überspringt der Test; die Spur wird nicht eingecheckt.
var (
	traceOn  = flag.Bool("trace", false, "Golden-Läufe Schritt für Schritt als .work/trace/sim-<name>.jsonl schreiben")
	traceRun = flag.String("trace-run", "", "nur diesen Golden-Lauf spuren (Name ohne sim-, z. B. forest-tag)")
)

func TestGoldenTrace(t *testing.T) {
	if !*traceOn {
		t.Skip("nur mit -trace")
	}
	dir := filepath.Join("..", "..", ".work", "trace")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	for _, run := range goldenRuns(t) {
		if *traceRun != "" && run.Name != *traceRun {
			continue
		}
		t.Run(run.Name, func(t *testing.T) { writeTrace(t, run, filepath.Join(dir, "sim-"+run.Name+".jsonl")) })
	}
}

// writeTrace rechnet den Lauf wie replay nach und schreibt den Zustand nach jedem Schritt.
func writeTrace(t *testing.T, run goldenRun, path string) {
	goldenPurse(t)
	f, err := os.Create(path)
	if err != nil {
		t.Fatal(err)
	}
	out := bufio.NewWriter(f)
	w := goldenWorld(t, run, nil)
	line := func(tick int) {
		b, err := json.Marshal(map[string]any{"tick": tick, "world": golden.Raw(t, w)})
		if err != nil {
			t.Fatal(err)
		}
		if _, err := out.Write(append(b, '\n')); err != nil {
			t.Fatal(err)
		}
	}
	line(0)
	tick := 0
	for _, seg := range run.Inputs {
		for range seg.Ticks {
			Step(w, seg.Commands, run.Dt)
			tick++
			line(tick)
		}
	}
	if err := out.Flush(); err != nil {
		t.Fatal(err)
	}
	if err := f.Close(); err != nil {
		t.Fatal(err)
	}
}
