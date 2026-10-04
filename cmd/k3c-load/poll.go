package main

import (
	"context"
	"time"
)

// poller fragt /api/status in festem Abstand und hängt die Proben an die Räume (Tick-Dauer, Phase, CPU).
type poller struct {
	a     *api
	runs  map[string]*roomRun
	start time.Time
	night bool            // true: nach der ersten Nacht aller Räume beenden
	seen  map[string]bool // Raum hat Nacht erlebt
	over  map[string]bool // Raum ist danach wieder bei Tag
}

func newPoller(a *api, runs []*roomRun, night bool) *poller {
	p := &poller{a: a, runs: map[string]*roomRun{}, start: time.Now(), night: night, seen: map[string]bool{}, over: map[string]bool{}}
	for _, r := range runs {
		p.runs[r.Name] = r
	}
	return p
}

// run nimmt sofort und dann alle every eine Probe, bis ctx endet oder (night) die erste Nacht vorbei ist; dann ruft es stop.
func (p *poller) run(ctx context.Context, every time.Duration, stop context.CancelFunc) {
	t := time.NewTicker(every)
	defer t.Stop()
	for {
		if p.poll(ctx) && p.night {
			stop()
			return
		}
		select {
		case <-ctx.Done():
			return
		case <-t.C:
		}
	}
}

// poll hängt eine Probe je Test-Raum an; true heißt: in allen Räumen ist die erste Nacht vorbei (Phase wieder „day“).
// Ein fehlgeschlagener Abruf lässt eine Lücke in der Reihe.
func (p *poller) poll(ctx context.Context) bool {
	st, err := p.a.status(ctx)
	if err != nil {
		return false
	}
	at := time.Since(p.start).Seconds()
	for _, sr := range st.Rooms {
		r, ok := p.runs[sr.Name]
		if !ok {
			continue
		}
		r.Samples = append(r.Samples, sample{AtS: at, Phase: sr.phase(), Tick: sr.Tick, TickLast: sr.TickMs.Last, TickP99: sr.TickMs.P99, CPU: st.CPU})
		switch {
		case sr.phase() == "night":
			p.seen[sr.Name] = true
		case p.seen[sr.Name] && sr.phase() == "day":
			p.over[sr.Name] = true
		}
	}
	return len(p.over) == len(p.runs)
}
