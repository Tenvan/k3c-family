package mcpsrv

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
	"time"
)

// fakeServer ersetzt den Prozessstart; started zählt, wie oft wirklich gestartet worden wäre.
func fakeServer(t *testing.T, run func(runSpec) runResult) (*Server, *atomic.Int32) {
	t.Helper()
	s := New(Config{Version: "test", Root: t.TempDir()})
	var started atomic.Int32
	s.run = func(_ context.Context, spec runSpec) runResult {
		started.Add(1)
		return run(spec)
	}
	return s, &started
}

func TestCheckRunLehntAbOhneProzessstart(t *testing.T) {
	s, started := fakeServer(t, func(runSpec) runResult { return runResult{} })
	ctx := context.Background()
	cases := []checkIn{
		{Target: "task:alles"},
		{Target: "task:lint", Pattern: "planning"},
		{Target: "task:test", Pattern: "--watch"},
		{Target: "go:test", Pattern: "-v"},
	}
	for _, bad := range []string{";", "&", "|", "$", "`", `"`, "'", "<", ">", "%", "^", " ", "a b", strings.Repeat("a", 101)} {
		cases = append(cases, checkIn{Target: "task:test", Pattern: "x" + bad})
	}
	for _, in := range cases {
		if _, err := s.checkRun(ctx, in); err == nil {
			t.Errorf("%+v nicht abgelehnt", in)
		}
	}
	if _, err := s.checkRun(ctx, checkIn{Target: "task:alles"}); err == nil || !strings.Contains(err.Error(), "task:check, task:test") {
		t.Errorf("Ablehnung nennt die Ziele nicht: %v", err)
	}
	if n := started.Load(); n != 0 {
		t.Errorf("%d Prozesse gestartet", n)
	}
}

func TestCheckRunEinLaufJeZiel(t *testing.T) {
	release := make(chan struct{})
	s, started := fakeServer(t, func(runSpec) runResult { <-release; return runResult{} })
	ctx := context.Background()
	first := make(chan error)
	go func() { _, err := s.checkRun(ctx, checkIn{Target: "task:test"}); first <- err }()
	for started.Load() == 0 {
		time.Sleep(time.Millisecond)
	}
	if _, err := s.checkRun(ctx, checkIn{Target: "task:test"}); err == nil || !strings.Contains(err.Error(), "läuft bereits") {
		t.Errorf("zweiter Lauf: %v", err)
	}
	close(release)
	if err := <-first; err != nil || started.Load() != 1 {
		t.Errorf("erster Lauf: %v, %d Starts", err, started.Load())
	}
}

func TestCheckRunGibtSperreNachPanikFrei(t *testing.T) {
	var calls atomic.Int32
	s, _ := fakeServer(t, func(runSpec) runResult {
		if calls.Add(1) == 1 {
			panic("kaputt")
		}
		return runResult{}
	})
	func() {
		defer func() { _ = recover() }()
		_, _ = s.checkRun(context.Background(), checkIn{Target: "task:test"})
	}()
	if _, err := s.checkRun(context.Background(), checkIn{Target: "task:test"}); err != nil {
		t.Errorf("Ziel nach Panik gesperrt: %v", err)
	}
}

func TestCheckRunMusterUndKonsole(t *testing.T) {
	var got runSpec
	s, _ := fakeServer(t, func(spec runSpec) runResult {
		got = spec
		spec.out("stdout", "läuft")
		spec.out("stderr", "warnung")
		return runResult{dur: 12400 * time.Millisecond}
	})
	text, err := s.checkRun(context.Background(), checkIn{Target: "task:test", Pattern: "planning"})
	if err != nil || text != "task:test · exit 0 · 12,4 s" {
		t.Errorf("grüner Lauf: %q, %v", text, err)
	}
	if strings.Join(got.args, " ") != "task test -- planning" || got.timeout != testTimeout {
		t.Errorf("Befehl: %v, %v", got.args, got.timeout)
	}
	tail, _ := s.consoleTail(context.Background(), tailIn{Source: "check:task:test"})
	if !strings.Contains(tail, "läuft\n! warnung") {
		t.Errorf("Konsole: %q", tail)
	}
	status, _ := s.workbenchStatus(context.Background(), struct{}{})
	if !strings.Contains(status, "Lauf task:test · exit 0 · 12,4 s") {
		t.Errorf("Status: %q", status)
	}
}

func sample(t *testing.T, name string) []string {
	t.Helper()
	b, err := os.ReadFile("testdata/check/" + name + ".txt")
	if err != nil {
		t.Fatal(err)
	}
	return strings.Split(strings.ReplaceAll(string(b), "\r\n", "\n"), "\n")
}

