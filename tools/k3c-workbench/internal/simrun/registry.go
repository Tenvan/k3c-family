// Package simrun steuert die Testläufe der Workbench (B-348, WZ28.4): ein Register je Workbench für sim_test (MCP) und
// die Balancing-Seite, mit höchstens MaxPerRoot gleichzeitigen Läufen je Checkout und einer Warteschlange dahinter.
// Der Runner eines Laufs ist eine Funktion; den Offline-Runner (Balance-Binary bauen und rechnen) liefert offline.go,
// die Online-Runner bleiben in mcpsrv.
package simrun

import (
	"context"
	"errors"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"sync"
	"time"
)

// MaxPerRoot ist die Zahl gleichzeitiger Läufe je Checkout, gleich für alle Modi.
const MaxPerRoot = 2

// Zustände eines Laufs.
const (
	Waiting  = "wartet"
	Running  = "läuft"
	Done     = "fertig"
	Canceled = "abgebrochen"
	Failed   = "fehler"
)

var (
	// ErrFull: alle Plätze des Checkouts belegt und der Aufrufer will nicht warten.
	ErrFull = errors.New("alle Plätze belegt")
	// ErrUnknown: keine Lauf-ID in diesem Checkout.
	ErrUnknown = errors.New("lauf unbekannt")
)

// Runner rechnet einen Lauf und meldet sein Ende mit Registry.Finish.
type Runner func(ctx context.Context, run *Run)

// Run ist ein Lauf. id, root, where, label, spec und runner ändern sich nicht; alles andere nur unter Registry.mu.
type Run struct {
	id, root, where, label string
	spec                   *Spec
	runner                 Runner

	started, ended time.Time
	state, phase   string
	verdict        string
	done, total    int
	lines          []string
	report         string
	active         bool // Runner arbeitet und hält einen Platz
	slot           int  // Platz 0 … MaxPerRoot-1, solange active (eigene EXE je Platz)
	cancel         context.CancelFunc
}

// Slot ist der Platz des laufenden Laufs im Checkout.
func (r *Run) Slot() int { return r.slot }

// ID ist die Lauf-ID, z. B. run-3.
func (r *Run) ID() string { return r.id }

// Root ist der Checkout des Laufs.
func (r *Run) Root() string { return r.root }

// Started ist der Beginn des Laufs (fest, sobald der Runner läuft).
func (r *Run) Started() time.Time { return r.started }

// Registry hält alle Läufe der Workbench.
type Registry struct {
	// Exec startet einen Prozess (Test-Naht); vor dem ersten Start setzen.
	Exec func(context.Context, Proc) Result

	log  *slog.Logger
	mu   sync.Mutex
	next int
	runs []*Run
}

// New baut ein leeres Register; Prozesse startet runProcess.
func New(log *slog.Logger) *Registry {
	if log == nil {
		log = slog.New(slog.DiscardHandler)
	}
	return &Registry{Exec: runProcess, log: log}
}

// Start legt einen Lauf an. Ist ein Platz frei, rechnet er sofort; sonst wartet er (queue) oder Start liefert ErrFull.
func (g *Registry) Start(root, where, label string, spec *Spec, runner Runner, queue bool) (*Run, error) {
	g.mu.Lock()
	defer g.mu.Unlock()
	free := g.active(root) < MaxPerRoot
	if !free && !queue {
		return nil, ErrFull
	}
	g.next++
	run := &Run{id: fmt.Sprintf("run-%d", g.next), root: root, where: where, label: label, spec: spec, runner: runner,
		started: time.Now(), state: Waiting, phase: "wartet"}
	g.runs = append(g.runs, run)
	if free {
		g.launch(run)
	} else {
		g.log.Info("⏳ testlauf wartet", "ns", "simtest", "id", run.id, "mode", label)
	}
	return run, nil
}

// launch startet den Runner; unter g.mu.
func (g *Registry) launch(run *Run) {
	ctx, cancel := context.WithCancel(context.Background()) // nicht der Kontext des Aufrufs: der Lauf überlebt ihn
	run.slot = g.freeSlot(run.root)
	run.state, run.phase, run.started, run.active, run.cancel = Running, "start", time.Now(), true, cancel
	g.log.Info("🚀 testlauf gestartet", "ns", "simtest", "id", run.id, "mode", run.label)
	go func() {
		defer cancel()
		func() {
			defer func() { // eine Panic im Runner beendet den Lauf, nicht die Workbench, und gibt den Platz frei
				if p := recover(); p != nil {
					g.log.Error("💥 testlauf abgestürzt", "ns", "simtest", "id", run.id, "panic", fmt.Sprint(p))
				}
			}()
			run.runner(ctx, run)
		}()
		g.mu.Lock()
		defer g.mu.Unlock()
		run.active = false
		if run.state == Running { // Runner ohne Finish
			run.state, run.ended = Failed, time.Now()
		}
		g.log.Info(map[bool]string{true: "✅ testlauf beendet", false: "❌ testlauf beendet"}[run.state == Done],
			"ns", "simtest", "id", run.id, "state", run.state, "verdict", run.verdict)
		g.launchNext(run.root)
	}()
}

