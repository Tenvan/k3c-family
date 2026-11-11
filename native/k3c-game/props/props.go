// Package props zeichnet Bauten: Burg, Bauplätze und Gebäude (Fach-Sprint der Bauten). Bis dahin Platzhalter:
// farbige Rechtecke an den Positionen der Simulation.
package props

import (
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"

	"k3c/native/k3c-game/view"
)

var (
	castleColor = color.RGBA{140, 140, 150, 255}
	builtColor  = color.RGBA{150, 100, 60, 255}
	siteColor   = color.RGBA{150, 100, 60, 110} // nicht gebaut: halb durchsichtig
)

type drawer struct{}

// New liefert den Drawer der Bauten.
func New() view.Drawer { return drawer{} }

func (drawer) Layer() view.Layer { return view.LayerBuildings }

func (drawer) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	w := f.World(c)
	if w == nil {
		return
	}
	k := c.Camera
	rect := func(x, width, h float64, clr color.RGBA) {
		if k.Visible(x, width/2) {
			vector.FillRect(dst, float32(k.ScreenX(x-width/2)), float32(c.ScreenY(h)), float32(width*k.Scale), float32(h*k.Scale), clr, false)
		}
	}
	rect(w.Castle.X, 6, 5, castleColor)
	for _, s := range w.Sites {
		clr := siteColor
		if s.State == "built" {
			clr = builtColor
		}
		rect(s.X, 3, 3, clr)
	}
}
