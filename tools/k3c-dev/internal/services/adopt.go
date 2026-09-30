package services

import (
	"context"
	"errors"
	"fmt"
	"path/filepath"
	"time"

	"k3c/tools/k3c-dev/internal/logs"
)

// lostAdopted ist der Grund, wenn ein übernommener Dienst nicht mehr antwortet.
const lostAdopted = "übernommener Prozess nicht mehr erreichbar"

// Adopt übernimmt beim Programmstart Dienste, deren Prüfung schon besteht: ohne Konsole, ohne Auto-Restart,
// Stopp nur mit force.
func (c *Controller) Adopt(ctx context.Context) {
	for _, u := range c.units {
		u.cmd.Lock()
		if u.status().State == Stopped && c.opts.Check(ctx, u.svc) == nil {
			pid, _ := c.opts.Listen(ctx, u.svc.Port)
			c.adopt(ctx, u, pid)
		}
		u.cmd.Unlock()
	}
}

func (c *Controller) adopt(ctx context.Context, u *unit, pid int) {
	started := c.opts.Now()
	if m, err := c.opts.Sample(ctx, pid); pid > 0 && err == nil && !m.Started.IsZero() {
		started = m.Started
	}
	wctx, cancel := context.WithCancel(context.Background())
	u.mu.Lock()
	u.unwatch = cancel
	u.mu.Unlock()
	c.set(u, func(s *Status) { s.State, s.PID, s.StartedAt, s.LastError = Adopted, pid, started, "" })
	go c.watchAdopted(wctx, u)
}

// watchAdopted prüft einen übernommenen Dienst; nach FailLimit Fehlschlägen in Folge gilt er als gestoppt.
func (c *Controller) watchAdopted(ctx context.Context, u *unit) {
	tick := time.NewTicker(c.opts.WatchEvery)
	defer tick.Stop()
	fails := 0
	for {
		select {
		case <-ctx.Done():
			return
		case <-tick.C:
			if c.opts.Check(ctx, u.svc) == nil {
				fails = 0
				continue
			}
			if fails++; fails < c.opts.FailLimit {
				continue
			}
			u.cmd.Lock()
			if ctx.Err() == nil && u.status().State == Adopted {
				c.release(u)
				c.set(u, func(s *Status) { s.State, s.PID, s.LastError, s.CPU, s.Memory = Stopped, 0, lostAdopted, 0, 0 })
			}
			u.cmd.Unlock()
			return
		}
	}
}

// release beendet die Überwachung eines übernommenen Dienstes. Aufruf unter u.cmd.
func (c *Controller) release(u *unit) {
	u.mu.Lock()
	if u.unwatch != nil {
		u.unwatch()
		u.unwatch = nil
	}
	u.mu.Unlock()
}

// stopAdopted beendet den fremden Prozessbaum eines übernommenen Dienstes, nur mit force. Aufruf unter u.cmd.
func (c *Controller) stopAdopted(u *unit, force bool) (Status, error) {
	st := u.status()
	if !force {
		return st, fmt.Errorf("%s ist übernommen (vor k3c-dev gestartet, PID %d); Stopp nur mit force", u.svc.Name, st.PID)
	}
	if st.PID <= systemPID {
		return st, errors.New(u.svc.Name + ": PID des übernommenen Prozesses unbekannt, bitte von Hand beenden")
	}
	// Die PID von der Übernahme kann veraltet sein (von Hand neu gestartet, inzwischen neu vergeben): beendet wird nur,
	// wer jetzt am Port lauscht, und nur, wenn es derselbe Prozess ist.
	pid, listening := c.opts.Listen(context.Background(), u.svc.Port)
	switch {
	case !listening:
		c.release(u)
		return c.set(u, func(s *Status) { s.State, s.PID, s.LastError, s.CPU, s.Memory = Stopped, 0, lostAdopted, 0, 0 }), nil
	case pid != st.PID:
		return st, fmt.Errorf("%s: am Port lauscht jetzt PID %d statt %d; nichts beendet, bitte prüfen", u.svc.Name, pid, st.PID)
	}
	c.release(u)
	c.set(u, func(s *Status) { s.State = Stopping })
	if err := c.opts.KillPID(pid); err != nil {
		c.opts.Log.Warn("dienst "+u.svc.Name+": übernommenen Prozess beenden: "+err.Error(), "ns", "svc")
	}
	if !c.waitPortFree(u) {
		c.adopt(context.Background(), u, pid) // lebt weiter: bleibt übernommen und überwacht
		return u.status(), fmt.Errorf("%s: PID %d läuft nach dem Beenden weiter (Port %d belegt)", u.svc.Name, pid, u.svc.Port)
	}
	return c.set(u, func(s *Status) { s.State, s.PID, s.LastError, s.CPU, s.Memory = Stopped, 0, "", 0, 0 }), nil
}

