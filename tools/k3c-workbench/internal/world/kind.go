package world

import (
	"context"
	"encoding/json"
)

// SetKindView setzt die Art der Welt id ("spinoff" oder "new"; leer entfernt das Feld, dann gilt spinoff mit extends,
// sonst new) wie world_set und liefert die Antwort für die Seite (nie ein Go-Fehler, B-1217).
func (e Editor) SetKindView(ctx context.Context, id, kind string) CreateView {
	var v any
	if kind != "" {
		v = kind
	}
	patch, _ := json.Marshal(map[string]any{"kind": v})
	warnings, err := e.Set(ctx, id, patch)
	view := CreateView{ID: id, Warnings: warnings}
	if view.Warnings == nil {
		view.Warnings = []string{}
	}
	if err != nil {
		view.Error = err.Error()
	}
	return view
}
