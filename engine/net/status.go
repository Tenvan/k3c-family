package net

import (
	"crypto/subtle"
	"errors"
	"net/http"
	"strings"
	"time"

	"k3c/engine/store"
)

// status ist GET /api/status (B-027): nur mit Authorization: Bearer <K3C_STATUS_TOKEN>. Ohne gesetztes Token ist die
// Diagnose aus (404, Entscheidung 🧑), der Spielbetrieb läuft normal.
func (s *server) status(w http.ResponseWriter, r *http.Request) {
	if s.cfg.StatusToken == "" {
		http.NotFound(w, r)
		return
	}
	given, ok := strings.CutPrefix(r.Header.Get("Authorization"), "Bearer ")
	if !ok || subtle.ConstantTimeCompare([]byte(given), []byte(s.cfg.StatusToken)) != 1 {
		w.Header().Set("WWW-Authenticate", `Bearer realm="k3c"`)
		fail(w, http.StatusUnauthorized, "Token fehlt oder falsch")
		return
	}
	if r.Method != http.MethodGet {
		fail(w, http.StatusMethodNotAllowed, "Nur GET")
		return
	}
	started := s.cfg.StartedAt
	writeJSON(w, http.StatusOK, map[string]any{
		"version": s.cfg.Version, "startedAt": started.UTC().Format(time.RFC3339),
		"uptimeS": int64(time.Since(started).Seconds()),
		"saves":   s.cfg.Saves.Count(), "reports": s.cfg.Reports.Count(),
	})
}

// backups ist GET /api/save/backups?slot= (B-028): Sicherungen eines Spielstands, neueste zuerst.
func (s *server) backups(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		fail(w, http.StatusMethodNotAllowed, "Nur GET")
		return
	}
	list, err := s.cfg.Saves.Backups(slotOf(r))
	if err != nil {
		s.saveError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, list)
}

// restore ist POST /api/save/restore?slot=&backup=<name> (B-028).
func (s *server) restore(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		fail(w, http.StatusMethodNotAllowed, "Nur POST")
		return
	}
	slot, name := slotOf(r), r.URL.Query().Get("backup")
	err := s.cfg.Saves.Restore(slot, name)
	if errors.Is(err, store.ErrNotFound) {
		fail(w, http.StatusNotFound, "Keine solche Sicherung")
		return
	}
	if err != nil {
		s.saveError(w, err)
		return
	}
	s.log.Info("spielstand wiederhergestellt", "ns", "save", "slot", slot, "backup", name)
	writeJSON(w, http.StatusOK, map[string]any{"ok": true, "slot": slot, "restored": name})
}

func slotOf(r *http.Request) string {
	if slot := r.URL.Query().Get("slot"); slot != "" {
		return slot
	}
	return "autosave"
}
