package net

import (
	"net/http"

	"k3c/engine/room"
)

// worlds: GET /api/worlds listet die Welten, die ein neuer Spielstand wählen darf (BD8.1, B-566, docs/protocol.md › HTTP:
// Welten). Nur lesend, ohne Token; ohne Raumverwaltung gibt es nur die Standard-Welt.
func (s *server) worlds(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet && r.Method != http.MethodHead {
		fail(w, http.StatusMethodNotAllowed, "Nur GET")
		return
	}
	list := []room.WorldInfo{{ID: "standard", Name: "Standard"}}
	if s.cfg.Rooms != nil {
		list = s.cfg.Rooms.WorldList()
	}
	writeJSON(w, http.StatusOK, list)
}
