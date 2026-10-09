package mcpsrv

import (
	"context"
	"fmt"
	"path/filepath"
	"slices"
	"strings"
	"time"

	"k3c/tools/k3c-dev/internal/balance"
	"k3c/tools/k3c-dev/internal/botdev"
	"k3c/tools/k3c-dev/internal/serverapi"
)

// Modus online headless (B-348/AC-03): `rooms` Räume mit je `players` Bot-Geräten gegen den Spielserver des Checkouts.
// Gemessen werden Tick-Dauer (p99 je Raum aus /api/status), CPU, Trennungen, Fehler und neue Server-Fehler; dazu die
// Ereignisse der Stufen (castleFallen, wave, built) für einen ersten Blick aufs Balancing.

const targetP99 = 10.0 // ms, Ziel aus B-042

var onlineSample = 5 * time.Second // Abstand der Proben; Test-Naht

// onlineRun sammelt die Messung eines Laufs.
type onlineRun struct {
	devices  []*botdev.Device
	codes    []string
	maxP99   float64
	maxCPU   float64
	samples  int
	failures int // Server-Fehler (/api/status › failures) beim Start
}

func (s *Server) runOnline(ctx context.Context, run *simRun, spec simSpec, c *serverapi.Client) {
	o := &onlineRun{}
	if st, err := c.Status(ctx); err == nil {
		o.failures = len(st.Failures)
	}
	s.sims.setPhase(run, "verbinden")
	err := o.connect(ctx, c.Base, run.id, spec)
	defer func() {
		for _, d := range o.devices {
			d.Stop()
		}
	}()
	if err != nil {
		s.finishRun(run, "fehler", "", []string{"verbinden: " + err.Error()}, simReportDir(run))
		return
	}
	s.sims.setPhase(run, "messen")
	deadline := time.After(spec.duration)
	tick := time.NewTicker(onlineSample)
	defer tick.Stop()
	for done := false; !done; {
		select {
		case <-ctx.Done():
			done = true
		case <-deadline:
			done = true
		case <-tick.C:
			o.sample(ctx, c)
			s.sims.setLines(run, o.lines(spec, 0))
		}
	}
	o.sample(context.Background(), c)
	newFailures := 0
	if st, err := c.Status(context.Background()); err == nil {
		newFailures = len(st.Failures) - o.failures
	}
	verdict := o.verdict(spec, newFailures)
	s.finishRun(run, "fertig", verdict, o.lines(spec, newFailures), simReportDir(run))
}

// connect: je Raum legt das erste Gerät einen Raum an (Name mit Präfix test-), die übrigen treten bei.
func (o *onlineRun) connect(ctx context.Context, base, id string, spec simSpec) error {
	bot, _ := balance.BotByName(spec.bots)
	for r := range spec.rooms {
		var code string
		for p := range spec.players {
			d, err := botdev.Dial(ctx, base, fmt.Sprintf("simtest-%s-%d-%d", id, r, p), bot)
			if err != nil {
				return err
			}
			o.devices = append(o.devices, d)
			if p == 0 {
				code, err = d.Create(ctx, fmt.Sprintf("test-sim-%s-%d", id, r))
				o.codes = append(o.codes, code)
			} else {
				err = d.Join(ctx, code)
			}
			if err != nil {
				return err
			}
		}
	}
	return nil
}

// sample liest /api/status: höchstes p99 der eigenen Räume und die CPU.
func (o *onlineRun) sample(ctx context.Context, c *serverapi.Client) {
	st, err := c.Status(ctx)
	if err != nil {
		return
	}
	o.samples++
	for _, r := range st.Rooms {
		if slices.Contains(o.codes, r.Code) {
			o.maxP99 = max(o.maxP99, r.TickMs.P99)
		}
	}
	if st.CPU != nil {
		o.maxCPU = max(o.maxCPU, *st.CPU)
	}
}

// totals summiert die Zähler aller Geräte.
func (o *onlineRun) totals() (states, inputs, errs, lost, moved int, events map[string]int) {
	events = map[string]int{}
	for _, d := range o.devices {
		st := d.Stats()
		states, inputs, errs = states+st.States, inputs+st.Inputs, errs+st.Errors
		if st.Disconnected {
			lost++
		}
		if st.Moved {
			moved++
		}
		for k, v := range st.Events {
			events[k] += v
		}
	}
	return states, inputs, errs, lost, moved, events
}

func (o *onlineRun) lines(spec simSpec, newFailures int) []string {
	states, inputs, errs, lost, moved, ev := o.totals()
	cpu := "–"
	if o.maxCPU > 0 {
		cpu = fmt.Sprintf("%.0f %%", o.maxCPU)
	}
	out := []string{
		fmt.Sprintf("%d Räume × %d Bots (%s), %d Proben", spec.rooms, spec.players, spec.bots, o.samples),
		fmt.Sprintf("Tick p99 max %.1f ms (Ziel ≤ %.0f ms), CPU max %s", o.maxP99, targetP99, cpu),
		fmt.Sprintf("Zustände %d, Eingaben %d, Monarchen bewegt %d/%d", states, inputs, moved, len(o.devices)),
		fmt.Sprintf("Trennungen %d, Fehler %d, neue Server-Fehler %d", lost, errs, newFailures),
	}
	if slices.Contains(spec.focus, "balance") {
		out = append(out, fmt.Sprintf("Ereignisse: Burgfälle %d, Wellen %d, gebaut %d", ev["castleFallen"], ev["wave"], ev["built"]))
	}
	return out
}

// verdict: perf besteht mit p99 ≤ Ziel, stability ohne Trennung, Fehler und neue Server-Fehler; balance online bewertet
// nichts (nur Ereignisse), die Ziele bewertet mode offline.
func (o *onlineRun) verdict(spec simSpec, newFailures int) string {
	_, _, errs, lost, _, _ := o.totals()
	var fails []string
	if slices.Contains(spec.focus, "perf") && (o.samples == 0 || o.maxP99 > targetP99) {
		fails = append(fails, "perf")
	}
	if slices.Contains(spec.focus, "stability") && errs+lost+newFailures > 0 {
		fails = append(fails, "stability")
	}
	if len(fails) > 0 {
		return "Fail (" + strings.Join(fails, ", ") + ")"
	}
	return "Pass"
}

func simReportDir(run *simRun) string { return filepath.Join("reports", "simtest-"+run.id) }
