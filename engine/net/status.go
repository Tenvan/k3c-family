package net

import (
	"crypto/subtle"
	"errors"
	"math"
	stdnet "net"
	"net/http"
	"runtime"
	"strings"
	"time"

	"k3c/engine/room"
	"k3c/engine/store"
)

// status ist GET /api/status (B-027): nur mit Authorization: Bearer <K3C_STATUS_TOKEN>. Ohne gesetztes Token ist die
// Diagnose aus (404, Entscheidung 🧑), der Spielbetrieb läuft normal.
func (s *server) status(w http.ResponseWriter, r *http.Request) {
	if !s.authorized(w, r, http.MethodGet) {
		return
	}
	if code := r.URL.Query().Get("room"); code != "" {
		s.roomStatus(w, code)
		return
	}
	started := s.cfg.StartedAt
	var mem runtime.MemStats
	runtime.ReadMemStats(&mem)
	body := map[string]any{
		"memory":  map[string]any{"heapMB": mb(mem.HeapAlloc), "sysMB": mb(mem.Sys), "numGC": mem.NumGC},
		"version": s.cfg.Version, "startedAt": started.UTC().Format(time.RFC3339),
		"uptimeS": int64(time.Since(started).Seconds()),
		"saves":   s.cfg.Saves.Count(), "reports": s.cfg.Reports.Count(),
		"rooms": []room.Status{}, "failures": []room.Failure{},
	}
	if s.cfg.CPU != nil {
		if pct, ok := s.cfg.CPU(); ok {
			body["cpu"] = pct
		}
	}
	if s.cfg.Rooms != nil {
		body["rooms"], body["failures"] = s.cfg.Rooms.Status()
	}
	writeJSON(w, http.StatusOK, body)
}

// authorized prüft alle Diagnose-Wege gleich (B-027, B-088): ohne gesetztes Token 404, falsches Token 401, falsche Methode 405.
// false heißt: die Antwort ist schon geschrieben.
func (s *server) authorized(w http.ResponseWriter, r *http.Request, method string) bool {
	return s.deny(w, r, method) == 0
}

// deny schreibt bei einer Ablehnung die Antwort und liefert ihren Status; 0 heißt erlaubt.
func (s *server) deny(w http.ResponseWriter, r *http.Request, method string) int {
	if s.cfg.StatusToken == "" {
		http.NotFound(w, r)
		return http.StatusNotFound
	}
	given, ok := strings.CutPrefix(r.Header.Get("Authorization"), "Bearer ")
	if !ok || subtle.ConstantTimeCompare([]byte(given), []byte(s.cfg.StatusToken)) != 1 {
		w.Header().Set("WWW-Authenticate", `Bearer realm="k3c"`)
		fail(w, http.StatusUnauthorized, "Token fehlt oder falsch")
		return http.StatusUnauthorized
	}
	if r.Method != method {
		fail(w, http.StatusMethodNotAllowed, "Nur "+method)
		return http.StatusMethodNotAllowed
	}
	return 0
}

// mb rechnet Bytes in MB mit einer Nachkommastelle.
func mb(n uint64) float64 { return math.Round(float64(n)/(1<<20)*10) / 10 }

// roomStatus ist GET /api/status?room=CODE: der verdichtete Zustand eines Raums.
func (s *server) roomStatus(w http.ResponseWriter, code string) {
	var r *room.Room
	if s.cfg.Rooms != nil {
		r = s.cfg.Rooms.Room(code)
	}
	if r == nil {
		fail(w, http.StatusNotFound, "Raum nicht gefunden")
		return
	}
	writeJSON(w, http.StatusOK, r.Summary())
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

// restoreDenied nennt den Grund einer Ablehnung von Restore im Log (B-143).
var restoreDenied = map[int]string{
	http.StatusNotFound:         "Restore aus",
	http.StatusUnauthorized:     "Token fehlt oder falsch",
	http.StatusMethodNotAllowed: "Methode",
}

// restore ist POST /api/save/restore?slot=&backup=<name> (B-028), nur mit K3C_STATUS_TOKEN (B-143, Q17). Jede Ablehnung
// steht genau einmal im Log; Aufrufer ist die Adresse der Verbindung, X-Forwarded-For zählt nicht, Token nie im Log.
func (s *server) restore(w http.ResponseWriter, r *http.Request) {
	if code := s.deny(w, r, http.MethodPost); code != 0 {
		caller, _, err := stdnet.SplitHostPort(r.RemoteAddr)
		if err != nil {
			caller = r.RemoteAddr
		}
		s.log.Warn("restore abgelehnt", "ns", "save", "path", r.URL.Path, "caller", caller, "reason", restoreDenied[code])
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
