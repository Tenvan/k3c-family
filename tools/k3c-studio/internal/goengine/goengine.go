// Package goengine ist die Go-Engine im Prozess (engine/simcli, B-924) als Ersatz für die Spiel-EXE in Tests: Welten-Editor,
// Gebäude-Planer und ihre MCP-Tools rechnen seit RU34.3 über `k3c-game sim` (internal/gameexe); die Tests brauchen keine
// gebaute EXE und nehmen die Go-Engine als Referenz (Entscheidung 012: Orakel ist das Verhalten), bis RU41 sie entfernt.
// Nur Tests importieren das Paket.
package goengine

import (
	"context"
	"encoding/json"
	"path/filepath"

	"k3c/engine/simcli"
)

// Call liefert die Engine-Seite für den Unterbefehl sub (world oder hub) mit den Daten des Checkouts root; sie passt auf
// world.Engine und hubplan.Engine. Eine Panik der Engine wird zum Fehler (recover in simcli.Exec).
func Call(sub, root string) func(ctx context.Context, input []byte) (json.RawMessage, error) {
	return func(ctx context.Context, input []byte) (json.RawMessage, error) {
		ev, err := simcli.Call(ctx, []string{sub}, input, simcli.DirFiles(filepath.Join(root, "data")))
		return ev.Report, err
	}
}
