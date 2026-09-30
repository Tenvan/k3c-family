package services

import (
	"context"
	"fmt"
	"time"
)

// start startet u und wartet auf die erste bestandene Prüfung. Aufruf unter u.cmd.
func (c *Controller) start(ctx context.Context, u *unit) (Status, error) {
	if st := u.status(); st.State == Running || st.State == Starting || st.State == Adopted {
		return st, fmt.Errorf("%s läuft bereits (%s)", u.svc.Name, st.State)
	}
	if reason := c.portBusy(ctx, u); reason != "" {
		st := c.fail(u, reason)
		return st, fmt.Errorf("%s: %s", u.svc.Name, reason)
	}
	c.set(u, func(s *Status) { s.State, s.LastError, s.PID = Starting, "", 0 })
	c.opts.Console.Reset(u.svc.Name)
	p, err := c.opts.Start(u.svc, c.opts.Root, func(stream, text string) { c.opts.Console.Add(u.svc.Name, stream, text) })
	if err != nil {
		st := c.fail(u, "start: "+err.Error())
		return st, fmt.Errorf("%s ließ sich nicht starten: %w", u.svc.Name, err)
	}
	r := &run{proc: p, done: make(chan struct{})}
	go func() { r.err = p.Wait(); close(r.done) }()
	u.mu.Lock()
	u.run = r
	u.mu.Unlock()
	c.set(u, func(s *Status) { s.PID, s.StartedAt = p.PID(), c.opts.Now() })
	began := time.Now()
	if err := c.awaitHealthy(ctx, u, r); err != nil {
		_ = c.killRun(u, r)
		st := c.fail(u, err.Error())
		return st, fmt.Errorf("%s: %w", u.svc.Name, err)
	}
	wctx, cancel := context.WithCancel(context.Background())
	r.stop = cancel
	go c.watch(wctx, u, r)
	c.opts.Log.Info("dienst "+u.svc.Name+" gesund", "ns", "svc", "ms", time.Since(began).Milliseconds())
	return c.set(u, func(s *Status) { s.State = Running }), nil
}

// awaitHealthy prüft alle StartPoll, bis die Prüfung besteht, der Prozess endet oder StartTimeout abläuft.
func (c *Controller) awaitHealthy(ctx context.Context, u *unit, r *run) error {
	deadline := time.NewTimer(c.opts.StartTimeout)
	defer deadline.Stop()
	tick := time.NewTicker(c.opts.StartPoll)
	defer tick.Stop()
	for {
		select {
		case <-r.done:
			return fmt.Errorf("prozess endete, bevor er gesund war (%v)", r.err)
		case <-deadline.C:
			return fmt.Errorf("nach %s nicht gesund", c.opts.StartTimeout)
		case <-ctx.Done():
			return fmt.Errorf("start abgebrochen: %w", ctx.Err())
		case <-tick.C:
			if c.opts.Check(ctx, u.svc) == nil {
				return nil
			}
		}
	}
}

// stop beendet den eigenen Prozess von u. Endet er nicht, meldet stop das als Fehler, statt "gestoppt" zu
// behaupten. Aufruf unter u.cmd.
func (c *Controller) stop(u *unit) (Status, error) {
	u.mu.Lock()
	r := u.run
	u.mu.Unlock()
	if r == nil {
		return u.status(), nil
	}
	c.set(u, func(s *Status) { s.State = Stopping })
	if !c.killRun(u, r) {
		reason := fmt.Sprintf("prozess (PID %d) endet nach %s nicht", r.proc.PID(), c.opts.StopTimeout)
		return c.fail(u, reason), fmt.Errorf("%s: %s", u.svc.Name, reason)
	}
	return c.set(u, func(s *Status) { s.State, s.PID, s.LastError, s.CPU, s.Memory = Stopped, 0, "", 0, 0 }), nil
}

// killRun beendet Überwachung und Prozessbaum und wartet höchstens StopTimeout auf das Ende; false heißt: der
// Prozess läuft weiter.
func (c *Controller) killRun(u *unit, r *run) bool {
	if r.stop != nil {
		r.stop()
	}
	if err := r.proc.Kill(); err != nil {
		c.opts.Log.Warn("dienst "+u.svc.Name+": beenden: "+err.Error(), "ns", "svc")
	}
	ended := true
	select {
	case <-r.done:
	case <-time.After(c.opts.StopTimeout):
		ended = false
		c.opts.Log.Error("dienst "+u.svc.Name+": prozess endet nicht", "ns", "svc", "pid", r.proc.PID())
	}
	u.mu.Lock()
	if u.run == r {
		u.run = nil
	}
	u.mu.Unlock()
	return ended
}

// watch prüft einen laufenden Dienst alle WatchEvery; Prozessende oder FailLimit Fehlschläge in Folge beenden ihn.
func (c *Controller) watch(ctx context.Context, u *unit, r *run) {
	tick := time.NewTicker(c.opts.WatchEvery)
	defer tick.Stop()
	fails := 0
	for {
		select {
		case <-ctx.Done():
			return
		case <-r.done:
			c.crashed(u, r, fmt.Sprintf("prozess beendet (%v)", r.err))
			return
		case <-tick.C:
			if err := c.opts.Check(ctx, u.svc); err == nil {
				fails = 0
			} else if fails++; fails >= c.opts.FailLimit {
				c.crashed(u, r, fmt.Sprintf("%d Prüfungen in Folge fehlgeschlagen: %v", fails, err))
				return
			}
		}
	}
}

// crashed behandelt den Ausfall eines laufenden Dienstes: beenden, melden und mit Grenze neu starten. Ein Befehl,
// der inzwischen gestoppt oder neu gestartet hat, gewinnt (u.run ist dann ein anderer Lauf).
func (c *Controller) crashed(u *unit, r *run, reason string) {
	u.cmd.Lock()
	defer u.cmd.Unlock()
	u.mu.Lock()
	current := u.run == r
	u.mu.Unlock()
	if !current {
		return
	}
	_ = c.killRun(u, r) // der Prozess ist schon beendet (Ausfall), Kill tut dann nichts mehr
	if !u.svc.AutoRestart {
		c.fail(u, reason)
		return
	}
	if !c.allowRestart(u) {
		c.fail(u, fmt.Sprintf("%s; kein Neustart mehr (%d in %s)", reason, c.opts.RestartLimit, c.opts.RestartWindow))
		return
	}
	c.fail(u, reason+"; Neustart")
	_, _ = c.start(context.Background(), u)
}

// allowRestart zählt einen Neustart, solange weniger als RestartLimit im RestartWindow liegen.
func (c *Controller) allowRestart(u *unit) bool {
	now := c.opts.Now()
	u.mu.Lock()
	defer u.mu.Unlock()
	kept := u.restarts[:0]
	for _, t := range u.restarts {
		if now.Sub(t) < c.opts.RestartWindow {
			kept = append(kept, t)
		}
	}
	u.restarts = kept
	if len(u.restarts) >= c.opts.RestartLimit {
		return false
	}
	u.restarts = append(u.restarts, now)
	u.st.Restarts++
	return true
}
