package net

import (
	"bytes"
	"errors"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strconv"
)

// Grenzen von GET /api/status/log (B-088): höchstens 1 MB und 500 Zeilen je Antwort, eine Zeile über 64 KB wird übersprungen.
const (
	logChunk       = 1 << 20
	logMaxLine     = 64 << 10
	logDefaultRows = 100
	logMaxRows     = 500
)

// logFileName muss zu cmd/k3c-server/logging.go passen.
const logFileName = "k3c-server.jsonl"

// logPage ist die Antwort: Zeilen ab dem Cursor und der Cursor für den nächsten Aufruf.
type logPage struct {
	Cursor    int64    `json:"cursor"`
	Truncated bool     `json:"truncated"`
	Lines     []string `json:"lines"`
}

// statusLog ist GET /api/status/log?since=<Byte-Cursor>&limit=<n>: die Zeilen des JSON-Logs, älteste zuerst. Gelesen wird nur
// <LogDir>/k3c-server.jsonl, nie ein Pfad aus der URL.
func (s *server) statusLog(w http.ResponseWriter, r *http.Request) {
	if !s.authorized(w, r, http.MethodGet) {
		return
	}
	since, ok1 := number(r, "since", 0, 1<<62)
	limit, ok2 := number(r, "limit", logDefaultRows, logMaxRows)
	if !ok1 || !ok2 || limit < 1 {
		fail(w, http.StatusBadRequest, "since und limit müssen Zahlen ≥ 0 bzw. 1 bis 500 sein")
		return
	}
	if s.cfg.LogDir == "" {
		fail(w, http.StatusNotFound, "Log aus")
		return
	}
	page, err := readLog(filepath.Join(s.cfg.LogDir, logFileName), since, int(limit))
	switch {
	case errors.Is(err, os.ErrNotExist):
		fail(w, http.StatusNotFound, "Log aus")
	case err != nil:
		s.log.Error("📄 Log nicht lesbar", "ns", "diag", "err", err)
		fail(w, http.StatusInternalServerError, "Log nicht lesbar")
	default:
		writeJSON(w, http.StatusOK, page)
	}
}

// number liest einen nicht negativen Parameter; fehlt er, gilt def. false bei Unsinn oder wenn er über max liegt (limit wird gekappt).
func number(r *http.Request, key string, def, max int64) (int64, bool) {
	raw := r.URL.Query().Get(key)
	if raw == "" {
		return def, true
	}
	n, err := strconv.ParseInt(raw, 10, 64)
	if err != nil || n < 0 {
		return 0, false
	}
	return min(n, max), true
}

// readLog liest ganze Zeilen ab since. Ist die Datei kleiner als since, beginnt es vorn und meldet truncated.
func readLog(path string, since int64, limit int) (logPage, error) {
	f, err := os.Open(path)
	if err != nil {
		return logPage{}, err
	}
	defer func() { _ = f.Close() }()
	info, err := f.Stat()
	if err != nil {
		return logPage{}, err
	}
	page := logPage{Lines: []string{}}
	if since > info.Size() {
		since, page.Truncated = 0, true
	}
	if _, err := f.Seek(since, io.SeekStart); err != nil {
		return logPage{}, err
	}
	buf, err := io.ReadAll(io.LimitReader(f, logChunk))
	if err != nil {
		return logPage{}, err
	}
	used := splitLines(buf, limit, &page)
	page.Cursor = since + int64(used)
	return page, nil
}

// splitLines hängt bis zu limit ganze Zeilen aus buf an page.Lines und liefert die Zahl der verbrauchten Bytes. Eine unvollständige
// letzte Zeile bleibt für den nächsten Aufruf liegen, außer der Puffer ist voll und enthält gar keinen Zeilenumbruch (dann wird er verworfen).
func splitLines(buf []byte, limit int, page *logPage) int {
	used := 0
	for len(page.Lines) < limit {
		i := bytes.IndexByte(buf[used:], '\n')
		if i < 0 {
			if used == 0 && len(buf) >= logChunk {
				return len(buf)
			}
			break
		}
		line := bytes.TrimRight(buf[used:used+i], "\r")
		used += i + 1
		if len(line) > 0 && len(line) <= logMaxLine {
			page.Lines = append(page.Lines, string(line))
		}
	}
	return used
}
