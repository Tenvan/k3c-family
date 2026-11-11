package main

// Fluss im Vordergrund (NV3.1, B-789): unteres Drittel jeder Sicht, eine Farbe, sechs Wellen-Reihen hintereinander. Der
// Fluss ist eine Ebene, die auf den Betrachter zuläuft: Jede Reihe weiter vorn ist um denselben Faktor höher, ihre Wellen
// sind breiter und sie fließt schneller nach links; Kamerafahrt verschiebt sie mit demselben Faktor (die Reihe am Ufer
// wie die Welt). Platzhalter-Grafik entsteht beim Start im Code, ein Quell-Image für alle Reihen.

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

const (
	riverRows = 6
	// Zuwachs je Reihe nach vorn (Maßstab der Reihe i = 1 + i·riverGrow)
	riverGrow = 0.3
	// Reihe am Ufer: Wellen-Kachel (Spiel-Pixel) und Tempo nach links (Spiel-Pixel/s)
	riverTile = 150.0
	riverFlow = 28.0
	// Anteil der Welt an der Sicht; der Rest darunter ist Fluss
	worldShare = 2.0 / 3
)

var (
	riverBase = color.RGBA{58, 118, 168, 255}
	riverWave = color.RGBA{84, 142, 188, 255}
)

type River struct {
	tile  *ebiten.Image
	total float64 // Summe der Maßstäbe aller Reihen
	op    ebiten.DrawImageOptions
}

func newRiver() *River {
	src := image.NewRGBA(image.Rect(0, 0, riverTileW, riverTileH))
	paintRow(src)
	r := &River{tile: ebiten.NewImageFromImage(src)}
	for i := range riverRows {
		r.total += 1 + float64(i)*riverGrow
	}
	return r
}

// paintRow malt eine Wellen-Reihe: ein gewelltes Band auf der Grundfarbe, in x periodisch (ganze Wellenzahl) und oben wie
// unten Grundfarbe, damit die Kachel in beide Richtungen nahtlos ist.
func paintRow(dst *image.RGBA) {
	for y := range riverTileH {
		for x := range riverTileW {
			c := riverBase
			line := float64(riverTileH)*0.5 + 7*math.Sin(2*math.Pi*2*float64(x)/riverTileW)
			if d := math.Abs(float64(y) - line); d < 4 {
				c = mix(c, riverWave, 1-d/4)
			}
			dst.SetRGBA(x, y, c)
		}
	}
}

func mix(a, b color.RGBA, t float64) color.RGBA {
	f := func(x, y uint8) uint8 { return uint8(float64(x)*(1-t) + float64(y)*t) }
	return color.RGBA{f(a.R, b.R), f(a.G, b.G), f(a.B, b.B), 255}
}

// draw füllt den Bereich ab Zeile top bis bottom (Bildschirm, absolut) mit den Reihen. s = Bildschirm-Pixel je
// Spiel-Pixel, cam = Kamera-x der Sicht, t = Laufzeit in Sekunden. Keine Allokation: Bild und Optionen sind wiederverwendet.
func (r *River) draw(dst *ebiten.Image, top, bottom, viewW, s, cam, t float64) {
	y := top
	r.op.Filter = ebiten.FilterLinear
	r.op.DisableMipmaps = true
	for i := range riverRows {
		k := 1 + float64(i)*riverGrow
		rowH := (bottom - top) * k / r.total
		tw := riverTile * k * s
		off := math.Mod(-(t*riverFlow+cam)*k*s, tw) // fließt nach links; Kamerafahrt nach rechts schiebt zusätzlich nach links
		if off > 0 {
			off -= tw
		}
		for x := off; x < viewW; x += tw {
			r.op.GeoM.Reset()
			r.op.GeoM.Scale((tw+1)/riverTileW, (rowH+1)/riverTileH) // 1 px Überlappung gegen Nähte
			r.op.GeoM.Translate(x, y)
			dst.DrawImage(r.tile, &r.op)
		}
		y += rowH
	}
}
