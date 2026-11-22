package mcpsrv

import (
	"context"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/workkit/mcpkit"
)

// connect verbindet einen Client über den In-Memory-Transport des SDK mit s (wie in k3c-workbench).
func connect(t *testing.T, s *Server) *mcp.ClientSession {
	t.Helper()
	ctx := context.Background()
	ct, st := mcp.NewInMemoryTransports()
	if _, err := s.kit.MCP().Connect(ctx, st, nil); err != nil {
		t.Fatal(err)
	}
	cs, err := mcp.NewClient(&mcp.Implementation{Name: "test", Version: "0"}, nil).Connect(ctx, ct, nil)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = cs.Close() })
	return cs
}

func callText(t *testing.T, cs *mcp.ClientSession, name string, args any) (string, bool) {
	t.Helper()
	res, err := cs.CallTool(context.Background(), &mcp.CallToolParams{Name: name, Arguments: args})
	if err != nil {
		t.Fatal(err)
	}
	return resultText(res), res.IsError
}

// balanceTools führt k3c-studio seit WZ63.5 (AC-07): Testläufe, Engine-Läufe und Spieldaten.
var balanceTools = []string{"sim_test", "sim_run", "replay_run", "reports_list", "report_read", "saves_list"}

// studioTool: die Tools, die k3c-studio führt (AC-04, WZ63; Balancing AC-07, WZ63.5).
func studioTool(name string) bool {
	return name == "level_generate" || strings.HasPrefix(name, "world_") || strings.HasPrefix(name, "assets_") ||
		strings.HasPrefix(name, "hub_") || slices.Contains(balanceTools, name)
}

// AC-04: Headless über HTTP (Port 0) gestartet listet der Server genau die Studio-Tools, jedes mit Beschreibung
// „Nutze es bei … Statt …“ und Annotations, und keins der Workbench.
func TestHeadlessListetNurStudioTools(t *testing.T) {
	t.Parallel()
	s := New(Config{Root: t.TempDir(), Port: 0, Version: "test"})
	if err := s.Start(); err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = s.Stop() })
	ctx := context.Background()
	cs, err := mcp.NewClient(&mcp.Implementation{Name: "test", Version: "0"}, nil).
		Connect(ctx, &mcp.StreamableClientTransport{Endpoint: s.URL()}, nil)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = cs.Close() })
	res, err := cs.ListTools(ctx, nil)
	if err != nil {
		t.Fatal(err)
	}
	if init := cs.InitializeResult(); init == nil || init.ServerInfo.Name != Name {
		t.Errorf("Servername %+v, erwartet %s", init, Name)
	}
	var names []string
	for _, tool := range res.Tools {
		names = append(names, tool.Name)
		if !studioTool(tool.Name) {
			t.Errorf("%s gehört nicht ins Studio", tool.Name)
		}
		if tool.Annotations == nil || !strings.Contains(tool.Description, "Nutze es bei:") || !strings.Contains(tool.Description, "Statt:") {
			t.Errorf("%s: Beschreibung ohne „Nutze es bei / Statt“ oder ohne Annotations", tool.Name)
		}
	}
	for _, want := range append([]string{"level_generate", "world_list", "world_set", "assets_list", "assets_set", "hub_get", "hub_save"}, balanceTools...) {
		if !slices.Contains(names, want) {
			t.Errorf("%s fehlt: %v", want, names)
		}
	}
	for _, wb := range []string{"plan_list", "plan_create", "svc_status", "check_run", "task_start", "workbench_status", "logs_errors"} {
		if slices.Contains(names, wb) {
			t.Errorf("Workbench-Tool %s im Studio", wb)
		}
	}
}

// Jedes Tool des Katalogs steht in instructions.md, damit Clients sie eindeutig finden (AC-04).
func TestInstructionsNennenJedesTool(t *testing.T) {
	t.Parallel()
	s := New(Config{Version: "test"})
	for _, ts := range s.Stats().Tools {
		if !strings.Contains(instructions, "`"+ts.Name) {
			t.Errorf("%s fehlt in instructions.md", ts.Name)
		}
	}
}

// B-924/B-925: level_generate rechnet im Prozess des Studios und lehnt einen fremden Checkout ab.
func TestLevelGenerateLehntFremdenCheckoutAb(t *testing.T) {
	t.Parallel()
	main := t.TempDir()
	if err := os.WriteFile(filepath.Join(main, "go.mod"), []byte("module k3c\n"), 0o600); err != nil {
		t.Fatal(err)
	}
	s := New(Config{Root: main, Version: "test"})
	ctx := mcpkit.WithWorkspace(context.Background(), workspace{Root: filepath.Join(main, ".claude", "worktrees", "wt-a")})
	if _, err := s.levelGenerate(ctx, levelIn{Seed: "1"}); err == nil || !strings.Contains(err.Error(), "nicht der Checkout dieses Studios") {
		t.Errorf("level_generate: %v", err)
	}
}
