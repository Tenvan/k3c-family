package simcli

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"log/slog"
	"os"
	"strings"
	"sync"
	"sync/atomic"
	"time"

	"k3c/data"
)

// Aufruf im Prozess (B-924): Die Workbench ruft die Unterbefehle ohne Kindprozess auf. Das Ergebnis sind dieselben
// JSON-Zeilen wie auf stdout der EXE, hier als Event. Jeder Aufruf läuft in einer eigenen Goroutine mit recover
// (Panik wird Fehler, nie Absturz der Workbench); der Context bricht ab (siehe Stream).

// Event ist eine JSON-Zeile von stdout.
type Event struct {
	Event   string          `json:"event"`
	Done    int             `json:"done,omitempty"`
	Total   int             `json:"total,omitempty"`
	Text    string          `json:"text,omitempty"`
	Message string          `json:"message,omitempty"`
	Files   []string        `json:"files,omitempty"`
	Report  json.RawMessage `json:"report,omitempty"`
}

// ParseEvent liest eine stdout-Zeile; false, wenn sie keine JSON-Zeile mit "event" ist.
func ParseEvent(line string) (Event, bool) {
	var ev Event
	if json.Unmarshal([]byte(line), &ev) != nil || ev.Event == "" {
		return Event{}, false
	}
	return ev, true
}

// eventWriter zerlegt die Ausgabe des Emitters in Zeilen und reicht jedes Event an on; nach close kommt keines mehr.
type eventWriter struct {
	on     func(Event)
	mu     sync.Mutex // macht „nicht geschlossen“ und on(ev) zu einem Schritt: nach close() läuft kein Rückruf mehr, auch keiner im Flug
	closed bool
	buf    bytes.Buffer
}

// deliver reicht ev an on, solange der Writer offen ist.
func (w *eventWriter) deliver(ev Event) {
	w.mu.Lock()
	defer w.mu.Unlock()
	if !w.closed {
		w.on(ev)
	}
}

func (w *eventWriter) close() {
	w.mu.Lock()
	w.closed = true
	w.mu.Unlock()
}

func (w *eventWriter) Write(p []byte) (int, error) {
	w.buf.Write(p) // der Emitter schreibt je Zeile unter Sperre, daher kein zweiter Schreiber
	for {
		line, err := w.buf.ReadString('\n')
		if err != nil {
			w.buf.WriteString(line)
			break
		}
		if ev, ok := ParseEvent(strings.TrimSpace(line)); ok {
			w.deliver(ev)
		}
	}
	return len(p), nil
}

// abortGrace ist die Frist, die Stream nach einem Abbruch auf das Ende der Rechnung wartet.
const abortGrace = 30 * time.Second

// Stream führt den Unterbefehl args aus (input auf stdin) und reicht jedes Event an on (aus der Rechen-Goroutine, auch
// aus mehreren zugleich). files sind die Daten für world und hub (nil = data.Files des Prozesses). Ende:
//   - nil: Exit 0
//   - ctx.Err(): der Context ist fertig. balance (--targets, --tune, Matrix, Sensitivität) endet beim nächsten fertigen Lauf
//     und schreibt danach keinen Bericht; Stream wartet darauf bis abortGrace. Andere Unterbefehle sind kurz; läuft
//     eine Rechnung länger, läuft sie im Hintergrund zu Ende, ihre Events gehen verloren.
//   - sonst die Meldung der error-Zeile bzw. Exit-Code und Panik.
func Stream(ctx context.Context, args []string, input []byte, files data.FS, on func(Event)) error {
	if len(args) == 0 {
		return errors.New("k3c-sim: Unterbefehl fehlt")
	}
	var stderr bytes.Buffer
	var failure atomic.Value
	w := &eventWriter{on: func(ev Event) {
		if ev.Event == "error" {
			failure.Store(ev.Message)
		}
		on(ev)
	}}
	done := make(chan int, 1)
	go func() { done <- Exec(ctx, args, bytes.NewReader(input), &stderr, NewEmitter(w), files) }()
	select {
	case <-ctx.Done():
		w.close()
		// balance bricht beim nächsten fertigen Lauf ab (Tick) und schreibt dann keinen Bericht mehr; so lange warten wir
		// (kurze Frist), damit der Aufrufer den Platz erst danach freigibt.
		select {
		case <-done:
		case <-time.After(abortGrace):
		}
		return ctx.Err()
	case code := <-done:
		w.close()
		if ctx.Err() != nil {
			return ctx.Err()
		}
		if strings.Contains(stderr.String(), "💥 k3c-sim: Panik") { // Ort der Panik (Stack) bleibt im Log der Workbench sichtbar
			slog.Error("💥 Panik in der Engine ("+args[0]+")", "ns", "diag", "stderr", stderr.String())
		}
		if code == ExitOK {
			return nil
		}
		if msg, ok := failure.Load().(string); ok && msg != "" {
			return errors.New(msg)
		}
		return fmt.Errorf("k3c-sim %s: Exit %d: %s", args[0], code, strings.TrimSpace(stderr.String()))
	}
}

// Call führt einen kurzen Unterbefehl aus (run, level, world, hub, balance --play) und liefert die Zeile result.
func Call(ctx context.Context, args []string, input []byte, files data.FS) (Event, error) {
	var result *Event
	err := Stream(ctx, args, input, files, func(ev Event) {
		if ev.Event == "result" {
			result = &ev
		}
	})
	switch {
	case err != nil:
		return Event{}, err
	case result == nil:
		return Event{}, fmt.Errorf("k3c-sim %s: ohne Ergebnis beendet", args[0])
	}
	return *result, nil
}

// DirFiles sind die Daten aus dem Ordner dir (data/ eines Checkouts), von der Platte bei jedem Lesen; nil, wenn der
// Ordner fehlt (dann gilt data.Files des Prozesses).
func DirFiles(dir string) data.FS {
	if info, err := os.Stat(dir); err != nil || !info.IsDir() {
		return nil
	}
	fsys, _ := os.DirFS(dir).(data.FS)
	return fsys
}
