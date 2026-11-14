package art

// Text mit schwarzer Kontur über ein Zwischenbild (Review Welle 2b): Die Kontur kostet je Text vier bzw. acht weitere
// text.Draw. OutlineCache rendert Text und Kontur einmal je (Text, Größe, Farbe, Kontur) in ein Bild und zeichnet es
// danach mit einem DrawImage. Lage auf ganze Bildpunkte gerundet (text.Draw setzt Glyphen sonst mit Bruchteilen);
// Ausblenden über die Deckkraft des ganzen Bilds statt je Schicht. Bilder, die Sweep-Durchläufe lang nicht gezeichnet
// wurden, gibt Sweep frei; mehr als Max Bilder legt Draw nicht an (dann direkt wie ohne Zwischenspeicher).

import (
	"image/color"
	"math"

	"github.com/hajimehoshi/ebiten/v2"
)

// Konturen: Versätze in Vielfachen der Konturstärke (Browser: stroke 6 bei 28 px).
var (
	Outline4 = [][2]float64{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	Outline8 = [][2]float64{{-1, 0}, {1, 0}, {0, -1}, {0, 1}, {-1, -1}, {1, -1}, {-1, 1}, {1, 1}}
)

// Standardwerte des Zwischenspeichers.
const (
	outlineMax  = 384 // Bilder höchstens
	outlineKeep = 30  // Sweep-Durchläufe (Takte) ohne Zeichnen, bis ein Bild frei wird
)

var outlineBlack = color.RGBA{0, 0, 0, 255}

// OutlineCache ist der Zwischenspeicher eines Pakets; Null-Wert ist bereit. Nur in der Goroutine der Schleife nutzen.
type OutlineCache struct {
	entries map[outlineKey]*outlineEntry
	gen     uint64
	op      ebiten.DrawImageOptions
}

type outlineKey struct {
	s    string
	size float64
	clr  color.RGBA
	ring int // Zahl der Versätze
}

type outlineEntry struct {
	img  *ebiten.Image
	pad  float64
	used uint64
}

// OutlineStroke ist die Konturstärke bei Größe size (mindestens 1 px).
func OutlineStroke(size float64) float64 { return max(1, size/14) }

// outlinePad ist der Rand des Zwischenbilds um die gemessene Fläche: Kontur und Überhang der Glyphen.
func outlinePad(size float64) float64 { return math.Ceil(OutlineStroke(size)) + math.Ceil(size*0.3) }

// Draw zeichnet s mit linker oberer Ecke bei (x, y), Farbe clr (vormultipliziert), schwarzer Kontur ring
// (Outline4/Outline8) und Deckkraft alpha (0 … 1). Debug-Schrift: ohne Kontur und ohne Zwischenbild.
func (c *OutlineCache) Draw(dst *ebiten.Image, f *Font, s string, x, y, size float64, clr color.RGBA, ring [][2]float64, alpha float64) {
	if f == nil || s == "" || alpha <= 0 {
		return
	}
	if f.Fallback {
		f.Draw(dst, s, x, y, size, clr)
		return
	}
	e := c.entry(f, s, size, clr, ring)
	if e == nil { // Zwischenspeicher voll: direkt
		d := OutlineStroke(size)
		black := scaleAlpha(outlineBlack, alpha)
		for _, o := range ring {
			f.Draw(dst, s, x+o[0]*d, y+o[1]*d, size, black)
		}
		f.Draw(dst, s, x, y, size, scaleAlpha(clr, alpha))
		return
	}
	e.used = c.gen
	c.op.GeoM.Reset()
	c.op.GeoM.Translate(math.Round(x)-e.pad, math.Round(y)-e.pad)
	c.op.ColorScale.Reset()
	c.op.ColorScale.ScaleAlpha(float32(alpha))
	c.op.Filter = ebiten.FilterNearest
	dst.DrawImage(e.img, &c.op)
}

// entry liefert das Zwischenbild oder legt es an (nil, wenn schon Max Bilder bestehen).
func (c *OutlineCache) entry(f *Font, s string, size float64, clr color.RGBA, ring [][2]float64) *outlineEntry {
	k := outlineKey{s, size, clr, len(ring)}
	if e, ok := c.entries[k]; ok {
		return e
	}
	if len(c.entries) >= outlineMax {
		return nil
	}
	if c.entries == nil {
		c.entries = map[outlineKey]*outlineEntry{}
	}
	w, h := f.Measure(s, size)
	pad := outlinePad(size)
	img := ebiten.NewImage(int(math.Ceil(w+2*pad)), int(math.Ceil(h+2*pad)))
	d := OutlineStroke(size)
	for _, o := range ring {
		f.Draw(img, s, pad+o[0]*d, pad+o[1]*d, size, outlineBlack)
	}
	f.Draw(img, s, pad, pad, size, clr)
	e := &outlineEntry{img: img, pad: pad, used: c.gen}
	c.entries[k] = e
	return e
}

// Sweep zählt einen Durchlauf weiter (einmal je Takt, z. B. in Listen) und gibt Bilder frei, die outlineKeep
// Durchläufe nicht gezeichnet wurden.
func (c *OutlineCache) Sweep() {
	c.gen++
	for k, e := range c.entries {
		if c.gen-e.used > outlineKeep {
			if e.img != nil {
				e.img.Deallocate()
			}
			delete(c.entries, k)
		}
	}
}

// Len ist die Zahl der Zwischenbilder.
func (c *OutlineCache) Len() int { return len(c.entries) }

// scaleAlpha skaliert eine vormultiplizierte Farbe mit a.
func scaleAlpha(c color.RGBA, a float64) color.RGBA {
	if a >= 1 {
		return c
	}
	return color.RGBA{uint8(float64(c.R) * a), uint8(float64(c.G) * a), uint8(float64(c.B) * a), uint8(float64(c.A) * a)}
}
