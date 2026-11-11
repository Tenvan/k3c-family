// Package input liest Tastatur und Gamepads und liefert view.Input (Fach-Sprint der Eingabe). Bis dahin Platzhalter:
// nur die Tastatur als ein Gerät (A/D oder Pfeile laufen, Shift Sprint, S oder Pfeil runter zahlen, Leertaste Schlag;
// Menü: Pfeile, Enter, Backspace zurück, Esc Pause-Menü).
package input

import (
	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/inpututil"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// actionKeys ordnet Tasten den Menü-Aktionen zu (Reihenfolge = Reihenfolge der Flanken).
var actionKeys = []struct {
	key    ebiten.Key
	action view.Action
}{
	{ebiten.KeyArrowUp, view.ActionUp},
	{ebiten.KeyArrowDown, view.ActionDown},
	{ebiten.KeyArrowLeft, view.ActionLeft},
	{ebiten.KeyArrowRight, view.ActionRight},
	{ebiten.KeyEnter, view.ActionConfirm},
	{ebiten.KeyBackspace, view.ActionBack},
	{ebiten.KeyEscape, view.ActionMenu},
}

type keyboard struct{}

// New liefert die Eingabe-Quelle; lib ist einheitlich dabei, Eingabe zeichnet nichts und braucht sie nicht.
func New(_ *art.Library) view.Source { return keyboard{} }

func (keyboard) Poll() view.Input {
	di := view.DeviceInput{
		Device:  view.DeviceID{Kind: view.DeviceKeyboard},
		Command: CommandOf(ebiten.IsKeyPressed),
	}
	for _, ak := range actionKeys {
		if inpututil.IsKeyJustPressed(ak.key) {
			di.Actions = append(di.Actions, ak.action)
		}
	}
	return view.Input{Devices: []view.DeviceInput{di}}
}

// CommandOf liefert den Befehl aus den gehaltenen Tasten (pressed meldet, ob eine Taste gehalten ist).
func CommandOf(pressed func(ebiten.Key) bool) view.Command {
	anyOf := func(keys ...ebiten.Key) bool {
		for _, k := range keys {
			if pressed(k) {
				return true
			}
		}
		return false
	}
	var c view.Command
	if anyOf(ebiten.KeyA, ebiten.KeyArrowLeft) {
		c.Move--
	}
	if anyOf(ebiten.KeyD, ebiten.KeyArrowRight) {
		c.Move++
	}
	c.Sprint = anyOf(ebiten.KeyShiftLeft, ebiten.KeyShiftRight)
	c.Pay = anyOf(ebiten.KeyS, ebiten.KeyArrowDown)
	c.Attack = anyOf(ebiten.KeySpace)
	return c
}
