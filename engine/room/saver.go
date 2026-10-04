package room

import (
	"encoding/json"
	"sync"
	"time"
)

// Speichern (docs/protocol.md › Spielstand, B-276): Unter der Raum-Sperre wird der Spielstand nur kodiert, die Datei
// schreibt eine Goroutine je Raum, Speicherungen nacheinander, der neueste wartende Stand gewinnt. Schließen,
// Herunterfahren und SaveNow warten auf sie (flush) und schreiben dann selbst. Fehler landen im Log, der Raum läuft weiter.

// saver hält den noch nicht geschriebenen Stand eines Raums.
type saver struct {
	mu      sync.Mutex
	wg      sync.WaitGroup // läuft eine Schreib-Goroutine? Add (save) und Wait (flush) nur unter der Raum-Sperre
	pending []byte         // neuester Stand, nil = keiner
	tick    int
	busy    bool
}

// save kodiert den Spielstand und lässt ihn außerhalb der Sperre schreiben (im Takt, beim Verlassen und Trennen).
func (r *Room) save() {
	data, err := r.encode()
	if err != nil {
		r.log().Error("💥 Spielstand nicht gespeichert", "err", err)
		return
	}
	s := &r.saves
	s.mu.Lock()
	defer s.mu.Unlock()
	s.pending, s.tick = data, r.tick
	if !s.busy {
		s.busy = true
		s.wg.Go(r.writeSaves)
	}
}

// writeSaves schreibt wartende Stände, bis keiner mehr wartet.
func (r *Room) writeSaves() {
	s := &r.saves
	s.mu.Lock()
	defer s.mu.Unlock()
	for s.pending != nil {
		data, tick := s.pending, s.tick
		s.pending = nil
		s.mu.Unlock()
		r.write(data, tick)
		s.mu.Lock()
	}
	s.busy = false
}

// flush wartet, bis kein Spielstand mehr aussteht (unter der Raum-Sperre).
func (r *Room) flush() { r.saves.wg.Wait() }

// saveNow speichert synchron (Aufräumen, Herunterfahren): erst ausstehende Stände, dann der aktuelle.
func (r *Room) saveNow() {
	r.flush()
	data, err := r.encode()
	if err != nil {
		r.log().Error("💥 Spielstand nicht gespeichert", "err", err)
		return
	}
	r.write(data, r.tick)
}

// write schreibt einen kodierten Stand und meldet das Ergebnis im Log.
func (r *Room) write(data []byte, tick int) {
	start := time.Now() // Wanduhr nur fürs Log, nicht für den Spielverlauf
	backup, err := r.m.Store.Store(r.Name, data)
	ms := time.Since(start).Milliseconds()
	if err != nil {
		r.log().Error("💥 Spielstand nicht gespeichert", "err", err, "ms", ms)
		return
	}
	if ms > slowSaveMs {
		r.log().Warn("🐢 Spielstand gespeichert, langsamer als ein Tick", "tick", tick, "sicherung", backup, "ms", ms)
		return
	}
	r.log().Debug("💾 Spielstand gespeichert", "tick", tick, "sicherung", backup, "ms", ms)
}

// slowSaveMs ist der Zielwert einer Speicherung (B-147, S2.2): ein Tick bei 30 Hz.
const slowSaveMs = 1000 / TickHz

// encode ist der Spielstand als JSON (unter der Raum-Sperre).
func (r *Room) encode() ([]byte, error) {
	return json.Marshal(r.isl.ToSave(r.m.now().UTC().Format("2006-01-02T15:04:05.000Z07:00")))
}
