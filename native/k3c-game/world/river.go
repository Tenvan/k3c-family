package world

// Fluss im unteren Drittel jeder Zelle (B-795/AC-02, B-789; aus native/k3c-ebiten/river.go übernommen): unter dem
// Uferstreifen sechs Wellen-Reihen in einer Farbe. Der Fluss ist eine Ebene, die auf den Betrachter zuläuft: Jede
// Reihe weiter vorn ist um denselben Faktor höher, ihre Wellen sind breiter, sie fließt schneller nach links, und
// eine Kamerafahrt verschiebt sie mit demselben Faktor (die Reihe am Ufer wie die Welt). Die Wellen-Kachel entsteht
// beim ersten Zeichnen im Code, ein Bild für alle Reihen.

import (
	"image"
	"image/color"
	"math"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

const (
	riverRows = 6
	// riverGrow ist der Zuwachs je Reihe nach vorn (Maßstab der Reihe i = 1 + i·riverGrow).
	riverGrow = 0.3
	// riverTileUnits ist die Breite einer Wellen-Kachel der Reihe am Ufer in Units (150 px wie NV3).
	riverTileUnits = 150.0 / 32
	// riverFlowUnits ist das Tempo der Reihe am Ufer nach links in Units je Sekunde (28 px/s wie NV3).
	riverFlowUnits = 28.0 / 32
	// Größe der Wellen-Kachel im Quell-Bild (Bildpunkte).
	riverSrcW, riverSrcH = 256, 64
)

var (
	riverBase = color.RGBA{58, 118, 168, 255}
	riverWave = color.RGBA{84, 142, 188, 255}
)

// riverRow ist die Lage einer Wellen-Reihe in Fenster-px: oben y, Höhe h, Kachelbreite tile, erste Kachel bei x0
// (≤ linker Rand der Zelle).
type riverRow struct{ y, h, tile, x0 float64 }

// riverTotal ist die Summe der Maßstäbe aller Reihen.
var riverTotal = func() float64 {
	t := 0.0
	for i := range riverRows {
		t += 1 + float64(i)*riverGrow
	}
	return t
}()

// riverLayout teilt den Fluss der Zelle (unter dem Uferstreifen bis zum unteren Rand) in die sechs Reihen und legt
// ihre Kacheln zur Zeit t (Sekunden) fest. Die Kachel-Lage hängt an der Welt: Welt-x (t·Tempo + Kamera-x) liegt in
// jeder Reihe auf einer Kachelkante, die Reihe i bewegt sich mit Maßstab 1 + i·riverGrow.
func riverLayout(c view.Cell, t float64) [riverRows]riverRow {
	var rows [riverRows]riverRow
	k := c.Camera
	top := float64(c.GroundY()) + bankHeight(c)
	height := float64(c.Rect.Max.Y) - top
	y := top
	for i := range rows {
		g := 1 + float64(i)*riverGrow
		h := height * g / riverTotal
		tile := riverTileUnits * g * k.Scale
		x0 := float64(c.Rect.Min.X)
		if tile > 0 {
			// Kachelkante liegt an der Zellenmitte minus Weg; nach links fließend, Kamerafahrt nach rechts schiebt mit.
			off := math.Mod(k.Center-float64(c.Rect.Min.X)-(t*riverFlowUnits+k.X)*g*k.Scale, tile)
			if off > 0 {
				off -= tile
			}
			x0 += off
		}
		rows[i] = riverRow{y: y, h: h, tile: tile, x0: x0}
		y += h
	}
	return rows
}

// riverTile ist das Quell-Bild der Wellen (lazy).
type riverTile struct{ img *ebiten.Image }

// image liefert das Wellen-Bild, beim ersten Aufruf gemalt.
func (r *riverTile) image() *ebiten.Image {
	if r.img == nil {
		src := image.NewRGBA(image.Rect(0, 0, riverSrcW, riverSrcH))
		paintWaves(src)
		r.img = ebiten.NewImageFromImage(src)
	}
	return r.img
}

// paintWaves malt eine Wellen-Reihe: ein gewelltes Band auf der Grundfarbe, in x periodisch (ganze Wellenzahl) und
// oben wie unten Grundfarbe, damit die Kachel in beide Richtungen nahtlos ist.
func paintWaves(dst *image.RGBA) {
	for y := range riverSrcH {
		for x := range riverSrcW {
			c := riverBase
			line := float64(riverSrcH)*0.5 + 7*math.Sin(2*math.Pi*2*float64(x)/riverSrcW)
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

// drawRiver zeichnet die Wellen-Reihen der Zelle zur Zeit Frame.Clock.
func (w *World) drawRiver(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	if stageOf(f, c) == nil {
		return
	}
	img := w.river.image()
	right := float64(c.Rect.Max.X)
	w.op.ColorScale.Reset()
	for _, row := range riverLayout(c, f.Clock) {
		if row.tile <= 0 || row.h <= 0 {
			continue
		}
		for x := row.x0; x < right; x += row.tile {
			w.op.GeoM.Reset()
			w.op.GeoM.Scale((row.tile+1)/riverSrcW, (row.h+1)/riverSrcH) // 1 px Überlappung gegen Nähte
			w.op.GeoM.Translate(x, row.y)
			dst.DrawImage(img, &w.op)
		}
	}
}
