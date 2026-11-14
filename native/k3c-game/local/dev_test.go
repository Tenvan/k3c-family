package local

import (
	"testing"

	"k3c/native/k3c-game/view"
)

// devGame startet ein Spiel mit zwei Spielern und rechnet einen Takt.
func devGame(t *testing.T) (*Session, view.Frame) {
	t.Helper()
	s := testSession(t, t.TempDir())
	if err := s.New(view.NewGame{Name: "dev", Devices: twoDevices}); err != nil {
		t.Fatal(err)
	}
	return s, s.Step()
}

// AC-02 (B-888): Halt hält den Takt an, StepOnce rechnet genau einen; ohne Halt wirkt StepOnce nicht.
func TestHaltStepOnce(t *testing.T) {
	s, f := devGame(t)
	s.StepOnce()
	if next := s.Step(); next.Tick != f.Tick+1 {
		t.Fatalf("StepOnce ohne Halt: Takt %d nach %d", next.Tick, f.Tick)
	}
	s.Halt(true)
	a := s.Step()
	b := s.Step()
	if !s.Halted() || !b.Paused || b.Tick != a.Tick {
		t.Fatalf("Halt: Takt %d → %d, Paused %v", a.Tick, b.Tick, b.Paused)
	}
	s.StepOnce()
	c := s.Step()
	d := s.Step()
	if c.Tick != b.Tick+1 || d.Tick != c.Tick {
		t.Errorf("Einzelschritt: %d → %d → %d", b.Tick, c.Tick, d.Tick)
	}
	s.Halt(false)
	if e := s.Step(); e.Tick != d.Tick+1 || e.Paused {
		t.Errorf("weiter: Takt %d, Paused %v", e.Tick, e.Paused)
	}
}

// AC-03 (B-888): Gold fällt beim Spieler, Material kommt in den Vorrat, Zeitraffer steht im Frame, nächste Tageszeit
// und Nacht wechseln die Phase; ohne Spiel und für fehlende Spieler lehnt Dev ab.
func TestDevActions(t *testing.T) {
	s, f := devGame(t)
	w := f.Stages[0].World
	coins, wood := len(w.Coins), w.Stock.Wood
	for _, a := range []view.DevAction{{Action: "gold", Amount: 5}, {Action: "material", Resource: "wood", Amount: 3}} {
		if err := s.Dev(1, a); err != nil {
			t.Fatalf("%s: %v", a.Action, err)
		}
	}
	if len(w.Coins) != coins+5 || w.Stock.Wood <= wood {
		t.Errorf("Münzen %d → %d, Holz %d → %d", coins, len(w.Coins), wood, w.Stock.Wood)
	}
	if err := s.Dev(0, view.DevAction{Action: "timescale", Factor: 4}); err != nil {
		t.Fatal(err)
	}
	if got := s.Step().Timescale; got != 4 {
		t.Errorf("Zeitraffer im Frame %d", got)
	}
	phase := w.Cycle.Phase
	if err := s.Dev(0, view.DevAction{Action: "nextPhase"}); err != nil {
		t.Fatal(err)
	}
	if s.Step(); w.Cycle.Phase == phase {
		t.Errorf("nächste Tageszeit: Phase bleibt %s", phase)
	}
	if err := s.Dev(0, view.DevAction{Action: "night"}); err != nil {
		t.Fatal(err)
	}
	if s.Step(); w.Cycle.Phase != "night" {
		t.Errorf("Nacht: Phase %s", w.Cycle.Phase)
	}
	if s.Dev(3, view.DevAction{Action: "gold", Amount: 1}) == nil || s.Dev(0, view.DevAction{Action: "gibt-es-nicht"}) == nil {
		t.Error("ungültige Dev-Aktion angenommen")
	}
}

// Reload verwirft den Fortschritt seit dem letzten Speichern und behält die Geräte.
func TestReload(t *testing.T) {
	s, _ := devGame(t)
	if err := s.Save(); err != nil {
		t.Fatal(err)
	}
	saved := s.Step().Stages[0].World.Time
	for range 30 {
		s.Step()
	}
	if err := s.Reload(); err != nil {
		t.Fatal(err)
	}
	f := s.Step()
	if got := f.Stages[0].World.Time; got > saved+0.1 || len(f.Seats) != 2 {
		t.Errorf("nach Reload: Zeit %.2f (gespeichert %.2f), Plätze %d", got, saved, len(f.Seats))
	}
	s.reset()
	if s.Reload() == nil {
		t.Error("Reload ohne Spiel")
	}
}
