package mcpsrv

import (
	"context"
	"strings"
	"testing"
)

func TestLogsStats(t *testing.T) {
	s := logServer(t)
	ctx := context.Background()
	text, err := s.logsStats(ctx, statsIn{Source: "k3c-dev", Since: "2026-09-30T00:00:00Z"})
	if err != nil || !strings.HasSuffix(text, "\nINFO 3, WARN 2, DEBUG 1, ERROR 1") {
		t.Errorf("je Level: %q, %v", text, err)
	}
	if text, _ = s.logsStats(ctx, statsIn{Since: "2026-09-30T00:00:00Z", GroupBy: "ns"}); !strings.Contains(text, "mcp 4") || !strings.Contains(text, "7 Einträge") {
		t.Errorf("je ns über alle Dienste: %q", text)
	}
	if _, err = s.logsStats(ctx, statsIn{GroupBy: "x"}); err == nil || !strings.Contains(err.Error(), "erlaubt level oder ns") {
		t.Errorf("groupBy: %v", err)
	}
}

func TestLogsContext(t *testing.T) {
	s := logServer(t)
	text, err := s.logsContext(context.Background(), contextIn{Source: "k3c-dev", TS: "2026-09-30T09:04:00Z", Before: 1, After: 1})
	lines := strings.Split(text, "\n")
	if err != nil || len(lines) != 4 || !strings.HasPrefix(lines[0], "  ") || !strings.HasPrefix(lines[1], "> ") ||
		!strings.Contains(lines[1], `go:y`) || !strings.Contains(lines[2], `task:x`) || lines[3] != "3 Zeilen um 2026-09-30T09:04:00Z" {
		t.Errorf("Kontext: %q, %v", text, err)
	}
	if _, err = s.logsContext(context.Background(), contextIn{Source: "k3c-dev", TS: "gestern"}); err == nil {
		t.Error("ungültiger Zeitpunkt ohne Fehler")
	}
}

func TestConsoleTailSince(t *testing.T) {
	s := logServer(t)
	for _, l := range []string{"a", "b", "c"} {
		s.console.Add("Vite", "stdout", l)
	}
	lines, _ := s.console.Tail("Vite", 0)
	text, err := s.consoleTail(context.Background(), tailIn{Source: "Vite", Since: lines[0].Seq, Lines: 1})
	if err != nil || !strings.HasSuffix(text, "\nb") {
		t.Errorf("ab Cursor: %q, %v", text, err)
	}
}
