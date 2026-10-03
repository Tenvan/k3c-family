// Package taskcat liefert den Task-Katalog der Taskfile-Hierarchie: alle Tasks
// aus Taskfile.yml samt eingebundener Taskfiles, gruppiert nach dem Namensraum
// vor dem Doppelpunkt.
//
// Quelle ist `task --list-all --json`, kein eigener YAML-Parser: go-task löst
// Includes, Aliase und `internal: true` selbst auf; ein Parser hier müsste das
// nachbauen und bliebe dauerhaft hinterher.
package taskcat

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os/exec"
	"path/filepath"
	"strings"

	"k3c/tools/k3c-dev/internal/proc"
)

// Workspace ist der Namensraum der Tasks ohne Doppelpunkt im Namen - die
// Aggregate aus dem Root-Taskfile (`test`, `check`, `build`).
const Workspace = "Workspace"

// Task ist ein Eintrag des Katalogs.
type Task struct {
	// Name ist der vollständige Name, wie `task <Name>` ihn erwartet.
	Name string `json:"name"`
	// Namespace ist der Teil vor dem ersten Doppelpunkt, sonst Workspace.
	Namespace string `json:"namespace"`
	// Leaf ist der Rest nach dem ersten Doppelpunkt, sonst der ganze Name.
	Leaf    string   `json:"leaf"`
	Desc    string   `json:"desc"`
	Summary string   `json:"summary"`
	Aliases []string `json:"aliases"`
	// File ist das definierende Taskfile, relativ zur Repo-Wurzel mit `/`.
	File string `json:"file"`
	Line int    `json:"line"`
}

// Namespace ist eine Gruppe des Baums.
type Namespace struct {
	Name  string `json:"name"`
	Tasks []Task `json:"tasks"`
}

// listOutput ist die JSON-Form von `task --list-all --json` (go-task 3.53).
type listOutput struct {
	Tasks []struct {
		Name     string   `json:"name"`
		Desc     string   `json:"desc"`
		Summary  string   `json:"summary"`
		Aliases  []string `json:"aliases"`
		Location struct {
			Line     int    `json:"line"`
			Taskfile string `json:"taskfile"`
		} `json:"location"`
	} `json:"tasks"`
}

// Load ruft `task --list-all --json --no-status` in root auf und liefert den
// Katalog. Die Frist steuert der Aufrufer über ctx; bei Ablauf beendet exec
// den Prozess.
//
// --no-status ist keine Kür: Ohne das Flag berechnet go-task für jeden Task
// `up_to_date` und wertet dafür die `sources`-Globs aus - darunter
// `dist/production/server/node_modules/**/*` im Deploy-Taskfile. Gemessen am
// 2026-09-22: 47,9 s statt 0,19 s. Das Feld fehlt deshalb im Katalog; ein
// Wert, der immer false wäre, stünde als Falschaussage im Tooltip.
//
// exec statt proc.StartArgs: Der Aufruf ist kurz, startet keine Kinder und
// braucht weder Pipes noch ein eigenes Job-Objekt - das App-Job erbt er wie
// jeder Kindprozess. Nur das Konsolenfenster muss unterdrückt werden.
func Load(ctx context.Context, root string) ([]Task, error) {
	cmd := proc.Command(ctx, []string{"task", "--list-all", "--json", "--no-status"})
	cmd.Dir = root
	out, err := cmd.Output()
	if err != nil {
		return nil, describe(ctx, err)
	}
	return Parse(out, root)
}

// describe macht aus dem exec-Fehler einen Text, der die Ursache nennt: das
// gesuchte Programm, die abgelaufene Frist oder die Fehlerausgabe von task.
func describe(ctx context.Context, err error) error {
	if errors.Is(err, exec.ErrNotFound) {
		return fmt.Errorf("%q nicht im PATH: %w", "task", err)
	}
	if ctx.Err() != nil {
		return fmt.Errorf("%s: Frist abgelaufen: %w", listCommand, ctx.Err())
	}
	var exitErr *exec.ExitError
	if errors.As(err, &exitErr) {
		// Die Fehlerausgabe kommt unter Windows nicht zwingend als UTF-8.
		msg := strings.TrimSpace(strings.ToValidUTF8(string(exitErr.Stderr), "?"))
		if msg == "" {
			msg = exitErr.String()
		}
		return fmt.Errorf("%s: %s", listCommand, msg)
	}
	return fmt.Errorf("%s: %w", listCommand, err)
}

// listCommand ist der Aufruf in Fehlertexten - so, wie er sich in einer Shell
// nachstellen lässt.
const listCommand = "task --list-all --json --no-status"

// Parse wandelt die JSON-Ausgabe in Tasks. Taskfile-Pfade werden relativ zu
// root und mit `/` geschrieben; ein Pfad außerhalb von root bleibt absolut.
// Unbekannte Felder der Ausgabe (etwa up_to_date) werden ignoriert.
func Parse(data []byte, root string) ([]Task, error) {
	var out listOutput
	if err := json.Unmarshal(data, &out); err != nil {
		return nil, fmt.Errorf("%s: Ausgabe nicht lesbar: %w", listCommand, err)
	}
	tasks := make([]Task, 0, len(out.Tasks))
	for _, t := range out.Tasks {
		ns, leaf := split(t.Name)
		aliases := t.Aliases
		if aliases == nil {
			// JSON-Vertrag mit der Oberfläche: Array, kein null.
			aliases = []string{}
		}
		tasks = append(tasks, Task{
			Name:      t.Name,
			Namespace: ns,
			Leaf:      leaf,
			Desc:      t.Desc,
			Summary:   t.Summary,
			Aliases:   aliases,
			File:      relative(root, t.Location.Taskfile),
			Line:      t.Location.Line,
		})
	}
	return tasks, nil
}

// split trennt am ersten Doppelpunkt. `backend:test:vitest` gehört zu
// `backend` mit dem Blatt `test:vitest` - eine Ebene reicht dem Baum.
func split(name string) (ns, leaf string) {
	if i := strings.IndexByte(name, ':'); i > 0 {
		return name[:i], name[i+1:]
	}
	return Workspace, name
}

func relative(root, path string) string {
	if root == "" || path == "" {
		return filepath.ToSlash(path)
	}
	rel, err := filepath.Rel(root, path)
	if err != nil || strings.HasPrefix(rel, "..") {
		return filepath.ToSlash(path)
	}
	return filepath.ToSlash(rel)
}
