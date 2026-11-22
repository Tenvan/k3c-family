package simrun

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"path/filepath"
	"slices"
	"strings"
	"sync"
	"time"

	"k3c/engine/simcli"
)

// Modus offline (B-348/AC-02, B-924): `balance --targets` rechnet im Prozess der Workbench (engine/simcli, in einer
// Goroutine, abbrechbar über den Context; eine Panik der Engine wird zum Fehler des Laufs). Die Workbench liest die
// Events (Fortschritt, Fehler) und den Bericht. Bericht des Testers und eigener Bericht liegen in reports/simtest-<id>/.
// Die Engine ist die der laufenden Workbench: Sim-Änderungen prüft eine eigene Instanz im Worktree (B-925).

const offlineTimeout = 60 * time.Minute

// Proc ist ein Lauf der Engine (Args wie bei `k3c-sim`: Unterbefehl zuerst); Out bekommt jede Zeile mit ihrem Strom
// („stdout“: JSON-Zeilen der Events, „stderr“: Meldungen für Menschen).
type Proc struct {
	Dir     string // Checkout; data/ darunter liest world/hub
	Args    []string
	Timeout time.Duration
	Out     func(stream, text string)
}

// Result ist das Ende eines Laufs.
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

// Offline ist der Runner des Modus offline: rechnen, Bericht lesen.
func (g *Registry) Offline(spec Spec) Runner {
	return func(ctx context.Context, run *Run) {
		dir := ReportDir(run)
		g.SetPhase(run, "rechnen")
		out := &procOutput{progress: func(done, total int) { g.SetProgress(run, done, total) }}
		p := Proc{Dir: run.root, Timeout: offlineTimeout, Args: spec.args(dir), Out: out.line}
		if failed := stepFailed(ctx, g.Exec(ctx, p)); failed != "" {
			g.finishFailed(run, append([]string{"rechnen: " + failed}, out.lines()...), dir, spec)
			return
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
	if ev, ok := simcli.ParseEvent(text); ok && stream == "stdout" {
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

// runEngine rechnet im Prozess (engine/simcli) mit den Daten von p.Dir/data; Zeitlimit und Abbruch beenden den Lauf
// beim nächsten fertigen Lauf der Engine. Jedes Event geht als JSON-Zeile nach p.Out, ein Fehler als „❌ k3c-sim: …“.
func runEngine(ctx context.Context, p Proc) Result {
	ctx, cancel := context.WithTimeout(ctx, p.Timeout)
	defer cancel()
	out := p.Out
	if out == nil {
		out = func(string, string) {}
	}
	err := simcli.Stream(ctx, p.Args, nil, simcli.DirFiles(filepath.Join(p.Dir, "data")), func(ev simcli.Event) {
		if b, err := json.Marshal(ev); err == nil {
			out("stdout", string(b))
		}
	})
	switch {
	case err == nil:
		return Result{}
	case errors.Is(ctx.Err(), context.DeadlineExceeded):
		return Result{TimedOut: true, Exit: -1}
	case ctx.Err() != nil:
		return Result{Exit: simcli.ExitAborted}
	}
	out("stderr", "❌ k3c-sim: "+err.Error())
	return Result{Exit: simcli.ExitFailed}
}

func lastLines(lines []string, n int) []string {
	if len(lines) > n {
		return lines[len(lines)-n:]
	}
	return lines
}
