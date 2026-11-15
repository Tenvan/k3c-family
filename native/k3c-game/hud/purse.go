package hud

// Geldsack der Spielerkarte (B-919, B-922): ein Sack in drei Größen nach sim.PurseSize (1 bis 3); die Fläche ist immer
// doppelt so groß wie ein Ressourcen-Icon, der Sack füllt sie je nach Größe. Das Bild ist das Papierschnitt-Icon
// icons/geldsack (B-940), bodenbündig und waagerecht mittig in der Fläche; ohne Grafik (Tests) bleibt nur der Blitz-Ring.

import "github.com/hajimehoshi/ebiten/v2"

// purseIconFactor: Kante des Geldsack-Icons als Vielfaches der Kante eines Ressourcen-Icons.
const purseIconFactor = 2.0

// purseFill ist der Anteil der Fläche, den der Sack je Größe 1 bis 3 füllt.
var purseFill = [3]float64{0.62, 0.8, 1}

// purseSide ist die Kante der Geldsack-Fläche bei der Schriftgröße sharedPx der Vorrats-Anzeige.
func purseSide(sharedPx float64) float64 { return purseIconFactor * iconScale * sharedPx }

// purseScale ist der Füllanteil der Größe size (außerhalb 1 bis 3 begrenzt).
func purseScale(size int) float64 { return purseFill[min(max(size, 1), len(purseFill))-1] }

// purseAsset ist das Papierschnitt-Icon des Geldsacks.
const purseAsset = "icons/geldsack"

// drawPurse zeichnet den Sack der Größe size in die quadratische Fläche r; flash legt einen hellen Ring darum.
func (h *hud) drawPurse(dst *ebiten.Image, r rect, size int, flash bool, k float64) {
	s := r.W * purseScale(size)
	cx, base := r.X+r.W/2, r.Y+r.H
	if ic := h.icon(purseAsset); ic != nil {
		drawIcon(dst, ic, cx-s/2, base-s, s)
	}
	if flash {
		h.paint.ringAt(dst, cx, base-s*0.4, s*0.36+4*k, rgb(0xffffff, 1))
	}
}
