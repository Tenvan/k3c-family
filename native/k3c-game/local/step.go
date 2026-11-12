package local

import (
	"fmt"
	"slices"
	"time"

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
	f := view.Frame{Clock: float64(s.steps) / stepHz, Paused: s.paused, Timescale: 1, Cues: s.cues, Settings: s.settings,
		Outcome: s.ended}
	s.cues = nil
	if s.p != nil {
		f.Tick, f.Stages, f.Events, f.Seats = s.p.tick, s.p.stages(), s.p.events, s.seats()
		s.p.events = s.p.events[:0]
	}
	return f
}

// tick taktet den Raum einmal und bestimmt den Ausgang. Ein Panic im Takt schließt der Raum selbst ohne Speichern
// (room.SafeTick, B-803: eine begonnene Speicherung läuft zu Ende, danach keine mehr); der letzte Spielstand bleibt. Im
// Aufnahme-Modus wartet es auf die in diesem Takt begonnenen Speicherungen.
func (s *Session) tick() {
	s.p.begin()
	base := s.store.count()
	s.inTick, s.encodes = true, 0
	ok := s.r.SafeTick()
	s.inTick = false
	if !ok {
		s.crash(s.failure())
		return
	}
	if s.recording && s.encodes > 0 {
		s.awaitSaves(base + s.encodes)
	}
	s.judge()
}

// saveWait ist die längste Wartezeit des Aufnahme-Modus auf eine Speicherung; danach schreibt der Raum wieder im
// Hintergrund (Log), statt das Programm anzuhalten.
const saveWait = 10 * time.Second

// awaitSaves wartet, bis der Store n Ergebnisse gesehen hat (Aufnahme-Modus, Config.Recording).
func (s *Session) awaitSaves(n int) {
	for deadline := time.Now().Add(saveWait); s.store.count() < n; time.Sleep(time.Millisecond) {
		if time.Now().After(deadline) {
			s.log.Error("⏳ Aufnahme: Speichern nicht abgewartet, CueSaved folgt der Wanduhr", "erwartet", n, "gesehen", s.store.count())
			s.recording = false
			return
		}
	}
}

// judge setzt den Ausgang des Spiels: victory bzw. gameOver unter den Ereignissen der Stufen mit lokalen Spielern,
// sonst verloren, wenn eine Stufe trotz Takt nicht weiterläuft (sim.StepIsland rechnet nach Island.Over nichts mehr,
// das Ereignis kann in einer Stufe ohne lokalen Spieler liegen). Der erste Ausgang bleibt.
func (s *Session) judge() {
	stalled := s.p.stalled()
	if s.ended != view.OutcomeNone {
		return
	}
	for _, e := range s.p.events {
		switch e.Type() {
		case "gameOver":
			s.ended = view.OutcomeLost
		case "victory":
			s.ended = view.OutcomeWon
		}
		if s.ended != view.OutcomeNone {
			break
		}
	}
	if s.ended == view.OutcomeNone && stalled {
		s.ended = view.OutcomeLost
	}
	if s.ended != view.OutcomeNone {
		s.log.Info("🚦 Spielende", "save", s.name, "ausgang", s.ended)
	}
}

// failure ist der Grund, aus dem der Raum geschlossen ist: der Panic aus Manager.Status, sonst „Raum geschlossen“.
func (s *Session) failure() string {
	if _, fails := s.m.Status(); len(fails) > 0 {
		return fails[len(fails)-1].Error
	}
	return "Raum geschlossen"
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
	s.seatBuf = s.appendSeats(s.seatBuf[:0])
	return s.seatBuf
}

func (s *Session) appendSeats(dst []view.Seat) []view.Seat {
	for _, st := range s.p.seats {
		dst = append(dst, view.Seat{Player: st.Slot, Device: s.devs[st.Slot], Monarch: st.Monarch, Stage: st.Stage})
	}
	return dst
}

// Seats liefert die Plätze des laufenden Spiels nach Slot (eigener Puffer, nicht der von Frame.Seats); leer ohne Spiel.
func (s *Session) Seats() []view.Seat {
	s.seatOut = s.seatOut[:0]
	if s.p != nil {
		s.seatOut = s.appendSeats(s.seatOut)
	}
	return s.seatOut
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
	times    []stageTime  // Spielzeit je Stufe nach dem letzten Takt (stalled)
	timeBuf  []stageTime
}

// stageTime ist die Spielzeit einer Stufe nach einem Takt.
type stageTime struct {
	stage int
	w     *sim.World
	t     float64
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

// stalled meldet nach einem Takt, ob eine Stufe mit derselben Welt wie im vorigen Takt stehen geblieben ist (Spielzeit
// gleich), und merkt sich die Zeiten. sim.Step zählt die Zeit in jedem Schritt weiter; steht sie, ruht die Insel.
func (p *peer) stalled() bool {
	out := false
	p.timeBuf = p.timeBuf[:0]
	for i, w := range p.worlds {
		for _, old := range p.times {
			out = out || (old.stage == i && old.w == w && old.t == w.Time)
		}
		p.timeBuf = append(p.timeBuf, stageTime{stage: i, w: w, t: w.Time})
	}
	p.times, p.timeBuf = p.timeBuf, p.times
	return out
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
