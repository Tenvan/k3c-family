package main

// Eingabe des Vorführ-Modus: je Spieler ein Gamepad (Gamepad 1 → Spieler 1), die Tastatur steuert einen wählbaren
// Spieler. Taste B bleibt unbelegt (im Spiel reserviert für „zurück“ in Menüs).

import (
	"math"
	"sort"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/inpututil"

	"k3c/native/k3c-rig/paint"
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
	d.keys(g, d.actors[d.keyboard])
	for i, a := range d.actors {
		if i < len(d.pads) {
			d.pad(g, a, d.pads[i])
		}
	}
	return false
}

func (d *Demo) keys(g *Game, a *paint.Actor) {
	if inpututil.IsKeyJustPressed(ebiten.KeyTab) {
		if ebiten.IsKeyPressed(ebiten.KeyShift) {
			apply(g, a, actPrev)
		} else {
			apply(g, a, actNext)
		}
	}
	for key, act := range keyActions {
		if inpututil.IsKeyJustPressed(key) {
			apply(g, a, act)
		}
	}
	for i, key := range digitKeys {
		if inpututil.IsKeyJustPressed(key) && i < len(states) {
			setState(a, i)
		}
	}
}

func (d *Demo) pad(g *Game, a *paint.Actor, id ebiten.GamepadID) {
	if !ebiten.IsStandardGamepadLayoutAvailable(id) {
		return
	}
	for button, act := range padActions {
		if inpututil.IsStandardGamepadButtonJustPressed(id, button) {
			apply(g, a, act)
		}
	}
	if inpututil.IsStandardGamepadButtonJustPressed(id, ebiten.StandardGamepadButtonCenterRight) {
		d.zoom = (d.zoom + 1) % len(zooms)
	}
	if x := ebiten.StandardGamepadAxisValue(id, ebiten.StandardGamepadAxisLeftStickHorizontal); math.Abs(x) > deadzone {
		a.Left = x < 0
	}
}

// setState wechselt den Zustand und beginnt die Animation von vorn.
func setState(a *paint.Actor, i int) {
	a.State, a.T = states[(i%len(states)+len(states))%len(states)], 0
}

// next liefert den Eintrag nach cur in list (zyklisch).
func next(list []string, cur string) string { return list[(indexOf(list, cur)+1)%len(list)] }

// apply führt eine Eingabe aus; ein Fehler beim Bildwechsel kann nicht auftreten (alle Varianten sind geladen).
func apply(g *Game, a *paint.Actor, act Action) {
	switch act {
	case actNext:
		setState(a, stateIndex(a.State.Name)+1)
	case actPrev:
		setState(a, stateIndex(a.State.Name)-1)
	case actColor:
		a.Variant = next(variants, a.Variant)
		_ = a.Refresh()
	case actWeapon:
		a.Weapon = next(g.assets.Weapons, a.Weapon)
	case actArmor:
		a.Armor = next(g.assets.Armors, a.Armor)
	case actFaster:
		a.Tempo = math.Min(a.Tempo+tempoStep, tempoMax)
	case actSlower:
		a.Tempo = math.Max(a.Tempo-tempoStep, tempoMin)
	case actLeft:
		a.Left = true
	case actRight:
		a.Left = false
	}
}
