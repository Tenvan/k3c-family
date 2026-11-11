package main

// Fluss im Vordergrund (NV3.1, B-789): untere Hälfte jeder Sicht, drei Bahnen hintereinander (hinten schmal und dunkel,
// vorn breit und hell). Jede Bahn besteht aus waagerecht durchlaufenden Kacheln mit eigenem Tempo; sie laufen auch bei
// stehender Kamera, Kamerafahrt verschiebt sie zusätzlich (vorn stärker = Parallaxe). Platzhalter-Grafik entsteht beim
// Start im Code (Papierwellen in drei Blautönen), ein Quell-Image für alle Bahnen.

import (
	"image"
	"image/color"
	"math"

	"github.com/hajimehoshi/ebiten/v2"
)

const (
	riverTileW = 256 // Breite einer Kachel im Quell-Image (Bildpunkte)
	riverTileH = 64  // Höhe je Bahn im Quell-Image
	// Uferstreifen unter der Boden-Linie vor dem Fluss (Spiel-Pixel)
	bankH = 20.0
	// Sichtbare Welthöhe über der Boden-Linie in der oberen Hälfte der Sicht (Spiel-Pixel)
	worldViewH = 420.0
)

// riverLane beschreibt eine Bahn: Anteil an der Flusshöhe, Kachelbreite (Spiel-Pixel), Tempo (Spiel-Pixel/s) und
// Kamera-Parallaxe (Anteil der Kamerafahrt, mit dem die Kacheln mitwandern).
var riverLanes = [3]struct {
	frac, tile, flow, parallax float64
	base, wave                 color.RGBA
}{
	{0.26, 200, 18, 0.15, color.RGBA{38, 82, 128, 255}, color.RGBA{64, 112, 160, 255}},
	{0.33, 260, 42, 0.35, color.RGBA{58, 118, 168, 255}, color.RGBA{104, 160, 204, 255}},
	{0.41, 330, 80, 0.60, color.RGBA{92, 156, 204, 255}, color.RGBA{160, 208, 236, 255}},
}

type River struct {
	lanes [3]*ebiten.Image // SubImages eines gemeinsamen Quell-Images, einmal beim Start angelegt
	op    ebiten.DrawImageOptions
}

func newRiver() *River {
	src := image.NewRGBA(image.Rect(0, 0, riverTileW, riverTileH*len(riverLanes)))
	for i, l := range riverLanes {
		paintLane(src, i*riverTileH, l.base, l.wave, 2+i)
	}
	img := ebiten.NewImageFromImage(src)
	r := &River{}
	for i := range r.lanes {
		r.lanes[i] = img.SubImage(image.Rect(0, i*riverTileH, riverTileW, (i+1)*riverTileH)).(*ebiten.Image)
	}
	return r
}

// paintLane malt Papierwellen: gewellte helle Bänder auf Grundfarbe, in x periodisch (ganze Wellenzahl), damit die Kachel nahtlos ist.
func paintLane(dst *image.RGBA, y0 int, base, wave color.RGBA, waves int) {
	for y := range riverTileH {
		for x := range riverTileW {
			c := base
			for band := range 2 {
				phase := float64(band) * 1.7
				line := float64(riverTileH)*(0.28+0.44*float64(band)) +
					5*math.Sin(2*math.Pi*float64(waves)*float64(x)/riverTileW+phase)
				d := math.Abs(float64(y) - line)
				if d < 3 {
					t := 1 - d/3
					c = mix(c, wave, t)
				} else if float64(y) > line && float64(y) < line+7 {
					c = shade(c, 0.88) // dunkle Papierkante unter dem Band
				}
			}
			dst.SetRGBA(x, y0+y, c)
		}
	}
}

func mix(a, b color.RGBA, t float64) color.RGBA {
	f := func(x, y uint8) uint8 { return uint8(float64(x)*(1-t) + float64(y)*t) }
	return color.RGBA{f(a.R, b.R), f(a.G, b.G), f(a.B, b.B), 255}
}

func shade(c color.RGBA, k float64) color.RGBA {
	return color.RGBA{uint8(float64(c.R) * k), uint8(float64(c.G) * k), uint8(float64(c.B) * k), 255}
}

// draw füllt den Bereich ab Zeile top bis bottom (Bildschirm, absolut) mit den drei Bahnen. s = Bildschirm-Pixel je
// Spiel-Pixel, cam = Kamera-x der Sicht, t = Laufzeit in Sekunden. Keine Allokation: SubImages und Optionen sind wiederverwendet.
func (r *River) draw(dst *ebiten.Image, top, bottom, viewW, s, cam, t float64) {
	h := bottom - top
	y := top
	for i, l := range riverLanes {
		laneH := h * l.frac
		tw := l.tile * s
		off := math.Mod((t*l.flow-cam*l.parallax)*s, tw) // Kacheln fließen nach rechts, Kamerafahrt nach rechts schiebt sie zurück
		if off < 0 {
			off += tw
		}
		for x := off - tw; x < viewW; x += tw {
			r.op.GeoM.Reset()
			r.op.GeoM.Scale((tw+1)/riverTileW, laneH/riverTileH) // 1 px Überlappung gegen Nähte
			r.op.GeoM.Translate(x, y)
			r.op.Filter = ebiten.FilterLinear
			r.op.DisableMipmaps = true
			dst.DrawImage(r.lanes[i], &r.op)
		}
		y += laneH
	}
}
