package simcli

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"runtime/debug"
	"sync"

	"k3c/data"
	"k3c/engine/balance"
)

// Die Unterbefehle des Engine-CLI (B-726, B-924): run, level, balance, world, hub. cmd/k3c-sim ist die dünne Hülle für
// Taskfile und CI; die Workbench ruft dieselbe Logik im Prozess auf (call.go). Die JSON-Schnittstelle (Eingabe, Zeilen
// auf stdout, Exit-Codes) beschreibt der Kopfkommentar von cmd/k3c-sim/main.go.

// Exit-Codes der Schnittstelle.
const (
	ExitOK      = 0
	ExitFailed  = 1 // Lauf gescheitert oder Panik der Engine (error-Zeile, Stack auf stderr)
	ExitUsage   = 2
	ExitAborted = 130
)

// usageError ist ein Aufruffehler (Exit 2) statt eines gescheiterten Laufs (Exit 1).
type usageError struct{ err error }

func (e usageError) Error() string { return e.err.Error() }
func (e usageError) Unwrap() error { return e.err }

func usagef(format string, a ...any) error { return usageError{fmt.Errorf(format, a...)} }

// Emitter schreibt JSON-Zeilen; Fortschritt kann aus mehreren Goroutinen kommen.
type Emitter struct {
	mu  sync.Mutex
	out io.Writer
}

// NewEmitter schreibt die JSON-Zeilen nach w.
func NewEmitter(w io.Writer) *Emitter { return &Emitter{out: w} }

// Emit schreibt v als eine JSON-Zeile.
func (e *Emitter) Emit(v any) {
	b, err := json.Marshal(v)
	if err != nil {
		b, _ = json.Marshal(map[string]string{"event": "error", "message": err.Error()})
	}
	e.mu.Lock()
	defer e.mu.Unlock()
	_, _ = e.out.Write(append(b, '\n'))
}

// progressEvent ist die Fortschrittszeile; Feldfolge fest, "event" zuerst.
type progressEvent struct {
	Event string `json:"event"`
	Done  int    `json:"done"`
	Total int    `json:"total"`
}

func (e *Emitter) progress(done, total int) {
	e.Emit(progressEvent{Event: "progress", Done: done, Total: total})
}

// env ist die Umgebung eines Aufrufs (in Tests ohne echten Prozess).
type env struct {
	stdin  io.Reader
	stderr io.Writer
	out    *Emitter
	files  data.FS         // Daten für world und hub; nil = data.Files des Prozesses
	ctx    context.Context // Abbruch zwischen zwei Läufen von balance; nil = nie
}

func (v env) logf(format string, a ...any) { _, _ = fmt.Fprintf(v.stderr, format+"\n", a...) }

// dataFiles sind die Daten, aus denen world und hub lesen.
func (v env) dataFiles() data.FS {
	if v.files != nil {
		return v.files
	}
	return data.Files
}

// aborted ist die Panik, mit der ein Fortschritts-Rückruf einen Lauf nach Abbruch des Contexts beendet; run fängt sie.
type aborted struct{ err error }

// checkAbort beendet den Lauf (Panik aborted), wenn der Context fertig ist. Die Rechner von engine/balance laufen in
// Goroutinen und fangen die Panik ab (parallel.go); sie kommt in run wieder an.
func (v env) checkAbort() {
	if v.ctx != nil && v.ctx.Err() != nil {
		panic(aborted{v.ctx.Err()})
	}
}

// Exec führt einen Unterbefehl mit eigenen Ein- und Ausgaben aus und liefert den Exit-Code (ExitOK …); files nil =
// data.Files des Prozesses, ctx nil = nicht abbrechbar (Aufrufer sollten context.Background() geben).
func Exec(ctx context.Context, args []string, stdin io.Reader, stderr io.Writer, out *Emitter, files data.FS) int {
	return run(args, env{stdin: stdin, stderr: stderr, out: out, files: files, ctx: ctx})
}

// run führt einen Unterbefehl aus und liefert den Exit-Code; eine Panik wird zum Fehler (Exit 1), nie zum Absturz.
func run(args []string, v env) (code int) {
	var err error
	defer func() {
		if r := recover(); r != nil {
			err = v.crashed(r)
		}
		if err == nil {
			code = ExitOK
			return
		}
		v.logf("❌ k3c-sim: %v", err)
		emitError(v.out, err)
		var ue usageError
		code = ExitFailed
		if errors.As(err, &ue) {
			code = ExitUsage
		}
	}()
	err = dispatch(args, v)
	return code
}

// emitError schreibt die error-Zeile; stürzt dabei der Empfänger ab (Rückruf der Workbench), bleibt der Fehler auf stderr.
func emitError(out *Emitter, err error) {
	defer func() { _ = recover() }()
	out.Emit(map[string]string{"event": "error", "message": err.Error()})
}

// crashed macht den Wert einer Panik zum Fehler; aborted (Abbruch über den Context) behält dessen Fehler. Jede andere Panik
// schreibt Wert und Stack nach stderr (Ort der Engine-Panik, auch aus Worker-Goroutinen von engine/balance, balance.Crash)
// und endet als Exit 1 mit error-Zeile, nie als Absturz des Prozesses.
func (v env) crashed(r any) error {
	stack := debug.Stack()
	if c, ok := r.(*balance.Crash); ok {
		r, stack = c.Value, c.Stack
	}
	if a, ok := r.(aborted); ok {
		return a.err
	}
	v.logf("💥 k3c-sim: Panik: %v\n%s", r, stack)
	if e, ok := r.(error); ok {
		return fmt.Errorf("💥 Absturz in der Engine: %w", e)
	}
	return fmt.Errorf("💥 Absturz in der Engine: %v", r)
}

func dispatch(args []string, v env) error {
	if len(args) == 0 {
		return usagef("Unterbefehl fehlt: run, balance, level, world oder hub")
	}
	switch args[0] {
	case "run":
		return cmdRun(args[1:], v)
	case "level":
		return cmdLevel(args[1:], v)
	case "balance":
		return cmdBalance(args[1:], v)
	case "world":
		return cmdWorld(args[1:], v)
	case "hub":
		return cmdHub(args[1:], v)
	}
	return usagef("unbekannter Unterbefehl %q (bekannt: run, balance, level, world, hub)", args[0])
}

// tick ist der Rückruf, den engine/balance nach jedem fertigen Lauf aufruft (Targets.Tick, Matrix.Tick); nil ohne Context.
func (v env) tick() func() {
	if v.ctx == nil {
		return nil
	}
	return v.checkAbort
}
