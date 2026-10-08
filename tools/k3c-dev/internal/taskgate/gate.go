// Package taskgate ist das Freigabe-Schloss der Tasks (Workbench-Spec § 4): ob ein Agent einen Task über task_start
// starten darf. Die Freigabedatei hält den dauerhaften Stand, Laufzeit-Schalter gelten bis zum Beenden von k3c-dev.
package taskgate

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"slices"
	"sort"
	"sync"
)

// FileName ist die Freigabedatei neben services.json.
const FileName = "task-freigaben.json"

// State ist einer der vier Zustände des Schlosses.
type State string

const (
	Open       State = "open"        // laut Datei frei
	Closed     State = "closed"      // laut Datei gesperrt
	OpenTemp   State = "open-temp"   // gesperrt laut Datei, bis zum Beenden frei
	ClosedTemp State = "closed-temp" // frei laut Datei, bis zum Beenden gesperrt
)

// Allowed meldet, ob der Zustand einen Start durch einen Agenten erlaubt.
func (s State) Allowed() bool { return s == Open || s == OpenTemp }

type file struct {
	Allowed []string `json:"allowed"`
}

// Gate hält Datei- und Laufzeitstand; sicher für mehrere Goroutinen (Oberfläche und MCP).
type Gate struct {
	path    string
	mu      sync.Mutex
	file    map[string]bool
	runtime map[string]bool // gesetzte Schalter, abweichend von der Datei
}

// Load liest die Freigabedatei; fehlt sie, ist nichts frei.
func Load(path string) (*Gate, error) {
	g := &Gate{path: path, file: map[string]bool{}, runtime: map[string]bool{}}
	data, err := os.ReadFile(path)
	if errors.Is(err, os.ErrNotExist) {
		return g, nil
	}
	if err != nil {
		return g, err
	}
	var f file
	if err := json.Unmarshal(data, &f); err != nil {
		return g, fmt.Errorf("%s: %w", path, err)
	}
	for _, n := range f.Allowed {
		g.file[n] = true
	}
	return g, nil
}

// State ist der Zustand eines Tasks.
func (g *Gate) State(name string) State {
	g.mu.Lock()
	defer g.mu.Unlock()
	return g.stateLocked(name)
}

func (g *Gate) stateLocked(name string) State {
	inFile := g.file[name]
	on, set := g.runtime[name]
	switch {
	case !set && inFile:
		return Open
	case !set:
		return Closed
	case on:
		return OpenTemp
	default:
		return ClosedTemp
	}
}

// Toggle kehrt die Wirkung eines Tasks bis zum Beenden um; zurück auf dem Dateistand entfällt der Schalter.
func (g *Gate) Toggle(name string) State {
	g.mu.Lock()
	defer g.mu.Unlock()
	next := !g.stateLocked(name).Allowed()
	if next == g.file[name] {
		delete(g.runtime, name)
	} else {
		g.runtime[name] = next
	}
	return g.stateLocked(name)
}

// Pending ist die Zahl der Laufzeit-Schalter, die „Schalter übernehmen“ in die Datei schreiben würde.
func (g *Gate) Pending() int {
	g.mu.Lock()
	defer g.mu.Unlock()
	return len(g.runtime)
}

// Save übernimmt die Laufzeit-Schalter in die Datei und schreibt sie sortiert.
func (g *Gate) Save() error {
	g.mu.Lock()
	defer g.mu.Unlock()
	next := map[string]bool{}
	for n, on := range g.file {
		next[n] = on
	}
	for n, on := range g.runtime {
		next[n] = on
	}
	f := file{Allowed: []string{}}
	for n, on := range next {
		if on {
			f.Allowed = append(f.Allowed, n)
		}
	}
	sort.Strings(f.Allowed)
	data, err := json.MarshalIndent(f, "", "  ")
	if err != nil {
		return err
	}
	if err := os.WriteFile(g.path, append(data, '\n'), 0o644); err != nil {
		return err
	}
	g.file, g.runtime = map[string]bool{}, map[string]bool{}
	for _, n := range f.Allowed {
		g.file[n] = true
	}
	return nil
}

// AllowedNames sind alle Tasks, die gerade starten dürfen, sortiert.
func (g *Gate) AllowedNames(all []string) []string {
	out := []string{}
	for _, n := range all {
		if g.State(n).Allowed() {
			out = append(out, n)
		}
	}
	return slices.Clip(out)
}
