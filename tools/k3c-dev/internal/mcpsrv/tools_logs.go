package mcpsrv

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"time"

	"k3c/tools/k3c-dev/internal/applog"
	"k3c/tools/k3c-dev/internal/logs"
)

const (
	defaultQueryLimit = 100
	maxQueryLimit     = 500
	// errorsScanLimit begrenzt die Einträge, die logs_errors verdichtet.
	errorsScanLimit = 5000
)

var sourceName = regexp.MustCompile(`^[A-Za-z0-9_.-]+$`)

type queryIn struct {
	Source   string `json:"source" jsonschema:"Log-Quelle, z. B. k3c-dev"`
	MinLevel string `json:"minLevel,omitempty" jsonschema:"DEBUG, INFO, WARN oder ERROR"`
	NS       string `json:"ns,omitempty" jsonschema:"nur dieser Bereich, z. B. mcp"`
	Pattern  string `json:"pattern,omitempty" jsonschema:"regulärer Ausdruck auf die Meldung"`
	Since    string `json:"since,omitempty" jsonschema:"Dauer wie 30m, 24h, 7d oder Zeitpunkt RFC 3339"`
	Limit    int    `json:"limit,omitempty" jsonschema:"höchstens so viele Einträge, Standard 100, höchstens 500"`
}

type errorsIn struct {
	Source   string `json:"source" jsonschema:"Log-Quelle, z. B. k3c-dev"`
	MinLevel string `json:"minLevel,omitempty" jsonschema:"Standard WARN"`
	Since    string `json:"since,omitempty" jsonschema:"Standard 24h"`
}

type sinceIn struct {
	Source string `json:"source" jsonschema:"Log-Quelle, z. B. k3c-dev"`
	Cursor int64  `json:"cursor,omitempty" jsonschema:"Byte-Cursor aus der letzten Antwort, 0 = Anfang"`
	Limit  int    `json:"limit,omitempty" jsonschema:"höchstens so viele Einträge, Standard 100, höchstens 500"`
}

// LogSources sind alle logs/*.jsonl und immer das eigene Log, sortiert.
func (s *Server) LogSources() []string {
	seen := map[string]bool{applog.Source: true}
	files, _ := filepath.Glob(filepath.Join(s.cfg.Root, "logs", "*.jsonl"))
	for _, f := range files {
		seen[strings.TrimSuffix(filepath.Base(f), ".jsonl")] = true
	}
	names := make([]string, 0, len(seen))
	for n := range seen {
		names = append(names, n)
	}
	sort.Strings(names)
	return names
}

// LogPath prüft eine Quelle gegen die Liste; nur so wird aus einem Namen ein Pfad.
func (s *Server) LogPath(source string) (string, error) {
	for _, n := range s.LogSources() {
		if n == source && sourceName.MatchString(source) {
			return filepath.Join(s.cfg.Root, "logs", source+".jsonl"), nil
		}
	}
	return "", fmt.Errorf("unbekannte Log-Quelle %q; gültig: %s", source, strings.Join(s.LogSources(), ", "))
}

// logsSources ist das Tool logs_sources: Log-Dateien und Konsolen-Quellen.
func (s *Server) logsSources(context.Context, struct{}) (string, error) {
	var out []string
	for _, name := range s.LogSources() {
		out = append(out, s.logFileLine(name))
	}
	for _, name := range s.console.Sources() {
		lines, _ := s.console.Tail(name, 0)
		out = append(out, fmt.Sprintf("Konsole %s · %d Zeilen", name, len(lines)))
	}
	return strings.Join(out, "\n"), nil
}

func (s *Server) logFileLine(name string) string {
	path, _ := s.LogPath(name)
	st, err := os.Stat(path)
	if err != nil {
		return fmt.Sprintf("Log %s · noch keine Einträge", name)
	}
	res, _ := logs.Scan(path, logs.Query{Limit: 1})
	if len(res.Entries) == 0 {
		return fmt.Sprintf("Log %s · %s · noch keine Einträge", name, formatBytes(st.Size()))
	}
	e := res.Entries[0]
	return fmt.Sprintf("Log %s · %s · zuletzt %s %s", name, formatBytes(st.Size()), e.Time.Local().Format("02.01. 15:04:05"), e.Level)
}

// logsQuery ist das Tool logs_query: gefilterte Einträge, neueste zuerst.
func (s *Server) logsQuery(_ context.Context, in queryIn) (string, error) {
	path, err := s.LogPath(in.Source)
	if err != nil {
		return "", err
	}
	q := logs.Query{MinLevel: in.MinLevel, NS: in.NS, Limit: clampLimit(in.Limit)}
	if q.Since, err = parseSince(in.Since, time.Now()); err != nil {
		return "", err
	}
	if in.Pattern != "" {
		if q.Pattern, err = regexp.Compile(in.Pattern); err != nil {
			return "", fmt.Errorf("pattern %q ist kein gültiger regulärer Ausdruck: %w", in.Pattern, err)
		}
	}
	res, err := logs.Scan(path, q)
	if err != nil {
		return "", err
	}
	out := make([]string, 0, len(res.Entries)+1)
	for _, e := range res.Entries {
		out = append(out, entryLine(e))
	}
	return strings.Join(append(out, scanFooter(fmt.Sprintf("%d Einträge", len(res.Entries)), res)), "\n"), nil
}

