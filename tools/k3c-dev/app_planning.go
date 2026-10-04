package main

import (
	"time"

	"k3c/tools/k3c-dev/internal/github"
	"k3c/tools/k3c-dev/internal/planning"
)

// planningPoll ist der Takt des Planungs-Wächters: Änderungen an docs/ erreichen die Oberfläche als Ereignis
// `planning:changed`, auch wenn die Seite nicht offen ist (sie lädt dann beim Öffnen ohnehin frisch).
const planningPoll = 2 * time.Second

// PlanningData liest Sprints und Tickets frisch aus docs/ (Binding), dieselben Daten wie plan_list: die Dateien sind
// die Quelle, ein Cache würde nur nach einem Commit oder einer Session falsch liegen.
func (a *App) PlanningData() (planning.Data, error) {
	a.wait()
	return planning.Load(a.root)
}

// PlanningDocs nennt die vorhandenen Dokumente in Umschalter-Reihenfolge (Binding); das Glossar nur mit Datei.
func (a *App) PlanningDocs() []string {
	a.wait()
	return planning.Available(a.root)
}

// PlanningDoc liefert eines der Dokumente `plan`, `fragen` oder `glossar` als Markdown (Binding).
func (a *App) PlanningDoc(name string) (string, error) {
	a.wait()
	return planning.Doc(a.root, name)
}

// GitHubStatus liefert PR, CI und Merge-Stand je Sprint aus `gh` (Binding, B-212); force umgeht den Zwischenspeicher.
func (a *App) GitHubStatus(force bool) github.Data {
	a.wait()
	return a.gh.Status(a.ctx, force)
}
