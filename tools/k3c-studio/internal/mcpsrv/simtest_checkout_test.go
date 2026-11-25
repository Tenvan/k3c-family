package mcpsrv

import (
	"context"
	"path/filepath"
	"strings"
	"testing"
)

// RU40.3: sim_test start rechnet mit der EXE des Checkouts aus `checkout`, auch eines Worktrees (kein Prozess des Studios
// mehr); head und speed erreichen den Lauf.
func TestSimTestStartetImWorktree(t *testing.T) {
	t.Parallel() // ohne t.Setenv und globale Zustände (B-830)
	main, wts := gitRepo(t, ".claude/worktrees/wt-a")
	s := New(Config{Root: main, Version: "test"})
	got := make(chan runSpec, 1)
	s.simsViaRun()
	s.run = func(_ context.Context, spec runSpec) runResult { got <- spec; return runResult{} }
	cs := connect(t, s)
	text, isErr := callText(t, cs, "sim_test", map[string]any{"action": "start", "checkout": "wt-a", "head": true, "speed": 4})
	if isErr || !strings.Contains(text, "head") {
		t.Fatalf("sim_test: %q", text)
	}
	spec := <-got
	callText(t, cs, "sim_test", map[string]any{"action": "wait", "id": "run-1", "checkout": "wt-a"}) // Lauf zu Ende, bevor TempDir aufräumt
	if filepath.Clean(spec.dir) != filepath.Clean(wts[0]) || !spec.head || spec.speed != 4 || spec.args[0] != "balance" {
		t.Errorf("Lauf: %+v", spec)
	}
	if text, isErr := callText(t, cs, "sim_test", map[string]any{"action": "start", "speed": 4}); !isErr || !strings.Contains(text, "speed") {
		t.Errorf("speed ohne head: %q", text)
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
