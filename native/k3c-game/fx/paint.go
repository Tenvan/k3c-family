package fx

// Zeichnen mit Ebitengine: Scheibe, Ring, Dreieck und Bildpunkt als weiße Vorlagen, eingefärbt über ColorScale (je
// Bild entsteht nichts Neues). Orte in Welt-px des Browsers (32 px je Unit) werden mit dem Maßstab der Kamera
// umgerechnet, Texte erscheinen in fester Größe (Fensterhöhe / 1080).

import (
	"image"
	"image/color"
	"math"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

const stencilPx = 64

// painter hält Vorlagen, Optionen und gemessene Texte (begrenzt; Schadenszahlen wiederholen sich).
type painter struct {
	pixel, disc, ring, tri *ebiten.Image
	op                     ebiten.DrawImageOptions
	widths                 map[widthKey][2]float64
	outline                art.OutlineCache // aufsteigende Texte mit Kontur; Sweep je Takt in Listen
}

type widthKey struct {
	s    string
	size float64
}

// maxWidths: so viele gemessene Texte bleiben gemerkt, danach beginnt die Liste neu.
const maxWidths = 256

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
	vector.StrokeCircle(pt.ring, r, r, r-3, 6, color.White, true)
	pt.tri = ebiten.NewImage(stencilPx, stencilPx)
	var path vector.Path
	path.MoveTo(r, 0)
	path.LineTo(stencilPx, stencilPx)
	path.LineTo(0, stencilPx)
	path.Close()
	vector.FillPath(pt.tri, &path, &vector.FillOptions{}, &vector.DrawPathOptions{AntiAlias: true})
	pt.widths = map[widthKey][2]float64{}
}

// rgba ist 0xRRGGBB mit Deckkraft a (0..1), vormultipliziert.
func rgba(hex uint32, a float64) color.RGBA {
	a = math.Max(0, math.Min(1, a))
	return color.RGBA{uint8(float64(hex>>16&0xff) * a), uint8(float64(hex>>8&0xff) * a), uint8(float64(hex&0xff) * a), uint8(255 * a)}
}

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

func (pt *painter) circle(dst *ebiten.Image, x, y, r float64, clr color.RGBA) {
	pt.stamp(dst, pt.disc, x-r, y-r, 2*r, 2*r, clr)
}

func (pt *painter) ringAt(dst *ebiten.Image, x, y, r float64, clr color.RGBA) {
	pt.stamp(dst, pt.ring, x-r, y-r, 2*r, 2*r, clr)
}

// line: gedrehter Bildpunkt von (x0, y0) nach (x1, y1) mit Breite lw.
func (pt *painter) line(dst *ebiten.Image, x0, y0, x1, y1, lw float64, clr color.RGBA) {
	dx, dy := x1-x0, y1-y0
	pt.op.GeoM.Reset()
	pt.op.GeoM.Translate(0, -0.5)
	pt.op.GeoM.Scale(math.Hypot(dx, dy), lw)
	pt.op.GeoM.Rotate(math.Atan2(dy, dx))
	pt.op.GeoM.Translate(x0, y0)
	pt.op.ColorScale.Reset()
	pt.op.ColorScale.ScaleWithColor(clr)
	dst.DrawImage(pt.pixel, &pt.op)
}

// at ist der Fenster-Punkt bei Welt-x x (Units) und Höhe y (Welt-px über dem Boden); s rechnet Welt-px in Fenster-px.
func at(c view.Cell, x, y float64) (sx, sy, s float64) {
	s = c.Camera.Scale / unitPx
	return c.Camera.ScreenX(x), c.ScreenY(y / unitPx), s
}

// effects zeichnet Ring und Partikel bzw. aufsteigenden Text jedes Effekts zur Zeit now.
func (pt *painter) effects(dst *ebiten.Image, list []effect, f *art.Font, c view.Cell, now, k float64) {
	for i := range list {
		e := &list[i]
		if !c.Camera.Visible(e.x, 4) {
			continue
		}
		p := (now - e.start) / e.look.duration
		if e.text != "" {
			pt.rising(dst, e, f, c, p, k)
			continue
		}
		x, y, s := at(c, e.x, e.y)
		for j := range 1 + e.look.particles {
			q := pieceAt(e.look, j, p)
			pt.circle(dst, x+q.dx*s, y+q.dy*s, q.radius*s, rgba(e.look.color, q.alpha))
		}
	}
}

// rising zeichnet den Text eines Effekts mittig, steigt auf und blendet aus (noTarget 24 px, Schadenszahl 28 px).
func (pt *painter) rising(dst *ebiten.Image, e *effect, f *art.Font, c view.Cell, p, k float64) {
	if f == nil {
		return
	}
	size, rise, clr := 24*k, 24.0, uint32(0xffffff)
	if e.kind == kindDamage {
		size, rise, clr = 28*k, 40, 0xfbf236
		if e.target {
			clr = 0xff4d4d
		}
	}
	w, h := pt.measure(f, e.text, size)
	x, y, s := at(c, e.x, e.y)
	p = math.Min(1, math.Max(0, p))
	tx, ty := x-w/2, y-h/2-rise*s*p
	pt.outline.Draw(dst, f, e.text, tx, ty, size, rgba(clr, 1), art.Outline4, 1-p) // ausblenden: ganzes Bild
}

// measure misst einen Text, gemerkt je Text und Größe.
func (pt *painter) measure(f *art.Font, s string, size float64) (w, h float64) {
	key := widthKey{s, size}
	if m, ok := pt.widths[key]; ok {
		return m[0], m[1]
	}
	if len(pt.widths) >= maxWidths {
		clear(pt.widths)
	}
	w, h = f.Measure(s, size)
	pt.widths[key] = [2]float64{w, h}
	return w, h
}

// warn zeichnet die Warnkreise vor dem Flächenschlag um den Boden (kampfView.ts › drawWarn).
func (pt *painter) warn(dst *ebiten.Image, w *sim.World, c view.Cell) {
	for _, e := range w.Enemies {
		if e == nil || e.Warn == nil || !c.Camera.Visible(e.Warn.X, e.Warn.R) {
			continue
		}
		x, y, s := at(c, e.Warn.X, 0)
		r, u := e.Warn.R*unitPx*s, urgency(e.Warn.In)
		pt.circle(dst, x, y, r, rgba(0xe63946, 0.15+0.35*u))
		pt.ringAt(dst, x, y, r, rgba(0xe63946, 0.6+0.4*u))
	}
}

// projectiles: Pfeile und Geschosse als kurze helle Striche (Platzhalter wie NG1).
func (pt *painter) projectiles(dst *ebiten.Image, w *sim.World, c view.Cell) {
	k := c.Camera
	for _, p := range w.Projectiles {
		if p != nil && k.Visible(p.X, 0.3) {
			pt.stamp(dst, pt.pixel, k.ScreenX(p.X-0.3), c.ScreenY(1.2), 0.6*k.Scale, 0.1*k.Scale, rgba(0xfafaf0, 1))
		}
	}
}
