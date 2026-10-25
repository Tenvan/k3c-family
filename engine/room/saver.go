package room

import (
	"encoding/json"
	"fmt"
	"sync"
	"time"

	"k3c/engine/sim"
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
	written bool // seit dem letzten Tick gelungen geschrieben: der nächste Zustand trägt das Ereignis saved (B-186)
	due     int  // Ticks seit dem letzten Speichern (Takt SaveEvery); nur unter der Raum-Sperre
}

// saveTicks ist SaveEvery in Raum-Ticks.
const saveTicks = int(SaveEvery * TickHz / time.Second)

// autosave speichert im Takt SaveEvery, bei Tagesanbruch und nach einem Stufen- oder Inselwechsel (B-186, im Tick).
func (r *Room) autosave(travelled, dawn bool) {
	r.saves.due++
	if travelled || dawn || r.saves.due >= saveTicks {
		r.save()
	}
}

// pushStatesSaved schickt die Zustände wie pushStates. Wurde seit dem letzten Tick ein Spielstand geschrieben, trägt
// jede Stufe dabei das Ereignis saved (docs/protocol.md › Ereignisse); danach verschwindet es wieder aus der Welt, denn
// es ist kein Ereignis der Simulation (Pause und Golden-Hash bleiben unberührt).
func (r *Room) pushStatesSaved() bool {
	s := &r.saves
	s.mu.Lock()
	written := s.written
	s.written = false
	s.mu.Unlock()
	if !written || r.isl.Over { // nach Game Over meldet der Raum kein „gesichert“ mehr (B-344), auch für ältere Stände
		return r.pushStates()
	}
	for i, w := range r.isl.Stages {
		w.Events = append(w.Events, sim.Event{"type": "saved", "stage": i})
	}
	sent := r.pushStates()
	for _, w := range r.isl.Stages {
		w.Events = w.Events[:len(w.Events)-1]
	}
	return sent
}

// save kodiert den Spielstand und lässt ihn außerhalb der Sperre schreiben (im Takt, beim Verlassen und Trennen).
func (r *Room) save() {
	r.saves.due = 0
	if r.gameOver() {
		return
	}
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

// writeSaves schreibt wartende Stände, bis keiner mehr wartet. Ein Panic beim Schreiben beendet nur diese Goroutine
// (Log), die nächste Speicherung startet eine neue.
func (r *Room) writeSaves() {
	s := &r.saves
	defer func() {
		if p := recover(); p != nil {
			r.log().Error("💥 Spielstand nicht gespeichert, Speichern abgestürzt", "err", fmt.Sprint(p))
			s.mu.Lock()
			s.busy = false
			s.mu.Unlock()
		}
	}()
	for {
		s.mu.Lock()
		data, tick := s.pending, s.tick
		s.pending = nil
		if data == nil {
			s.busy = false
			s.mu.Unlock()
			return
		}
		s.mu.Unlock()
		if r.write(data, tick) {
			s.mu.Lock()
			s.written = true
			s.mu.Unlock()
		}
	}
}

// flush wartet, bis kein Spielstand mehr aussteht (unter der Raum-Sperre).
func (r *Room) flush() { r.saves.wg.Wait() }

// saveNow speichert synchron (Aufräumen, Herunterfahren): erst ausstehende Stände, dann der aktuelle.
func (r *Room) saveNow() {
	r.flush()
	if r.gameOver() {
		return
	}
	data, err := r.encode()
	if err != nil {
		r.log().Error("💥 Spielstand nicht gespeichert", "err", err)
		return
	}
	r.write(data, r.tick)
}

// gameOver: Die Insel ist „Komplett verloren“ (Island.Over, B-344). Der Raum schreibt dann keinen Spielstand mehr,
// der zuletzt gespeicherte bleibt Byte für Byte gleich; ausstehende Stände von vor dem Game Over schreibt er noch.
func (r *Room) gameOver() bool {
	if !r.isl.Over {
		return false
	}
	r.log().Info("🚫 Spielstand nicht gespeichert, Insel komplett verloren", "tick", r.tick)
	return true
}

// write schreibt einen kodierten Stand und meldet das Ergebnis im Log. true: geschrieben.
func (r *Room) write(data []byte, tick int) bool {
	start := time.Now() // Wanduhr nur fürs Log, nicht für den Spielverlauf
	backup, err := r.m.Store.Store(r.Name, data)
	took := time.Since(start)
	r.m.Monitor.saved(took)
	ms := took.Milliseconds()
	if err != nil {
		r.log().Error("💥 Spielstand nicht gespeichert", "err", err, "ms", ms)
		return false
	}
	if ms > slowSaveMs {
		r.log().Warn("🐢 Spielstand gespeichert, langsamer als ein Tick", "tick", tick, "sicherung", backup, "ms", ms)
		return true
	}
	r.log().Debug("💾 Spielstand gespeichert", "tick", tick, "sicherung", backup, "ms", ms)
	return true
}

// slowSaveMs ist der Zielwert einer Speicherung (B-147, S2.2): ein Tick bei 30 Hz.
const slowSaveMs = 1000 / TickHz

// encode ist der Spielstand als JSON (unter der Raum-Sperre).
func (r *Room) encode() ([]byte, error) {
	return json.Marshal(r.isl.ToSave(r.m.now().UTC().Format("2006-01-02T15:04:05.000Z07:00")))
}
