package hud

// Zeichnen mit Ebitengine: Flächen aus einem weißen Bildpunkt und einer Scheibe (eingefärbt über ColorScale, je Bild
// nichts Neues), Text über art.Font mit schwarzer Kontur. Ohne Schriftdatei zeichnet art.Font die Debug-Schrift
// (feste Größe, weiß); dann steht jeder Text auf einer dunklen Fläche statt mit Kontur. Maße kommen immer aus
// Font.Measure und werden je Element gemerkt (nur neu gemessen, wenn sich Text oder Größe ändern).

import (
	"image"
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"

	"k3c/native/k3c-game/art"
)

// discPx: Kantenlänge der Scheiben-Vorlage.
const discPx = 64

// painter hält Vorlagen, Zeichen-Optionen und die gemessenen Texte je Spieler und Element.
type painter struct {
	pixel, disc, ring *ebiten.Image
	tri               *ebiten.Image
	op                ebiten.DrawImageOptions
	measured          [2][slotCount]measure
	outline           art.OutlineCache // Texte mit Kontur; Sweep je Takt in Listen
}

// measure ist ein gemessener Text.
type measure struct {
	s       string
	size    float64
	w, h    float64
	present bool
}

// textSlot benennt einen gemessenen Text je Spieler.
type textSlot int

const (
	slotGold textSlot = iota
	slotP
	slotClock
	slotFight
	slotShared
	slotPlace
	slotCit
	slotBanner
	slotBoss
	slotEvents
	slotTravel
	slotJoin
	slotSkillBar
	slotHint
	slotGuide
	slotMenuTitle
	slotMenuFooter
	slotMenuRow                                // menuWindow Zeilen ab hier
	slotMenuGlyph = slotMenuRow + menuWindow   // Glyph-Beschriftung je Zeile
	slotLabel     = slotMenuGlyph + menuWindow // Welt-Labels ab hier (höchstens maxLabels)
	slotCount     = slotLabel + maxLabels
)

// maxLabels: höchstens so viele Welt-Labels je Zelle werden gemessen und gezeichnet.
const maxLabels = 24

// stencils legt die Vorlagen einmal an.
func (pt *painter) stencils() {
	if pt.pixel != nil {
		return
	}
	white := ebiten.NewImage(3, 3)
	white.Fill(color.White)
	pt.pixel = white.SubImage(image.Rect(1, 1, 2, 2)).(*ebiten.Image)
	const r = discPx / 2
	pt.disc = ebiten.NewImage(discPx, discPx)
	vector.FillCircle(pt.disc, r, r, r, color.White, true)
	pt.ring = ebiten.NewImage(discPx, discPx)
	vector.StrokeCircle(pt.ring, r, r, r-2, 4, color.White, true)
	pt.tri = ebiten.NewImage(discPx, discPx)
	var path vector.Path
	path.MoveTo(r, 0)
	path.LineTo(discPx, discPx)
	path.LineTo(0, discPx)
	path.Close()
	vector.FillPath(pt.tri, &path, &vector.FillOptions{}, &vector.DrawPathOptions{AntiAlias: true})
}

// measureText misst s in Größe size mit der Schrift f, gemerkt je Spieler und Element.
func (pt *painter) measureText(f *art.Font, player int, slot textSlot, s string, size float64) (w, h float64) {
	if f == nil || s == "" || player < 0 || player >= len(pt.measured) {
		return 0, 0
	}
	m := &pt.measured[player][slot]
	if !m.present || m.s != s || m.size != size {
		m.w, m.h = f.Measure(s, size)
		m.s, m.size, m.present = s, size, true
	}
	return m.w, m.h
}

// stamp zeichnet die Vorlage img gestreckt auf (x, y, w, h) in Farbe clr (vormultipliziert).
func (pt *painter) stamp(dst, img *ebiten.Image, x, y, w, h float64, clr color.RGBA) {
	if w <= 0 || h <= 0 || clr.A == 0 {
		return
	}
	b := img.Bounds()
	pt.op.GeoM.Reset()
	pt.op.GeoM.Scale(w/float64(b.Dx()), h/float64(b.Dy()))
	pt.op.GeoM.Translate(x, y)
	pt.op.ColorScale.Reset()
	pt.op.ColorScale.ScaleWithColor(clr)
	pt.op.Filter = ebiten.FilterLinear
	dst.DrawImage(img, &pt.op)
}

func (pt *painter) fill(dst *ebiten.Image, r rect, clr color.RGBA) {
	pt.stamp(dst, pt.pixel, r.X, r.Y, r.W, r.H, clr)
}

// frame zeichnet vier Kanten der Breite lw innen an r.
func (pt *painter) frame(dst *ebiten.Image, r rect, lw float64, clr color.RGBA) {
	pt.fill(dst, rect{r.X, r.Y, r.W, lw}, clr)
	pt.fill(dst, rect{r.X, r.Y + r.H - lw, r.W, lw}, clr)
	pt.fill(dst, rect{r.X, r.Y, lw, r.H}, clr)
	pt.fill(dst, rect{r.X + r.W - lw, r.Y, lw, r.H}, clr)
}

func (pt *painter) circle(dst *ebiten.Image, x, y, r float64, clr color.RGBA) {
	pt.stamp(dst, pt.disc, x-r, y-r, 2*r, 2*r, clr)
}

func (pt *painter) ringAt(dst *ebiten.Image, x, y, r float64, clr color.RGBA) {
	pt.stamp(dst, pt.ring, x-r, y-r, 2*r, 2*r, clr)
}

// text zeichnet s mit der linken oberen Ecke bei (x, y): echte Schrift mit schwarzer Kontur (über art.OutlineCache,
// ein Zwischenbild je Text), Debug-Schrift ohne.
func (pt *painter) text(dst *ebiten.Image, f *art.Font, s string, x, y, size float64, clr color.RGBA) {
	pt.outline.Draw(dst, f, s, x, y, size, clr, art.Outline8, 1)
}
