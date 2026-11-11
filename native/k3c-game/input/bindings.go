// Package input liest Tastatur und Gamepads und liefert view.Input (Sprint NG5, B-798). Die Belegung folgt dem
// Browser-Client (src/input/slotBindings.ts, keyboardLayouts.ts; Tastatur 2 wie das frühere KEYBOARD_2):
//
//	Befehl (gehalten)  keys1 (links)      keys2 (rechts)         Gamepad (Xbox)
//	laufen             A / D              ← / →                  linker Stick oder Steuerkreuz ←/→
//	Galopp (Sprint)    Shift links        Strg rechts, Shift re. RT
//	zahlen             Leertaste          Enter, Num-Enter       A
//	Schlag             E                  Num 0                  X
//	Skill 1 … 4        Q, R, T, Z         Num 1 … Num 4          LB, RB, LT, Steuerkreuz ↑ (der niedrigste gewinnt)
//
//	Aktion (Flanke)    keys1              keys2                  Gamepad
//	↑ ↓ ← →            W S A D            Pfeile                 Steuerkreuz oder Stick (Wiederholung beim Halten)
//	Bestätigen         Leertaste          Enter, Num-Enter       A
//	Zurück             Esc                Rücktaste              B (im Spiel unbelegt, game wertet sie dort nicht aus)
//	Menü               Esc                Rücktaste              Menu kurz (beim Loslassen, < 600 ms, ohne View)
//	View               Tab                –                      View (beim Loslassen, ohne Menu)
//	Seite zurück/vor   Q / E              Bild ↑ / Bild ↓        LB / RB
//	Home               Pos1               –                      View + Menu gemeinsam (statt Menu und View)
//	Skill-Menü         K                  Num 5                  Steuerkreuz ↓ (zusätzlich zu ↓, nicht der Stick)
//	Vollbild           F                  –                      rechter Stick gedrückt
//
// input unterscheidet Spiel und Menü nicht: Steuerkreuz ↓ liefert ActionDown und ActionSkillMenu zugleich; das
// Skill-Menü (Overlay) wertet ActionSkillMenu nur im Spiel aus, Menüs nehmen ActionDown.
//
// Pads ohne Standard-Belegung werden mit einer Meldung im Log ignoriert. Die Auswertung ist rein (Rohzustand →
// view.DeviceInput, tracker.go), nur source.go fragt Ebitengine.
package input

