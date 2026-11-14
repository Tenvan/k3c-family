package hud

// Glyphen und Beschriftungen der Tasten (Spiegel von src/scenes/hints/glyphs.ts und src/input/slotBindings.ts, S6.2,
// B-149): Aktion und Gerät → Glyph-Schlüssel und Beschriftung. Nur Xbox-Glyphen für Pads; die Tastatur hat zwei
// Hälften (native/k3c-game/input: links WASD, rechts Pfeile und Ziffernblock). B und View bekommen nie ein Bild.

import (
	"image/color"
	"strings"

	"k3c/native/k3c-game/view"
)

// glyph ist das Bild einer Taste: key "" = nur Text (Rückfall), label = Beschriftung im Bild bzw. Text.
type glyph struct {
	key, label string
}

// Aktionen in der Reihenfolge der Belegung (slotBindings.ts › KEY_ACTIONS).
var glyphActions = [...]string{"confirm", "pause", "fullscreen", "attack", "skill1", "skill2", "skill3", "skill4", "skillMenu"}

// Tasten je Aktion: Pad (Name der Xbox-Taste), linke und rechte Tastatur-Hälfte. "" = nicht belegt. Die Tastatur
// spiegelt input/bindings.go (game/keys_test.go prüft das über KeyName); left/right sind Laufen bzw. Wählen.
var (
	padKeys   = map[string]string{"confirm": "A", "pause": "MENU", "fullscreen": "RS", "attack": "X", "skill1": "LB", "skill2": "RB", "skill3": "LT", "skill4": "UP", "skillMenu": "DOWN"}
	leftKeys  = map[string]string{"confirm": "SPACE", "pause": "ESC", "fullscreen": "F", "attack": "E", "skill1": "Q", "skill2": "R", "skill3": "T", "skill4": "Z", "skillMenu": "K", "left": "A", "right": "D"}
	rightKeys = map[string]string{"confirm": "ENTER", "pause": "BACKSPACE", "attack": "NUM0", "skill1": "NUM1", "skill2": "NUM2", "skill3": "NUM3", "skill4": "NUM4", "skillMenu": "NUM5", "left": "LEFT", "right": "RIGHT"}
)

// KeyActions sind die Aktionen mit Tastatur-Taste (KeyName).
var KeyActions = [...]string{"confirm", "pause", "fullscreen", "attack", "skill1", "skill2", "skill3", "skill4", "skillMenu", "left", "right"}

// KeyName liefert den Namen der Taste der Tastatur-Hälfte half (0 links, 1 rechts) für action ("" = nicht belegt), wie
// ihn das HUD beschriftet: Buchstaben, SPACE, ESC, ENTER, BACKSPACE, LEFT, RIGHT, NUMn. Für den Abgleich mit input.
func KeyName(half int, action string) string {
	if half == 1 {
		return rightKeys[action]
	}
	return leftKeys[action]
}

// padGlyphs sind die Controller-Tasten mit Bild (B-149).
var padGlyphs = [...]string{"A", "X", "Y", "LB", "RB", "LT", "RT", "UP", "DOWN", "MENU"}

// padLabels: Beschriftung der Steuerkreuz-Tasten (slotBindings.ts › PAD_LABELS).
var padLabels = map[string]string{"UP": "D-Pad ↑", "DOWN": "D-Pad ↓"}

// glyphColors: Füllung und Schrift je Glyph-Art (Xbox-Farben für A/X/Y), Kontrast ≥ 4,5:1 (Test).
var glyphColors = map[string][2]color.RGBA{
	"A":   {rgb(0x107c10, 1), rgb(0xffffff, 1)},
	"X":   {rgb(0x0e6ac7, 1), rgb(0xffffff, 1)},
	"Y":   {rgb(0xffb900, 1), rgb(0x000000, 1)},
	"pad": {rgb(0x2b2b2b, 1), rgb(0xffffff, 1)},
	"key": {rgb(0xf2f2f2, 1), rgb(0x111111, 1)},
}

// keyLabel ist die Beschriftung einer Tastatur-Taste (Leertaste und Umschalttasten aus den Texten, Ziffernblock als
// „Num n“).
func keyLabel(lang, name string) string {
	switch {
	case name == "SPACE":
		return tr(lang, "hint.space")
	case name == "SHIFTLEFT":
		return tr(lang, "key.shiftLeft")
	case name == "SHIFTRIGHT":
		return tr(lang, "key.shiftRight")
	case name == "CONTROLRIGHT":
		return tr(lang, "key.ctrlRight")
	case name == "ESC":
		return "Esc"
	case name == "ENTER":
		return "Enter"
	case name == "BACKSPACE":
		return "Backspace"
	case name == "LEFT", name == "RIGHT": // Wörter statt Pfeilzeichen: die Debug-Schrift kennt nur ASCII
		return tr(lang, "hint."+strings.ToLower(name))
	case strings.HasPrefix(name, "NUM"):
		return "Num " + name[3:]
	}
	return name
}

