package world

// Zeichnen der Kulisse und des Bodens: dünn, Lage aus den reinen Funktionen (cull.go, backdrop.go, ground.go).

import (
	"math"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// animStand ist die Animation der Kulissen-Kacheln.
const animStand = "stand"

// drawFar füllt den Welt-Teil der Zelle mit dem Himmel des Bioms und zeichnet die feste Schicht `fern`; beide sind
// nachts dunkler als der Vordergrund (backgroundBrightness).
func (w *World) drawFar(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	st := stageOf(f, c)
	if st == nil {
		return
	}
	lk := w.looks.of(st.Level)
	back := w.light.backdrop(st.World.Cycle)
	w.fillRect(dst, c.WorldRect(), lk.sky, back)
	w.drawLayer(dst, c, st.World, lk, 0, back)
}

func (w *World) drawMid(dst *ebiten.Image, f *view.Frame, c view.Cell)  { w.drawScrolling(dst, f, c, 1) }
func (w *World) drawNear(dst *ebiten.Image, f *view.Frame, c view.Cell) { w.drawScrolling(dst, f, c, 2) }

// drawScrolling zeichnet die scrollende Schicht i (1 = mitte, 2 = nah).
func (w *World) drawScrolling(dst *ebiten.Image, f *view.Frame, c view.Cell, i int) {
	st := stageOf(f, c)
	if st == nil {
		return
	}
	w.drawLayer(dst, c, st.World, w.looks.of(st.Level), i, w.light.backdrop(st.World.Cycle))
}

// drawLayer zeichnet die sichtbaren Kacheln der Schicht i, Fußlinie auf der Boden-Linie, getönt mit Helligkeit back.
func (w *World) drawLayer(dst *ebiten.Image, c view.Cell, wd *sim.World, lk *look, i int, back float64) {
	names := lk.assets[i]
	if len(names) == 0 || w.lib == nil {
		return
	}
	factor := lk.scroll[i]
	first, last := tileRange(c.Camera, factor, tileUnits)
	for k := first; k <= last; k++ {
		sh := w.lib.Sheet(names[variant(wd.Level.Backdrops, k, len(names))-1], animStand)
		if sh == nil || sh.FW == 0 || sh.FH == 0 {
			continue
		}
		r := tileRect(c, factor, k, sh.FW, sh.FH, sh.OriginX, sh.OriginY, sh.UnitsPerPx)
		w.op.GeoM.Reset()
		w.op.GeoM.Scale(r.w/float64(sh.FW), r.h/float64(sh.FH))
		w.op.GeoM.Translate(r.x, r.y)
		w.op.ColorScale.Reset()
		w.op.ColorScale.Scale(float32(back), float32(back), float32(back), 1)
		dst.DrawImage(sh.Frame(0), &w.op)
	}
}

// rectF ist ein Rechteck in Fenster-px.
type rectF struct{ x, y, w, h float64 }

// tileRect liefert die Lage der Kachel k einer Schicht mit Scroll-Faktor factor: links bei Welt-x k·tileUnits auf der
// Ebene, Fußpunkt (Anteil originX, originY des Frames) auf der Boden-Linie. Die Kanten liegen auf ganzen Bildpunkten,
// so stoßen Nachbarn ohne Naht aneinander.
func tileRect(c view.Cell, factor float64, k, fw, fh int, originX, originY, unitsPerPx float64) rectF {
	s := c.Camera.Scale * unitsPerPx / framePerUnit
	w, h := float64(fw)*s, float64(fh)*s
	x0 := c.Camera.ParallaxX(float64(k)*tileUnits, factor) - originX*w
	x1 := c.Camera.ParallaxX(float64(k+1)*tileUnits, factor) - originX*w
	left, right := math.Round(x0), math.Round(x0+w)
	if math.Abs(x1-(x0+w)) < 1 { // Kachel so breit wie der Abstand: an die gerundete Nachbarkante legen
		right = math.Round(x1)
	}
	return rectF{x: left, y: float64(c.GroundY()) - originY*h, w: right - left, h: h}
}

// stageOf liefert die Stufe der Zelle mit Welt, sonst nil.
func stageOf(f *view.Frame, c view.Cell) *view.Stage {
	seat, ok := f.Seat(c.Player)
	if !ok {
		return nil
	}
	st, ok := f.Stage(seat.Stage)
	if !ok || st.World == nil {
		return nil
	}
	return st
}
