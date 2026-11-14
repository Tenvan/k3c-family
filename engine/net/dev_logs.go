package net

import (
	"bytes"
	"errors"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// devLogFiles sind die Log-Dateien, die /api/dev/log und /api/dev/logs lesen (B-770): feste Namen, nie ein Pfad aus der URL.
// Die Dateinamen müssen zu cmd/k3c-server/logging.go passen (Server-Log, Client-Log und dessen eine alte Generation).
var devLogFiles = []struct{ name, file string }{
	{"server", logFileName},
	{"client", "k3c-client.jsonl"},
	{"client.1", "k3c-client.1.jsonl"},
}

// devLogPath löst den Namen aus ?file= auf (leer = server); false bei einem unbekannten Namen.
func devLogPath(dir, name string) (string, bool) {
	if name == "" {
		name = "server"
	}
	for _, f := range devLogFiles {
		if f.name == name {
			return filepath.Join(dir, f.file), true
		}
	}
	return "", false
}

// devLogNames nennt die gültigen Werte von ?file= für die Fehlermeldung.
func devLogNames() string {
	names := make([]string, len(devLogFiles))
	for i, f := range devLogFiles {
		names[i] = f.name
	}
	return strings.Join(names, ", ")
}

// logFilesPresent listet die vorhandenen Log-Dateien (Name, Größe, Änderungszeit); ohne Log-Ordner leer.
func (s *server) logFilesPresent() []string {
	if s.cfg.LogDir == "" {
		return nil
	}
	var rows []string
	for _, f := range devLogFiles {
		if info, err := os.Stat(filepath.Join(s.cfg.LogDir, f.file)); err == nil && !info.IsDir() {
			rows = append(rows, f.name+" "+strconv.FormatInt(info.Size(), 10)+" "+info.ModTime().UTC().Format("2006-01-02T15:04:05Z"))
		}
	}
	return rows
}

// logInfo ist der Log-Teil von /api/dev/status: ob ein Log-Ordner gesetzt ist und welche Dateien es gibt (nur Namen).
func (s *server) logInfo() map[string]any {
	files := []string{}
	for _, row := range s.logFilesPresent() {
		files = append(files, strings.Fields(row)[0])
	}
	return map[string]any{"on": s.cfg.LogDir != "", "files": files}
}

// devLogs ist GET /api/dev/logs: die abrufbaren Log-Dateien als Text, eine Zeile je Datei: Name, Größe in Bytes, Zeit.
func (s *server) devLogs(w http.ResponseWriter, r *http.Request) {
	if !s.devRead(w, r) {
		return
	}
	if s.cfg.LogDir == "" {
		fail(w, http.StatusNotFound, logOffMsg)
		return
	}
	w.Header().Set("Content-Type", "text/plain; charset=utf-8")
	_, _ = io.WriteString(w, strings.Join(s.logFilesPresent(), "\n"))
}

// logOffMsg erklärt, warum kein Log da ist.
const logOffMsg = "Log aus (K3C_LOG_DIR nicht gesetzt)"

// logWindow ist das Ergebnis eines Lesens: die Zeilen, der Cursor für den nächsten Aufruf (Byte nach der letzten vollständigen Zeile),
// truncated (since lag hinter dem Dateiende, es wurde von vorn gelesen) und partial (das Fenster maxBytes hat ältere Zeilen abgeschnitten).
type logWindow struct {
	lines              []string
	cursor             int64
	truncated, partial bool
}

// linesSince liest die Zeilen ab dem Byte-Cursor since, höchstens die letzten maxBytes. Liegt since hinter dem Dateiende (die Datei ist
// rotiert oder neu), beginnt es von vorn und meldet truncated. Gelesen wird nur bis zur beim Öffnen gemessenen Größe und nur bis zur
// letzten vollständigen Zeile; der Cursor steht dahinter, eine noch halb geschriebene Zeile kommt beim nächsten Aufruf ganz.
func linesSince(path string, since, maxBytes int64) (logWindow, error) {
	f, err := os.Open(path)
	if err != nil {
		return logWindow{}, err
	}
	defer func() { _ = f.Close() }()
	info, err := f.Stat()
	if err != nil {
		return logWindow{}, err
	}
	size := info.Size()
	w := logWindow{}
	start := since
	if start > size {
		start, w.truncated = 0, true
	}
	if size-start > maxBytes {
		start, w.partial = size-maxBytes, true
	}
	if _, err := f.Seek(start, io.SeekStart); err != nil {
		return logWindow{}, err
	}
	buf, err := io.ReadAll(io.LimitReader(f, size-start))
	if err != nil {
		return logWindow{}, err
	}
	end := bytes.LastIndexByte(buf, '\n') + 1 // 0, wenn keine Zeile vollständig ist
	buf, w.cursor = buf[:end], start+int64(end)
	if w.partial && start > 0 {
		if i := bytes.IndexByte(buf, '\n'); i >= 0 {
			buf = buf[i+1:]
		} else {
			buf = nil
		}
	}
	if text := strings.TrimRight(string(buf), "\r\n"); text != "" {
		w.lines = strings.Split(text, "\n")
	}
	return w, nil
}

// readDevLog liest je nach ?since= das Ende der Datei oder alles ab dem Cursor.
func readDevLog(path string, r *http.Request, maxBytes int64) (logWindow, error) {
	if _, has := r.URL.Query()["since"]; has {
		since, _ := number(r, "since", 0, 1<<62)
		return linesSince(path, since, maxBytes)
	}
	lines, size, err := tailLines(path, maxBytes)
	return logWindow{lines: lines, cursor: size}, err
}

// logReadError schreibt die Antwort auf einen Lesefehler und liefert true, wenn es einen gab.
func (s *server) logReadError(w http.ResponseWriter, err error) bool {
	switch {
	case err == nil:
		return false
	case errors.Is(err, os.ErrNotExist):
		fail(w, http.StatusNotFound, "Log-Datei fehlt")
	default:
		s.log.Error("📄 Log nicht lesbar", "ns", "diag", "err", err)
		fail(w, http.StatusInternalServerError, "Log nicht lesbar")
	}
	return true
}
