package net

import (
	"bytes"
	"errors"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

// Grenzen der Lese-Endpunkte unter /api/dev (RX1): Der Dev-Mode ist im Heimnetz an, damit ein Entwickler den Server (zum Beispiel den Pi) von
// seinem Rechner aus auslesen kann, ohne SSH und ohne Token. Es wird nur gelesen, und nur aus festen Ordnern.
const (
	devLogTailMiB   = 2       // Standard: so viel vom Ende des Logs wird durchsucht
	devLogTailMax   = 8       // höchstens so viele MiB (ein Pi liest sie in Millisekunden)
	devLogRows      = 200     // Standard: Zeilen je Antwort
	devLogRowsMax   = 1000    // höchstens so viele Zeilen
	devLogBodyMax   = 1 << 20 // höchstens 1 MiB Antwort, die ältesten Zeilen fallen zuerst weg
	devPatternMax   = 200     // längstes Suchmuster
	devReportMaxLen = 2 << 20 // größter lesbarer Bericht
	devReportsLimit = 200     // längste Berichtsliste
)

// devLogSlots begrenzt die gleichzeitigen Log-Anfragen: Ein Fenster von 8 MiB belegt auf dem Pi 3 kurz 16 bis 25 MiB.
var devLogSlots = make(chan struct{}, 2)

// devRead prüft Methode und Dev-Mode der Lese-Endpunkte; false heißt: die Antwort ist schon geschrieben.
func (s *server) devRead(w http.ResponseWriter, r *http.Request) bool {
	if r.Method != http.MethodGet {
		fail(w, http.StatusMethodNotAllowed, "Nur GET")
		return false
	}
	if s.cfg.Rooms == nil || !s.cfg.Rooms.Dev {
		fail(w, http.StatusForbidden, "Nur im Dev-Mode (K3C_DEV)")
		return false
	}
	return true
}

// devStatus ist GET /api/dev/status: wie /api/status (Speicher, CPU, Räume, Abstürze), aber ohne Token und nur im Dev-Mode.
func (s *server) devStatus(w http.ResponseWriter, r *http.Request) {
	if !s.devRead(w, r) {
		return
	}
	writeJSON(w, http.StatusOK, s.statusBody())
}

// devLog ist GET /api/dev/log?pattern=<regex>&limit=<n>&mib=<n>: die letzten passenden Zeilen des JSON-Logs, ältere zuerst, als Text.
// pattern ist ein Regex (RE2, ohne Groß-/Kleinschreibung) auf die ganze Zeile; ohne pattern kommen die letzten Zeilen. Gelesen wird
// nur <LogDir>/k3c-server.jsonl, nie ein Pfad aus der URL. Kopfzeilen: X-Log-Size (Bytes der Datei), X-Log-Matched (Treffer im Fenster).
func (s *server) devLog(w http.ResponseWriter, r *http.Request) {
	if !s.devRead(w, r) {
		return
	}
	limit, ok1 := number(r, "limit", devLogRows, devLogRowsMax)
	mib, ok2 := number(r, "mib", devLogTailMiB, devLogTailMax)
	re, ok3 := logPattern(r.URL.Query().Get("pattern"))
	if !ok1 || !ok2 || !ok3 || limit < 1 || mib < 1 {
		fail(w, http.StatusBadRequest, "limit 1 bis 1000, mib 1 bis 8, pattern ein gültiger Regex bis 200 Zeichen")
		return
	}
	if s.cfg.LogDir == "" {
		fail(w, http.StatusNotFound, "Log aus")
		return
	}
	select {
	case devLogSlots <- struct{}{}:
		defer func() { <-devLogSlots }()
	default:
		fail(w, http.StatusTooManyRequests, "Zu viele gleichzeitige Log-Anfragen")
		return
	}
	lines, size, err := tailLines(filepath.Join(s.cfg.LogDir, logFileName), mib<<20)
	if errors.Is(err, os.ErrNotExist) {
		fail(w, http.StatusNotFound, "Log aus")
		return
	}
	if err != nil {
		s.log.Error("📄 Log nicht lesbar", "ns", "diag", "err", err)
		fail(w, http.StatusInternalServerError, "Log nicht lesbar")
		return
	}
	hits, matched := lastMatches(lines, re, int(limit), devLogBodyMax)
	w.Header().Set("Content-Type", "text/plain; charset=utf-8")
	w.Header().Set("X-Log-Size", strconv.FormatInt(size, 10))
	w.Header().Set("X-Log-Matched", strconv.Itoa(matched))
	_, _ = io.WriteString(w, strings.Join(hits, "\n"))
}

// logPattern übersetzt das Suchmuster; leer = alle Zeilen. false bei zu langem oder ungültigem Muster.
func logPattern(raw string) (*regexp.Regexp, bool) {
	if raw == "" {
		return nil, true
	}
	if len(raw) > devPatternMax {
		return nil, false
	}
	re, err := regexp.Compile("(?i)" + raw)
	return re, err == nil
}

// tailLines liest höchstens die letzten maxBytes der Datei als ganze Zeilen (die angeschnittene erste fällt weg) und meldet die Dateigröße.
func tailLines(path string, maxBytes int64) ([]string, int64, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, 0, err
	}
	defer func() { _ = f.Close() }()
	info, err := f.Stat()
	if err != nil {
		return nil, 0, err
	}
	start := max(0, info.Size()-maxBytes)
	if _, err := f.Seek(start, io.SeekStart); err != nil {
		return nil, 0, err
	}
	buf, err := io.ReadAll(f)
	if err != nil {
		return nil, 0, err
	}
	if start > 0 {
		if i := bytes.IndexByte(buf, '\n'); i >= 0 {
			buf = buf[i+1:]
		} else {
			buf = nil
		}
	}
	lines := strings.Split(strings.TrimRight(string(buf), "\r\n"), "\n")
	if len(lines) == 1 && lines[0] == "" {
		lines = nil
	}
	return lines, info.Size(), nil
}

