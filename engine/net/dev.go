package net

import (
	"encoding/json"
	"errors"
	"net/http"

	"k3c/engine/room"
)

// devBody ist eine Dev-Aktion der Seite /dm (B-232) als JSON; slot ist der Index des Monarchen.
type devBody struct {
	Action   string `json:"action"`
	Slot     *int   `json:"slot"`
	Amount   int    `json:"amount"`
	Resource string `json:"resource"`
	Factor   int    `json:"factor"`
	Paused   *bool  `json:"paused"`
	Phase    string `json:"phase"`
}

// dev ist /api/dev für die Seite /dm (B-232): GET ohne room liefert Dev-Mode und Raumliste, GET mit room die Diagnose
// des Raums, POST mit room eine Dev-Aktion (nur im Dev-Mode, sonst 403). Lesen geht ohne Dev-Mode, damit die Seite die
// Diagnose zeigt und die Aktionen ausgraut.
func (s *server) dev(w http.ResponseWriter, r *http.Request) {
	if s.cfg.Rooms == nil {
		http.NotFound(w, r)
		return
	}
	dev := s.cfg.Rooms.Dev
	if r.Method == http.MethodGet && r.URL.Query().Get("room") == "" {
		writeJSON(w, http.StatusOK, map[string]any{"dev": dev, "rooms": s.cfg.Rooms.Rooms()})
		return
	}
	if r.Method != http.MethodGet && r.Method != http.MethodPost {
		fail(w, http.StatusMethodNotAllowed, "Nur GET oder POST")
		return
	}
	rm, ok := s.roomOf(w, r)
	if !ok {
		return
	}
	if r.Method == http.MethodGet {
		writeJSON(w, http.StatusOK, map[string]any{"dev": dev, "room": rm.Summary()})
		return
	}
	s.devAction(w, r, rm)
}

// devAction liest die Aktion aus dem Body (höchstens 4 KB) und führt sie im Raum aus.
func (s *server) devAction(w http.ResponseWriter, r *http.Request, rm *room.Room) {
	var b devBody
	if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 4096)).Decode(&b); err != nil {
		fail(w, http.StatusBadRequest, "Aktion unlesbar")
		return
	}
	err := rm.DevPage(room.DevAction{Action: b.Action, Slot: b.Slot, Amount: b.Amount, Resource: b.Resource,
		Factor: b.Factor, Paused: b.Paused, Phase: b.Phase})
	switch {
	case errors.Is(err, room.ErrForbidden):
		fail(w, http.StatusForbidden, messages["forbidden"])
	case err != nil:
		fail(w, http.StatusBadRequest, "Aktion ungültig")
	default:
		writeJSON(w, http.StatusOK, map[string]any{"ok": true, "room": rm.Summary()})
	}
}
