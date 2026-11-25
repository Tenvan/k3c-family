package mcpsrv

import (
	"context"
	"os"
	"os/exec"
	"path/filepath"
	"sync/atomic"
	"testing"
	"time"

	"k3c/tools/k3c-studio/internal/simrun"
	"k3c/tools/workkit/mcpkit"
)

// Hilfen der sim_test-Tests (WZ63.5, aus check_test.go und checkout_test.go der Workbench übernommen).

// fakeServer ersetzt den Prozessstart; started zählt, wie oft wirklich gestartet worden wäre.
func fakeServer(t *testing.T, run func(runSpec) runResult) (*Server, *atomic.Int32) {
	t.Helper()
	s := New(Config{Version: "test", Root: t.TempDir()})
	var started atomic.Int32
	s.simsViaRun()
	s.run = func(_ context.Context, spec runSpec) runResult {
		started.Add(1)
		return run(spec)
	}
	return s, &started
}

// simsViaRun lässt die Offline-Läufe von sim_test über die Test-Naht s.run laufen statt im Prozess (B-924): Die Tests
// prüfen so den Weg Tool → Register ohne Rechnung der Engine. Vor dem ersten Start aufrufen.
func (s *Server) simsViaRun() {
	s.sims.Exec = func(ctx context.Context, p simrun.Proc) simrun.Result {
		res := s.run(ctx, runSpec{dir: p.Dir, args: p.Args, head: p.Head, speed: p.Speed, timeout: p.Timeout, out: p.Out})
		return simrun.Result{Exit: res.exit, TimedOut: res.timedOut, Err: res.err}
	}
}

// gitRepo legt ein Git-Repo (module k3c) mit den Worktrees wts an und liefert seinen Pfad und die der Worktrees.
func gitRepo(t *testing.T, wts ...string) (main string, paths []string) {
	t.Helper()
	if testing.Short() {
		t.Skip("Test mit echtem Git-Repo und Worktrees, läuft ohne -short (B-619, B-830)")
	}
	if _, err := exec.LookPath("git"); err != nil {
		t.Skip("git fehlt")
	}
	main = filepath.Join(t.TempDir(), "k3c")
	if err := os.MkdirAll(main, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(main, "go.mod"), []byte("module k3c\n"), 0o600); err != nil {
		t.Fatal(err)
	}
	git := func(args ...string) {
		t.Helper()
		cmd := exec.Command("git", append([]string{"-C", main, "-c", "user.name=t", "-c", "user.email=t@t"}, args...)...)
		if out, err := cmd.CombinedOutput(); err != nil {
			t.Fatalf("git %v: %v\n%s", args, err, out)
		}
	}
	git("init", "-q")
	git("add", ".")
	git("commit", "-qm", "start")
	for i, wt := range wts {
		p := filepath.Join(main, filepath.FromSlash(wt))
		git("worktree", "add", "-q", "-b", "b"+string(rune('a'+i)), p)
		paths = append(paths, p)
	}
	return main, paths
}

// progressCtx ist ein Aufruf mit progressToken: Jede Meldung landet in got.
func progressCtx(got chan<- string) context.Context {
	return mcpkit.WithProgressFunc(context.Background(), func(msg string, _ time.Duration) {
		select {
		case got <- msg:
		default:
		}
	})
}
