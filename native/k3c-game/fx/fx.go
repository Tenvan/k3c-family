// Package fx zeichnet Effekte: Geschosse, Treffer, Zauber (Fach-Sprint der Effekte). Bis dahin Platzhalter: kleine
// helle Rechtecke für Geschosse.
package fx

import (
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

var projectileColor = color.RGBA{250, 250, 240, 255}

type drawer struct{}

// New liefert den Drawer der Effekte; lib (Grafik) darf nil sein, die Platzhalter brauchen sie nicht.
func New(_ *art.Library) view.Drawer { return drawer{} }

func (drawer) Layer() view.Layer { return view.LayerEffects }

func (drawer) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	w := f.World(c)
	if w == nil {
		return
	}
	k := c.Camera
	for _, p := range w.Projectiles {
		if k.Visible(p.X, 0.3) {
			vector.FillRect(dst, float32(k.ScreenX(p.X-0.3)), float32(c.ScreenY(1.2)), float32(0.6*k.Scale), float32(0.1*k.Scale), projectileColor, false)
		}
	}
}
