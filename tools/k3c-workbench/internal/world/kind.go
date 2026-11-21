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
	fields := map[string]any{"kind": v}
	// Ein Ableger braucht eine Eltern-Welt: ohne extends wird die Standard-Welt eingetragen (B-1217).
	if kind == "spinoff" && e.Load(id).Extends == "" {
		fields["extends"] = standardID
	}
	patch, _ := json.Marshal(fields)
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
