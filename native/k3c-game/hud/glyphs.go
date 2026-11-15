package hud

// Glyphen und Beschriftungen der Tasten (Spiegel von src/scenes/hints/glyphs.ts und src/input/slotBindings.ts, S6.2,
// B-149): Aktion und Gerät → Glyph-Schlüssel und Beschriftung. Nur Xbox-Glyphen für Pads; die Tastatur hat zwei
// Belegungen (Eingabe-Profile wasd und pfeile, native/k3c-game/input, B-893). B und View bekommen nie ein Bild.

import (
	"image/color"
	"slices"
	"strings"
	"unicode/utf8"

	"k3c/native/k3c-game/view"
)

// glyph ist das Bild einer Taste: key "" = nur Text (Rückfall), label = Beschriftung im Bild bzw. Text, icon = das
// Papierschnitt-Icon dazu (IconAsset; "" ohne Bild).
type glyph struct {
	key, label, icon string
}

// newGlyph ist die Glyph zum Schlüssel key mit Beschriftung label und ihrem Icon.
func newGlyph(key, label string) glyph { return glyph{key, label, IconAsset(key, label)} }

// Aktionen in der Reihenfolge der Belegung (slotBindings.ts › KEY_ACTIONS).
var glyphActions = [...]string{"confirm", "pause", "fullscreen", "attack", "skill1", "skill2", "skill3", "skill4", "skillMenu"}

// Tasten je Aktion: Pad (Name der Xbox-Taste), Tastatur-Profil wasd (leftKeys) und pfeile (rightKeys). "" = nicht
// belegt. Die Tastatur spiegelt input/bindings.go (game/keys_test.go prüft das über KeyName); left/right sind Laufen
// bzw. Wählen.
var (
	padKeys   = map[string]string{"confirm": "A", "pause": "MENU", "fullscreen": "RS", "attack": "X", "skill1": "LB", "skill2": "RB", "skill3": "LT", "skill4": "UP", "skillMenu": "DOWN"}
	leftKeys  = map[string]string{"confirm": "SPACE", "pause": "ESC", "fullscreen": "F11", "attack": "E", "skill1": "Q", "skill2": "R", "skill3": "T", "skill4": "Z", "skillMenu": "K", "left": "A", "right": "D"}
	rightKeys = map[string]string{"confirm": "ENTER", "pause": "ESC", "fullscreen": "F11", "attack": "CONTROLRIGHT", "skill1": "INSERT", "skill2": "DELETE", "skill3": "END", "skill4": "PAGEDOWN", "skillMenu": "DOWN", "left": "LEFT", "right": "RIGHT"}
)

// KeyActions sind die Aktionen mit Tastatur-Taste (KeyName).
var KeyActions = [...]string{"confirm", "pause", "fullscreen", "attack", "skill1", "skill2", "skill3", "skill4", "skillMenu", "left", "right"}

// KeyName liefert den Namen der Taste der Tastatur-Belegung layout (0 wasd, 1 pfeile) für action ("" = nicht
// belegt), wie ihn das HUD beschriftet: Buchstaben, SPACE, ESC, ENTER, LEFT, RIGHT, DOWN, INSERT … Für den Abgleich
// mit input.
func KeyName(layout int, action string) string {
	if layout == 1 {
		return rightKeys[action]
	}
	return leftKeys[action]
}

// padGlyphs sind die Controller-Tasten mit Bild (B-149).
var padGlyphs = [...]string{"A", "X", "Y", "LB", "RB", "LT", "RT", "RS", "UP", "DOWN", "MENU"}

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
	case name == "INSERT", name == "DELETE", name == "END", name == "PAGEDOWN", name == "PAGEUP":
		return tr(lang, "key."+strings.ToLower(name))
	case name == "ESC":
		return "Esc"
	case name == "ENTER":
		return "Enter"
	case name == "BACKSPACE":
		return "Backspace"
	case name == "LEFT", name == "RIGHT", name == "UP", name == "DOWN": // Wörter statt Pfeilzeichen: die Debug-Schrift kennt nur ASCII
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

