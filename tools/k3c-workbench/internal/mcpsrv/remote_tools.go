package mcpsrv

import (
	"context"
	"encoding/json"
	"fmt"
	"net/url"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"

	"k3c/tools/k3c-workbench/internal/logs"
	"k3c/tools/k3c-workbench/internal/serverapi"
)

// mirroredSources sind die Quellen der gespiegelten Dateien (logs/*.jsonl unter dem Cache eines Servers), sortiert.
func mirroredSources(root string) []string {
	files, _ := filepath.Glob(filepath.Join(root, "logs", "*.jsonl"))
	names := make([]string, 0, len(files))
	for _, f := range files {
		names = append(names, strings.TrimSuffix(filepath.Base(f), ".jsonl"))
	}
	sort.Strings(names)
	return names
}

// remoteSources ist logs_services mit `server`: die Log-Dateien des Servers mit Größe und Stand, unter dem Namen, den die anderen Tools
// als service erwarten.
func (s *Server) remoteSources(ctx context.Context, server string) (string, error) {
	t, err := resolveRemote(server)
	if err != nil {
		return "", err
	}
	list, err := t.remoteLogs(ctx)
	if err != nil {
		return "", err
	}
	out := []string{fmt.Sprintf("Quelle: Server %s (%s)", t.name, t.base)}
	for _, l := range list {
		out = append(out, fmt.Sprintf("Log %s · %s · Stand %s", sourceOf(l.file), formatBytes(l.size), l.when))
	}
	if len(list) == 0 {
		out = append(out, "keine Log-Dateien auf dem Server")
	}
	return strings.Join(out, "\n"), nil
}

// remoteStatus ist server_status mit `server`: Version, Laufzeit und Zähler wie lokal, dazu der Log-Status des Servers
// (ob K3C_LOG_DIR gesetzt ist und welche Dateien es gibt).
func (s *Server) remoteStatus(ctx context.Context, server string) (string, error) {
	t, err := resolveRemote(server)
	if err != nil {
		return "", err
	}
	body, _, err := t.get(ctx, "/api/dev/status", nil)
	if err != nil {
		return "", err
	}
	var st struct {
		serverapi.Status
		Log struct {
			On    bool     `json:"on"`
			Files []string `json:"files"`
		} `json:"log"`
	}
	if err := json.Unmarshal([]byte(body), &st); err != nil {
		return "", fmt.Errorf("server %s: Status kein JSON: %w", t.name, err)
	}
	logLine := "Log: aus (am Server ist K3C_LOG_DIR nicht gesetzt)"
	if st.Log.On {
		logLine = "Log: an · " + strings.Join(st.Log.Files, ", ")
	}
	return fmt.Sprintf("Quelle: Server %s (%s)\n%s\n%s", t.name, t.base, serverapi.FormatStatus(st.Status), logLine), nil
}

// remoteReport ist reports_list (name leer) und report_read mit `server`: die Berichte des Servers über /api/dev/reports.
func (s *Server) remoteReport(ctx context.Context, server, name string) (string, error) {
	t, err := resolveRemote(server)
	if err != nil {
		return "", err
	}
	var q url.Values
	if name != "" {
		q = url.Values{"name": {name}}
	}
	body, _, err := t.get(ctx, "/api/dev/reports", q)
	if err != nil {
		return "", err
	}
	return fmt.Sprintf("Quelle: Server %s (%s)\n%s", t.name, t.base, body), nil
}

// remoteSince ist logs_since mit `server`: neue Zeilen ab dem Byte-Cursor im Log des Servers, älteste zuerst, und der nächste Cursor
// (X-Log-Size des Servers). Der Server liefert Text; ausgewertet wird er mit demselben Leser wie lokal.
func (s *Server) remoteSince(ctx context.Context, in sinceIn) (string, error) {
	t, err := resolveRemote(in.Server)
	if err != nil {
		return "", err
	}
	file, ok := fileOf(in.Source)
	if !ok {
		return "", fmt.Errorf("service %q: %w", in.Source, errRemoteSource)
	}
	if in.Cursor < 0 {
		return "", fmt.Errorf("cursor %d ungültig: erwartet 0 oder den Wert aus der letzten Antwort", in.Cursor)
	}
	limit := clampLimit(in.Limit)
	q := url.Values{"file": {file}, "since": {strconv.FormatInt(in.Cursor, 10)}, "limit": {strconv.Itoa(limit)}, "mib": {strconv.Itoa(remoteMiB)}}
	body, head, err := t.get(ctx, "/api/dev/log", q)
	if err != nil {
		return "", err
	}
	cursor, err := strconv.ParseInt(head.Get("X-Log-Size"), 10, 64)
	if err != nil || cursor < 0 {
		return "", fmt.Errorf("server %s: Antwort ohne gültigen Cursor (X-Log-Size fehlt, älterer Server?)", t.name)
	}
	dir, done, err := s.tempDir(t)
	if err != nil {
		return "", err
	}
	defer done()
	path := filepath.Join(dir, in.Source+".jsonl")
	if body != "" {
		body += "\n"
	}
	if err := os.WriteFile(path, []byte(body), 0o644); err != nil {
		return "", err
	}
	res, err := logs.ReadSince(path, 0, limit, logs.DefaultBudget)
	if err != nil {
		return "", err
	}
	out := []string{fmt.Sprintf("Quelle: Server %s (%s)", t.name, t.base)}
	for _, e := range res.Entries {
		out = append(out, entryLine(e))
	}
	foot := fmt.Sprintf("%d neue Einträge · cursor=%d", len(res.Entries), cursor)
	if head.Get("X-Log-Truncated") == "1" {
		foot += " · Datei auf dem Server neu begonnen (von vorn gelesen)"
	}
	if head.Get("X-Log-Partial") == "1" || res.Truncated {
		foot += " · gekürzt: ältere Zeilen wurden weggelassen"
	}
	return strings.Join(append(out, foot), "\n"), nil
}
