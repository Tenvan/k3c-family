package net

import (
	"errors"
	"net/http"

	"k3c/engine/room"
	"k3c/engine/store"
)

// saves: GET /api/saves listet alle Spielstände mit Zeitpunkt, Version, Tag, Phase, Welt und Tiefen der Spieler (B-147,
// S2.2, B-506, docs/protocol.md › HTTP: Spielstände). Nur lesend, ohne Token wie GET /api/save; die Antwort ist immer ein Array.
func (s *server) saves(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet && r.Method != http.MethodHead {
		fail(w, http.StatusMethodNotAllowed, "Nur GET")
		return
	}
	w.Header().Set("Cache-Control", "no-store")
	writeJSON(w, http.StatusOK, s.cfg.Saves.List())
}

// deleteSave ist DELETE /api/save?slot=NAME (B-506, Entscheidung B-533): Der Stand wandert in seine Sicherungen. Ist er in
// einem Raum offen → 409, damit der Raum ihn nicht beim nächsten Speichern neu schreibt.
func (s *server) deleteSave(w http.ResponseWriter, slot string) {
	discard := func() error { return s.cfg.Saves.Discard(slot) }
	var err error
	if s.cfg.Rooms != nil {
		err = s.cfg.Rooms.DeleteSave(slot, discard)
	} else {
		err = discard()
	}
	switch {
	case errors.Is(err, room.ErrSaveInUse):
		fail(w, http.StatusConflict, "Spielstand ist in einem Raum offen")
	case errors.Is(err, room.ErrBadRequest):
		s.saveError(w, store.ErrSlot)
	case err != nil:
		s.saveError(w, err)
	default:
		s.log.Info("🧹 Spielstand gelöscht, liegt in den Sicherungen", "ns", "save", "slot", slot)
		writeJSON(w, http.StatusOK, map[string]any{"ok": true, "slot": slot})
	}
}
