package simrun

import (
	"context"
	"errors"
	"fmt"
	"os"
	"slices"
	"strings"
	"sync"
	"time"

	"k3c/tools/k3c-workbench/internal/console"
	"k3c/tools/k3c-workbench/internal/proc"
	"k3c/tools/k3c-workbench/internal/simexe"
)

// Modus offline (B-348/AC-02, RS2.2): das Engine-CLI im Checkout bauen (bin/k3c-sim-<platz>, fester Pfad, kein go run)
// und `k3c-sim balance --targets` ausführen. So rechnet die Engine des Checkouts, nicht die der Workbench; die
// Workbench liest nur die JSON-Zeilen (Fortschritt, Fehler) und den Bericht. Bericht des Testers und eigener Bericht
// liegen in reports/simtest-<id>/.

const (
	buildTimeout   = 5 * time.Minute
	offlineTimeout = 60 * time.Minute
)

// Proc ist ein Prozesslauf; Out bekommt jede Zeile mit ihrem Strom („stdout“, „stderr“).
type Proc struct {
	Dir     string
	Args    []string
	Timeout time.Duration
	Env     []string // zusätzlich zur Umgebung der Workbench, NAME=Wert
	Out     func(stream, text string)
}

// Result ist das Ende eines Prozesses.
type Result struct {
	Exit     int
	TimedOut bool
	Err      error // Start gescheitert
}

// StartOffline prüft die Spec und reiht einen Offline-Lauf ein (wartet, wenn alle Plätze belegt sind). Mit PerVariant
// reiht es n + 1 Läufe ein (pervariant.go) und liefert den Basis-Lauf; geprüft wird jeder Lauf, bevor einer startet.
func (g *Registry) StartOffline(root, where string, spec Spec) (Info, error) {
	if err := spec.Check(); err != nil {
		return Info{}, err
	}
	specs := []Spec{spec}
	if spec.PerVariant {
		specs = spec.perVariant()
	}
	for _, s := range specs {
		if err := s.Check(); err != nil {
			return Info{}, err
		}
	}
	var first *Run
	for _, s := range specs {
		run, err := g.Start(root, where, s.Title(), &s, g.Offline(s), true)
		if err != nil {
			return Info{}, err
		}
		if first == nil {
			first = run
		}
	}
	return g.Get(root, first.id)
}

// Offline ist der Runner des Modus offline: bauen, rechnen, Bericht lesen.
func (g *Registry) Offline(spec Spec) Runner {
	return func(ctx context.Context, run *Run) {
		dir := ReportDir(run)
		// Ein fester EXE-Pfad je Platz: läuft der erste Lauf noch, sperrt Windows seine EXE, und der zweite Bau
		// schlüge fehl (kein go run, B-348/AC-02). bin/k3c-sim selbst baut `task k3c-sim:build` (sim_run, Bots).
		exe := balanceExe(run.Slot())
		steps := []struct {
			phase string
			proc  Proc
		}{
			{"bauen", Proc{Dir: run.root, Timeout: buildTimeout,
				Args: []string{"go", "build", "-o", exe, "./cmd/k3c-sim"}}},
			{"rechnen", Proc{Dir: run.root, Timeout: offlineTimeout, Args: spec.args(exe, dir), Env: simexe.Env(run.root)}},
		}
		out := &procOutput{progress: func(done, total int) { g.SetProgress(run, done, total) }}
		for _, st := range steps {
			g.SetPhase(run, st.phase)
			st.proc.Out = out.line
			if failed := stepFailed(ctx, g.Exec(ctx, st.proc)); failed != "" {
				g.finishFailed(run, append([]string{st.phase + ": " + failed}, out.lines()...), dir, spec)
				return
			}
		}
		verdict, lines, err := g.readStamped(run, dir, spec)
		if err != nil {
			g.Finish(run, Failed, "", []string{err.Error()}, dir)
			return
		}
		g.Finish(run, Done, verdict, lines, dir)
	}
}