func TestVerdichtungEchterLaeufe(t *testing.T) {
	cases := []struct {
		target string
		sample string
		want   []string
		not    string
	}{
		{"task:test", "vitest", []string{"FAIL  tests/zz-tmp-fail.test.ts > tmp > rechnet falsch",
			"AssertionError: expected 2 to be 3 // Object.is equality", "❯ tests/zz-tmp-fail.test.ts:4:19",
			"Test Files  1 failed (1)", "Tests  1 failed | 1 passed (2)"}, "Start at"},
		{"task:typecheck", "tsc", []string{"src/zzTmpFail.ts(1,14): error TS2322: Type 'string' is not assignable to type 'number'."}, ""},
		{"task:lint", "oxlint", []string{"src/zzTmpFail.ts:2:55: error eslint(max-depth): Blocks are nested too deeply (5). Maximum allowed is 4. help: Consider refactoring your code."}, "warning"},
		{"go:test", "gotest", []string{"--- FAIL: TestZZFalsch (0.00s)", "zz_tmp_test.go:7: 1+1 = 2, erwartet 3", "FAIL\tk3c/tools/k3c-dev\t0.907s"}, "ok "},
		{"go:lint", "golangci", []string{"zz_tmp_test.go:11:35: Error return value is not checked (errcheck)",
			"zz_tmp_test.go:13:6: func tmpErr is unused (unused)", "2 issues:"}, "func zz"},
		{"task:build", "vite", []string{"error during build:", "Build failed with 1 error:", "[plugin vite:build-html] C:/WORKSPACE/k3c/game.html",
			"Error: Failed to resolve /src/gibtsnicht.ts from C:/WORKSPACE/k3c/game.html"}, "at async"},
	}
	for _, c := range cases {
		target, _ := findTarget(c.target)
		text := report(target, runResult{exit: 1, dur: time.Second}, sample(t, c.sample))
		want := strings.Join(append([]string{c.target + " · exit 1 · 1,0 s"}, c.want...), "\n")
		if text != want {
			t.Errorf("%s:\n%s\nerwartet:\n%s", c.sample, text, want)
		}
		if c.not != "" && strings.Contains(text, c.not) {
			t.Errorf("%s enthält %q", c.sample, c.not)
		}
	}
}

func TestVerdichtungOhneMusterUndDeckel(t *testing.T) {
	target, _ := findTarget("task:test")
	var lines []string
	for i := range 30 {
		lines = append(lines, fmt.Sprintf("Zeile %d", i))
	}
	text := report(target, runResult{exit: 2}, lines)
	if got := strings.Split(text, "\n"); len(got) != 21 || got[1] != "Zeile 10" {
		t.Errorf("letzte 20 Zeilen: %q", text)
	}
	lines = nil
	for i := range 70 {
		lines = append(lines, fmt.Sprintf(" FAIL  test %d", i))
	}
	text = report(target, runResult{exit: 1}, lines)
	if got := strings.Split(text, "\n"); len(got) != maxErrorLines+2 || got[len(got)-1] != "… 10 weitere" {
		t.Errorf("Deckel: %d Zeilen, letzte %q", len(got), got[len(got)-1])
	}
	text = report(target, runResult{exit: -1, timedOut: true, dur: 5 * time.Minute}, nil)
	if text != "task:test · Zeitlimit 5 min 0 s überschritten · 5 min 0 s" {
		t.Errorf("Zeitlimit: %q", text)
	}
}

// TestHelperProcess ist kein Test: runProcess startet das Test-Binary damit als Kind- bzw. Enkelprozess.
func TestHelperProcess(t *testing.T) {
	switch os.Getenv("K3C_HELPER") {
	case "exit3":
		fmt.Println("vor dem Fehler")
		fmt.Fprintln(os.Stderr, "fehler")
		os.Exit(3)
	case "parent":
		// Startet einen Enkel, der stdout geerbt hat und offen hält, und wartet selbst.
		child := exec.Command(os.Args[0], "-test.run=^TestHelperProcess$")
		child.Env = append(os.Environ(), "K3C_HELPER=sleep")
		child.Stdout = os.Stdout
		_ = child.Start()
		time.Sleep(time.Minute)
	case "sleep":
		time.Sleep(time.Minute)
	}
}

func helper(t *testing.T, mode string, timeout time.Duration) (runResult, []string) {
	t.Setenv("K3C_HELPER", mode)
	var mu sync.Mutex
	var lines []string
	res := runProcess(context.Background(), runSpec{args: []string{os.Args[0], "-test.run=^TestHelperProcess$"},
		timeout: timeout, out: func(stream, text string) {
			mu.Lock()
			defer mu.Unlock()
			lines = append(lines, stream+":"+text)
		}})
	return res, lines
}

func TestRunProcessExitUndAusgabe(t *testing.T) {
	res, lines := helper(t, "exit3", time.Minute)
	joined := strings.Join(lines, "|")
	if res.exit != 3 || res.err != nil || !strings.Contains(joined, "stdout:vor dem Fehler") || !strings.Contains(joined, "stderr:fehler") {
		t.Errorf("exit %d, err %v, Zeilen %v", res.exit, res.err, lines)
	}
	res = runProcess(context.Background(), runSpec{args: []string{"gibt-es-nicht-k3c"}, timeout: time.Second, out: func(string, string) {}})
	if res.err == nil {
		t.Error("fehlendes Programm ohne Fehler")
	}
}

func TestZeitlimitBeendetDenProzessbaum(t *testing.T) {
	start := time.Now()
	res, _ := helper(t, "parent", 500*time.Millisecond)
	// Ohne Baum-Ende hielte der Enkel stdout offen und Run wartete bis WaitDelay (5 s) oder länger.
	if !res.timedOut || res.exit != -1 || time.Since(start) > 4*time.Second {
		t.Errorf("Zeitlimit: %+v nach %v", res, time.Since(start))
	}
}
