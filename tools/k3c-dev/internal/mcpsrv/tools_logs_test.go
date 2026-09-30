package mcpsrv

import (
	"bytes"
	"context"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// logServer ist ein Server, dessen Repo-Wurzel logs/k3c-dev.jsonl aus dem Beispiel des Log-Pakets enthält.
func logServer(t *testing.T) *Server {
	t.Helper()
	root := t.TempDir()
	data, err := os.ReadFile("../logs/testdata/k3c-dev.jsonl")
	if err != nil {
		t.Fatal(err)
	}
	if err := os.MkdirAll(filepath.Join(root, "logs"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(root, "logs", "k3c-dev.jsonl"), data, 0o644); err != nil {
		t.Fatal(err)
	}
	return New(Config{Version: "test", Root: root})
}

func TestLogsSourcesUndUnbekannteQuelle(t *testing.T) {
	s := logServer(t)
	ctx := context.Background()
	s.console.Add("check:npm:test", "stdout", "x")
	text, _ := s.logsSources(ctx, struct{}{})
	if !strings.Contains(text, "Log k3c-dev · ") || !strings.Contains(text, " zuletzt ") || !strings.Contains(text, "WARN") ||
		!strings.Contains(text, "Konsole check:npm:test · 1 Zeilen") {
		t.Errorf("Quellen: %q", text)
	}
	for _, bad := range []string{"../k3c-dev", "gibtsnicht", ""} {
		if _, err := s.logsQuery(ctx, queryIn{Source: bad}); err == nil || !strings.Contains(err.Error(), "gültig: k3c-dev") {
			t.Errorf("Quelle %q: %v", bad, err)
		}
	}
}

func TestLogsQuery(t *testing.T) {
	s := logServer(t)
	ctx := context.Background()
	text, err := s.logsQuery(ctx, queryIn{Source: "k3c-dev", MinLevel: "WARN", Limit: 2})
	lines := strings.Split(text, "\n")
	if err != nil || len(lines) != 3 || !strings.HasSuffix(lines[0], " WARN main server gestoppt") ||
		!strings.Contains(lines[1], ` ERROR mcp check_run: Ziel "npm:x" unbekannt`) || !strings.HasPrefix(lines[2], "2 Einträge · ") {
		t.Errorf("Abfrage: %q, %v", text, err)
	}
	text, _ = s.logsQuery(ctx, queryIn{Source: "k3c-dev", NS: "check"})
	if !strings.Contains(text, `lauf beendet {"exit":0,"target":"npm:test"}`) || !strings.Contains(text, "1 unlesbare Zeilen") {
		t.Errorf("Daten und übersprungene Zeile: %q", text)
	}
	if _, err := s.logsQuery(ctx, queryIn{Source: "k3c-dev", Pattern: "["}); err == nil || !strings.Contains(err.Error(), "regulärer Ausdruck") {
		t.Errorf("ungültiger Ausdruck: %v", err)
	}
	if _, err := s.logsQuery(ctx, queryIn{Source: "k3c-dev", Since: "gestern"}); err == nil {
		t.Error("ungültiges since ohne Fehler")
	}
}

func TestLogsErrorsUndSince(t *testing.T) {
	s := logServer(t)
	ctx := context.Background()
	text, _ := s.logsErrors(ctx, errorsIn{Source: "k3c-dev", Since: "2026-09-30T00:00:00Z"})
	lines := strings.Split(text, "\n")
	if len(lines) != 3 || !strings.HasPrefix(lines[0], `2× ERROR mcp check_run: Ziel "npm:x" unbekannt · `) ||
		!strings.HasPrefix(lines[1], "1× WARN main server gestoppt") || !strings.HasPrefix(lines[2], "2 Gruppen aus 3 Einträgen ab WARN") {
		t.Errorf("Verdichtung: %q", text)
	}
	text, _ = s.logsSince(ctx, sinceIn{Source: "k3c-dev"})
	lines = strings.Split(text, "\n")
	if len(lines) != 8 || !strings.HasSuffix(lines[0], "k3c-dev gestartet {\"url\":\"http://127.0.0.1:5180/mcp\"}") ||
		!strings.HasPrefix(lines[7], "7 neue Einträge · cursor=") {
		t.Errorf("since: %q", text)
	}
	cursor := strings.TrimPrefix(lines[7], "7 neue Einträge · cursor=")
	var next sinceIn
	next.Source = "k3c-dev"
	for _, c := range cursor {
		next.Cursor = next.Cursor*10 + int64(c-'0')
	}
	if text, _ = s.logsSince(ctx, next); text != "0 neue Einträge · cursor="+cursor {
		t.Errorf("zweiter Aufruf: %q", text)
	}
}

func TestParseSinceUndFormatBytes(t *testing.T) {
	now := time.Date(2026, 9, 30, 12, 0, 0, 0, time.UTC)
	cases := map[string]time.Time{"30m": now.Add(-30 * time.Minute), "24h": now.Add(-24 * time.Hour),
		"7d": now.Add(-7 * 24 * time.Hour), "2026-09-29T08:00:00Z": time.Date(2026, 9, 29, 8, 0, 0, 0, time.UTC), "": {}}
	for raw, want := range cases {
		if got, err := parseSince(raw, now); err != nil || !got.Equal(want) {
			t.Errorf("parseSince(%q) = %v, %v", raw, got, err)
		}
	}
	for n, want := range map[int64]string{512: "512 B", 4200: "4,1 KB", 2411724: "2,3 MB"} {
		if got := formatBytes(n); got != want {
			t.Errorf("formatBytes(%d) = %q", n, got)
		}
	}
}

func TestEreignisseImEigenenLog(t *testing.T) {
	var buf bytes.Buffer
	s := New(Config{Version: "test", Root: t.TempDir(), Log: slog.New(slog.NewJSONHandler(&buf, nil))})
	s.run = func(context.Context, runSpec) runResult { return runResult{exit: 1} }
	if _, err := s.checkRun(context.Background(), checkIn{Target: "npm:lint"}); err != nil {
		t.Fatal(err)
	}
	cs := connect(t, s)
	callText(t, cs, "check_run", map[string]any{"target": "npm:alles"})
	log := buf.String()
	if !strings.Contains(log, `"level":"WARN","msg":"lauf beendet","ns":"check","target":"npm:lint","exit":1`) ||
		!strings.Contains(log, `"msg":"check_run: unbekanntes Ziel \"npm:alles\"; gültige Ziele: npm:check`) {
		t.Errorf("Log: %s", log)
	}
	if cs.InitializeResult().Instructions == "" || !strings.Contains(cs.InitializeResult().Instructions, "check_run") {
		t.Error("Instructions fehlen")
	}
}