// launchNext startet wartende Läufe des Checkouts, solange Plätze frei sind; unter g.mu.
func (g *Registry) launchNext(root string) {
	for _, run := range g.runs {
		if g.active(root) >= MaxPerRoot {
			return
		}
		if run.root == root && run.state == Waiting {
			g.launch(run)
		}
	}
}

// freeSlot ist der kleinste Platz, den kein aktiver Lauf des Checkouts hält; unter g.mu.
func (g *Registry) freeSlot(root string) int {
	used := map[int]bool{}
	for _, run := range g.runs {
		if run.root == root && run.active {
			used[run.slot] = true
		}
	}
	slot := 0
	for used[slot] {
		slot++
	}
	return slot
}

func (g *Registry) active(root string) int {
	n := 0
	for _, run := range g.runs {
		if run.root == root && run.active {
			n++
		}
	}
	return n
}

func (g *Registry) find(root, id string) *Run {
	for _, run := range g.runs {
		if run.root == root && run.id == id {
			return run
		}
	}
	return nil
}

// Stop bricht einen Lauf ab und nennt seinen Zustand davor: ein wartender verschwindet aus dem Register, ein laufender
// heißt „abgebrochen“ und räumt im Hintergrund auf; sonst ändert sich nichts.
func (g *Registry) Stop(root, id string) (string, error) {
	g.mu.Lock()
	defer g.mu.Unlock()
	run := g.find(root, id)
	if run == nil {
		return "", ErrUnknown
	}
	switch run.state {
	case Waiting:
		g.runs = slices.DeleteFunc(g.runs, func(r *Run) bool { return r == run })
		g.log.Info("🧹 wartender testlauf entfernt", "ns", "simtest", "id", id)
		return Waiting, nil
	case Running:
		run.state = Canceled
		run.cancel()
		g.log.Info("🛑 testlauf abgebrochen", "ns", "simtest", "id", id)
		return Running, nil
	}
	return run.state, nil
}

// CancelAll beendet alle Läufe und verwirft die wartenden (Workbench stoppt).
func (g *Registry) CancelAll() {
	g.mu.Lock()
	defer g.mu.Unlock()
	g.runs = slices.DeleteFunc(g.runs, func(r *Run) bool { return r.state == Waiting })
	for _, run := range g.runs {
		if run.state == Running {
			run.cancel()
		}
	}
}

// Count ist die Zahl der gemerkten Läufe.
func (g *Registry) Count() int {
	g.mu.Lock()
	defer g.mu.Unlock()
	return len(g.runs)
}

// SetPhase setzt den Schritt des Runners, z. B. „bauen“.
func (g *Registry) SetPhase(run *Run, phase string) {
	g.mu.Lock()
	defer g.mu.Unlock()
	run.phase = phase
}

// SetLines setzt die Kennzahlen eines laufenden Laufs (Zwischenstand).
func (g *Registry) SetLines(run *Run, lines []string) {
	g.mu.Lock()
	defer g.mu.Unlock()
	run.lines = lines
}

// SetProgress setzt den Fortschritt (fertige und alle Szenarien).
func (g *Registry) SetProgress(run *Run, done, total int) {
	g.mu.Lock()
	defer g.mu.Unlock()
	run.done, run.total = done, total
}

// Finish setzt das Ende und schreibt den eigenen Bericht dir/simtest.md (dir relativ zum Checkout). Ein Abbruch über
// Stop gewinnt gegen ein späteres „fertig“ des Runners. Den Bericht nennt der Lauf erst, wenn die Datei steht.
func (g *Registry) Finish(run *Run, state, verdict string, lines []string, dir string) {
	g.mu.Lock()
	if run.state == Running {
		run.state = state
	}
	run.ended, run.verdict, run.lines = time.Now(), verdict, lines
	text := "# Testlauf " + run.id + "\n\n" + run.info().Head(time.Now()) + "\n\n- " + strings.Join(run.lines, "\n- ") + "\n"
	g.mu.Unlock()
	full := filepath.Join(run.root, dir)
	err := os.MkdirAll(full, 0o755)
	if err == nil {
		err = os.WriteFile(filepath.Join(full, "simtest.md"), []byte(text), 0o644)
	}
	if err != nil {
		g.log.Error("❌ testlauf-bericht nicht geschrieben", "ns", "simtest", "id", run.id, "error", err.Error())
	}
	g.mu.Lock()
	run.report = filepath.ToSlash(filepath.Join(dir, "simtest.md"))
	g.mu.Unlock()
}

// Get liefert den Stand eines Laufs.
func (g *Registry) Get(root, id string) (Info, error) {
	g.mu.Lock()
	defer g.mu.Unlock()
	run := g.find(root, id)
	if run == nil {
		return Info{}, ErrUnknown
	}
	return run.info(), nil
}

// List liefert die Läufe des Checkouts, neueste zuerst.
func (g *Registry) List(root string) []Info {
	g.mu.Lock()
	defer g.mu.Unlock()
	out := []Info{}
	for _, run := range slices.Backward(g.runs) {
		if run.root == root {
			out = append(out, run.info())
		}
	}
	return out
}

// ReportDir ist der Berichtsordner eines Laufs relativ zum Checkout.
func ReportDir(run *Run) string { return filepath.Join("reports", "simtest-"+run.id) }
