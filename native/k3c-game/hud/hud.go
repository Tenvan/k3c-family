// Package hud zeichnet die Anzeigen je Zelle: Münzen, Leben, Skills, Hinweise (Fach-Sprint des HUD). Bis dahin
// Platzhalter: ein Balken oben links, so lang wie die Münzen des Monarchen.
package hud

import (
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

var goldColor = color.RGBA{240, 200, 50, 255}

type drawer struct{}

// New liefert den Drawer des HUD; lib (Grafik) darf nil sein, die Platzhalter brauchen sie nicht.
func New(_ *art.Library) view.Drawer { return drawer{} }

func (drawer) Layer() view.Layer { return view.LayerHUD }

func (drawer) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	m := f.Monarch(c.Player)
	if m == nil {
		return
	}
	w := GoldBar(m.Gold, c.Rect.Dx())
	vector.FillRect(dst, float32(c.Rect.Min.X+16), float32(c.Rect.Min.Y+16), float32(w), 12, goldColor, false)
}

// GoldBar liefert die Balkenlänge in px für gold Münzen: 8 px je Münze, höchstens die halbe Zellenbreite.
func GoldBar(gold, cellW int) int {
	return max(0, min(gold*8, cellW/2))
}
