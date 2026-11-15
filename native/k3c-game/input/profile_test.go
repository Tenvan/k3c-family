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
