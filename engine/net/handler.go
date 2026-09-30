// Package net ist die HTTP-Schicht des Spiel-Servers (Entscheidung 001, engine/net): Auslieferung des Builds und die
// API für Spielstände, Testberichte und Health. Räume und WebSocket folgen in SP07.
package net

import (
	"encoding/json"
	"errors"
	"io"
	"log/slog"
	"net/http"
	"time"

	"k3c/engine/store"
)

// Config sind die Teile des Servers; Log darf nil sein.
type Config struct {
	Dist    string // Produktions-Build (dist/)
	Saves   *store.Saves
	Reports *store.Reports
	Log     *slog.Logger
	// StatusToken schützt /api/status (B-027); leer = Diagnose aus.
	StatusToken string
	Version     string
	StartedAt   time.Time
}

type server struct {
	cfg Config
	log *slog.Logger
}

// NewHandler baut den Handler mit allen Routen.
func NewHandler(cfg Config) http.Handler {
	s := &server{cfg: cfg, log: cfg.Log}
	if s.log == nil {
		s.log = slog.New(slog.NewTextHandler(io.Discard, nil))
	}
	mux := http.NewServeMux()
	mux.HandleFunc("/api/health", s.health)
	mux.HandleFunc("/api/save", s.save)
	mux.HandleFunc("/api/save/backups", s.backups)
	mux.HandleFunc("/api/save/restore", s.restore)
	mux.HandleFunc("/api/status", s.status)
	mux.HandleFunc("/api/report", s.report)
	mux.HandleFunc("/", s.static)
	return mux
}

func writeJSON(w http.ResponseWriter, status int, body any) {
	w.Header().Set("Content-Type", "application/json; charset=utf-8")
	w.Header().Set("Cache-Control", "no-store")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(body)
}

func fail(w http.ResponseWriter, status int, msg string) {
	writeJSON(w, status, map[string]string{"error": msg})
}

// readBody liest höchstens max Bytes; mehr ergibt store.ErrTooLarge.
func readBody(r *http.Request, max int64) ([]byte, error) {
	data, err := io.ReadAll(io.LimitReader(r.Body, max+1))
	if err == nil && int64(len(data)) > max {
		err = store.ErrTooLarge
	}
	return data, err
}

func (s *server) health(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet && r.Method != http.MethodHead {
		fail(w, http.StatusMethodNotAllowed, "Nur GET")
		return
	}
	writeJSON(w, http.StatusOK, map[string]bool{"ok": true})
}

// save ist GET/POST /api/save?slot=autosave wie server/saves.mjs.
func (s *server) save(w http.ResponseWriter, r *http.Request) {
	slot := slotOf(r)
	switch r.Method {
	case http.MethodGet:
		data, err := s.cfg.Saves.Load(slot)
		if err != nil {
			s.saveError(w, err)
			return
		}
		w.Header().Set("Content-Type", "application/json; charset=utf-8")
		w.Header().Set("Cache-Control", "no-store")
		_, _ = w.Write(data)
	case http.MethodPost:
		data, err := readBody(r, store.MaxSaveBytes)
		var backup string
		if err == nil {
			backup, err = s.cfg.Saves.Store(slot, data)
		}
		if err != nil {
			s.saveError(w, err)
			return
		}
		if backup != "" {
			s.log.Info("anderes Spiel, Sicherung angelegt", "ns", "save", "backup", backup)
		}
		writeJSON(w, http.StatusOK, map[string]any{"ok": true, "slot": slot, "backup": nullable(backup)})
	default:
		fail(w, http.StatusMethodNotAllowed, "Nur GET/POST")
	}
}

func nullable(s string) any {
	if s == "" {
		return nil
	}
	return s
}

func (s *server) saveError(w http.ResponseWriter, err error) {
	switch {
	case errors.Is(err, store.ErrSlot):
		fail(w, http.StatusBadRequest, "Ungültiger Slot")
	case errors.Is(err, store.ErrNotFound):
		fail(w, http.StatusNotFound, "Kein Spielstand")
	case errors.Is(err, store.ErrTooLarge):
		fail(w, http.StatusRequestEntityTooLarge, "Spielstand zu groß")
	case errors.Is(err, store.ErrInvalid):
		fail(w, http.StatusBadRequest, "Kein Spielstand")
	default:
		s.log.Error("spielstand: "+err.Error(), "ns", "save")
		fail(w, http.StatusInternalServerError, "Speichern fehlgeschlagen")
	}
}

// report ist POST /api/report wie server/reports.mjs.
func (s *server) report(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		fail(w, http.StatusMethodNotAllowed, "Nur POST")
		return
	}
	data, err := readBody(r, store.MaxReportBytes)
	var file string
	if err == nil {
		file, err = s.cfg.Reports.Store(data, r.RemoteAddr)
	}
	switch {
	case errors.Is(err, store.ErrTooLarge):
		http.Error(w, "Bericht zu groß", http.StatusRequestEntityTooLarge)
	case errors.Is(err, store.ErrInvalid):
		http.Error(w, "Kein gültiges JSON", http.StatusBadRequest)
	case err != nil:
		s.log.Error("bericht: "+err.Error(), "ns", "report")
		http.Error(w, "Speichern fehlgeschlagen", http.StatusInternalServerError)
	default:
		s.log.Info("bericht gespeichert", "ns", "report", "file", file)
		writeJSON(w, http.StatusOK, map[string]any{"ok": true, "file": file})
	}
}
