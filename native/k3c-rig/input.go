package main

// Eingabe des Vorführ-Modus: je Spieler ein Gamepad (Gamepad 1 → Spieler 1), die Tastatur steuert einen wählbaren
// Spieler. Taste B bleibt unbelegt (im Spiel reserviert für „zurück“ in Menüs).

import (
	"math"
	"sort"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/inpututil"
)

const (
	deadzone  = 0.5
	tempoStep = 0.25
	tempoMin  = 0.25
	tempoMax  = 2
)

// Action ist eine Eingabe für einen Monarchen.
type Action int

const (
	actNext Action = iota
	actPrev
	actColor
	actWeapon
	actArmor
	actFaster
	actSlower
	actLeft
	actRight
)

// sortedPads: Gamepad 1 → Spieler 1, Gamepad 2 → Spieler 2, nach ID sortiert.
func sortedPads() []ebiten.GamepadID {
	ids := ebiten.AppendGamepadIDs(nil)
	sort.Slice(ids, func(a, b int) bool { return ids[a] < ids[b] })
	return ids
}

var keyActions = map[ebiten.Key]Action{
	ebiten.KeyF: actColor, ebiten.KeyW: actWeapon, ebiten.KeyR: actArmor,
	ebiten.KeyArrowUp: actFaster, ebiten.KeyArrowDown: actSlower, ebiten.KeyArrowLeft: actLeft, ebiten.KeyArrowRight: actRight,
}

var padActions = map[ebiten.StandardGamepadButton]Action{
	ebiten.StandardGamepadButtonRightBottom: actNext, ebiten.StandardGamepadButtonFrontTopLeft: actPrev,
	ebiten.StandardGamepadButtonFrontTopRight: actColor, ebiten.StandardGamepadButtonRightLeft: actWeapon,
	ebiten.StandardGamepadButtonRightTop: actArmor, ebiten.StandardGamepadButtonLeftTop: actFaster,
	ebiten.StandardGamepadButtonLeftBottom: actSlower, ebiten.StandardGamepadButtonLeftLeft: actLeft,
	ebiten.StandardGamepadButtonLeftRight: actRight,
}

var digitKeys = []ebiten.Key{ebiten.Key1, ebiten.Key2, ebiten.Key3, ebiten.Key4, ebiten.Key5, ebiten.Key6, ebiten.Key7, ebiten.Key8, ebiten.Key9, ebiten.Key0}

// input wertet Tastatur und Gamepads aus; true = beenden.
func (d *Demo) input(g *Game) bool {
	if inpututil.IsKeyJustPressed(ebiten.KeyEscape) {
		return true
	}
	if inpututil.IsKeyJustPressed(ebiten.KeyZ) {
		d.zoom = (d.zoom + 1) % len(zooms)
	}
	if inpututil.IsKeyJustPressed(ebiten.KeyP) {
		d.keyboard = (d.keyboard + 1) % len(d.actors)
	}
	d.keys(d.actors[d.keyboard])
	for i, a := range d.actors {
		if i < len(d.pads) {
			d.pad(a, d.pads[i])
		}
	}
	return false
}

func (d *Demo) keys(a *Actor) {
	if inpututil.IsKeyJustPressed(ebiten.KeyTab) {
		if ebiten.IsKeyPressed(ebiten.KeyShift) {
			a.apply(actPrev)
		} else {
			a.apply(actNext)
		}
	}
	for key, act := range keyActions {
		if inpututil.IsKeyJustPressed(key) {
			a.apply(act)
		}
	}
	for i, key := range digitKeys {
		if inpututil.IsKeyJustPressed(key) && i < len(states) {
			a.setState(i)
		}
	}
}

func (d *Demo) pad(a *Actor, id ebiten.GamepadID) {
	if !ebiten.IsStandardGamepadLayoutAvailable(id) {
		return
	}
	for button, act := range padActions {
		if inpututil.IsStandardGamepadButtonJustPressed(id, button) {
			a.apply(act)
		}
	}
	if inpututil.IsStandardGamepadButtonJustPressed(id, ebiten.StandardGamepadButtonCenterRight) {
		d.zoom = (d.zoom + 1) % len(zooms)
	}
	if x := ebiten.StandardGamepadAxisValue(id, ebiten.StandardGamepadAxisLeftStickHorizontal); math.Abs(x) > deadzone {
		a.left = x < 0
	}
}

// setState wechselt den Zustand und beginnt die Animation von vorn.
func (a *Actor) setState(i int) {
	a.state, a.t = (i%len(states)+len(states))%len(states), 0
}

// apply führt eine Eingabe aus; ein Fehler beim Bildwechsel kann nicht auftreten (alle Varianten sind geladen).
func (a *Actor) apply(act Action) {
	switch act {
	case actNext:
		a.setState(a.state + 1)
	case actPrev:
		a.setState(a.state - 1)
	case actColor:
		a.variant = (a.variant + 1) % len(variants)
		_ = a.refresh()
	case actWeapon:
		a.weapon = (a.weapon + 1) % len(a.assets.weapons)
	case actArmor:
		a.armor = (a.armor + 1) % len(a.assets.armors)
	case actFaster:
		a.tempo = math.Min(a.tempo+tempoStep, tempoMax)
	case actSlower:
		a.tempo = math.Max(a.tempo-tempoStep, tempoMin)
	case actLeft:
		a.left = true
	case actRight:
		a.left = false
	}
}
