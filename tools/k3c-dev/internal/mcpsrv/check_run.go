package mcpsrv

import (
	"context"
	"errors"
	"sync"
	"time"

	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/proc"
)

type checkIn struct {
	Target  string `json:"target,omitempty" jsonschema:"Ziel aus dem Katalog: task:check, task:test, task:typecheck, task:lint, task:build, task:check:go, task:check:diff, task:check:race, go:test, go:lint, dev:test"`
	Pattern string `json:"pattern,omitempty" jsonschema:"optionales Testmuster, nur bei task:test, go:test und dev:test"`
	Run     string `json:"run,omitempty" jsonschema:"Lauf-ID aus der Antwort läuft im Hintergrund: liefert das Ergebnis dieses Laufs statt einen neuen zu starten"`
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
	running map[string]time.Time // Beginn des laufenden Laufs
	last    map[string]runResult
	jobs    map[string]*checkJob // Läufe mit Lauf-ID (check_job.go)
	order   []string             // IDs der Läufe, älteste zuerst
	seq     int
}

func newCheckRuns() *checkRuns {
	return &checkRuns{running: map[string]time.Time{}, last: map[string]runResult{}, jobs: map[string]*checkJob{}}
}

func (c *checkRuns) begin(name string) bool {
	c.mu.Lock()
	defer c.mu.Unlock()
	if _, ok := c.running[name]; ok {
		return false
	}
	c.running[name] = time.Now()
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


func (s *Server) logRun(name string, res runResult) {
	attrs := []any{"ns", "check", "target", name, "exit", res.exit, "ms", int64(res.ms())}
	switch {
	case res.err != nil:
		s.log.Error("💥 lauf nicht gestartet", append(attrs, "error", res.err.Error())...)
	case res.timedOut:
		s.log.Warn("⏳ lauf abgebrochen: zeitlimit", attrs...)
	case res.exit != 0:
		s.log.Warn("❌ lauf beendet", attrs...)
	default:
		s.log.Info("✅ lauf beendet", attrs...)
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
