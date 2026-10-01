package room

import (
	"context"
	"fmt"
	"slices"
	"time"
)

// Takt: eine Goroutine je Raum mit TickHz, dazu einmal pro Sekunde Sweep für die Fristen. Ein Panic im Tick schließt
// nur diesen Raum (room_closed an seine Geräte), der Fehler bleibt im Log und im Status.

const durations = 300 // Tick-Dauern für p99 (10 s)

// Failure ist ein abgestürzter Raum.
type Failure struct {
	Code  string    `json:"code"`
	Name  string    `json:"name"`
	Error string    `json:"error"`
	At    time.Time `json:"at"`
}

// Run taktet alle Räume, bis ctx endet. Räume, die später entstehen, bekommen ihre Goroutine bei Create.
func (m *Manager) Run(ctx context.Context) {
	m.mu.Lock()
	m.ctx = ctx
	for _, r := range m.rooms {
		go r.run(ctx)
	}
	m.mu.Unlock()
	sweep := time.NewTicker(time.Second)
	defer sweep.Stop()
	for {
		select {
		case <-ctx.Done():
			return
		case <-sweep.C:
			m.Sweep()
		}
	}
}

func (r *Room) run(ctx context.Context) {
	t := time.NewTicker(time.Second / TickHz)
	defer t.Stop()
	for {
		select {
		case <-ctx.Done():
			return
		case <-t.C:
			if !r.safeTick() {
				return
			}
		}
	}
}

// safeTick rechnet einen Tick und fängt einen Panic. false: Raum ist geschlossen.
func (r *Room) safeTick() (alive bool) {
	defer func() {
		if p := recover(); p != nil {
			r.m.crash(r, fmt.Sprint(p))
			alive = false
		}
	}()
	start := time.Now()
	if !r.Tick() {
		return false
	}
	r.mu.Lock()
	r.durations = append(r.durations, time.Since(start))
	if len(r.durations) > durations {
		r.durations = r.durations[1:]
	}
	r.mu.Unlock()
	return true
}

// crash schließt einen abgestürzten Raum, ohne ihn zu speichern (sein Zustand ist nicht mehr sicher).
func (m *Manager) crash(r *Room, msg string) {
	m.log().Error("Raum abgestürzt", "room", r.Code, "save", r.Name, "err", msg)
	m.mu.Lock()
	r.mu.Lock()
	r.closed = true
	for _, d := range r.devices {
		if d.connected {
			d.peer.Closed(false)
		}
	}
	r.mu.Unlock()
	delete(m.rooms, r.Code)
	m.failures = append(m.failures, Failure{r.Code, r.Name, msg, m.now()})
	if len(m.failures) > 10 {
		m.failures = m.failures[1:]
	}
	m.mu.Unlock()
	m.notify(true)
}

// tickMs: letzte Tick-Dauer und p99 der letzten Ticks in Millisekunden (unter Raum-Sperre).
func (r *Room) tickMs() (last, p99 float64) {
	if len(r.durations) == 0 {
		return 0, 0
	}
	sorted := slices.Clone(r.durations)
	slices.Sort(sorted)
	ms := func(d time.Duration) float64 { return float64(d) / float64(time.Millisecond) }
	return ms(r.durations[len(r.durations)-1]), ms(sorted[(len(sorted)*99)/100])
}
