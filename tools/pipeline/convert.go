package main

import (
	"image"
	"image/color"
)

// Ausnahmen aus der Liste (docs/assets/stil-bibel.md › Ausnahmen).
const (
	softAlpha  = "alpha-weich" // Alpha bleibt weich (Schatten, Glühen); Farben trotzdem DB32
	keepSource = "quelldichte" // nur Hochskalierung zurücknehmen, nicht auf 16 px verkleinern
	// minSide: Eine erkannte Hochskalierung gilt nur, wenn die kürzere Seite danach noch so viele Pixel hat.
	minSide = 8
)

// options sind die Angaben je Grafik in der Liste.
type options struct {
	Dichte   int    `json:"dichte,omitempty"`   // Raster der Quelle in px (16, 32, 48, 64 …), Standard 16
	Ausnahme string `json:"ausnahme,omitempty"` // leer, softAlpha oder keepSource
	Grund    string `json:"grund,omitempty"`    // Begründung der Ausnahme (nur Doku)
}

// result ist eine umgerechnete Grafik samt erkanntem Maßstab.
type result struct {
	img     image.Image
	upscale int // erkannte Hochskalierung der Quelle (größter gemeinsamer Teiler der Lauflängen)
	factor  int // tatsächlicher Verkleinerungsfaktor
}

// convert rechnet eine Quelle um: Alpha hart, Hochskalierung erkennen, auf 16 px verkleinern, DB32 in OKLab.
func convert(src image.Image, o options) result {
	soft := o.Ausnahme == softAlpha
	img := prepare(src, soft)
	up := upscale(img)
	if min(img.Rect.Dx(), img.Rect.Dy())/up < minSide {
		up = 1 // flache Kachel (z. B. einfarbiger Schatten): Lauflängen sagen nichts über eine Hochskalierung
	}
	f := up
	if o.Ausnahme != keepSource {
		f = up * max(o.Dichte, 16) / 16
	}
	if f > 1 {
		img = downscale(img, f)
	}
	return result{img: quantize(img, soft), upscale: up, factor: f}
}

// prepare kopiert nach NRGBA, macht Alpha hart (≥ 128 → 255, sonst transparent) und setzt Transparent auf 0.
func prepare(src image.Image, soft bool) *image.NRGBA {
	b := src.Bounds()
	img := image.NewNRGBA(image.Rect(0, 0, b.Dx(), b.Dy()))
	for y := 0; y < b.Dy(); y++ {
		for x := 0; x < b.Dx(); x++ {
			c := color.NRGBAModel.Convert(src.At(b.Min.X+x, b.Min.Y+y)).(color.NRGBA)
			switch {
			case c.A == 0 || (!soft && c.A < 128):
				c = color.NRGBA{}
			case !soft:
				c.A = 0xff
			}
			img.SetNRGBA(x, y, c)
		}
	}
	return img
}

// upscale liefert den größten gemeinsamen Teiler aller waagerechten und senkrechten Lauflängen gleicher Farbe.
func upscale(img *image.NRGBA) int {
	w, h := img.Rect.Dx(), img.Rect.Dy()
	g := 0
	run := func(n, start, step int) {
		length := 1
		for i := 1; i < n && g != 1; i++ {
			if pixel(img, start+i*step) == pixel(img, start+(i-1)*step) {
				length++
				continue
			}
			g = gcd(g, length)
			length = 1
		}
		g = gcd(g, length)
	}
	for y := 0; y < h && g != 1; y++ {
		run(w, y*w, 1)
	}
	for x := 0; x < w && g != 1; x++ {
		run(h, x, w)
	}
	return max(g, 1)
}

// pixel liefert das Pixel mit Index i (Zeile für Zeile) als gepackten RGBA-Wert.
func pixel(img *image.NRGBA, i int) uint32 {
	p := img.Pix[i*4 : i*4+4]
	return uint32(p[0])<<24 | uint32(p[1])<<16 | uint32(p[2])<<8 | uint32(p[3])
}

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

// downscale verkleinert um f: je Block f×f die häufigste Farbe (Gleichstand: deckend vor transparent, dann kleinerer Wert).
// Randblöcke dürfen kleiner sein.
func downscale(img *image.NRGBA, f int) *image.NRGBA {
	w, h := img.Rect.Dx(), img.Rect.Dy()
	out := image.NewNRGBA(image.Rect(0, 0, (w+f-1)/f, (h+f-1)/f))
	counts := make(map[uint32]int, f*f)
	for by := 0; by < out.Rect.Dy(); by++ {
		for bx := 0; bx < out.Rect.Dx(); bx++ {
			clear(counts)
			for y := by * f; y < min(by*f+f, h); y++ {
				for x := bx * f; x < min(bx*f+f, w); x++ {
					counts[pixel(img, y*w+x)]++
				}
			}
			v := mode(counts)
			out.SetNRGBA(bx, by, color.NRGBA{R: uint8(v >> 24), G: uint8(v >> 16), B: uint8(v >> 8), A: uint8(v)})
		}
	}
	return out
}

// mode wählt die häufigste Farbe mit fester Ordnung, damit das Ergebnis nicht von der Map-Reihenfolge abhängt.
func mode(counts map[uint32]int) uint32 {
	var best uint32
	bestN := -1
	for v, n := range counts {
		better := n > bestN
		if n == bestN {
			opaque, bestOpaque := v&0xff != 0, best&0xff != 0
			better = (opaque && !bestOpaque) || (opaque == bestOpaque && v < best)
		}
		if better {
			best, bestN = v, n
		}
	}
	return best
}

// quantize bildet jede deckende Farbe auf DB32 ab. Mit hartem Alpha entsteht ein Palettenbild (Index 0 = transparent),
// mit weichem Alpha ein NRGBA-Bild mit DB32-Farben und dem Alpha der Quelle.
func quantize(img *image.NRGBA, soft bool) image.Image {
	cache := map[uint32]int{}
	index := func(c color.NRGBA) int {
		k := uint32(c.R)<<16 | uint32(c.G)<<8 | uint32(c.B)
		i, ok := cache[k]
		if !ok {
			i = nearest(c.R, c.G, c.B)
			cache[k] = i
		}
		return i
	}
	b := img.Rect
	if soft {
		out := image.NewNRGBA(b)
		for y := 0; y < b.Dy(); y++ {
			for x := 0; x < b.Dx(); x++ {
				if c := img.NRGBAAt(x, y); c.A != 0 {
					v := db32[index(c)]
					out.SetNRGBA(x, y, color.NRGBA{R: uint8(v >> 16), G: uint8(v >> 8), B: uint8(v), A: c.A})
				}
			}
		}
		return out
	}
	out := image.NewPaletted(b, palette())
	for y := 0; y < b.Dy(); y++ {
		for x := 0; x < b.Dx(); x++ {
			if c := img.NRGBAAt(x, y); c.A != 0 {
				out.SetColorIndex(x, y, uint8(index(c)+1))
			}
		}
	}
	return out
}
