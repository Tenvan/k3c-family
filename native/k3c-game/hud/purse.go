package hud

// Geldsack der Spielerkarte (B-919, B-922): ein Sack in drei Größen nach sim.PurseSize (1 bis 3); die Fläche ist immer
// doppelt so groß wie ein Ressourcen-Icon, der Sack füllt sie je nach Größe. Es gibt noch kein Papierschnitt-Icon
// dafür; bis ein Asset über assets_* vorliegt, zeichnet das HUD den Sack aus Kreisen und Flächen (vorläufig).

import "github.com/hajimehoshi/ebiten/v2"

// purseIconFactor: Kante des Geldsack-Icons als Vielfaches der Kante eines Ressourcen-Icons.
const purseIconFactor = 2.0

// purseFill ist der Anteil der Fläche, den der Sack je Größe 1 bis 3 füllt.
var purseFill = [3]float64{0.62, 0.8, 1}

// purseSide ist die Kante der Geldsack-Fläche bei der Schriftgröße sharedPx der Vorrats-Anzeige.
func purseSide(sharedPx float64) float64 { return purseIconFactor * iconScale * sharedPx }

// purseScale ist der Füllanteil der Größe size (außerhalb 1 bis 3 begrenzt).
func purseScale(size int) float64 { return purseFill[min(max(size, 1), len(purseFill))-1] }

// drawPurse zeichnet den Sack der Größe size in die quadratische Fläche r; flash legt einen hellen Ring darum.
func (h *hud) drawPurse(dst *ebiten.Image, r rect, size int, flash bool, k float64) {
	pt, s := &h.paint, r.W*purseScale(size)
	cx, base := r.X+r.W/2, r.Y+r.H
	body := s * 0.36
	cy := base - body - s*0.04
	pt.circle(dst, cx, cy, body, rgb(0xc9a66b, 1))
	pt.circle(dst, cx-body*0.3, cy-body*0.25, body*0.4, rgb(0xdcbd86, 1))
	neck := rect{cx - s*0.12, cy - body - s*0.1, s * 0.24, s * 0.2}
	pt.fill(dst, neck, rgb(0xc9a66b, 1))
	pt.fill(dst, rect{cx - s*0.18, neck.Y + neck.H*0.45, s * 0.36, s * 0.07}, rgb(0x9c6644, 1))
	pt.fill(dst, rect{cx - s*0.22, neck.Y - s*0.06, s * 0.44, s * 0.08}, rgb(0xc9a66b, 1))
	if flash {
		pt.ringAt(dst, cx, cy, body+4*k, rgb(0xffffff, 1))
	}
}
