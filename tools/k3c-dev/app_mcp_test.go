package main

import (
	"context"
	"encoding/json"
	"strings"
	"testing"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// callTool ruft ein Tool über HTTP auf, wie ein Agent.
func callTool(t *testing.T, url, name string) {
	t.Helper()
	ctx := context.Background()
	cs, err := mcp.NewClient(&mcp.Implementation{Name: "test", Version: "0"}, nil).
		Connect(ctx, &mcp.StreamableClientTransport{Endpoint: url}, nil)
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = cs.Close() }()
	if _, err := cs.CallTool(ctx, &mcp.CallToolParams{Name: name}); err != nil {
		t.Fatal(err)
	}
}

func TestMcpBindings(t *testing.T) {
	app, _ := startedApp(t)
	ov := app.McpOverview()
	if !ov.MCP.Listening || ov.Stats.StartedAt == "" || len(ov.Stats.Tools) == 0 || ov.Stats.TotalCalls != 0 {
		t.Fatalf("McpOverview = %+v", ov)
	}
	if text := app.McpInstructions(); !strings.HasPrefix(text, "# k3c-dev") || !strings.Contains(text, "`check_run`") {
		t.Errorf("McpInstructions = %.40q", text)
	}
	if calls := app.McpCalls(); calls == nil || len(calls) != 0 {
		t.Errorf("McpCalls = %v", calls)
	}
	u := app.McpUsage()
	if u.Rules.OutlierFactor != 2 || u.Rules.OutlierFloorMs != 1000 || u.Rules.BaselineCalls != 8 || u.Rules.PercentileErrorPct != 12 {
		t.Errorf("Regeln = %+v", u.Rules)
	}
	data, _ := json.Marshal(u)
	for _, key := range []string{`"session":`, `"allTime":`, `"minutes":`, `"rules":`} {
		if !strings.Contains(string(data), key) {
			t.Errorf("McpUsage ohne %s: %.120s", key, data)
		}
	}
}

// Neu starten behält Zähler und Katalog; der Server lauscht danach wieder.
func TestMcpRestart(t *testing.T) {
	app, _ := startedApp(t)
	if st := app.McpRestart(); !st.Listening || st.Error != "" {
		t.Errorf("McpRestart = %+v", st)
	}
	if !app.McpOverview().MCP.Listening {
		t.Error("nach Neustart lauscht der Server nicht")
	}
}

// Ein Aufruf über MCP kommt als mcp:start und mcp:call an die Oberfläche.
func TestMcpEreignisse(t *testing.T) {
	app, _ := startedApp(t)
	var names []string
	emit := app.emit
	app.emit = func(ctx context.Context, name string, data ...any) {
		if name == evMCPStart || name == evMCPCall {
			names = append(names, name)
		}
		emit(ctx, name, data...)
	}
	callTool(t, app.srv.URL(), "workbench_status")
	if strings.Join(names, ",") != "mcp:start,mcp:call" {
		t.Errorf("Ereignisse = %v", names)
	}
}