// waitPortFree wartet höchstens StopTimeout, bis niemand mehr am Port lauscht.
func (c *Controller) waitPortFree(u *unit) bool {
	deadline := time.Now().Add(c.opts.StopTimeout)
	for time.Now().Before(deadline) {
		if _, busy := c.opts.Listen(context.Background(), u.svc.Port); !busy {
			return true
		}
		time.Sleep(50 * time.Millisecond)
	}
	return false
}

// portBusy liefert den Grund, wenn ein fremder Prozess den Port schon belegt, sonst "". Aufruf unter u.cmd.
func (c *Controller) portBusy(ctx context.Context, u *unit) string {
	pid, busy := c.opts.Listen(ctx, u.svc.Port)
	if !busy {
		return ""
	}
	who := "PID unbekannt"
	if pid > 0 {
		who = fmt.Sprintf("PID %d", pid)
	}
	return fmt.Sprintf("Port %d bereits belegt (%s)", u.svc.Port, who)
}

// Monitor misst alle every die laufenden und übernommenen Dienste, bis ctx endet.
func (c *Controller) Monitor(ctx context.Context, every time.Duration) {
	tick := time.NewTicker(every)
	defer tick.Stop()
	for {
		select {
		case <-ctx.Done():
			return
		case <-tick.C:
			c.sampleAll(ctx)
		}
	}
}

// sampleAll misst einmal; eine gescheiterte Messung lässt den letzten Wert stehen, über den Zustand entscheidet die
// Überwachung. Gemessen wird der Prozess, der am Port lauscht: der eigene Start ist oft nur eine Hülle (npm.cmd →
// cmd.exe → node), CPU und Speicher braucht aber der Server selbst.
func (c *Controller) sampleAll(ctx context.Context) {
	for _, u := range c.units {
		st := u.status()
		if (st.State != Running && st.State != Adopted) || st.PID <= 0 {
			continue
		}
		pid := st.PID
		if lp, ok := c.opts.Listen(ctx, u.svc.Port); ok && lp > 0 {
			pid = lp
		}
		m, err := c.opts.Sample(ctx, pid)
		if err != nil {
			continue
		}
		c.set(u, func(s *Status) {
			if s.PID == st.PID {
				s.CPU, s.Memory = m.CPU, m.Memory
			}
		})
	}
}

// LevelCounts sind die Log-Einträge eines Dienstes je Level der letzten 60 min.
type LevelCounts struct {
	Counts    map[string]int `json:"counts"`
	Total     int            `json:"total"`
	BudgetHit bool           `json:"budgetHit"` // ältere Einträge der 60 min nicht gelesen
}

// LogLevels zählt die Einträge je Level aus logs/<log>.jsonl; ohne log in der Konfiguration ist ok false.
func (c *Controller) LogLevels(name string) (counts LevelCounts, ok bool, err error) {
	u, err := c.unit(name)
	if err != nil || u.svc.Log == "" {
		return LevelCounts{}, false, err
	}
	res, err := logs.Scan(filepath.Join(c.opts.Root, "logs", u.svc.Log+".jsonl"),
		logs.Query{Since: c.opts.Now().Add(-time.Hour)})
	if err != nil {
		return LevelCounts{}, true, err
	}
	counts = LevelCounts{Counts: map[string]int{}, Total: len(res.Entries), BudgetHit: res.BudgetHit}
	for _, e := range res.Entries {
		counts.Counts[e.Level]++
	}
	return counts, true, nil
}
