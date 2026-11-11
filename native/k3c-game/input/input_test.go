package input

import (
	"testing"

	"github.com/hajimehoshi/ebiten/v2"
)

func held(keys ...ebiten.Key) func(ebiten.Key) bool {
	return func(k ebiten.Key) bool {
		for _, h := range keys {
			if h == k {
				return true
			}
		}
		return false
	}
}

func TestCommandOf(t *testing.T) {
	if c := CommandOf(0, held(ebiten.KeyA, ebiten.KeyShiftLeft)); c.Move != -1 || !c.Sprint {
		t.Errorf("links mit Sprint: %+v", c)
	}
	if c := CommandOf(0, held(ebiten.KeyA, ebiten.KeyD)); c.Move != 0 {
		t.Errorf("beide Richtungen heben sich auf: %+v", c)
	}
	if c := CommandOf(1, held(ebiten.KeyArrowRight, ebiten.KeyArrowDown, ebiten.KeyEnter)); c.Move != 1 || !c.Pay || !c.Attack {
		t.Errorf("rechts, zahlen, Schlag: %+v", c)
	}
	// Die Hälften sind getrennt: Pfeile steuern nicht die linke, A/D nicht die rechte.
	if c := CommandOf(0, held(ebiten.KeyArrowRight)); c.Move != 0 {
		t.Errorf("Pfeil an der linken Hälfte: %+v", c)
	}
	if c := CommandOf(1, held(ebiten.KeyD, ebiten.KeySpace)); c.Move != 0 || c.Attack {
		t.Errorf("A/D an der rechten Hälfte: %+v", c)
	}
}

func padHeld(bs ...ebiten.StandardGamepadButton) func(ebiten.StandardGamepadButton) bool {
	return func(b ebiten.StandardGamepadButton) bool {
		for _, h := range bs {
			if h == b {
				return true
			}
		}
		return false
	}
}

func TestPadCommand(t *testing.T) {
	if c := PadCommand(0.2, padHeld()); c.Move != 0 {
		t.Errorf("Totzone: %+v", c)
	}
	if c := PadCommand(1, padHeld(ebiten.StandardGamepadButtonFrontBottomRight)); c.Move != 1 || !c.Sprint {
		t.Errorf("Stick rechts mit RT: %+v", c)
	}
	if c := PadCommand(-0.625, padHeld()); c.Move != -0.5 {
		t.Errorf("Stick halb links: %+v", c)
	}
	if c := PadCommand(0.9, padHeld(ebiten.StandardGamepadButtonLeftLeft, ebiten.StandardGamepadButtonRightBottom)); c.Move != -1 || !c.Pay {
		t.Errorf("D-Pad geht vor Stick, A zahlt: %+v", c)
	}
	// B ist im Spiel unbelegt.
	if c := PadCommand(0, padHeld(ebiten.StandardGamepadButtonRightRight)); c != (PadCommand(0, padHeld())) {
		t.Errorf("B wirkt im Spiel: %+v", c)
	}
}
