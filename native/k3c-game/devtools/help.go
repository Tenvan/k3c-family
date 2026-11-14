package devtools

// Tastenübersicht (F1): eine Zeile je Eintrag von view.DevKeys, Taste als Tastenkappe (UI-Tasten, unabhängig vom
// Eingabe-Profil, B-894), Wirkung und Gamepad-Kombi, mittig im Fenster.

import (
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// helpTitle ist die Überschrift der Übersicht.
const helpTitle = "Entwickler-Tasten"

// HelpRow ist eine Zeile der Übersicht.
type HelpRow struct {
	Key, Text, Pad string
}

// HelpRows liefert die Zeilen der Übersicht aus der Tabelle view.DevKeys (Reihenfolge der Tabelle).
func HelpRows() []HelpRow {
	out := make([]HelpRow, 0, len(view.DevKeys))
	for _, k := range view.DevKeys {
		out = append(out, HelpRow{Key: k.Label, Text: k.Text, Pad: k.Pad})
	}
	return out
}

// helpRows sind die Zeilen (die Tabelle ändert sich zur Laufzeit nicht).
var helpRows = HelpRows()

// drawHelp zeichnet die Übersicht mittig in die Zelle c (Fenster); Schriftgröße nach der Fensterhöhe.
func drawHelp(dst *ebiten.Image, font *art.Font, c view.Cell) {
	r := c.Rect
	px := max(14, float64(r.Dy())/36)
	lineH := px * art.LineSpacing
	keyW, textW, padW := px*3, px*18, px*7
	if font != nil {
		keyW, textW, padW = columns(font, px)
	}
	gap := px
	w := keyW + textW + padW + 4*gap
	h := lineH*float64(len(helpRows)+2) + 2*gap
	x := float64(r.Min.X) + (float64(r.Dx())-w)/2
	y := float64(r.Min.Y) + (float64(r.Dy())-h)/2
	box(dst, x, y, w, h)
	if font == nil {
		return
	}
	font.Draw(dst, helpTitle, x+gap, y+gap, px, textColor)
	for i, row := range helpRows {
		ry := y + gap + lineH*float64(i+2)
		keyCap(dst, font, row.Key, x+gap, ry, px)
		font.Draw(dst, row.Text, x+2*gap+keyW, ry, px, textColor)
		if row.Pad != "" {
			font.Draw(dst, row.Pad, x+3*gap+keyW+textW, ry, px, dimColor)
		}
	}
}

// Farben der Tastenkappe (wie die Kappen im HUD, hud/glyphs.go › glyphColors["key"]).
var (
	capFill = color.RGBA{242, 242, 242, 255}
	capText = color.RGBA{17, 17, 17, 255}
	capEdge = color.RGBA{0, 0, 0, 255}
)

// keyCap zeichnet die Taste label als helle Kappe mit dunkler Schrift, oben links an x, y (Zeilenhöhe px).
func keyCap(dst *ebiten.Image, font *art.Font, label string, x, y, px float64) {
	w, _ := font.Measure(label, px)
	pad := px * 0.3
	fill(dst, x-pad-1, y-1, w+2*pad+2, px*1.2+2, capEdge)
	fill(dst, x-pad, y, w+2*pad, px*1.2, capFill)
	font.Draw(dst, label, x, y, px, capText)
}

// columns misst die Breiten der drei Spalten.
func columns(font *art.Font, px float64) (keyW, textW, padW float64) {
	for _, row := range helpRows {
		kw, _ := font.Measure(row.Key, px)
		tw, _ := font.Measure(row.Text, px)
		pw, _ := font.Measure(row.Pad, px)
		keyW, textW, padW = max(keyW, kw), max(textW, tw), max(padW, pw)
	}
	tw, _ := font.Measure(helpTitle, px)
	return keyW, max(textW, tw), padW
}
