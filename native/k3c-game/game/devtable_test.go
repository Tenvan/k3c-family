package game

import (
	"errors"
	"slices"
	"testing"

	"k3c/native/k3c-game/input"
	"k3c/native/k3c-game/view"
)

// devSim ist fakeSim mit der Entwickler-Rolle (view.DevSession).
type devSim struct {
	*fakeSim
	halt     bool
	steps    int
	reloads  int
	actions  []view.DevAction
	players  []int
	failNext error
}

func (s *devSim) Dev(p int, a view.DevAction) error {
	s.players, s.actions = append(s.players, p), append(s.actions, a)
	return nil
}
func (s *devSim) Halt(on bool)  { s.halt = on }
func (s *devSim) Halted() bool  { return s.halt }
func (s *devSim) StepOnce()     { s.steps++ }
func (s *devSim) Reload() error { s.reloads++; return s.failNext }

// AC-01 (B-888): Jede Taste der Tabelle wird ausgewertet: game liest sie selbst, oder input liefert sie als
// Geräte-Aktion auf der linken Tastatur-Hälfte (F9, F11); jede Taste kommt genau einmal vor.
func TestDevKeyTableEvaluated(t *testing.T) {
	g := devGame(newFakeSim())
	var seen []string
	set := g.frame.Settings
	for _, k := range view.DevKeys {
		if slices.Contains(seen, k.Label) {
			t.Errorf("%s doppelt", k.Label)
		}
		seen = append(seen, k.Label)
		if k.Action >= 0 {
			if !slices.Contains(input.ActionKeys(0, k.Action), k.Key) {
				t.Errorf("%s: input liefert die Aktion %d nicht", k.Label, k.Action)
			}
			if g.devCommand(k.Cmd, &set) {
				t.Errorf("%s: game wertet eine Geräte-Aktion selbst aus", k.Label)
			}
			continue
		}
		if !g.devCommand(k.Cmd, &set) {
			t.Errorf("%s: kein Befehl in game", k.Label)
		}
	}
}

// AC-02: F1 und F10 schalten den Frame-Zustand, F12 merkt das nächste Bild vor; im Aufnahme-Modus wirkt keine Taste.
func TestDevKeyViewToggles(t *testing.T) {
	g := devGame(newFakeSim())
	g.pressed = keysDown(view.DevKeyOf(view.DevHelp), view.DevKeyOf(view.DevDebug), view.DevKeyOf(view.DevShot))
	g.devKeys()
	if !g.dev.Help || g.dev.Debug != view.DebugGeometry || !g.shotNext {
		t.Fatalf("nach F1, F10, F12: %+v, Bild %v", g.dev, g.shotNext)
	}
	g.pressed = keysDown(view.DevKeyOf(view.DevDebug))
	g.devKeys()
	g.devKeys()
	if g.dev.Debug != view.DebugOff {
		t.Errorf("F10 dreimal: Seite %d", g.dev.Debug)
	}
	g.args.Shots = "x"
	g.pressed = keysDown(view.DevKeyOf(view.DevHelp))
	g.devKeys()
	if !g.dev.Help {
		t.Error("F1 im Aufnahme-Modus gewirkt")
	}
}

// AC-02: F7 hält an, erneut F7 rechnet einen Takt; F8 beendet den Halt, danach schaltet es den Zeitraffer über den
// ersten Spieler weiter; F6 lädt neu und meldet einen Fehler als Cue.
func TestDevKeySession(t *testing.T) {
	s := &devSim{fakeSim: newFakeSim()}
	g := devGame(s.fakeSim)
	g.sim = s
	g.frame.Timescale = 2
	press := func(c view.DevCmd) { g.pressed = keysDown(view.DevKeyOf(c)); g.devKeys() }
	press(view.DevPause)
	press(view.DevPause)
	if !s.halt || s.steps != 1 {
		t.Fatalf("F7, F7: halt %v, Schritte %d", s.halt, s.steps)
	}
	press(view.DevSpeed)
	if s.halt || len(s.actions) != 0 {
		t.Fatalf("F8 im Halt: halt %v, Aktionen %v", s.halt, s.actions)
	}
	press(view.DevSpeed)
	if len(s.actions) != 1 || s.actions[0].Action != "timescale" || s.actions[0].Factor != 4 || s.players[0] != 0 {
		t.Fatalf("F8: %+v an %v", s.actions, s.players)
	}
	s.failNext = errors.New("kaputt")
	press(view.DevLoad)
	if s.reloads != 1 || len(g.cues) != 1 || g.cues[0].Kind != view.CueDenied {
		t.Errorf("F6: %d Ladevorgänge, Cues %+v", s.reloads, g.cues)
	}
}
