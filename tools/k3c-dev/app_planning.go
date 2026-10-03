package main

import "k3c/tools/k3c-dev/internal/planning"

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
