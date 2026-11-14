package view

import (
	"testing"

	"github.com/hajimehoshi/ebiten/v2"
)

func TestNextTimescaleAndDebug(t *testing.T) {
	want := map[int]int{1: 2, 2: 4, 4: 8, 8: 1, 3: 2}
	for cur, next := range want {
		if got := NextTimescale(cur); got != next {
			t.Errorf("NextTimescale(%d) = %d, erwartet %d", cur, got, next)
		}
	}
	if NextDebug(DebugOff) != DebugGeometry || NextDebug(DebugGeometry) != DebugValues || NextDebug(DebugValues) != DebugOff {
		t.Error("NextDebug schaltet nicht aus → Geometrie → Werte → aus")
	}
}

// Jeder Befehl hat genau eine Taste, nur F-Tasten (keine Spieltasten), F für Vollbild entfällt.
func TestDevKeysOnlyFunctionKeys(t *testing.T) {
	cmds := map[DevCmd]bool{}
	for _, k := range DevKeys {
		if k.Key < ebiten.KeyF1 || k.Key > ebiten.KeyF12 || cmds[k.Cmd] || k.Cmd == DevNone {
			t.Errorf("%+v", k)
		}
		cmds[k.Cmd] = true
	}
	if DevKeyOf(DevFullscreen) != ebiten.KeyF11 || DevKeyOf(DevNone) != ebiten.KeyMax {
		t.Error("DevKeyOf")
	}
}
