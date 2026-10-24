package mcpsrv

import (
	"context"
	"path/filepath"
	"strings"
	"sync"
	"testing"
	"time"
)

// sim_test mit checkout (B-423, WZ20.3): Build und Daten des Läufe liegen im Worktree, der Status nennt ihn.
func TestSimTestMitCheckout(t *testing.T) {
	main, wts := gitRepo(t, ".claude/worktrees/wt-a")
	s := New(Config{Root: main, Version: "test"})
	var mu sync.Mutex
	var specs []runSpec
	s.run = func(_ context.Context, spec runSpec) runResult {
		mu.Lock()
		defer mu.Unlock()
		specs = append(specs, spec)
		return runResult{}
	}
	cs := connect(t, s)
	text, isErr := callText(t, cs, "sim_test", map[string]any{"action": "start", "checkout": "wt-a"})
	if isErr {
		t.Fatalf("start: %s", text)
	}
	id := strings.Fields(text)[1]
	var status string
	for range 500 {
		status, _ = callText(t, cs, "sim_test", map[string]any{"action": "status", "id": id, "checkout": "wt-a"})
		if strings.Contains(status, "Bericht:") {
			break
		}
		time.Sleep(10 * time.Millisecond)
	}
	mu.Lock()
	defer mu.Unlock()
	if len(specs) < 2 {
		t.Fatalf("%d Schritte statt bauen und rechnen\n%s", len(specs), status)
	}
	if want := filepath.Join(wts[0], "tools", "k3c-workbench"); !samePath(specs[0].dir, want) {
		t.Errorf("Build in %q, erwartet %q", specs[0].dir, want)
	}
	if !samePath(specs[1].dir, wts[0]) {
		t.Errorf("Lauf in %q, erwartet %q", specs[1].dir, wts[0])
	}
	if want := "K3C_DATA_DIR=" + filepath.Join(wts[0], "data"); len(specs[1].env) != 1 || !strings.HasPrefix(specs[1].env[0], "K3C_DATA_DIR=") || !samePath(strings.TrimPrefix(specs[1].env[0], "K3C_DATA_DIR="), strings.TrimPrefix(want, "K3C_DATA_DIR=")) {
		t.Errorf("env %v, erwartet %q", specs[1].env, want)
	}
	if !strings.Contains(status, "wt-a") {
		t.Errorf("status nennt den Checkout nicht:\n%s", status)
	}
	if list, _ := callText(t, cs, "sim_test", map[string]any{"action": "list", "checkout": "wt-a"}); !strings.Contains(list, "wt-a") {
		t.Errorf("list nennt den Checkout nicht: %s", list)
	}
	if list, _ := callText(t, cs, "sim_test", map[string]any{"action": "list"}); strings.Contains(list, id) {
		t.Errorf("Lauf des Worktrees steht in der Liste der Wurzel: %s", list)
	}
}

func TestSimTestUnbekannterCheckout(t *testing.T) {
	main, _ := gitRepo(t, ".claude/worktrees/wt-a", ".claude/worktrees/wt-b")
	s := New(Config{Root: main, Version: "test"})
	started := 0
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
