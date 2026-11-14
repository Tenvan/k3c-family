package devtools

// Zeichnen des Debug-Overlays (F10): Formen aus debuggeo.go je Zelle, beschnitten auf deren Rechteck, und eine Zeile
// mit der Kamera unten links. Auf der Seite Werte bleiben die Formen stehen, dazu kommt die Werte-Tafel.

import (
	"fmt"
	"image"
	"image/color"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// geoColors sind die Farben je Art.
var geoColors = [geoKinds]color.RGBA{
	geoMonarch: {250, 210, 60, 255}, geoTroop: {90, 200, 250, 255}, geoEnemy: {250, 80, 70, 255},
	geoCoin: {250, 240, 140, 255}, geoSite: {140, 250, 140, 255}, geoChunk: {200, 120, 250, 200},
	geoEdge: {250, 250, 250, 255}, geoCamera: {80, 160, 255, 160},
}

// drawDebug zeichnet die Geometrie der Zelle c.
func (o *overlay) drawDebug(dst *ebiten.Image, font *art.Font, f *view.Frame, c view.Cell) {
	w := f.World(c)
	if w == nil {
		return
	}
	sub, ok := dst.SubImage(c.Rect).(*ebiten.Image)
	if !ok {
		return
	}
	o.shapes = geometry(o.shapes[:0], w, c)
	for _, s := range o.shapes {
		clr := geoColors[s.kind]
		if s.x0 == s.x1 {
			vector.StrokeLine(sub, float32(s.x0), float32(s.y0), float32(s.x1), float32(s.y1), 1, clr, false)
			continue
		}
		vector.StrokeRect(sub, float32(s.x0), float32(s.y0), float32(s.x1-s.x0), float32(s.y1-s.y0), 1.5, clr, false)
	}
	if font == nil {
		return
	}
	l, r := c.Camera.Span()
	px := max(12, float64(c.Rect.Dy())/40)
	text := fmt.Sprintf("Kamera x %.1f · %.0f px/Unit · sichtbar %.1f–%.1f · Welt %.0f Units", c.Camera.X, c.Camera.Scale, l, r, w.WidthUnits)
	tw, th := font.Measure(text, px)
	at := image.Pt(c.Rect.Min.X+int(px), c.Rect.Max.Y-int(th+px))
	box(sub, float64(at.X)-px/2, float64(at.Y)-px/4, tw+px, th+px/2)
	font.Draw(sub, text, float64(at.X), float64(at.Y), px, textColor)
	if f.Dev.Debug == view.DebugValues {
		o.drawValues(sub, font, f, c, px)
	}
}

// drawValues zeichnet die Werte-Tafel oben rechts in die Zelle c (unter der Anzeige oben rechts).
func (o *overlay) drawValues(dst *ebiten.Image, font *art.Font, f *view.Frame, c view.Cell, px float64) {
	o.lines = valueLines(o.lines[:0], f, c)
	text := strings.Join(o.lines, "\n")
	tw, th := font.Measure(text, px)
	x := float64(c.Rect.Max.X) - tw - 2*px
	y := float64(c.Rect.Min.Y) + float64(view.StatsRect(c.Rect).Dy()) + px
	box(dst, x-px/2, y-px/2, tw+px, th+px)
	font.Draw(dst, text, x, y, px, textColor)
}
