package net

import "net/http"

// saves: GET /api/saves listet alle Spielstände mit Zeitpunkt, Version, Tag, Phase und Tiefen der Spieler (B-147, S2.2,
// docs/protocol.md › HTTP: Spielstände). Nur lesend, ohne Token wie GET /api/save; die Antwort ist immer ein Array.
func (s *server) saves(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet && r.Method != http.MethodHead {
		fail(w, http.StatusMethodNotAllowed, "Nur GET")
		return
	}
	w.Header().Set("Cache-Control", "no-store")
	writeJSON(w, http.StatusOK, s.cfg.Saves.List())
}
