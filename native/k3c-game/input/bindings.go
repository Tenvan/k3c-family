// Package input liest Tastatur und Gamepads und liefert view.Input (Sprint NG5, B-798). Die Tastatur ist ein Gerät
// (keys1) für einen Spieler; ihre Belegung folgt dem Tastatur-Profil der Einstellungen (B-893, view/profile.go,
// SetKeyboard). Belegung je Profil:
//
//	Befehl (gehalten)  wasd          pfeile                     Gamepad (Xbox)
//	laufen             A / D         ← / →                      linker Stick oder Steuerkreuz ←/→
//	Galopp (Sprint)    Shift links   Shift rechts               RT
//	zahlen             Leertaste     Enter, Num-Enter, Leertaste  A   (B-917: die Leertaste löst in jedem Profil aus)
//	Schlag             E             Strg rechts, Num 0         X
//	Skill 1 … 4        Q, R, T, Z    Einfg, Entf, Ende, Bild ↓  LB, RB, LT, Steuerkreuz ↑ (der niedrigste gewinnt)
//	                                 oder Num 1 … Num 4
//
//	Aktion (Flanke)    wasd          pfeile                     Gamepad
//	↑ ↓ ← →            W S A D       Pfeile                     Steuerkreuz oder Stick (Wiederholung beim Halten)
//	Bestätigen         Leertaste     Enter, Num-Enter           A
//	Zurück             Esc           Esc, Rücktaste             B (im Spiel unbelegt, game wertet sie dort nicht aus)
//	Menü               Esc           Esc, Rücktaste             Menu kurz (beim Loslassen, < 600 ms, ohne View)
//	View               Tab           Tab                        View (beim Loslassen, ohne Menu)
//	Seite zurück/vor   Q / E         Bild ↑ / Bild ↓            LB / RB
//	Home               Pos1          Pos1                       View + Menu gemeinsam (statt Menu und View)
//	Skill-Menü         K             ↓, Num 5                   Steuerkreuz ↓ (zusätzlich zu ↓, nicht der Stick)
//	Vollbild, Cheats   F11, F9       F11, F9                    rechter Stick gedrückt, View + Y
//
// input unterscheidet Spiel und Menü nicht: Steuerkreuz ↓ (und ↓ im Profil pfeile) liefert ActionDown und
// ActionSkillMenu zugleich; das Skill-Menü (Overlay) wertet ActionSkillMenu nur im Spiel aus, Menüs nehmen ActionDown.
// Die übrigen Entwickler-Tasten F1 … F12 (view/devkeys.go) gelten unabhängig vom Profil.
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
const actionCount = int(view.ActionCheats) + 1

// keyLayout ist die Belegung eines Tastatur-Profils; je Eintrag gelten alle genannten Tasten.
type keyLayout struct {
	left, right, sprint, pay, attack []ebiten.Key
	skills                           [4][]ebiten.Key
	actions                          [actionCount][]ebiten.Key
}

func keys(k ...ebiten.Key) []ebiten.Key { return k }

// Indizes der Tastatur-Profile in layouts (Layout).
const (
	layoutWASD   = 0
	layoutArrows = 1
)

// Layouts ist die Zahl der Tastatur-Belegungen (Index für ActionKeys und CommandKeys).
const Layouts = 2

// Layout liefert den Index der Belegung zum Tastatur-Profil p (view.ProfileArrows: 1, sonst 0 = WASD).
func Layout(p string) int {
	if p == view.ProfileArrows {
		return layoutArrows
	}
	return layoutWASD
}

