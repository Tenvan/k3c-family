package room

import (
	"fmt"
	"log/slog"
	"time"
)

// Logging des Raummodells: Lebenszyklus der Räume und Geräte (Info), langsame Ticks (Warn, höchstens einmal je
// slowEvery) und eine Takt-Statistik je Raum einmal pro statsEvery (Info). Geräte-IDs erscheinen nur gekürzt.

const (
	slowTick   = time.Second / TickHz // ein Tick, der länger dauert, überzieht den Takt
	slowEvery  = 10 * time.Second
	statsEvery = time.Minute
)

// short kürzt eine Geräte-ID für das Log.
func short(id string) string {
	if len(id) > 8 {
		return id[:8]
	}
	return id
}

// log ist der Logger des Raums mit Code und Spielstand-Name.
func (r *Room) log() *slog.Logger {
	return r.m.log().With("ns", "room", "room", r.Code, "save", r.Name)
}

// noteTick merkt langsame Ticks vor und meldet sie gesammelt (unter Raum-Sperre).
func (r *Room) noteTick(d time.Duration) {
	if d <= slowTick {
		return
	}
	r.slowCount++
	now := time.Now()
	if now.Sub(r.slowLogged) < slowEvery {
		return
	}
	r.log().Warn("🐢 Tick zu langsam", "ms", float64(d)/float64(time.Millisecond), "budgetMs", float64(slowTick)/float64(time.Millisecond),
		"seitLetzterMeldung", r.slowCount, "tick", r.tick, "geraete", r.connected(), "faktor", r.scale())
	r.m.Monitor.Event(r.m.now(), r.Code, "slow", fmt.Sprintf("%.1f ms, %d seit letzter Meldung", float64(d)/float64(time.Millisecond), r.slowCount))
	r.slowLogged, r.slowCount = now, 0
}

// logStats schreibt die Takt-Statistik aller laufenden Räume (Manager.Run, einmal pro statsEvery).
func (m *Manager) logStats() {
	m.mu.Lock()
	defer m.mu.Unlock()
	for _, r := range m.rooms {
		r.mu.Lock()
		if n := r.connected(); n > 0 {
			last, p99 := r.tickMs()
			r.log().Info("💓 Raum-Takt", "tick", r.tick, "geraete", n, "spieler", r.count(Taken), "tickMs", last, "p99Ms", p99)
		}
		r.mu.Unlock()
	}
}

// logRejected meldet einen abgelehnten Beitritt (Fehler-Code aus docs/protocol.md); ziel ist Spielstand-Name oder Raum-Code.
func (m *Manager) logRejected(op, id, ziel string, err error) {
	if err != nil {
		m.log().Warn("🚫 Beitritt abgelehnt", "ns", "room", "op", op, "device", short(id), "ziel", ziel, "code", err.Error())
	}
}
