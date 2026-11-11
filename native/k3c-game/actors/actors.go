// Package actors zeichnet Figuren: Monarchen, Truppen und Gegner (Fach-Sprint der Figuren). Bis dahin Platzhalter:
// farbige Rechtecke an den Positionen der Simulation.
package actors

import (
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

var (
	monarchColor = color.RGBA{230, 190, 40, 255}
	troopColor   = color.RGBA{60, 90, 200, 255}
	enemyColor   = color.RGBA{200, 50, 50, 255}
)

// Box ist ein Platzhalter-Rechteck in Welt-Maßen: Mitte x, Breite und Höhe in Units, auf dem Boden stehend.
type Box struct {
	X, W, H float64
	Color   color.RGBA
}

// drawer hält den Puffer der Rechtecke, damit Draw nach dem ersten Bild nichts mehr anlegt.
type drawer struct{ buf []Box }

// New liefert den Drawer der Figuren; lib (Grafik) darf nil sein, die Platzhalter brauchen sie nicht.
func New(_ *art.Library) view.Drawer { return &drawer{} }

func (*drawer) Layer() view.Layer { return view.LayerActors }

func (d *drawer) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	d.buf = Boxes(d.buf[:0], f.World(c), c.Camera)
	for _, b := range d.buf {
		x := c.Camera.ScreenX(b.X - b.W/2)
		y := c.ScreenY(b.H)
		s := c.Camera.Scale
		vector.FillRect(dst, float32(x), float32(y), float32(b.W*s), float32(b.H*s), b.Color, false)
	}
}

// Boxes hängt die sichtbaren Figuren der Welt w als Rechtecke an out, hinten nach vorn: Truppen, Gegner, Monarchen.
func Boxes(out []Box, w *sim.World, k view.Camera) []Box {
	if w == nil {
		return out
	}
	add := func(x, width, h float64, clr color.RGBA) {
		if k.Visible(x, width/2) {
			out = append(out, Box{X: x, W: width, H: h, Color: clr})
		}
	}
	for _, t := range w.Troops {
		add(t.X, 0.6, 1.4, troopColor)
	}
	for _, e := range w.Enemies {
		add(e.X, 0.8, 1.2, enemyColor)
	}
	for _, p := range w.Players {
		if p.RespawnIn <= 0 {
			add(p.X, 1, 2.2, monarchColor)
		}
	}
	return out
}