// lastMatches liefert die letzten limit Zeilen, die re treffen (nil = alle), ältere zuerst und zusammen höchstens maxBytes, und die Zahl
// aller Treffer.
func lastMatches(lines []string, re *regexp.Regexp, limit, maxBytes int) ([]string, int) {
	var hits []string
	matched, bytesUsed := 0, 0
	for i := len(lines) - 1; i >= 0; i-- {
		if re != nil && !re.MatchString(lines[i]) {
			continue
		}
		matched++
		if len(hits) < limit && bytesUsed+len(lines[i])+1 <= maxBytes {
			hits = append(hits, lines[i])
			bytesUsed += len(lines[i]) + 1
		}
	}
	for l, r := 0, len(hits)-1; l < r; l, r = l+1, r-1 {
		hits[l], hits[r] = hits[r], hits[l]
	}
	return hits, matched
}

// devReports ist GET /api/dev/reports: die Berichte (neueste zuerst) als Text, eine Zeile je Datei: Name, Größe, Zeit. Mit ?name=<datei>
// kommt der Inhalt eines Berichts. Der Name ist ein reiner Dateiname (<name>.json) im Berichte-Ordner.
func (s *server) devReports(w http.ResponseWriter, r *http.Request) {
	if !s.devRead(w, r) {
		return
	}
	if s.cfg.Reports == nil || s.cfg.Reports.Dir == "" {
		fail(w, http.StatusNotFound, "Berichte aus")
		return
	}
	if name := r.URL.Query().Get("name"); name != "" {
		s.devReportFile(w, name)
		return
	}
	entries, err := os.ReadDir(s.cfg.Reports.Dir)
	if err != nil && !errors.Is(err, os.ErrNotExist) {
		fail(w, http.StatusInternalServerError, "Berichte nicht lesbar")
		return
	}
	var names []string
	for _, e := range entries {
		if !e.IsDir() && strings.HasSuffix(e.Name(), ".json") {
			names = append(names, e.Name())
		}
	}
	sort.Sort(sort.Reverse(sort.StringSlice(names)))
	names = names[:min(len(names), devReportsLimit)]
	var b strings.Builder
	for _, n := range names {
		if info, err := os.Stat(filepath.Join(s.cfg.Reports.Dir, n)); err == nil {
			b.WriteString(n + " " + strconv.FormatInt(info.Size(), 10) + " " + info.ModTime().UTC().Format("2006-01-02T15:04:05Z") + "\n")
		}
	}
	w.Header().Set("Content-Type", "text/plain; charset=utf-8")
	_, _ = io.WriteString(w, b.String())
}

// devReportFile antwortet mit dem Inhalt eines Berichts; Pfade, versteckte und fremde Dateien lehnt es ab.
func (s *server) devReportFile(w http.ResponseWriter, name string) {
	if filepath.Base(name) != name || strings.HasPrefix(name, ".") || !strings.HasSuffix(name, ".json") || strings.ContainsAny(name, `\/:`) {
		fail(w, http.StatusBadRequest, "name muss ein Dateiname <name>.json sein")
		return
	}
	path := filepath.Join(s.cfg.Reports.Dir, name)
	info, err := os.Stat(path)
	if err != nil || info.IsDir() {
		fail(w, http.StatusNotFound, "Bericht nicht gefunden")
		return
	}
	if info.Size() > devReportMaxLen {
		fail(w, http.StatusRequestEntityTooLarge, "Bericht zu groß")
		return
	}
	data, err := os.ReadFile(path)
	if err != nil {
		fail(w, http.StatusInternalServerError, "Bericht nicht lesbar")
		return
	}
	w.Header().Set("Content-Type", "application/json; charset=utf-8")
	_, _ = w.Write(data)
}