import (
	"slices"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// actionCount ist die Zahl der Menü-Aktionen in view (Index = view.Action).
const actionCount = int(view.ActionFullscreen) + 1

// keyLayout ist die Belegung einer Tastatur-Hälfte; je Eintrag gelten alle genannten Tasten.
type keyLayout struct {
	left, right, sprint, pay, attack []ebiten.Key
	skills                           [4][]ebiten.Key
	actions                          [actionCount][]ebiten.Key
}

func keys(k ...ebiten.Key) []ebiten.Key { return k }

// halves sind die beiden Tastatur-Hälften (DeviceID.Index 0 und 1). Z liegt auf QWERTZ auf der Taste KeyY, darum
// gelten für Skill 4 beide.
var halves = [2]keyLayout{
	{
		left: keys(ebiten.KeyA), right: keys(ebiten.KeyD), sprint: keys(ebiten.KeyShiftLeft),
		pay: keys(ebiten.KeySpace), attack: keys(ebiten.KeyE),
		skills: [4][]ebiten.Key{keys(ebiten.KeyQ), keys(ebiten.KeyR), keys(ebiten.KeyT), keys(ebiten.KeyZ, ebiten.KeyY)},
		actions: [actionCount][]ebiten.Key{
			view.ActionUp: keys(ebiten.KeyW), view.ActionDown: keys(ebiten.KeyS),
			view.ActionLeft: keys(ebiten.KeyA), view.ActionRight: keys(ebiten.KeyD),
			view.ActionConfirm: keys(ebiten.KeySpace), view.ActionBack: keys(ebiten.KeyEscape),
			view.ActionMenu: keys(ebiten.KeyEscape), view.ActionView: keys(ebiten.KeyTab),
			view.ActionPrevPage: keys(ebiten.KeyQ), view.ActionNextPage: keys(ebiten.KeyE),
			view.ActionHome: keys(ebiten.KeyHome), view.ActionSkillMenu: keys(ebiten.KeyK),
			view.ActionFullscreen: keys(ebiten.KeyF),
		},
	},
	{
		left: keys(ebiten.KeyArrowLeft), right: keys(ebiten.KeyArrowRight),
		sprint: keys(ebiten.KeyControlRight, ebiten.KeyShiftRight),
		pay:    keys(ebiten.KeyEnter, ebiten.KeyNumpadEnter), attack: keys(ebiten.KeyNumpad0),
		skills: [4][]ebiten.Key{keys(ebiten.KeyNumpad1), keys(ebiten.KeyNumpad2), keys(ebiten.KeyNumpad3), keys(ebiten.KeyNumpad4)},
		actions: [actionCount][]ebiten.Key{
			view.ActionUp: keys(ebiten.KeyArrowUp), view.ActionDown: keys(ebiten.KeyArrowDown),
			view.ActionLeft: keys(ebiten.KeyArrowLeft), view.ActionRight: keys(ebiten.KeyArrowRight),
			view.ActionConfirm: keys(ebiten.KeyEnter, ebiten.KeyNumpadEnter), view.ActionBack: keys(ebiten.KeyBackspace),
			view.ActionMenu: keys(ebiten.KeyBackspace), view.ActionPrevPage: keys(ebiten.KeyPageUp),
			view.ActionNextPage: keys(ebiten.KeyPageDown), view.ActionSkillMenu: keys(ebiten.KeyNumpad5),
		},
	},
}

// Tasten des Standard-Layouts mit Xbox-Namen.
const (
	padA     = ebiten.StandardGamepadButtonRightBottom
	padB     = ebiten.StandardGamepadButtonRightRight
	padX     = ebiten.StandardGamepadButtonRightLeft
	padLB    = ebiten.StandardGamepadButtonFrontTopLeft
	padRB    = ebiten.StandardGamepadButtonFrontTopRight
	padLT    = ebiten.StandardGamepadButtonFrontBottomLeft
	padRT    = ebiten.StandardGamepadButtonFrontBottomRight
	padView  = ebiten.StandardGamepadButtonCenterLeft
	padMenu  = ebiten.StandardGamepadButtonCenterRight
	padUp    = ebiten.StandardGamepadButtonLeftTop
	padDown  = ebiten.StandardGamepadButtonLeftBottom
	padLeft  = ebiten.StandardGamepadButtonLeftLeft
	padRight = ebiten.StandardGamepadButtonLeftRight
	padRS    = ebiten.StandardGamepadButtonRightStick
)

// padSkills sind die Skill-Slots 1 … 4 des Gamepads.
var padSkills = [4]ebiten.StandardGamepadButton{padLB, padRB, padLT, padUp}

// padActions ist die Menü-Belegung des Gamepads; -1 = keine Taste (Home ist die Kombi View + Menu).
var padActions = [actionCount]ebiten.StandardGamepadButton{
	view.ActionUp: padUp, view.ActionDown: padDown, view.ActionLeft: padLeft, view.ActionRight: padRight,
	view.ActionConfirm: padA, view.ActionBack: padB, view.ActionMenu: padMenu, view.ActionView: padView,
	view.ActionPrevPage: padLB, view.ActionNextPage: padRB, view.ActionHome: -1,
	view.ActionSkillMenu: padDown, view.ActionFullscreen: padRS,
}

// usedKeys sind alle Tasten beider Hälften (Ebitengine wird nur nach ihnen gefragt).
var usedKeys = collectKeys()

func collectKeys() []ebiten.Key {
	var out []ebiten.Key
	add := func(ks []ebiten.Key) {
		for _, k := range ks {
			if !slices.Contains(out, k) {
				out = append(out, k)
			}
		}
	}
	for i := range halves {
		l := &halves[i]
		for _, ks := range [][]ebiten.Key{l.left, l.right, l.sprint, l.pay, l.attack} {
			add(ks)
		}
		for _, ks := range l.skills {
			add(ks)
		}
		for _, ks := range l.actions {
			add(ks)
		}
	}
	return out
}
