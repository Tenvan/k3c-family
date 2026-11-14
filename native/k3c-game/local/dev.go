package local

// Entwickler-Rolle der Session (view.DevSession, B-888): Dev-Aktionen über den Raum (Cheat-Menü, Zeitraffer), Halt mit
// Einzelschritt (F7) und Neu-Laden ohne Speichern (F6).

import (
	"fmt"

	"k3c/engine/room"
	"k3c/native/k3c-game/view"
)

var _ view.DevSession = (*Session)(nil)

// Dev führt die Dev-Aktion a für den Spieler player aus (Room.DevLocal; der Raum loggt sie mit 🐛).
func (s *Session) Dev(player int, a view.DevAction) error {
	if err := s.seated(player); err != nil {
		return err
	}
	slot := player
	ra := room.DevAction{Action: a.Action, Slot: &slot, Amount: a.Amount, Resource: a.Resource, Factor: a.Factor,
		Phase: a.Phase, Grade: a.Grade, On: a.On, Level: a.Level, Boss: a.Boss, Event: a.Event, Kind: a.Kind, Unit: a.Unit}
	if err := s.r.DevLocal(deviceID, s.p, ra); err != nil {
		return userErr(fmt.Sprintf("Cheat „%s“ geht gerade nicht", a.Action), err)
	}
	return nil
}

// Halt hält die Simulation an bzw. lässt sie weiterlaufen (unabhängig von der Pause des Menüs).
func (s *Session) Halt(on bool) {
	if s.halt == on {
		return
	}
	s.halt, s.single = on, false
	s.log.Info("🚦 Halt", "an", on)
}

// Halted meldet, ob Halt gilt.
func (s *Session) Halted() bool { return s.halt }

// StepOnce lässt im Halt den nächsten Step genau einen Takt rechnen.
func (s *Session) StepOnce() {
	if s.halt {
		s.single = true
	}
}

// Reload verwirft das laufende Spiel ohne Speichern und lädt seinen zuletzt gespeicherten Stand (F5 oder Autosave)
// mit denselben Geräten.
func (s *Session) Reload() error {
	if s.r == nil {
		return errNoGame
	}
	name, devs := s.name, s.devices()
	s.reset()
	if err := s.Load(name, devs); err != nil {
		return err
	}
	s.log.Info("📂 Spielstand neu geladen", "save", name)
	return nil
}

// running meldet, ob Step in diesem Aufruf einen Takt rechnet (Spiel läuft, keine Pause, kein Halt außer Einzelschritt).
func (s *Session) running() bool {
	if s.r == nil || s.paused {
		return false
	}
	if !s.halt {
		return true
	}
	one := s.single
	s.single = false
	return one
}

// timescale ist der Zeitraffer des Raums (1 ohne Spiel).
func (s *Session) timescale() int {
	if s.r == nil {
		return 1
	}
	return s.r.Timescale()
}
