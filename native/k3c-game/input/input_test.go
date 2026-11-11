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
	if c := CommandOf(held(ebiten.KeyA, ebiten.KeyShiftLeft)); c.Move != -1 || !c.Sprint {
		t.Errorf("links mit Sprint: %+v", c)
	}
	if c := CommandOf(held(ebiten.KeyA, ebiten.KeyD)); c.Move != 0 {
		t.Errorf("beide Richtungen heben sich auf: %+v", c)
	}
	if c := CommandOf(held(ebiten.KeyArrowRight, ebiten.KeyArrowDown, ebiten.KeySpace)); c.Move != 1 || !c.Pay || !c.Attack {
		t.Errorf("rechts, zahlen, Schlag: %+v", c)
	}
}
