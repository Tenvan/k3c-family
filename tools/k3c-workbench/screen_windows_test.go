//go:build windows

package main

import "testing"

func TestOnScreenFehlenderBildschirm(t *testing.T) {
	if onScreen(windowState{X: -100000, Y: 100000, Width: defaultWidth, Height: defaultHeight}) {
		t.Error("Position weit außerhalb gilt als sichtbar")
	}
}