// KeyLabel ist die Beschriftung der Tastatur-Taste name (Schreibweise wie KeyName, dazu SHIFTLEFT, SHIFTRIGHT,
// CONTROLRIGHT) in der Sprache lang, wie das HUD sie zeigt; für die Tastenbelegung im Pause-Menü (game, B-807).
func KeyLabel(lang, name string) string { return keyLabel(lang, name) }

// PadLabel ist die Beschriftung der Gamepad-Taste für action (Aktionen wie KeyName, dazu "sprint" und "move") in der
// Sprache lang; "" = nicht belegt. Für die Tastenbelegung im Pause-Menü (game, B-807).
func PadLabel(lang, action string) string {
	switch action {
	case "move":
		return tr(lang, "key.padMove")
	case "sprint":
		return "RT"
	}
	name, ok := padKeys[action]
	if !ok {
		return ""
	}
	if l, ok := padLabels[name]; ok {
		return l
	}
	if name == "MENU" {
		return "Menu"
	}
	return name
}

// glyphOf liefert Glyph-Schlüssel und Beschriftung einer Aktion auf dem Gerät d; unbekannte oder nicht belegte Aktion:
// Text-Rückfall mit dem Aktionsnamen (rechter Stick: „RS“).
func glyphOf(lang, action string, d view.DeviceID) glyph {
	if d.Kind == view.DeviceGamepad {
		name, ok := padKeys[action]
		if !ok {
			return glyph{"", action}
		}
		label := name
		if l, ok := padLabels[name]; ok {
			label = l
		}
		for _, g := range padGlyphs {
			if g == name {
				return glyph{"pad:" + name, label}
			}
		}
		return glyph{"", label}
	}
	keys := leftKeys
	if d.Index == 1 {
		keys = rightKeys
	}
	name, ok := keys[action]
	if !ok {
		return glyph{"", action}
	}
	return glyph{"key:" + name, keyLabel(lang, name)}
}

// slotLabels ist die Beschriftung von Schlag, Skill-Slot 1–4 und Skill-Menü auf dem Gerät d.
func slotLabels(lang string, d view.DeviceID) [6]string {
	var out [6]string
	for i, a := range glyphActions[3:] {
		out[i] = glyphOf(lang, a, d).label
	}
	return out
}

// joinText ist der Beitritts-Hinweis für die freien Geräte (Frame.Unseated): je Art des Geräts ein Satz, in der
// Reihenfolge der Eingabe, jede Art höchstens einmal (joinHint.ts, B-391); ohne freies Gerät "".
func joinText(lang string, unseated []view.DeviceID) string {
	var seen [3]bool
	parts := make([]string, 0, 3)
	for _, d := range unseated {
		i, key := 0, "hud.join.pad"
		switch {
		case d.Kind == view.DeviceKeyboard && d.Index == 0:
			i, key = 1, "hud.join.keyboard"
		case d.Kind == view.DeviceKeyboard:
			i, key = 2, "hud.join.keyboard2"
		}
		if !seen[i] {
			seen[i] = true
			parts = append(parts, tr(lang, key))
		}
	}
	return strings.Join(parts, sep)
}

// joinKey: welche Arten freier Geräte es gibt (Pad, linke, rechte Tastatur) in Reihenfolge des ersten Auftretens.
type joinKey struct {
	lang  string
	order [3]int8
	n     int
}

// newJoinKey fasst die freien Geräte für memo zusammen, ohne anzulegen.
func newJoinKey(lang string, unseated []view.DeviceID) joinKey {
	k := joinKey{lang: lang}
	for _, d := range unseated {
		var i int8
		if d.Kind == view.DeviceKeyboard {
			i = 1 + int8(min(d.Index, 1))
		}
		known := false
		for j := range k.n {
			known = known || k.order[j] == i
		}
		if !known && k.n < len(k.order) {
			k.order[k.n] = i
			k.n++
		}
	}
	return k
}

// joinFromKey baut den Text aus dem Schlüssel.
func joinFromKey(k joinKey) string {
	devs := make([]view.DeviceID, 0, k.n)
	for _, i := range k.order[:k.n] {
		switch i {
		case 0:
			devs = append(devs, view.DeviceID{Kind: view.DeviceGamepad})
		default:
			devs = append(devs, view.DeviceID{Kind: view.DeviceKeyboard, Index: int(i - 1)})
		}
	}
	return joinText(k.lang, devs)
}
