//go:build windows

package windowstate

import "testing"

func TestOnScreenFehlenderBildschirm(t *testing.T) {
	if OnScreen(State{X: -100000, Y: 100000, Width: DefaultWidth, Height: DefaultHeight}) {
		t.Error("Position weit außerhalb gilt als sichtbar")
	}
}
