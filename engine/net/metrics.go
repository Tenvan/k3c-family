package net

import (
	"net/http"
	"strconv"
	"time"
)

// metrics ist GET /api/metrics?since=<Unix-ms> (B-281): Messreihen und Diagnose-Ereignisse mit Zeit nach since, hinter
// demselben Token wie /api/status (ohne Token 404, falsches Token 401). since fehlt, ist ungültig oder liegt vor dem
// Start des Servers: alles im Puffer; die Seite erkennt einen Neustart an startedAt. Kein Push, die Seite pollt.
func (s *server) metrics(w http.ResponseWriter, r *http.Request) {
	if !s.authorized(w, r, http.MethodGet) {
		return
	}
	if s.cfg.Rooms == nil {
		fail(w, http.StatusNotFound, "Keine Räume")
		return
	}
	since, _ := strconv.ParseInt(r.URL.Query().Get("since"), 10, 64)
	writeJSON(w, http.StatusOK, s.cfg.Rooms.Monitor.Since(since, time.Now()))
}
