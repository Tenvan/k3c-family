// Package world zeichnet Kulisse, Boden und Fluss einer Zelle (Fach-Sprint der Welt). Bis dahin Platzhalter: farbige
// Bänder je Ebene.
package world

import (
	"image"
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// band ist ein Platzhalter-Drawer: ein farbiges Band der Zelle auf einer Ebene.
type band struct {
	layer view.Layer
	clr   color.RGBA
}

// New liefert die Drawer der Welt: fern, mitte, nah, Boden und Fluss; lib (Grafik) darf nil sein, die Platzhalter brauchen sie nicht.
func New(_ *art.Library) []view.Drawer {
	return []view.Drawer{
		band{view.LayerFar, color.RGBA{158, 199, 219, 255}},
		band{view.LayerMid, color.RGBA{120, 160, 130, 255}},
		band{view.LayerNear, color.RGBA{80, 130, 80, 255}},
		band{view.LayerGround, color.RGBA{110, 85, 55, 255}},
		band{view.LayerRiver, color.RGBA{60, 110, 170, 255}},
	}
}

func (b band) Layer() view.Layer { return b.layer }

func (b band) Draw(dst *ebiten.Image, _ *view.Frame, c view.Cell) {
	r := bandRect(b.layer, c)
	if r.Empty() {
		return
	}
	vector.FillRect(dst, float32(r.Min.X), float32(r.Min.Y), float32(r.Dx()), float32(r.Dy()), b.clr, false)
}

// bandRect liefert das Band einer Ebene in der Zelle: fern füllt die Welt, mitte und nah die untere Hälfte bzw. das
// untere Fünftel der Welt, Boden einen Streifen über der Boden-Linie, Fluss das untere Drittel.
func bandRect(l view.Layer, c view.Cell) image.Rectangle {
	w := c.WorldRect()
	switch l {
	case view.LayerFar:
		return w
	case view.LayerMid:
		return image.Rect(w.Min.X, w.Max.Y-w.Dy()/2, w.Max.X, w.Max.Y)
	case view.LayerNear:
		return image.Rect(w.Min.X, w.Max.Y-w.Dy()/5, w.Max.X, w.Max.Y)
	case view.LayerGround:
		return image.Rect(w.Min.X, w.Max.Y-max(2, w.Dy()/30), w.Max.X, w.Max.Y)
	case view.LayerRiver:
		return c.RiverRect()
	}
	return image.Rectangle{}
}
