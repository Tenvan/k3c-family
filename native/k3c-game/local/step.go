package local

import (
	"fmt"
	"slices"

	"k3c/engine/level"
	"k3c/engine/room"
	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// stepHz ist der Takt von Step (Entscheidung B-793: Simulation 30 Hz ohne Zwischenbilder).
const stepHz = room.TickHz

// Command setzt die gehaltene Eingabe des lokalen Spielers player; sie gilt ab dem nächsten Step (wie Room.Input).
// Ohne laufendes Spiel geschieht nichts.
func (s *Session) Command(player int, c view.Command) error {
	if s.r == nil {
		return nil
	}
	if _, ok := s.devs[player]; !ok {
		return userErr(fmt.Sprintf("Spieler %d spielt nicht", player+1), nil)
	}
	clear(s.cmd)
	s.cmd[player] = c.Sim()
	return s.r.Input(deviceID, s.p, s.cmd)
}

// Step rechnet einen Takt (1/30 s), außer ohne Spiel und in der Pause, und liefert den Frame. Die Schleife ruft Step
// einmal je Update bei 30 Updates je Sekunde. Stages, Seats und Events des Frames liegen in Puffern, die der nächste
// Step überschreibt.
func (s *Session) Step() view.Frame {
	s.steps++
	if s.r != nil && !s.paused {
		s.tick()
	}
	s.collectSaves()
	f := view.Frame{Clock: float64(s.steps) / stepHz, Paused: s.paused, Timescale: 1, Cues: s.cues, Settings: s.settings}
	s.cues = nil
	if s.p != nil {
		f.Tick, f.Stages, f.Events, f.Seats = s.p.tick, s.p.stages(), s.p.events, s.seats()
		s.p.events = s.p.events[:0]
	}
	return f
}

// tick taktet den Raum einmal. Ein Panic schließt das Spiel ohne Speichern; der letzte Spielstand bleibt.
func (s *Session) tick() {
	defer func() {
		if p := recover(); p != nil {
			s.crash(fmt.Sprint(p))
		}
	}()
	s.p.begin()
	if !s.r.Tick() {
		s.crash("Raum geschlossen")
	}
}

// crash verwirft Raum und Manager, ohne zu speichern (der Zustand ist nicht mehr sicher), und meldet es dem Menü.
func (s *Session) crash(msg string) {
	s.log.Error("💥 Spiel abgestürzt", "save", s.name, "err", msg)
	s.cue(view.CueError, -1, "Spiel abgestürzt, der letzte Spielstand bleibt: "+msg)
	s.reset()
}

// collectSaves meldet die Ergebnisse der Speicherungen seit dem letzten Frame (Autosave, Save, Close).
func (s *Session) collectSaves() {
	for _, err := range s.store.take() {
		if err != nil {
			s.cue(view.CueSaveFailed, -1, "Speichern fehlgeschlagen, das Spiel läuft weiter: "+err.Error())
			continue
		}
		s.cue(view.CueSaved, -1, "")
	}
}

// seats sind die Plätze der lokalen Spieler nach Slot (im Puffer seatBuf).
func (s *Session) seats() []view.Seat {
	s.seatBuf = s.seatBuf[:0]
	for _, st := range s.p.seats {
		s.seatBuf = append(s.seatBuf, view.Seat{Player: st.Slot, Device: s.devs[st.Slot], Monarch: st.Monarch, Stage: st.Stage})
	}
	return s.seatBuf
}

// peer ist das lokale Gerät im Raum (room.Peer). Der Raum ruft es unter seiner Sperre in der Goroutine der Schleife
// auf (Create, Tick, AddSlot, CloseRoom); es merkt sich nur Welten, Ereignisse und Plätze.
type peer struct {
	tick     uint64
	worlds   map[int]*sim.World // Stufe → Welt des letzten Zustands
	events   []view.Event       // seit dem letzten Frame
	seats    []room.Seat
	stageBuf []view.Stage // Puffer von Frame.Stages
	onState  func()       // Test-Naht: läuft in State
}

var _ room.Peer = (*peer)(nil)

func newPeer() *peer { return &peer{worlds: map[int]*sim.World{}} }

// begin leert die Stufen vor einem Takt: Stufen ohne Spieler bekommen keinen Zustand mehr.
func (p *peer) begin() { clear(p.worlds) }

func (p *peer) Joined(_, _, _ string, you []room.Seat) { p.seats = you }
func (p *peer) Level(int, int, level.Layout)           {}
func (p *peer) Seats(you []room.Seat, _ []string)      { p.seats = you }
func (p *peer) Replaced()                              {}
func (p *peer) Closed(bool)                            {}

// State merkt sich die Welt der Stufe und kopiert ihre Ereignisse; saved ist kein Ereignis der Simulation (Speichern
// meldet die Session als Cue).
func (p *peer) State(tick, stage int, state any) {
	if p.onState != nil {
		p.onState()
	}
	w, ok := state.(*sim.World)
	if !ok {
		return
	}
	p.tick, p.worlds[stage] = uint64(tick), w
	for _, e := range w.Events {
		if t, _ := e["type"].(string); t != "saved" {
			p.events = append(p.events, view.Event{Stage: stage, Tick: uint64(tick), Data: e})
		}
	}
}

// stages sind die Stufen mit Welt nach Index (im Puffer stageBuf).
func (p *peer) stages() []view.Stage {
	p.stageBuf = p.stageBuf[:0]
	for i, w := range p.worlds {
		p.stageBuf = append(p.stageBuf, view.Stage{Index: i, Depth: w.Biome.Depth, Level: w.Biome.ID, World: w})
	}
	slices.SortFunc(p.stageBuf, func(a, b view.Stage) int { return a.Index - b.Index })
	return p.stageBuf
}
