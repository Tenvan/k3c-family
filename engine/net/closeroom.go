package net

import (
	"errors"
	"net/http"

	"k3c/engine/room"
)

// closeRoom ist POST /api/room/close?slot=NAME (B-581, BD9.1, docs/protocol.md › HTTP: Raum schließen): Der offene Raum
// mit diesem Spielstand speichert, seine Geräte bekommen `room_closed` und gehen zurück zur Raumliste. Ohne Token wie
// DELETE /api/save; 404 ohne offenen Raum, 400 bei fehlendem oder ungültigem Namen.
func (s *server) closeRoom(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		fail(w, http.StatusMethodNotAllowed, "Nur POST")
		return
	}
	slot := r.URL.Query().Get("slot")
	err := s.cfg.Rooms.CloseRoom(slot)
	switch {
	case errors.Is(err, room.ErrBadRequest):
		fail(w, http.StatusBadRequest, "Ungültiger Slot")
	case errors.Is(err, room.ErrRoomNotFound):
		fail(w, http.StatusNotFound, "Kein Raum offen")
	case err != nil:
		s.log.Error("💥 Raum schließen: "+err.Error(), "ns", "room", "slot", slot)
		fail(w, http.StatusInternalServerError, "Schließen fehlgeschlagen")
	default:
		s.log.Info("🛑 Raum vom Gerät geschlossen", "ns", "room", "slot", slot, "remote", r.RemoteAddr)
		writeJSON(w, http.StatusOK, map[string]any{"ok": true, "slot": slot})
	}
}