// logsErrors ist das Tool logs_errors: Warnungen und Fehler, gleichartige zu je einer Zeile verdichtet.
func (s *Server) logsErrors(_ context.Context, in errorsIn) (string, error) {
	path, err := s.LogPath(in.Source)
	if err != nil {
		return "", err
	}
	q := logs.Query{MinLevel: in.MinLevel, Limit: errorsScanLimit}
	if q.MinLevel == "" {
		q.MinLevel = "WARN"
	}
	if in.Since == "" {
		in.Since = "24h"
	}
	if q.Since, err = parseSince(in.Since, time.Now()); err != nil {
		return "", err
	}
	res, err := logs.Scan(path, q)
	if err != nil {
		return "", err
	}
	groups := logs.Digest(res.Entries)
	out := make([]string, 0, len(groups)+1)
	for _, g := range groups {
		out = append(out, fmt.Sprintf("%d× %s %s %s · %s", g.Count, g.Level, g.NS, clip(g.Example), window(g.First, g.Last)))
	}
	head := fmt.Sprintf("%d Gruppen aus %d Einträgen ab %s seit %s", len(groups), len(res.Entries), q.MinLevel, in.Since)
	return strings.Join(append(out, scanFooter(head, res)), "\n"), nil
}

// logsSince ist das Tool logs_since: neue Einträge ab einem Byte-Cursor, älteste zuerst, und der nächste Cursor.
func (s *Server) logsSince(_ context.Context, in sinceIn) (string, error) {
	path, err := s.LogPath(in.Source)
	if err != nil {
		return "", err
	}
	res, err := logs.ReadSince(path, in.Cursor, clampLimit(in.Limit), logs.DefaultBudget)
	if err != nil {
		return "", err
	}
	out := make([]string, 0, len(res.Entries)+1)
	for _, e := range res.Entries {
		out = append(out, entryLine(e))
	}
	foot := fmt.Sprintf("%d neue Einträge · cursor=%d", len(res.Entries), res.Cursor)
	if res.Truncated {
		foot += " · gekürzt (Datei neu begonnen oder mehr als das Limit)"
	}
	return strings.Join(append(out, foot), "\n"), nil
}

func clampLimit(n int) int {
	if n <= 0 {
		return defaultQueryLimit
	}
	return min(n, maxQueryLimit)
}

// entryLine ist ein Eintrag als Zeile: Uhrzeit, Level, Bereich, Meldung und gekürzte Daten.
func entryLine(e logs.Entry) string {
	line := fmt.Sprintf("%s %s %s %s", e.Time.Local().Format("15:04:05"), e.Level, e.NS, e.Msg)
	if len(e.Data) > 0 {
		data, _ := json.Marshal(e.Data)
		line += " " + clip(string(data))
	}
	return line
}

func scanFooter(head string, res logs.Result) string {
	foot := head + " · " + formatBytes(res.BytesRead) + " gelesen"
	if res.BudgetHit {
		foot += " · Budget erreicht, ältere Treffer möglich"
	}
	if res.Skipped > 0 {
		foot += fmt.Sprintf(" · %d unlesbare Zeilen übersprungen", res.Skipped)
	}
	return foot
}

// window schreibt eine Zeitspanne kurz: "13:02–13:40", über Tage mit Datum.
func window(first, last time.Time) string {
	first, last = first.Local(), last.Local()
	if first.Equal(last) {
		return first.Format("15:04")
	}
	if first.YearDay() == last.YearDay() && first.Year() == last.Year() {
		return first.Format("15:04") + "–" + last.Format("15:04")
	}
	return first.Format("02.01. 15:04") + "–" + last.Format("02.01. 15:04")
}

// parseSince liest eine Dauer (30m, 24h, 7d) oder einen Zeitpunkt (RFC 3339); leer heißt kein Filter.
func parseSince(raw string, now time.Time) (time.Time, error) {
	if raw == "" {
		return time.Time{}, nil
	}
	if days, ok := strings.CutSuffix(raw, "d"); ok {
		if d, err := time.ParseDuration(days + "h"); err == nil {
			return now.Add(-24 * d), nil
		}
	}
	if d, err := time.ParseDuration(raw); err == nil {
		return now.Add(-d), nil
	}
	if t, err := time.Parse(time.RFC3339, raw); err == nil {
		return t, nil
	}
	return time.Time{}, fmt.Errorf("since %q: erwartet eine Dauer wie 30m, 24h, 7d oder einen Zeitpunkt RFC 3339", raw)
}

// formatBytes schreibt eine Größe deutsch: "512 B", "4,1 KB", "2,3 MB".
func formatBytes(n int64) string {
	switch {
	case n < 1024:
		return fmt.Sprintf("%d B", n)
	case n < 1<<20:
		return strings.Replace(fmt.Sprintf("%.1f KB", float64(n)/1024), ".", ",", 1)
	default:
		return strings.Replace(fmt.Sprintf("%.1f MB", float64(n)/(1<<20)), ".", ",", 1)
	}
}