// glyphOf ist die zentrale Zuordnung Aktion + Eingabe-Profil → Tasten-Icon (B-894): Glyph-Schlüssel ("pad:A",
// "key:E") und Beschriftung; alle Tasten-Hinweise des HUD gehen darüber. profile ist view.ProfileWASD,
// view.ProfileArrows oder view.ProfileGamepad (view.Settings.ProfileOf); unbekannte oder nicht belegte Aktion:
// Text-Rückfall mit dem Aktionsnamen.
func glyphOf(lang, action, profile string) glyph {
	if profile == view.ProfileGamepad {
		name, ok := padKeys[action]
		if !ok {
			return glyph{label: action}
		}
		label := name
		if l, ok := padLabels[name]; ok {
			label = l
		}
		if slices.Contains(padGlyphs[:], name) {
			return newGlyph("pad:"+name, label)
		}
		return glyph{label: label}
	}
	keys := leftKeys
	if profile == view.ProfileArrows {
		keys = rightKeys
	}
	name, ok := keys[action]
	if !ok {
		return glyph{label: action}
	}
	return newGlyph("key:"+name, keyLabel(lang, name))
}

// Tastenkappen (B-911): leer, die Beschriftung setzt das HUD in der Sprache des Spiels darauf (drawglyph.go).
const (
	capNarrow = "icons/taste-kappe"       // ein Zeichen
	capWide   = "icons/taste-kappe-breit" // längere Beschriftung
)

// IconAsset ist das Papierschnitt-Icon (Gruppe icons) zum Glyph-Schlüssel key mit Beschriftung label: "pad:A" →
// icons/xbox-a; eine Tastatur-Taste bekommt nach der Länge ihrer Beschriftung die schmale (ein Zeichen) oder die
// breite Kappe; "" ohne Bild. Ist das Icon nicht geladen, zeichnet drawGlyph Kappe bzw. Xbox-Taste selbst.
func IconAsset(key, label string) string {
	kind, name, ok := strings.Cut(key, ":")
	switch {
	case !ok || name == "":
		return ""
	case kind == "pad":
		return "icons/xbox-" + strings.ToLower(name)
	case utf8.RuneCountInString(label) <= 1:
		return capNarrow
	}
	return capWide
}

// slotLabels ist die Beschriftung von Schlag, Skill-Slot 1–4 und Skill-Menü im Eingabe-Profil profile.
func slotLabels(lang, profile string) [6]string {
	var out [6]string
	for i, a := range glyphActions[3:] {
		out[i] = glyphOf(lang, a, profile).label
	}
	return out
}

// joinText ist der Beitritts-Hinweis für die freien Geräte (Frame.Unseated): je Art des Geräts ein Satz, in der
// Reihenfolge der Eingabe, jede Art höchstens einmal (joinHint.ts, B-391). Die Tastatur nennt nur Enter (B-946: „Leertaste“
// stört; Enter bestätigt in jedem Profil). Ohne freies Gerät "".
func joinText(lang string, unseated []view.DeviceID) string {
	var seen [2]bool
	parts := make([]string, 0, 2)
	for _, d := range unseated {
		i, key := 0, "hud.join.pad"
		if d.Kind == view.DeviceKeyboard {
			i, key = 1, "hud.join.keyboard"
		}
		if !seen[i] {
			seen[i] = true
			parts = append(parts, tr(lang, key))
		}
	}
	return strings.Join(parts, sep)
}

// joinKey: welche Arten freier Geräte es gibt (0 Pad, 1 Tastatur) in Reihenfolge des ersten Auftretens.
type joinKey struct {
	lang  string
	order [2]int8
	n     int
}

// newJoinKey fasst die freien Geräte für memo zusammen, ohne anzulegen.
func newJoinKey(lang string, unseated []view.DeviceID) joinKey {
	k := joinKey{lang: lang}
	for _, d := range unseated {
		var i int8
		if d.Kind == view.DeviceKeyboard {
			i = 1
		}
		if !slices.Contains(k.order[:k.n], i) && k.n < len(k.order) {
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
		if i == 1 {
			devs = append(devs, view.DeviceID{Kind: view.DeviceKeyboard})
		} else {
			devs = append(devs, view.DeviceID{Kind: view.DeviceGamepad})
		}
	}
	return joinText(k.lang, devs)
}
