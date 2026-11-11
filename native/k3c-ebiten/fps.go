package main

// Große FPS-Anzeige oben links (NV3): Die kleine Debug-Schrift wird in ein eigenes Bild geschrieben und vergrößert
// gezeichnet. Der Text wird nur viermal je Sekunde neu gesetzt, dazwischen wird nichts angelegt.

import (
	"fmt"
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"
)

const (
	fpsScale    = 6.0  // Vergrößerung der Debug-Schrift
	fpsInterval = 0.25 // Sekunden zwischen zwei Aktualisierungen
)

var fpsBack = color.RGBA{0, 0, 0, 170}

type fpsHUD struct {
	img  *ebiten.Image
	next float64
	op   ebiten.DrawImageOptions
}

func newFpsHUD() *fpsHUD {
	h := &fpsHUD{img: ebiten.NewImage(64, 18)}
	h.op.GeoM.Scale(fpsScale, fpsScale)
	h.op.GeoM.Translate(16, 16)
	return h
}

func (h *fpsHUD) draw(screen *ebiten.Image, elapsed float64) {
	if elapsed >= h.next {
		h.next = elapsed + fpsInterval
		h.img.Fill(fpsBack)
		ebitenutil.DebugPrintAt(h.img, fmt.Sprintf("%4.0f FPS", ebiten.ActualFPS()), 4, 1)
	}
	screen.DrawImage(h.img, &h.op)
}
