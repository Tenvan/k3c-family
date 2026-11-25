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

	"k3c/tools/k3c-studio/internal/gameexe"
)

// Modus offline (B-348/AC-02, RU40.3): `balance --targets` rechnet die Spiel-EXE des Checkouts (`k3c-game sim balance`,
// internal/gameexe, docs/standards/k3c-game-laeufe.md); Stop schließt ihre stdin, nach der Frist beendet das Job-Objekt
// den Baum. Das Studio liest die Events (Fortschritt, Fehler) und den Bericht. Bericht des Testers und eigener Bericht
// liegen in reports/simtest-<id>/. Mit Head läuft der Lauf sichtbar im Spiel (`--window [--speed N]`).

const offlineTimeout = 60 * time.Minute

// Proc ist ein Lauf der Spiel-EXE (Args: Unterbefehl `balance` zuerst, dann die Schalter); Out bekommt jede Zeile mit
// ihrem Strom („stdout“: JSON-Zeilen der Events, „stderr“: Meldungen für Menschen).
type Proc struct {
	Dir     string // Checkout; EXE und data/ darunter
	Args    []string
	Head    bool // mit Fenster (--window)
	Speed   int  // --speed N, nur mit Head
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
		p := Proc{Dir: run.root, Timeout: offlineTimeout, Args: spec.args(dir), Head: spec.Head, Speed: spec.Speed, Out: out.line}
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
	if ev, ok := gameexe.ParseEvent(text); ok && stream == "stdout" {
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

// streamExe startet die EXE; Tests ersetzen es durch eine Attrappe.
var streamExe = gameexe.Stream

// pathFlags sind die Schalter von `balance`, deren Wert ein Pfad ist; relative Werte gelten ab dem Checkout.
var pathFlags = []string{"--report-dir", "--baseline", "--write-baseline", "--out", "--replay-dir", "--play", "--insights"}

// exeArgs ist p.Args ohne den Unterbefehl, Pfadwerte absolut ab dir (die EXE läuft im Ordner ihres Pakets, nicht im Checkout).
func exeArgs(dir string, args []string) []string {
	out := slices.Clone(args)
	if len(out) > 0 && out[0] == "balance" {
		out = out[1:]
	}
	for i := 0; i+1 < len(out); i++ {
		if slices.Contains(pathFlags, out[i]) && !filepath.IsAbs(out[i+1]) {
			out[i+1] = filepath.Join(dir, out[i+1])
		}
	}
	return out
}

// runExe rechnet mit der Spiel-EXE des Checkouts p.Dir (`k3c-game sim balance`, RU40.3). Jedes Event geht als JSON-Zeile
// nach p.Out, jede Zeile von stderr als „stderr“. Stop und Zeitlimit schließen stdin der EXE (Exit 130, `aborted`); bleibt
// sie hängen, beendet gameexe nach der Frist den ganzen Prozessbaum. Ein Fehler ohne Meldung der EXE steht als „❌ k3c-game sim: …“ auf stderr.
func runExe(ctx context.Context, p Proc) Result {
	ctx, cancel := context.WithTimeout(ctx, p.Timeout)
	defer cancel()
	out := p.Out
	if out == nil {
		out = func(string, string) {}
	}
	err := streamExe(ctx, gameexe.Options{
		Root: p.Dir, Sub: "balance", Input: []byte("{}"), Window: p.Head, Speed: p.Speed, Args: exeArgs(p.Dir, p.Args),
		OnEvent: func(ev gameexe.Event) {
			if b, err := json.Marshal(ev); err == nil {
				out("stdout", string(b))
			}
		},
		OnStderr: func(l string) { out("stderr", l) },
	})
	switch {
	case err == nil:
		return Result{}
	case errors.Is(ctx.Err(), context.DeadlineExceeded):
		return Result{TimedOut: true, Exit: -1}
	case ctx.Err() != nil:
		return Result{Exit: exitAborted}
	}
	out("stderr", "❌ k3c-game sim: "+err.Error())
	return Result{Exit: 1}
}

// exitAborted ist der Exit-Code eines abgebrochenen Laufs (docs/standards/k3c-game-laeufe.md).
const exitAborted = 130

func lastLines(lines []string, n int) []string {
	if len(lines) > n {
		return lines[len(lines)-n:]
	}
	return lines
}
