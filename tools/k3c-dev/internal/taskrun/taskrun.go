// Package taskrun führt Taskfile-Tasks aus k3c-dev aus: Start, Stopp, Zustand und Exit-Code je Task, höchstens ein
// Lauf je Task. Kein Supervisor: ein Task endet mit einem Exit-Code und wird nie neu gestartet. Der Prozessbaum hängt
// über internal/proc in einem Job, Stopp beendet ihn ganz.
package taskrun

import (
	"context"
	"errors"
	"fmt"
	"sort"
	"sync"
	"time"

	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/proc"
)

// State ist der Zustand eines Laufs. Ein Task, der nie lief, hat keinen Eintrag; die Oberfläche zeigt ihn als bereit.
type State string

const (
	Running   State = "running"
	Succeeded State = "succeeded"
	Failed    State = "failed"
	Cancelled State = "cancelled"
)

// Run ist der aktuelle oder letzte Lauf eines Tasks.
type Run struct {
	Name       string    `json:"name"`
	State      State     `json:"state"`
	PID        int       `json:"pid"`
	Args       []string  `json:"args"`
	StartedAt  time.Time `json:"startedAt"`
	EndedAt    time.Time `json:"endedAt"` // Nullwert, solange er läuft
	ExitCode   int       `json:"exitCode"`
	DurationMs int64     `json:"durationMs"`
	Reason     string    `json:"reason"` // Grund eines Fehlschlags ohne Exit-Code (Prozess ließ sich nicht starten)
}

// Options konfigurieren einen Runner.
type Options struct {
	Root     string                                             // Arbeitsverzeichnis von `task`: die Repo-Wurzel
	Out      func(name, stream, text string)                    // eine Ausgabezeile des Tasks
	Reset    func(name string)                                  // vor einem neuen Lauf: Konsole leeren
	OnChange func(Run)                                          // jeder Zustandswechsel, nie unter der Sperre
	Command  func(ctx context.Context, argv []string) *proc.Cmd // Test-Naht; Vorgabe proc.Command
}

// ErrAlreadyRunning: derselbe Task läuft schon. ErrNotRunning: Stopp auf einen Task, der nicht läuft.
var (
	ErrAlreadyRunning = errors.New("task läuft bereits")
	ErrNotRunning     = errors.New("task läuft nicht")
)

type entry struct {
	run      Run
	cmd      *proc.Cmd
	stopping bool // angeforderter Stopp: das Ende zählt als Cancelled, gleich mit welchem Exit-Code
	done     chan struct{}
}

// Runner besitzt die gestarteten Läufe.
type Runner struct {
	opts Options
	mu   sync.Mutex
	runs map[string]*entry
}

// New legt einen Runner an.
func New(opts Options) *Runner {
	if opts.Command == nil {
		opts.Command = proc.Command
	}
	return &Runner{opts: opts, runs: map[string]*entry{}}
}

// Start führt `task <name> [-- args...]` aus. Ein Startfehler steht als Lauf im Zustand Failed mit Reason, damit die
// Oberfläche ihn in der Zeile zeigt statt nur in einem Hinweis.
func (r *Runner) Start(name string, args []string) (Run, error) {
	if err := ValidateArgs(args); err != nil {
		return Run{}, err
	}
	r.mu.Lock()
	if e, ok := r.runs[name]; ok && e.run.State == Running {
		run := e.run
		r.mu.Unlock()
		return run, fmt.Errorf("%w: %s (PID %d)", ErrAlreadyRunning, name, run.PID)
	}
	argv := []string{"task", name}
	if len(args) > 0 {
		argv = append(append(argv, "--"), args...)
	}
	if args == nil {
		args = []string{}
	}
	if r.opts.Reset != nil {
		r.opts.Reset(name)
	}
	// Das Zeitlimit liegt beim Aufrufer (Stopp), nicht hier: ein Dauerläufer wie `dev` soll laufen.
	cmd := r.opts.Command(context.Background(), argv)
	cmd.Dir = r.opts.Root
	cmd.Env = append(cmd.Environ(), "PW_TEST_HTML_REPORT_OPEN=never")
	out := console.NewLineWriter(func(t string) { r.line(name, "stdout", t) })
	errw := console.NewLineWriter(func(t string) { r.line(name, "stderr", t) })
	cmd.Stdout, cmd.Stderr = out, errw
	now := time.Now()
	if err := cmd.Start(); err != nil {
		run := Run{Name: name, State: Failed, Args: args, StartedAt: now, EndedAt: now, Reason: err.Error()}
		r.runs[name] = &entry{run: run}
		r.mu.Unlock()
		r.emit(run)
		return run, err
	}
	e := &entry{run: Run{Name: name, State: Running, PID: cmd.Process.Pid, Args: args, StartedAt: now},
		cmd: cmd, done: make(chan struct{})}
	r.runs[name] = e
	run := e.run
	r.mu.Unlock()
	r.emit(run)
	go r.waitFor(e, out, errw)
	return run, nil
}