// layouts sind die Belegungen der Tastatur-Profile wasd und pfeile (Index Layout). Z liegt auf QWERTZ auf der Taste
// KeyY, darum gelten für Skill 4 beide.
var layouts = [Layouts]keyLayout{
	layoutWASD: {
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
			view.ActionFullscreen: keys(view.DevKeyOf(view.DevFullscreen)),
			view.ActionCheats:     keys(view.DevKeyOf(view.DevCheats)),
		},
	},
	layoutArrows: {
		left: keys(ebiten.KeyArrowLeft), right: keys(ebiten.KeyArrowRight), sprint: keys(ebiten.KeyShiftRight),
		pay:    keys(ebiten.KeyEnter, ebiten.KeyNumpadEnter, ebiten.KeySpace),
		attack: keys(ebiten.KeyControlRight, ebiten.KeyNumpad0),
		skills: [4][]ebiten.Key{keys(ebiten.KeyInsert, ebiten.KeyNumpad1), keys(ebiten.KeyDelete, ebiten.KeyNumpad2),
			keys(ebiten.KeyEnd, ebiten.KeyNumpad3), keys(ebiten.KeyPageDown, ebiten.KeyNumpad4)},
		actions: [actionCount][]ebiten.Key{
			view.ActionUp: keys(ebiten.KeyArrowUp), view.ActionDown: keys(ebiten.KeyArrowDown),
			view.ActionLeft: keys(ebiten.KeyArrowLeft), view.ActionRight: keys(ebiten.KeyArrowRight),
			view.ActionConfirm: keys(ebiten.KeyEnter, ebiten.KeyNumpadEnter),
			view.ActionBack:    keys(ebiten.KeyEscape, ebiten.KeyBackspace),
			view.ActionMenu:    keys(ebiten.KeyEscape, ebiten.KeyBackspace), view.ActionView: keys(ebiten.KeyTab),
			view.ActionPrevPage: keys(ebiten.KeyPageUp), view.ActionNextPage: keys(ebiten.KeyPageDown),
			view.ActionHome: keys(ebiten.KeyHome), view.ActionSkillMenu: keys(ebiten.KeyArrowDown, ebiten.KeyNumpad5),
			view.ActionFullscreen: keys(view.DevKeyOf(view.DevFullscreen)),
			view.ActionCheats:     keys(view.DevKeyOf(view.DevCheats)),
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
	padY     = ebiten.StandardGamepadButtonRightTop
)

// padSkills sind die Skill-Slots 1 … 4 des Gamepads.
var padSkills = [4]ebiten.StandardGamepadButton{padLB, padRB, padLT, padUp}

// padActions ist die Menü-Belegung des Gamepads; -1 = keine Taste (Home ist die Kombi View + Menu). Y zählt nur mit
// gehaltenem View als ActionCheats (padChord).
var padActions = [actionCount]ebiten.StandardGamepadButton{
	view.ActionUp: padUp, view.ActionDown: padDown, view.ActionLeft: padLeft, view.ActionRight: padRight,
	view.ActionConfirm: padA, view.ActionBack: padB, view.ActionMenu: padMenu, view.ActionView: padView,
	view.ActionPrevPage: padLB, view.ActionNextPage: padRB, view.ActionHome: -1,
	view.ActionSkillMenu: padDown, view.ActionFullscreen: padRS, view.ActionCheats: padY,
}

// usedKeys sind alle Tasten beider Profile (Ebitengine wird nur nach ihnen gefragt).
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
	for i := range layouts {
		l := &layouts[i]
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

// ActionKeys liefert die Tasten der Belegung layout (Layout: 0 wasd, 1 pfeile) für die Menü-Aktion a (nil = keine).
// Für die Tastenbelegung und Abgleich-Tests anderer Pakete (Tasten-Texte im HUD, game/keys_test.go).
func ActionKeys(layout int, a view.Action) []ebiten.Key {
	if layout < 0 || layout >= len(layouts) || a < 0 || int(a) >= actionCount {
		return nil
	}
	return layouts[layout].actions[a]
}

// CommandKeys liefert die Tasten der Belegung layout für einen gehaltenen Befehl: left, right, sprint, pay, attack,
// skill1 … skill4 (nil = unbekannt).
func CommandKeys(layout int, name string) []ebiten.Key {
	if layout < 0 || layout >= len(layouts) {
		return nil
	}
	l := &layouts[layout]
	switch name {
	case "left":
		return l.left
	case "right":
		return l.right
	case "sprint":
		return l.sprint
	case "pay":
		return l.pay
	case "attack":
		return l.attack
	case "skill1", "skill2", "skill3", "skill4":
		return l.skills[name[5]-'1']
	}
	return nil
}
