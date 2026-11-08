// Command k3c-sim ist das Engine-CLI der Workbench (B-726, RS2.1): Simulation, Balancing und Level-Erzeugung als
// Kindprozess statt als Import von engine/sim. Feste EXE bin/k3c-sim (`task k3c-sim:build`), kein go run.
//
// Schnittstelle (RS2.2 baut die Workbench darauf um):
//
//	k3c-sim run     [--input DATEI|-]          JSON {"biome","seed","ticks","inputs":[Segment…]} wie sim_run
//	k3c-sim level   [--input DATEI|-]          JSON {"biome","seed"} wie level_generate
//	k3c-sim balance [--input DATEI|-] [Flags]  Flags wie bin/k3c-balance (--targets, --report-dir, --seed-list …)
//
// Eingabe: JSON aus der Datei hinter --input, bei run und level ohne --input von stdin („-“ = stdin). Bei balance ist
// das JSON ein Objekt mit Flag-Namen als Schlüssel ({"targets": true, "seed-list": "k3c-dev", "vary": ["a=1"]});
// Flags in der Befehlszeile stehen danach: Einfache Werte gewinnen, wiederholbare (--vary, --param) kommen hinzu.
//
// Ausgabe auf stdout: nur JSON-Zeilen, je Zeile ein Objekt mit "event":
//
//	{"event":"progress","done":3,"total":6}          nach jedem fertigen Lauf (balance --targets --progress, auch --tune)
//	{"event":"result","text":"…"}                     Ergebnis von run und level (Text wie sim_run/level_generate)
//	{"event":"result","report":{…}}                   balance ohne --targets und ohne --out (Matrix-Bericht, --play)
//	{"event":"report","files":["…json","…md","…html"]} balance --targets: geschriebene Berichtsdateien
//	{"event":"error","message":"…"}                   vor Exit 1 oder 2
//	{"event":"aborted"}                               vor Exit 130
//
// Berichte schreibt balance nach --report-dir (Workbench: reports/simtest-<id>/), Dateien und Inhalt wie bin/k3c-balance.
// Meldungen für Menschen gehen nach stderr. Exit-Codes: 0 geschafft, 1 Lauf gescheitert, 2 Aufruf falsch (Unterbefehl,
// Flags, JSON), 130 abgebrochen (Strg+C, CTRL_BREAK, SIGTERM). Abbruch heißt Prozess-Ende: Die Workbench beendet den
// Prozess (Job-Objekt wie engine/devrun); k3c-sim hat keine Kindprozesse. Berichtsdateien entstehen erst nach der
// Rechenphase; ein Ende genau beim Schreiben kann einen unvollständigen Satz (.json ohne .md/.html) hinterlassen.
package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"os/signal"
	"sync"
	"sync/atomic"
	"syscall"
)

// Exit-Codes der Schnittstelle.
const (
	exitOK      = 0
	exitFailed  = 1
	exitUsage   = 2
	exitAborted = 130
)

// usageError ist ein Aufruffehler (Exit 2) statt eines gescheiterten Laufs (Exit 1).
type usageError struct{ err error }

func (e usageError) Error() string { return e.err.Error() }
func (e usageError) Unwrap() error { return e.err }

func usagef(format string, a ...any) error { return usageError{fmt.Errorf(format, a...)} }

// emitter schreibt JSON-Zeilen nach stdout; Fortschritt kann aus mehreren Goroutinen kommen.
type emitter struct {
	mu  sync.Mutex
	out io.Writer
}

func (e *emitter) emit(v any) {
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

func (e *emitter) progress(done, total int) {
	e.emit(progressEvent{Event: "progress", Done: done, Total: total})
}

// env ist die Umgebung eines Aufrufs (in Tests ohne echten Prozess).
type env struct {
	stdin  io.Reader
	stderr io.Writer
	out    *emitter
}

func (v env) logf(format string, a ...any) { _, _ = fmt.Fprintf(v.stderr, format+"\n", a...) }

func main() {
	stop := make(chan os.Signal, 1)
	signal.Notify(stop, os.Interrupt, syscall.SIGTERM)
	out := &emitter{out: os.Stdout}
	var finished atomic.Bool
	go func() {
		<-stop
		onAbort(out, os.Stderr, &finished, os.Exit)
	}()
	code := run(os.Args[1:], env{stdin: os.Stdin, stderr: os.Stderr, out: out})
	finished.Store(true)
	os.Exit(code)
}

// onAbort meldet den Abbruch (Zeile aborted, Exit 130), außer der Lauf ist schon fertig: Dann gilt dessen Exit-Code.
func onAbort(out *emitter, stderr io.Writer, finished *atomic.Bool, exit func(int)) {
	if finished.Load() {
		return
	}
	out.emit(map[string]string{"event": "aborted"})
	_, _ = fmt.Fprintln(stderr, "🛑 k3c-sim: abgebrochen")
	exit(exitAborted)
}

// run führt einen Unterbefehl aus und liefert den Exit-Code.
func run(args []string, v env) int {
	err := dispatch(args, v)
	if err == nil {
		return exitOK
	}
	v.out.emit(map[string]string{"event": "error", "message": err.Error()})
	v.logf("❌ k3c-sim: %v", err)
	var ue usageError
	if errors.As(err, &ue) {
		return exitUsage
	}
	return exitFailed
}

func dispatch(args []string, v env) error {
	if len(args) == 0 {
		return usagef("Unterbefehl fehlt: run, balance oder level")
	}
	switch args[0] {
	case "run":
		return cmdRun(args[1:], v)
	case "level":
		return cmdLevel(args[1:], v)
	case "balance":
		return cmdBalance(args[1:], v)
	}
	return usagef("unbekannter Unterbefehl %q (bekannt: run, balance, level)", args[0])
}
