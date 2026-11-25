package mcpsrv

import (
	"context"
	"k3c/tools/workkit/mcpkit"
	"path/filepath"
	"strings"
	"testing"
)

// B-924/B-925: Die Engine rechnet im Prozess des Studios (seit WZ63.5). sim_test start und replay_run lehnen
// ein checkout ab, das nicht der Checkout dieses Studios ist, und nennen das eigene Studio im Worktree.
func TestSimToolsLehnenFremdenCheckoutAb(t *testing.T) {
	t.Parallel() // ohne t.Setenv und globale Zustände (B-830)
	main, _ := gitRepo(t, ".claude/worktrees/wt-a")
	s := New(Config{Root: main, Version: "test"})
	started := 0
	s.simsViaRun()
	s.run = func(context.Context, runSpec) runResult { started++; return runResult{} }
	cs := connect(t, s)
	text, isErr := callText(t, cs, "sim_test", map[string]any{"action": "start", "checkout": "wt-a"})
	if !isErr || !strings.Contains(text, "nicht der Checkout dieses Studios") || !strings.Contains(text, "wt-a") {
		t.Errorf("sim_test: %q", text)
	}
	// die übrigen Tools bekommen den Checkout über den Header X-K3C-Root, hier über den Context
	ctx := mcpkit.WithWorkspace(context.Background(), workspace{Root: filepath.Join(main, ".claude", "worktrees", "wt-a")})
	if _, err := s.replayRun(ctx, replayIn{Path: "x.json"}); err == nil || !strings.Contains(err.Error(), EnvPort) {
		t.Errorf("replay_run: %v", err)
	}
	if started != 0 {
		t.Errorf("%d Läufe trotz Ablehnung", started)
	}
}

func TestSimTestUnbekannterCheckout(t *testing.T) {
	t.Parallel() // ohne t.Setenv und globale Zustände (B-830)
	main, _ := gitRepo(t, ".claude/worktrees/wt-a", ".claude/worktrees/wt-b")
	s := New(Config{Root: main, Version: "test"})
	started := 0
	s.simsViaRun()
	s.run = func(context.Context, runSpec) runResult { started++; return runResult{} }
	cs := connect(t, s)
	text, isErr := callText(t, cs, "sim_test", map[string]any{"action": "start", "checkout": "gibtsnicht"})
	if !isErr || !strings.Contains(text, "wt-a") || !strings.Contains(text, "wt-b") {
		t.Errorf("Ablehnung ohne Liste der Worktrees: %q", text)
	}
	if started != 0 {
		t.Errorf("%d Prozesse trotz Ablehnung", started)
	}
}
