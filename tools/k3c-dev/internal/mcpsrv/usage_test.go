package mcpsrv

import (
	"context"
	"encoding/json"
	"strings"
	"testing"
	"time"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-dev/internal/usage"
)

func TestNutzungsstatistikBekommtRoheArgumente(t *testing.T) {
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
	callText(t, cs, "check_run", map[string]any{"target": "npm:alles"})
	// Erfundener Name: das SDK antwortet mit einem Protokollfehler, die Middleware sieht ihn trotzdem.
	_, _ = cs.CallTool(context.Background(), &mcp.CallToolParams{Name: "gibt_es_nicht"})
	if text, _ := callText(t, cs, "reports_list", nil); text != "keine Berichte" {
		t.Errorf("reports_list ohne Ordner: %q", text)
	}
	if text, _ := callText(t, cs, "report_read", map[string]any{"name": "../x.json"}); !strings.Contains(text, "abgelehnt") {
		t.Errorf("report_read mit ..: %q", text)
	}
	if status, _ := s.workbenchStatus(context.Background(), struct{}{}); !strings.Contains(status, "Sitzung: p95 ") {
		t.Errorf("Status ohne Statistik: %q", status)
	}
	snap := tr.Snapshot().Session
	if snap.Calls != 4 || snap.Errors != 2 {
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
