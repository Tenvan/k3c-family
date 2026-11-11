package main

// Zeichnen mit Ebitengine: Teile als Bild mit Matrix, Bänder (Umhang, Beine) als Dreiecksband, Zügel als Linie.

import (
	"image/color"
	"math"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-rig/rig"
)

// Painter zeichnet in ein Ziel; view bildet Welt-Einheiten auf Bildpunkte ab. Die Puffer werden wiederverwendet.
type Painter struct {
	dst   *ebiten.Image
	view  rig.M
	white *ebiten.Image
	verts []ebiten.Vertex
	idx   []uint16
	strip []rig.Vertex
	pts   []rig.Point
	op    ebiten.DrawImageOptions
	tri   ebiten.DrawTrianglesOptions
}

func NewPainter(white *ebiten.Image) *Painter {
	p := &Painter{white: white, view: rig.Identity}
	p.op.Filter = ebiten.FilterLinear
	p.tri.Filter = ebiten.FilterLinear
	return p
}

// image zeichnet ein Teil-Bild: Pixel → Einheiten (1/density, Versatz), dann die Matrix des Teils, dann die Sicht.
func (p *Painter) image(img *ebiten.Image, m rig.M, offset rig.Point, density float64) {
	f := rig.Mul(p.view, rig.Mul(m, rig.Mul(rig.Translate(offset[0], offset[1]), rig.Scale(1/density))))
	p.op.GeoM.SetElement(0, 0, f[0])
	p.op.GeoM.SetElement(1, 0, f[1])
	p.op.GeoM.SetElement(0, 1, f[2])
	p.op.GeoM.SetElement(1, 1, f[3])
	p.op.GeoM.SetElement(0, 2, f[4])
	p.op.GeoM.SetElement(1, 2, f[5])
	p.dst.DrawImage(img, &p.op)
}

// band biegt das Bild eines Bands entlang der Stützpunkte; m ist die Matrix des Rahmens, in dem die Punkte liegen.
func (p *Painter) band(img *ebiten.Image, b *rig.Band, pts []rig.Point, m rig.M, density float64) {
	p.strip = b.Strip(pts, density, p.strip)
	f := rig.Mul(p.view, m)
	p.verts, p.idx = p.verts[:0], p.idx[:0]
	for i, v := range p.strip {
		at := f.Apply(rig.Point{v.X, v.Y})
		p.verts = append(p.verts, ebiten.Vertex{DstX: float32(at[0]), DstY: float32(at[1]), SrcX: float32(v.U), SrcY: float32(v.V),
			ColorR: 1, ColorG: 1, ColorB: 1, ColorA: 1})
		if i >= 2 && i%2 == 0 {
			k := uint16(i)
			p.idx = append(p.idx, k-2, k-1, k, k-1, k+1, k)
		}
	}
	p.tri.AntiAlias = false
	p.dst.DrawTriangles(p.verts, p.idx, img, &p.tri)
}

const curveSteps = 12

// curve zeichnet eine Bézier-Kurve zweiten Grades (Welt-Einheiten) als Band der Breite width in einer Farbe.
func (p *Painter) curve(a, c, b rig.Point, width float64, col color.RGBA, opacity float64) {
	p.pts = p.pts[:0]
	for i := 0; i <= curveSteps; i++ {
		t := float64(i) / curveSteps
		u := 1 - t
		p.pts = append(p.pts, p.view.Apply(rig.Point{u*u*a[0] + 2*u*t*c[0] + t*t*b[0], u*u*a[1] + 2*u*t*c[1] + t*t*b[1]}))
	}
	half := width / 2 * math.Hypot(p.view[0], p.view[1])
	// Ecken-Farben sind nicht vormultipliziert (Standard von DrawTriangles)
	r, g, bl, al := float32(col.R)/255, float32(col.G)/255, float32(col.B)/255, float32(opacity)
	p.verts, p.idx = p.verts[:0], p.idx[:0]
	for i, q := range p.pts {
		prev, next := p.pts[max(i-1, 0)], p.pts[min(i+1, len(p.pts)-1)]
		tx, ty := next[0]-prev[0], next[1]-prev[1]
		d := math.Hypot(tx, ty)
		if d == 0 {
			d = 1
		}
		nx, ny := ty/d, -tx/d
		// vier Ecken je Punkt: außen durchsichtig, innen deckend. Das glättet die Kante ohne Ebitengines AntiAlias,
		// das je Aufruf ein eigenes Zwischenbild braucht und das Bündeln der Zeichenaufrufe bricht (Messung NV2.3).
		for k, off := range [4]float64{-half - 0.5, -math.Max(half-0.5, 0), math.Max(half-0.5, 0), half + 0.5} {
			alpha := al
			if k == 0 || k == 3 {
				alpha = 0
			}
			p.verts = append(p.verts, ebiten.Vertex{DstX: float32(q[0] + off*nx), DstY: float32(q[1] + off*ny), SrcX: 1.5, SrcY: 1.5,
				ColorR: r, ColorG: g, ColorB: bl, ColorA: alpha})
		}
		if i > 0 {
			for c := range uint16(3) {
				k := uint16(4*i) + c
				p.idx = append(p.idx, k-4, k-3, k, k-3, k+1, k)
			}
		}
	}
	p.tri.AntiAlias = false
	p.dst.DrawTriangles(p.verts, p.idx, p.white, &p.tri)
}

// parseColor liest #rrggbb; alles andere ergibt Schwarz.
func parseColor(hex string) color.RGBA {
	var r, g, b uint8
	if len(hex) == 7 && hex[0] == '#' {
		v := [3]uint8{}
		for i := range 3 {
			v[i] = hexByte(hex[1+2*i])<<4 | hexByte(hex[2+2*i])
		}
		r, g, b = v[0], v[1], v[2]
	}
	return color.RGBA{r, g, b, 255}
}

func hexByte(c byte) uint8 {
	switch {
	case c >= '0' && c <= '9':
		return c - '0'
	case c >= 'a' && c <= 'f':
		return c - 'a' + 10
	case c >= 'A' && c <= 'F':
		return c - 'A' + 10
	}
	return 0
}
