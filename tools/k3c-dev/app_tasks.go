package main

import (
	"context"
	"fmt"
	"sync"
	"time"

	"k3c/tools/k3c-dev/internal/taskcat"
	"k3c/tools/k3c-dev/internal/taskrun"
)

// taskLoadTimeout fängt nur ein hängendes task.exe ab; ein Katalog braucht unter einer Sekunde.
const taskLoadTimeout = 10 * time.Second

// taskStopTimeout begrenzt einen Stopp aus der Oberfläche.
const taskStopTimeout = 15 * time.Second

// TaskCatalog ist der Katalog der Tasks-Seite. Error ist ein Feld, keine Ablehnung: nach einem Ladefehler bleibt
// der zuletzt bekannte Baum sichtbar.
type TaskCatalog struct {
	Namespaces []taskcat.Namespace `json:"namespaces"`
	Count      int                 `json:"count"`
	LoadedAt   string              `json:"loadedAt"` // RFC3339, leer solange nie geladen
	Error      string              `json:"error"`
}

type taskState struct {
	mu       sync.Mutex
	loaded   bool
	tasks    []taskcat.Task
	loadedAt time.Time
	err      error
}

func taskSource(name string) string { return "task:" + name }

// initTasks legt den Runner an; die Ausgabe jedes Laufs liegt als Konsolen-Quelle `task:<name>` im Puffer.
func (a *App) initTasks() {
	a.taskRunner = taskrun.New(taskrun.Options{
		Root:  a.root,
		Out:   func(name, stream, text string) { a.store.Add(taskSource(name), stream, text) },
		Reset: func(name string) { a.store.Reset(taskSource(name)) },
		OnChange: func(run taskrun.Run) {
			a.emit(a.ctx, evTaskState, run)
		},
	})
}

// Tasks liefert den Katalog und lädt ihn beim ersten Aufruf (Binding).
func (a *App) Tasks() TaskCatalog {
	a.wait()
	a.tasks.mu.Lock()
	defer a.tasks.mu.Unlock()
	if !a.tasks.loaded {
		a.loadTasksLocked()
	}
	return a.catalogLocked()
}

// TasksReload liest den Katalog neu (Binding); eine Taskfile-Änderung wirkt erst damit, kein Datei-Watcher.
func (a *App) TasksReload() TaskCatalog {
	a.wait()
	a.tasks.mu.Lock()
	defer a.tasks.mu.Unlock()
	a.loadTasksLocked()
	return a.catalogLocked()
}

func (a *App) loadTasksLocked() {
	ctx, cancel := context.WithTimeout(context.Background(), taskLoadTimeout)
	defer cancel()
	tasks, err := taskcat.Load(ctx, a.root)
	a.tasks.loaded = true
	if err != nil {
		a.tasks.err = err // der alte Stand bleibt stehen
		a.log.Warn("💥 Task-Katalog nicht ladbar: "+err.Error(), "ns", "tasks")
		return
	}
	a.tasks.tasks, a.tasks.loadedAt, a.tasks.err = tasks, time.Now(), nil
	a.log.Info("📂 Task-Katalog geladen", "ns", "tasks", "anzahl", len(tasks))
}

func (a *App) catalogLocked() TaskCatalog {
	c := TaskCatalog{Namespaces: taskcat.Group(a.tasks.tasks), Count: len(a.tasks.tasks)}
	if !a.tasks.loadedAt.IsZero() {
		c.LoadedAt = a.tasks.loadedAt.Format(time.RFC3339)
	}
	if a.tasks.err != nil {
		c.Error = a.tasks.err.Error()
	}
	return c
}

func (a *App) taskKnown(name string) bool {
	a.tasks.mu.Lock()
	defer a.tasks.mu.Unlock()
	if !a.tasks.loaded {
		a.loadTasksLocked()
	}
	for _, t := range a.tasks.tasks {
		if t.Name == name {
			return true
		}
	}
	return false
}

// TaskStart startet `task <name> [-- args...]` (Binding). Ein Name außerhalb des Katalogs wird vor dem Start
// abgelehnt, sonst stünde ein Lauf in der Liste, den task selbst erst nach dem Prozessstart verwirft.
func (a *App) TaskStart(name string, args []string) (taskrun.Run, error) {
	a.wait()
	if !a.taskKnown(name) {
		return taskrun.Run{}, fmt.Errorf("unbekannter Task %q (steht nicht im Katalog, task --list-all)", name)
	}
	return a.taskRunner.Start(name, args)
}

// TaskStop beendet den Prozessbaum eines laufenden Tasks (Binding).
func (a *App) TaskStop(name string) (taskrun.Run, error) {
	a.wait()
	ctx, cancel := context.WithTimeout(context.Background(), taskStopTimeout)
	defer cancel()
	return a.taskRunner.Stop(ctx, name)
}

// TaskRuns liefert den aktuellen oder letzten Lauf je Task (Binding).
func (a *App) TaskRuns() []taskrun.Run {
	a.wait()
	return a.taskRunner.All()
}
