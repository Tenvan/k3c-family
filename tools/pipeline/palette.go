package main

import (
	"image/color"
	"math"
)

// db32 ist die DawnBringer-32-Palette in der Reihenfolge von Lospec (wie tools/buildings/db32.go).
var db32 = [32]uint32{
	0x000000, 0x222034, 0x45283c, 0x663931, 0x8f563b, 0xdf7126, 0xd9a066, 0xeec39a,
	0xfbf236, 0x99e550, 0x6abe30, 0x37946e, 0x4b692f, 0x524b24, 0x323c39, 0x3f3f74,
	0x306082, 0x5b6ee1, 0x639bff, 0x5fcde4, 0xcbdbfc, 0xffffff, 0x9badb7, 0x847e87,
	0x696a6a, 0x595652, 0x76428a, 0xac3232, 0xd95763, 0xd77bba, 0x8f974a, 0x8a6f30,
}

// lab ist eine Farbe im OKLab-Farbraum (Björn Ottosson, 2020).
type lab struct{ l, a, b float64 }

var (
	linear  [256]float64 // sRGB-Kanal → linear
	db32Lab [len(db32)]lab
)

func init() {
	for i := range linear {
		c := float64(i) / 255
		if c <= 0.04045 {
			linear[i] = c / 12.92
		} else {
			linear[i] = math.Pow((c+0.055)/1.055, 2.4)
		}
	}
	for i, c := range db32 {
		db32Lab[i] = toLab(uint8(c>>16), uint8(c>>8), uint8(c))
	}
}

// toLab rechnet sRGB nach OKLab. Die float64-Umwandlungen verhindern zusammengefasste Rechenschritte (FMA),
// damit jede Plattform dieselbe Zuordnung liefert.
func toLab(r, g, b uint8) lab {
	lr, lg, lb := linear[r], linear[g], linear[b]
	l := math.Cbrt(float64(0.4122214708*lr) + float64(0.5363325363*lg) + float64(0.0514459929*lb))
	m := math.Cbrt(float64(0.2119034982*lr) + float64(0.6806995451*lg) + float64(0.1073969566*lb))
	s := math.Cbrt(float64(0.0883024619*lr) + float64(0.2817188376*lg) + float64(0.6299787005*lb))
	return lab{
		l: float64(0.2104542553*l) + float64(0.7936177850*m) - float64(0.0040720468*s),
		a: float64(1.9779984951*l) - float64(2.4285922050*m) + float64(0.4505937099*s),
		b: float64(0.0259040371*l) + float64(0.7827717662*m) - float64(0.8086757660*s),
	}
}

// nearest liefert den DB32-Index mit dem kleinsten Abstand in OKLab; bei Gleichstand gewinnt der kleinere Index.
func nearest(r, g, b uint8) int {
	p := toLab(r, g, b)
	best, bestD := 0, math.Inf(1)
	for i, q := range db32Lab {
		dl, da, db := p.l-q.l, p.a-q.a, p.b-q.b
		if d := float64(dl*dl) + float64(da*da) + float64(db*db); d < bestD {
			best, bestD = i, d
		}
	}
	return best
}

// palette liefert transparent (Index 0) + DB32 (Index 1–32) für image.Paletted.
func palette() color.Palette {
	p := color.Palette{color.NRGBA{}}
	for _, c := range db32 {
		p = append(p, color.NRGBA{R: uint8(c >> 16), G: uint8(c >> 8), B: uint8(c), A: 0xff})
	}
	return p
}

// inDB32 meldet, ob die Farbe (ohne Alpha) zur Ziel-Palette gehört.
func inDB32(r, g, b uint8) bool {
	v := uint32(r)<<16 | uint32(g)<<8 | uint32(b)
	for _, c := range db32 {
		if c == v {
			return true
		}
	}
	return false
}
