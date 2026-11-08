// Package simexe ruft das Engine-CLI bin/k3c-sim als Kindprozess auf (B-726, RS2.2): sim_run, level_generate,
// replay_run und die Bot-Entscheider der Online-Testläufe. Die Workbench importiert die Engine nicht; ob k3c-sim in Go
// oder Rust rechnet, sieht sie nicht. Schnittstelle: Kopfkommentar von cmd/k3c-sim/main.go.
package simexe

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"time"

	"k3c/data"
	"k3c/tools/k3c-workbench/internal/console"
	"k3c/tools/k3c-workbench/internal/proc"
)

// BuildHint ist der Befehl, der die EXE baut.
const BuildHint = "task k3c-sim:build"

// tailLines ist die Zahl der stderr-Zeilen, die ein Fehler nennt.
const tailLines = 5

// Event ist eine JSON-Zeile von k3c-sim auf stdout.
type Event struct {
	Event    string          `json:"event"`
	Done     int             `json:"done,omitempty"`
	Total    int             `json:"total,omitempty"`
	Text     string          `json:"text,omitempty"`
	Message  string          `json:"message,omitempty"`
	Files    []string        `json:"files,omitempty"`
	Report   json.RawMessage `json:"report,omitempty"`
	Commands []Command       `json:"commands,omitempty"`
}

// ParseEvent liest eine stdout-Zeile; false, wenn sie keine JSON-Zeile mit "event" ist.
func ParseEvent(line string) (Event, bool) {
	var ev Event
	if json.Unmarshal([]byte(line), &ev) != nil || ev.Event == "" {
		return Event{}, false
	}
	return ev, true
}

// Rel ist der Pfad der EXE relativ zum Checkout; slot > 0 sind die Plätze der Offline-Läufe (bin/k3c-sim-<slot>),
// die die Workbench selbst baut, damit ein laufender Prozess den Bau eines anderen Platzes nicht sperrt.
func Rel(slot int) string {
	name := "k3c-sim"
	if slot > 0 {
		name += fmt.Sprintf("-%d", slot)
	}
	if runtime.GOOS == "windows" {
		name += ".exe"
	}
	return filepath.Join("bin", name)
}

// Env ist die Umgebung eines Aufrufs: k3c-sim liest data/ des Checkouts von der Platte (nicht die eingebettete Kopie).
func Env(root string) []string {
	return []string{data.DirEnv + "=" + filepath.Join(root, "data")}
}

// sources sind die Quellen, aus denen k3c-sim gebaut wird (wie cmd/k3c-sim/Taskfile.yml ohne data/, das k3c-sim zur
// Laufzeit liest).
var sources = []string{"cmd/k3c-sim", "engine", "go.mod", "go.sum"}

// Check prüft, ob bin/k3c-sim im Checkout steht und nicht älter als seine Go-Quellen ist.
func Check(root string) error {
	exe := filepath.Join(root, Rel(0))
	st, err := os.Stat(exe)
	if err != nil {
		return fmt.Errorf("%s fehlt im Checkout: erst `%s`", filepath.ToSlash(Rel(0)), BuildHint)
	}
	if newer := newerSource(root, st.ModTime()); newer != "" {
		return fmt.Errorf("%s ist veraltet (%s ist neuer): erst `%s` (bei unverändertem Inhalt mit --force)",
			filepath.ToSlash(Rel(0)), newer, BuildHint)
	}
	return nil
}

// newerSource liefert die erste Go-Quelle (ohne Tests) bzw. go.mod/go.sum, die nach t geändert wurde; leer = keine.
func newerSource(root string, t time.Time) string {
	found := ""
	for _, src := range sources {
		_ = filepath.WalkDir(filepath.Join(root, src), func(path string, d fs.DirEntry, err error) error {
			if err != nil || found != "" {
				return filepath.SkipAll
			}
			name := d.Name()
			if d.IsDir() || strings.HasSuffix(name, "_test.go") ||
				(!strings.HasSuffix(name, ".go") && name != "go.mod" && name != "go.sum") {
				return nil
			}
			if info, err := d.Info(); err == nil && info.ModTime().After(t) {
				rel, _ := filepath.Rel(root, path)
				found = filepath.ToSlash(rel)
			}
			return nil
		})
	}
	return found
}

// Call führt einen kurzen Unterbefehl aus (run, level, balance --play): input auf stdin, Ergebnis ist die Zeile
// result. Fehler nennen die Meldung von k3c-sim bzw. die letzten stderr-Zeilen (Absturz) oder das Zeitlimit.
func Call(ctx context.Context, root string, args []string, input []byte, timeout time.Duration) (Event, error) {
	if err := Check(root); err != nil {
		return Event{}, err
	}
	ctx, cancel := context.WithTimeout(ctx, timeout)
	defer cancel()
	cmd := proc.Command(ctx, append([]string{filepath.Join(root, Rel(0))}, args...))
	cmd.Dir = root
	cmd.Env = append(os.Environ(), Env(root)...)
	cmd.Stdin = strings.NewReader(string(input))
	var result, failure *Event
	var tail []string
	stdout := console.NewLineWriter(func(line string) {
		if ev, ok := ParseEvent(line); ok {
			switch ev.Event {
			case "result":
				result = &ev
			case "error":
				failure = &ev
			}
		}
	})
	stderr := console.NewLineWriter(func(line string) { tail = last(append(tail, line), tailLines) })
	cmd.Stdout, cmd.Stderr = stdout, stderr
	err := cmd.Run()
	stdout.Flush()
	stderr.Flush()
	name := "k3c-sim " + args[0]
	switch {
	case errors.Is(ctx.Err(), context.DeadlineExceeded):
		return Event{}, fmt.Errorf("%s: Zeitlimit %s erreicht", name, timeout)
	case failure != nil:
		return Event{}, fmt.Errorf("%s", failure.Message)
	case err != nil && cmd.ProcessState == nil:
		return Event{}, fmt.Errorf("%s nicht gestartet: %w", name, err)
	case err != nil || result == nil:
		return Event{}, crashError(name, cmd.ProcessState.ExitCode(), tail)
	}
	return *result, nil
}

// crashError beschreibt einen Prozess, der ohne Ergebnis endete.
func crashError(name string, exit int, tail []string) error {
	msg := fmt.Sprintf("%s ohne Ergebnis beendet (Exit %d)", name, exit)
	if len(tail) > 0 {
		msg += ": " + strings.Join(tail, " | ")
	}
	return errors.New(msg)
}

func last(lines []string, n int) []string {
	if len(lines) > n {
		return lines[len(lines)-n:]
	}
	return lines
}