// procOutput liest die Ausgabe von Bau und k3c-sim: Fortschritt aus den JSON-Zeilen, sonst die letzten Zeilen für
// den Status. Die Zeile „panic: …“ eines Absturzes bleibt erhalten, auch wenn der Stack danach die Zeilen füllt.
type procOutput struct {
	progress func(done, total int)
	mu       sync.Mutex // stdout und stderr schreiben nebenläufig
	tail     []string
	panic    string
}

func (o *procOutput) line(stream, text string) {
	if ev, ok := simexe.ParseEvent(text); ok && stream == "stdout" {
		if ev.Event == "progress" {
			o.progress(ev.Done, ev.Total)
		}
		return // error steht auch auf stderr („❌ k3c-sim: …“), report und aborted liest der Runner selbst
	}
	o.mu.Lock()
	defer o.mu.Unlock()
	if stream == "stderr" && strings.HasPrefix(text, "panic:") && o.panic == "" {
		o.panic = text
	}
	o.tail = lastLines(append(o.tail, text), 3)
}

// lines sind die Zeilen für den Status: die Panic-Zeile (falls nicht schon im Rest), dann die letzten Zeilen.
func (o *procOutput) lines() []string {
	o.mu.Lock()
	defer o.mu.Unlock()
	if o.panic != "" && !slices.Contains(o.tail, o.panic) {
		return append([]string{o.panic}, o.tail...)
	}
	return slices.Clone(o.tail)
}

// finishFailed beendet einen gescheiterten oder abgebrochenen Lauf; steht schon ein Bericht (balance-*.json bzw.
// tuning-*.json), gilt er als Teilbericht und liefert seine Kennzahlen.
func (g *Registry) finishFailed(run *Run, lines []string, dir string, spec Spec) {
	verdict, part, err := g.readStamped(run, dir, spec)
	if err != nil {
		g.Finish(run, Failed, "", lines, dir)
		return
	}
	g.Finish(run, Failed, verdict, append(append(lines, "Teilbericht:"), part...), dir)
}

// stepFailed beschreibt einen gescheiterten Schritt; leer heißt geschafft. Ein Abbruch über Stop zählt als Grund.
func stepFailed(ctx context.Context, res Result) string {
	switch {
	case errors.Is(ctx.Err(), context.Canceled):
		return "abgebrochen"
	case res.Err != nil:
		return "nicht gestartet: " + res.Err.Error()
	case res.TimedOut:
		return "Zeitlimit erreicht"
	case res.Exit != 0:
		return fmt.Sprintf("Exit %d", res.Exit)
	}
	return ""
}

// runProcess startet einen Prozess mit Zeitlimit; Zeitlimit und Abbruch beenden den ganzen Prozessbaum (Job-Objekt).
func runProcess(ctx context.Context, p Proc) Result {
	ctx, cancel := context.WithTimeout(ctx, p.Timeout)
	defer cancel()
	cmd := proc.Command(ctx, p.Args)
	cmd.Dir = p.Dir
	if len(p.Env) > 0 {
		cmd.Env = append(os.Environ(), p.Env...)
	}
	out := p.Out
	if out == nil {
		out = func(string, string) {}
	}
	stdout := console.NewLineWriter(func(text string) { out("stdout", text) })
	stderr := console.NewLineWriter(func(text string) { out("stderr", text) })
	cmd.Stdout, cmd.Stderr = stdout, stderr
	err := cmd.Run()
	stdout.Flush()
	stderr.Flush()
	var res Result
	switch {
	case errors.Is(ctx.Err(), context.DeadlineExceeded):
		res.TimedOut, res.Exit = true, -1
	case cmd.ProcessState != nil:
		res.Exit = cmd.ProcessState.ExitCode()
	case err != nil:
		res.Err = err
	}
	return res
}

func lastLines(lines []string, n int) []string {
	if len(lines) > n {
		return lines[len(lines)-n:]
	}
	return lines
}

// balanceExe ist der feste Pfad des Engine-CLI für einen Platz (relativ zum Checkout): Platz n baut
// bin/k3c-sim-<n+1>, getrennt von bin/k3c-sim, das sim_run und die Bots der Online-Läufe nutzen.
func balanceExe(slot int) string { return simexe.Rel(slot + 1) }
