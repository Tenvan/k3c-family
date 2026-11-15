package input

import (
	"slices"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// commands sind die gehaltenen Befehle (CommandKeys).
var commands = []string{"left", "right", "sprint", "pay", "attack", "skill1", "skill2", "skill3", "skill4"}

// B-893/AC-01: Jedes Profil liefert alle Spiel-Aktionen: jeder Befehl und jede Menü-Aktion hat in wasd und pfeile
// eine Taste, am Gamepad eine Taste oder (Home) die Kombi View + Menu.
func TestEveryProfileHasAllActions(t *testing.T) {
	for _, p := range []string{view.ProfileWASD, view.ProfileArrows} {
		l := Layout(p)
		for _, c := range commands {
			if len(CommandKeys(l, c)) == 0 {
				t.Errorf("%s: Befehl %s ohne Taste", p, c)
			}
		}
		for a := range actionCount {
			if len(ActionKeys(l, view.Action(a))) == 0 {
				t.Errorf("%s: Aktion %d ohne Taste", p, a)
			}
		}
	}
	for a, b := range padActions {
		if b < 0 && view.Action(a) != view.ActionHome {
			t.Errorf("gamepad: Aktion %d ohne Taste", a)
		}
	}
	if Layout(view.ProfileGamepad) != layoutWASD || Layout("") != layoutWASD || Layout(view.ProfileArrows) != layoutArrows {
		t.Error("Layout: Rückfall auf WASD bzw. Pfeile falsch")
	}
}

// B-893/AC-03: SetKeyboard wirkt ab dem nächsten Poll; es gibt immer genau ein Tastatur-Gerät, und eine beim Wechsel
// gehaltene Taste löst erst nach dem Loslassen aus.
func TestSetKeyboardSwitchesLayout(t *testing.T) {
	s := &source{}
	s.poll(keyState(), nil)
	in := s.poll(keyState(ebiten.KeyArrowLeft), nil)
	if di, _ := in.Of(keys1); di.Command.Move != 0 || len(in.Devices) != 1 {
		t.Fatalf("wasd: Pfeil wirkt oder Geräte %v", deviceIDs(in))
	}
	s.SetKeyboard(view.ProfileArrows)
	in = s.poll(keyState(ebiten.KeyArrowLeft), nil)
	di, _ := in.Of(keys1)
	if di.Command.Move != -1 || len(di.Actions) != 0 {
		t.Errorf("pfeile nach Wechsel: %+v", di)
	}
	s.poll(keyState(), nil)
	in = s.poll(keyState(ebiten.KeyEnter), nil)
	if di, _ := in.Of(keys1); !slices.Equal(di.Actions, []view.Action{view.ActionConfirm}) {
		t.Errorf("Enter im Profil pfeile: %v", di.Actions)
	}
	if in := s.poll(keyState(ebiten.KeyA), nil); in.Devices[0].Command.Move != 0 {
		t.Error("A wirkt im Profil pfeile")
	}
}

// B-917/AC-01: In jedem Tastatur-Profil löst die Leertaste aus (zahlen), unabhängig von den Richtungstasten.
func TestSpaceTriggersInEveryProfile(t *testing.T) {
	for _, p := range []string{view.ProfileWASD, view.ProfileArrows} {
		if !slices.Contains(CommandKeys(Layout(p), "pay"), ebiten.KeySpace) {
			t.Errorf("%s: Leertaste löst nicht aus", p)
		}
		s := &source{}
		s.SetKeyboard(p)
		s.poll(keyState(), nil)
		in := s.poll(keyState(ebiten.KeySpace), nil)
		if di, _ := in.Of(keys1); !di.Command.Pay {
			t.Errorf("%s: Leertaste setzt Pay nicht", p)
		}
	}
}

// B-945/AC-01: Im Menü gelten in jedem Profil Pfeile, Enter, Leertaste und Esc; außerhalb eines Menüs gilt nur das Profil.
func TestMenuKeysInEveryProfile(t *testing.T) {
	cases := []struct {
		key ebiten.Key
		act view.Action
	}{
		{ebiten.KeyArrowUp, view.ActionUp}, {ebiten.KeyArrowDown, view.ActionDown},
		{ebiten.KeyArrowLeft, view.ActionLeft}, {ebiten.KeyArrowRight, view.ActionRight},
		{ebiten.KeyEnter, view.ActionConfirm}, {ebiten.KeySpace, view.ActionConfirm}, {ebiten.KeyEscape, view.ActionBack},
	}
	for _, p := range []string{view.ProfileWASD, view.ProfileArrows} {
		for _, c := range cases {
			s := &source{}
			s.SetKeyboard(p)
			s.SetMenu(true)
			s.poll(keyState(), nil)
			in := s.poll(keyState(c.key), nil)
			if di, _ := in.Of(keys1); !slices.Contains(di.Actions, c.act) {
				t.Errorf("%s: Taste %v löst %d im Menü nicht aus: %v", p, c.key, c.act, di.Actions)
			}
		}
	}
	s := &source{}
	s.poll(keyState(), nil)
	if in := s.poll(keyState(ebiten.KeyArrowUp), nil); len(in.Devices[0].Actions) != 0 {
		t.Error("Pfeil hoch wirkt im Profil wasd außerhalb eines Menüs")
	}
}

// Review FB4: Beim Wechsel Menü ↔ Spiel mit gehaltener Taste (Profil pfeile) löst weder das Menü aus noch zahlt das Spiel,
// bis die Taste losgelassen ist.
func TestMenuSwitchIgnoresHeldKey(t *testing.T) {
	s := &source{}
	s.SetKeyboard(view.ProfileArrows)
	s.poll(keyState(), nil)
	s.SetMenu(true) // Menü öffnet, Enter noch gehalten
	in := s.poll(keyState(ebiten.KeyEnter), nil)
	if di, _ := in.Of(keys1); len(di.Actions) != 0 {
		t.Errorf("gehaltenes Enter bestätigt im ersten Menü-Frame: %v", di.Actions)
	}
	s.poll(keyState(), nil)
	if in = s.poll(keyState(ebiten.KeyEnter), nil); !slices.Contains(in.Devices[0].Actions, view.ActionConfirm) {
		t.Error("frisches Enter bestätigt im Menü nicht")
	}
	s.SetMenu(false) // Menü schließt, Enter noch gehalten
	for range 2 {
		if in = s.poll(keyState(ebiten.KeyEnter), nil); in.Devices[0].Command.Pay {
			t.Fatal("gehaltenes Enter zahlt nach dem Schließen")
		}
	}
	s.poll(keyState(), nil)
	if in = s.poll(keyState(ebiten.KeyEnter), nil); !in.Devices[0].Command.Pay {
		t.Error("frisches Enter zahlt nicht")
	}
}
