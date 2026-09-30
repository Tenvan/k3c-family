package mcpsrv

import (
	"context"
	"errors"
	"fmt"
	"path/filepath"
	"sync"
	"time"

	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/proc"
)

type checkIn struct {
	Target  string `json:"target" jsonschema:"Ziel aus dem Katalog: npm:check, npm:test, npm:typecheck, npm:lint, npm:build, go:test, go:lint, dev:test"`
	Pattern string `json:"pattern,omitempty" jsonschema:"optionales Testmuster, nur bei npm:test, go:test und dev:test"`
}

// runSpec ist ein Prozesslauf; out bekommt jede Zeile mit ihrem Strom.
type runSpec struct {
	dir     string
	args    []string
	timeout time.Duration
	out     func(stream, text string)
}

type runResult struct {
	exit     int
	dur      time.Duration
	timedOut bool
	err      error // Start gescheitert
	at       time.Time
}

func (r runResult) ms() float64 { return float64(r.dur.Microseconds()) / 1000 }

// checkRuns sperrt je Ziel einen Lauf und merkt sich den letzten Lauf je Ziel.
type checkRuns struct {
	mu      sync.Mutex
	running map[string]bool
	last    map[string]runResult
}

func newCheckRuns() *checkRuns {
	return &checkRuns{running: map[string]bool{}, last: map[string]runResult{}}
}

func (c *checkRuns) begin(name string) bool {
	c.mu.Lock()
	defer c.mu.Unlock()
	if c.running[name] {
		return false
	}
	c.running[name] = true
	return true
}

func (c *checkRuns) end(name string, res runResult) {
	c.mu.Lock()
	defer c.mu.Unlock()
	delete(c.running, name)
	c.last[name] = res
}

func (c *checkRuns) lastRun(name string) (runResult, bool) {
	c.mu.Lock()
	defer c.mu.Unlock()
	r, ok := c.last[name]
	return r, ok
}

// checkRun ist das Tool check_run: prüfen, sperren, laufen lassen, verdichten. Die volle Ausgabe steht im
// Konsolenpuffer check:<ziel>.
func (s *Server) checkRun(ctx context.Context, in checkIn) (string, error) {
	t, err := findTarget(in.Target)
	if err != nil {
		return "", err
	}
	args, err := t.args(in.Pattern)
	if err != nil {
		return "", err
	}
	if !s.checks.begin(t.name) {
		return "", fmt.Errorf("%s läuft bereits; auf das Ende warten, Ausgabe über console_tail check:%s", t.name, t.name)
	}
	s.notifyCheck(CheckState{Name: t.name, Running: true, At: time.Now()})
	var res runResult
	defer func() { // auch nach einer Panik, sonst bliebe das Ziel gesperrt
		s.checks.end(t.name, res)
		s.notifyCheck(stateOf(t.name, res))
	}()
	source := "check:" + t.name
	s.console.Reset(source)
	var mu sync.Mutex
	var output []string
	res = s.run(ctx, runSpec{dir: filepath.Join(s.cfg.Root, t.dir), args: args, timeout: t.timeout,
		out: func(stream, text string) {
			s.console.Add(source, stream, text)
			mu.Lock()
			output = append(output, text)
			mu.Unlock()
		}})
	s.logRun(t.name, res)
	if res.err != nil {
		return "", fmt.Errorf("%s ließ sich nicht starten: %w", t.name, res.err)
	}
	return report(t, res, output), nil
}

func (s *Server) logRun(name string, res runResult) {
	attrs := []any{"ns", "check", "target", name, "exit", res.exit, "ms", int64(res.ms())}
	switch {
	case res.err != nil:
		s.log.Error("lauf nicht gestartet", append(attrs, "error", res.err.Error())...)
	case res.timedOut:
		s.log.Warn("lauf abgebrochen: zeitlimit", attrs...)
	case res.exit != 0:
		s.log.Warn("lauf beendet", attrs...)
	default:
		s.log.Info("lauf beendet", attrs...)
	}
}

// runProcess startet einen Prozess mit Zeitlimit; das Zeitlimit beendet den ganzen Prozessbaum.
func runProcess(ctx context.Context, spec runSpec) runResult {
	ctx, cancel := context.WithTimeout(ctx, spec.timeout)
	defer cancel()
	cmd := proc.Command(ctx, spec.args)
	cmd.Dir = spec.dir
	stdout := console.NewLineWriter(func(text string) { spec.out("stdout", text) })
	stderr := console.NewLineWriter(func(text string) { spec.out("stderr", text) })
	cmd.Stdout, cmd.Stderr = stdout, stderr
	start := time.Now()
	err := cmd.Run()
	stdout.Flush()
	stderr.Flush()
	res := runResult{dur: time.Since(start), at: start}
	switch {
	case errors.Is(ctx.Err(), context.DeadlineExceeded):
		res.timedOut, res.exit = true, -1
	case cmd.ProcessState != nil:
		res.exit = cmd.ProcessState.ExitCode()
	case err != nil:
		res.err = err
	}
	return res
}
