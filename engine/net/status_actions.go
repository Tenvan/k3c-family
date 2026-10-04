package net

import (
	"errors"
	"net/http"
	"strings"

	"k3c/engine/room"
)

// Aktionen der Diagnose (B-088): nur POST, nur mit Token, jede mit Log-Eintrag (ns diag). Die Antworten und das Log nennen
// Geräte nur mit der Kennung (Anfang der Geräte-ID), nie mit der vollen ID.

// roomOf liefert den Raum aus ?room=; false heißt: 404 ist geschrieben.
func (s *server) roomOf(w http.ResponseWriter, r *http.Request) (*room.Room, bool) {
	var found *room.Room
	if s.cfg.Rooms != nil {
		found = s.cfg.Rooms.Room(r.URL.Query().Get("room"))
	}
	if found == nil {
		fail(w, http.StatusNotFound, "Raum nicht gefunden")
		return nil, false
	}
	return found, true
}

// statusDisconnect ist POST /api/status/disconnect?room=CODE&device=KENNUNG: trennt die Verbindung des Geräts. Das Gerät
// verhält sich wie nach einem Verbindungsabbruch: Monarchen werden waiting, es darf sich bis zur Frist wieder verbinden.
func (s *server) statusDisconnect(w http.ResponseWriter, r *http.Request) {
	if !s.authorized(w, r, http.MethodPost) {
		return
	}
	rm, ok := s.roomOf(w, r)
	if !ok {
		return
	}
	kennung := r.URL.Query().Get("device")
	c, err := s.connectedDevice(rm, kennung)
	switch {
	case errors.Is(err, errAmbiguous):
		fail(w, http.StatusConflict, "Kennung nicht eindeutig")
	case err != nil:
		fail(w, http.StatusNotFound, "Gerät nicht gefunden oder nicht verbunden")
	default:
		c.cancel() // die Lese-Schleife endet, der Abbruch läuft über den normalen Weg (Drop)
		s.log.Info("👋 diagnose: gerät getrennt", "ns", "diag", "room", rm.Code, "device", kennung)
		writeJSON(w, http.StatusOK, map[string]any{"ok": true, "room": rm.Code, "device": kennung})
	}
}

var (
	errNoDevice  = errors.New("kein Gerät")
	errAmbiguous = errors.New("mehrdeutig")
)

// connectedDevice findet die verbundene Verbindung im Raum, deren Geräte-ID mit der Kennung beginnt.
func (s *server) connectedDevice(rm *room.Room, kennung string) (*conn, error) {
	if kennung == "" {
		return nil, errNoDevice
	}
	s.mu.Lock()
	defer s.mu.Unlock()
	var found *conn
	for c := range s.conns {
		if !strings.HasPrefix(c.device, kennung) || c.current() != rm {
			continue
		}
		if found != nil {
			return nil, errAmbiguous
		}
		found = c
	}
	if found == nil {
		return nil, errNoDevice
	}
	return found, nil
}

// statusSave ist POST /api/status/save?room=CODE: sichert den Spielstand des Raums sofort.
func (s *server) statusSave(w http.ResponseWriter, r *http.Request) {
	if !s.authorized(w, r, http.MethodPost) {
		return
	}
	rm, ok := s.roomOf(w, r)
	if !ok {
		return
	}
	backup, err := rm.SaveNow()
	switch {
	case errors.Is(err, room.ErrClosed):
		fail(w, http.StatusConflict, "Raum ist geschlossen")
	case err != nil:
		s.log.Error("💥 diagnose: spielstand nicht gesichert", "ns", "diag", "room", rm.Code, "err", err)
		fail(w, http.StatusInternalServerError, "Spielstand nicht gesichert")
	default:
		s.log.Info("💾 diagnose: spielstand gesichert", "ns", "diag", "room", rm.Code, "save", rm.Name)
		body := map[string]any{"ok": true, "room": rm.Code, "save": rm.Name}
		if backup != "" {
			body["backup"] = backup
		}
		writeJSON(w, http.StatusOK, body)
	}
}
