package props

// Zeichnen der Zeichenliste mit Ebitengine. Formen kommen aus wenigen weißen Vorlagen (Bildpunkt, Scheibe, Ring,
// Dreieck), eingefärbt über ColorScale: so entsteht je Bild nichts Neues und Ebitengine stapelt die Aufrufe.

import (
	"image"
	"image/color"
	"math"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// stencilPx: Kantenlänge der Vorlagen für Scheibe, Ring und Dreieck.
const stencilPx = 64

// painter hält die Vorlagen (beim ersten Zeichnen angelegt), die Zeichen-Optionen und die gemessenen Textbreiten.
type painter struct {
	pixel, disc, ring, tri *ebiten.Image
	op                     ebiten.DrawImageOptions
	widths                 map[widthKey]float64
}

type widthKey struct {
	text string
	size float64
}

// stencils legt die Vorlagen einmal an.
func (pt *painter) stencils() {
	if pt.pixel != nil {
		return
	}
	white := ebiten.NewImage(3, 3)
	white.Fill(color.White)
	pt.pixel = white.SubImage(image.Rect(1, 1, 2, 2)).(*ebiten.Image)
	const r = stencilPx / 2
	pt.disc = ebiten.NewImage(stencilPx, stencilPx)
	vector.FillCircle(pt.disc, r, r, r, color.White, true)
	pt.ring = ebiten.NewImage(stencilPx, stencilPx)
	vector.StrokeCircle(pt.ring, r, r, r-r*0.125, r*0.25, color.White, true) // Ring: Breite = Radius/4
	pt.tri = ebiten.NewImage(stencilPx, stencilPx)
	var path vector.Path
	path.MoveTo(r, 0)
	path.LineTo(stencilPx, stencilPx)
	path.LineTo(0, stencilPx)
	path.Close()
	vector.FillPath(pt.tri, &path, &vector.FillOptions{}, &vector.DrawPathOptions{AntiAlias: true, ColorScale: ebiten.ColorScale{}})
	pt.widths = map[widthKey]float64{}
}

// paint zeichnet die Einträge in Reihenfolge in dst (Zelle c).
func (pt *painter) paint(dst *ebiten.Image, items []item, lib *art.Library, c view.Cell, clock float64) {
	pt.stencils()
	for i := range items {
		it := &items[i]
		switch it.shape {
		case shapeSprite:
			pt.sprite(dst, it, lib, c, clock)
		case shapeTiles:
			pt.tiles(dst, it, lib, c)
		case shapeText:
			if lib != nil {
				pt.text(dst, it, lib.Font, c)
			}
		default:
			pt.form(dst, it, c)
		}
	}
}

// colorScale setzt Farbe und Deckkraft der Vorlage (clr vormultipliziert, alpha 0 = voll).
func (pt *painter) colorScale(clr color.RGBA, alpha float32) {
	pt.op.ColorScale.Reset()
	pt.op.ColorScale.ScaleWithColor(clr)
	if alpha > 0 {
		pt.op.ColorScale.ScaleAlpha(alpha)
	}
}

// sprite zeichnet ein Bild mit Fuß bei (x, y), Startframe phase und Abdunklung.
func (pt *painter) sprite(dst *ebiten.Image, it *item, lib *art.Library, c view.Cell, clock float64) {
	s := lib.Sheet(it.look.asset, it.look.anim)
	if s == nil {
		return
	}
	img := s.Frame(it.phase + int(clock*s.FPS))
	if img == nil {
		return
	}
	k := s.UnitsPerPx / unitPx * c.Camera.Scale
	ky := k
	if it.sy > 0 {
		ky *= it.sy
	}
	x0 := c.Camera.ScreenX(it.x) - s.OriginX*float64(s.FW)*k
	y0 := c.ScreenY(it.y) - s.OriginY*float64(s.FH)*ky
	pt.op.GeoM.Reset()
	pt.op.GeoM.Scale(k, ky)
	pt.op.GeoM.Translate(x0, y0)
	pt.op.ColorScale.Reset()
	if it.shade > 0 {
		pt.op.ColorScale.Scale(1-it.shade, 1-it.shade, 1-it.shade, 1)
	}
	if it.alpha > 0 {
		pt.op.ColorScale.ScaleAlpha(it.alpha)
	}
	pt.op.Filter, pt.op.DisableMipmaps = ebiten.FilterLinear, true
	dst.DrawImage(img, &pt.op)
}

// tiles kachelt das Bild von x bis x2 (Hintergrundmauer); die letzte Kachel liegt bündig am rechten Ende.
func (pt *painter) tiles(dst *ebiten.Image, it *item, lib *art.Library, c view.Cell) {
	s := lib.Sheet(it.look.asset, it.look.anim)
	if s == nil || s.Frame(0) == nil || s.FW == 0 {
		return
	}
	tw := float64(s.FW) * s.UnitsPerPx / unitPx
	k := s.UnitsPerPx / unitPx * c.Camera.Scale
	y0 := c.ScreenY(it.y) - s.OriginY*float64(s.FH)*k
	pt.op.ColorScale.Reset()
	pt.op.Filter, pt.op.DisableMipmaps = ebiten.FilterLinear, true
	n := int(math.Ceil((it.x2 - it.x) / tw))
	for i := range n {
		tx := min(it.x+float64(i)*tw, it.x2-tw)
		if !c.Camera.Visible(tx+tw/2, tw/2) {
			continue
		}
		pt.op.GeoM.Reset()
		pt.op.GeoM.Scale(k, k)
		pt.op.GeoM.Translate(c.Camera.ScreenX(tx), y0)
		dst.DrawImage(s.Frame(0), &pt.op)
	}
}

// stamp zeichnet die Vorlage img gestreckt auf das Rechteck (x, y, w, h) in Fenster-Pixeln.
func (pt *painter) stamp(dst, img *ebiten.Image, x, y, w, h float64) {
	b := img.Bounds()
	pt.op.GeoM.Reset()
	pt.op.GeoM.Scale(w/float64(b.Dx()), h/float64(b.Dy()))
	pt.op.GeoM.Translate(x, y)
	pt.op.Filter, pt.op.DisableMipmaps = ebiten.FilterLinear, true
	dst.DrawImage(img, &pt.op)
}

// form zeichnet Rechteck, Rahmen, Kreis, Ring, Linie oder Dreieck.
func (pt *painter) form(dst *ebiten.Image, it *item, c view.Cell) {
	s := c.Camera.Scale
	x, y := c.Camera.ScreenX(it.x), c.ScreenY(it.y)
	w, h, r := it.w*s, it.h*s, it.w*s
	pt.colorScale(it.clr, it.alpha)
	switch it.shape {
	case shapeRect:
		pt.stamp(dst, pt.pixel, x-w/2, y-h, w, h)
	case shapeFrame:
		pt.frame(dst, x-w/2, y-h, w, h, max(1, it.size*s))
	case shapeCircle:
		pt.stamp(dst, pt.disc, x-r, y-r, 2*r, 2*r)
	case shapeRing:
		pt.stamp(dst, pt.ring, x-r, y-r, 2*r, 2*r)
	case shapeTri:
		pt.stamp(dst, pt.tri, x-w/2, y-h, w, h)
	case shapeLine:
		pt.line(dst, x, y, c.Camera.ScreenX(it.x2), c.ScreenY(it.y2), max(1, it.size*s))
	}
}

// frame: vier Kanten der Breite lw innen am Rechteck.
func (pt *painter) frame(dst *ebiten.Image, x, y, w, h, lw float64) {
	pt.stamp(dst, pt.pixel, x, y, w, lw)
	pt.stamp(dst, pt.pixel, x, y+h-lw, w, lw)
	pt.stamp(dst, pt.pixel, x, y, lw, h)
	pt.stamp(dst, pt.pixel, x+w-lw, y, lw, h)
}

// line: gedrehter Bildpunkt von (x0, y0) nach (x1, y1) mit Breite lw.
func (pt *painter) line(dst *ebiten.Image, x0, y0, x1, y1, lw float64) {
	dx, dy := x1-x0, y1-y0
	pt.op.GeoM.Reset()
	pt.op.GeoM.Translate(0, -0.5)
	pt.op.GeoM.Scale(math.Hypot(dx, dy), lw)
	pt.op.GeoM.Rotate(math.Atan2(dy, dx))
	pt.op.GeoM.Translate(x0, y0)
	dst.DrawImage(pt.pixel, &pt.op)
}

// textOutline: Versätze der schwarzen Kontur um den Text (siteView.ts › TEXT: stroke 4).
var textOutline = [4][2]float64{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

// text zeichnet den Text mittig über y (Unterkante), mit schwarzer Kontur; Breiten werden je Text einmal gemessen.
func (pt *painter) text(dst *ebiten.Image, it *item, font *art.Font, c view.Cell) {
	if font == nil {
		return
	}
	size := max(8, it.size*c.Camera.Scale)
	key := widthKey{it.text, size}
	w, ok := pt.widths[key]
	if !ok {
		w, _ = font.Measure(it.text, size)
		pt.widths[key] = w
	}
	x := c.Camera.ScreenX(it.x) - w/2
	y := c.ScreenY(it.y) - size*1.2
	d := max(1, size/16)
	black := color.RGBA{0, 0, 0, 255}
	for _, o := range textOutline {
		font.Draw(dst, it.text, x+o[0]*d, y+o[1]*d, size, black)
	}
	font.Draw(dst, it.text, x, y, size, it.clr)
}
