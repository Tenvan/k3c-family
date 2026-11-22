package mcpsrv

import (
	"context"
	"encoding/json"
	"strings"
	"testing"
	"time"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/workkit/usage"
)

func TestNutzungsstatistikBekommtRoheArgumente(t *testing.T) {
	t.Parallel()
	tr := usage.New(time.Now)
	s := New(Config{Version: "test", Usage: tr, Root: t.TempDir()})
	add(s, &mcp.Tool{Name: "echo", Description: "Test"}, func(_ context.Context, in echoIn) (string, error) {
		return in.Text, nil
	})
	cs := connect(t, s)
	// Unsortiert und lang: aus dem auf 120 Zeichen gekürzten Aufruf-Log ließe sich das nicht mehr normieren.
	raw := json.RawMessage(`{"times":2,"text":"` + strings.Repeat("x", 300) + `"}`)
	if text, isErr := callText(t, cs, "echo", raw); isErr {
		t.Fatal(text)
	}
	callText(t, cs, "check_run", map[string]any{"target": "task:alles"})
	// Erfundener Name: das SDK antwortet mit einem Protokollfehler, die Middleware sieht ihn trotzdem.
	_, _ = cs.CallTool(context.Background(), &mcp.CallToolParams{Name: "gibt_es_nicht"})
	if status, _ := s.workbenchStatus(context.Background(), struct{}{}); !strings.Contains(status, "Sitzung: p95 ") {
		t.Errorf("Status ohne Statistik: %q", status)
	}
	snap := tr.Snapshot().Session
	if snap.Calls != 2 || snap.Errors != 1 { // reports_list und report_read: k3c-studio (WZ63.5)
		t.Fatalf("Zähler: %+v", snap)
	}
	for _, tool := range snap.Tools {
		if tool.Name == "gibt_es_nicht" {
			t.Error("unbekanntes Tool in der Statistik")
		}
		if tool.Name == "echo" && !strings.HasPrefix(tool.Args[0].Value, `{"text":"xxx`) {
			t.Errorf("Argumente nicht aus dem Rohtext normiert: %q", tool.Args[0].Value)
		}
		if tool.Name == "check_run" && (len(tool.TopErrors) != 1 || !strings.HasPrefix(tool.TopErrors[0].Value, "unbekanntes Ziel")) {
			t.Errorf("Fehler nicht gezählt: %+v", tool.TopErrors)
		}
	}
}