func (r *Runner) line(name, stream, text string) {
	if r.opts.Out != nil {
		r.opts.Out(name, stream, text)
	}
}

func (r *Runner) waitFor(e *entry, out, errw *console.LineWriter) {
	waitErr := e.cmd.Wait() // WaitDelay begrenzt das Warten auf Pipes, die Enkel offen halten
	out.Flush()
	errw.Flush()
	r.mu.Lock()
	now := time.Now()
	e.run.EndedAt, e.run.DurationMs = now, now.Sub(e.run.StartedAt).Milliseconds()
	e.run.ExitCode = e.cmd.ProcessState.ExitCode()
	switch {
	case e.stopping:
		e.run.State = Cancelled
	case waitErr == nil:
		e.run.State = Succeeded
	default:
		e.run.State = Failed
		var exit interface{ ExitCode() int }
		if !errors.As(waitErr, &exit) {
			e.run.Reason = waitErr.Error()
		}
	}
	run := e.run
	r.mu.Unlock()
	close(e.done)
	r.emit(run)
}

// Stop beendet den Prozessbaum und wartet, bis der Lauf abgerechnet ist (höchstens bis ctx endet).
func (r *Runner) Stop(ctx context.Context, name string) (Run, error) {
	r.mu.Lock()
	e, ok := r.runs[name]
	if !ok || e.run.State != Running {
		var run Run
		if ok {
			run = e.run
		}
		r.mu.Unlock()
		return run, fmt.Errorf("%w: %s", ErrNotRunning, name)
	}
	e.stopping = true
	r.mu.Unlock()
	if err := e.cmd.Kill(); err != nil {
		return e.run, fmt.Errorf("task %s: %w", name, err)
	}
	select {
	case <-e.done:
	case <-ctx.Done():
	}
	r.mu.Lock()
	defer r.mu.Unlock()
	return e.run, nil
}

// StopAll beendet alle laufenden Tasks (Beenden der Anwendung); Fehler werden gesammelt.
func (r *Runner) StopAll(ctx context.Context) error {
	var errs []error
	for _, run := range r.All() {
		if run.State != Running {
			continue
		}
		if _, err := r.Stop(ctx, run.Name); err != nil && !errors.Is(err, ErrNotRunning) {
			errs = append(errs, err)
		}
	}
	return errors.Join(errs...)
}

// All liefert alle bekannten Läufe nach Name; leer ist ein leeres Array.
func (r *Runner) All() []Run {
	r.mu.Lock()
	runs := make([]Run, 0, len(r.runs))
	for _, e := range r.runs {
		runs = append(runs, e.run)
	}
	r.mu.Unlock()
	sort.Slice(runs, func(i, j int) bool { return runs[i].Name < runs[j].Name })
	return runs
}

func (r *Runner) emit(run Run) {
	if r.opts.OnChange != nil {
		r.opts.OnChange(run)
	}
}
