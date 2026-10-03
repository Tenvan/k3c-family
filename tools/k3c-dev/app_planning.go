package main

import (
	"time"

	"k3c/tools/k3c-dev/internal/planning"
)

// planningPoll ist der Takt des Planungs-Wächters: Änderungen an docs/ erreichen die Oberfläche als Ereignis
// `planning:changed`, auch wenn die Seite nicht offen ist (sie lädt dann beim Öffnen ohnehin frisch).
const planningPoll = 2 * time.Second

// Planning liest Sprints und Tickets frisch von der Platte (Binding): die Dateien sind die Quelle, ein Cache würde
// nur nach einem Commit oder einer Session falsch liegen.
func (a *App) Planning() (planning.Data, error) {
	a.wait()
	return planning.Load(a.root)
}

// PlanningDoc liefert eines der Dokumente `plan` oder `fragen` als Markdown (Binding).
func (a *App) PlanningDoc(name string) (string, error) {
	a.wait()
	return planning.Doc(a.root, name)
}
