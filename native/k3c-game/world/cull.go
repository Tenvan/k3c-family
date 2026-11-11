package world

// Culling (B-795/AC-01, Vorbild src/scenes/render/cameraCull.ts): Gezeichnet wird nur, was in den Ausschnitt einer
// Kamera ragt. Der Ausschnitt einer Ebene mit Scroll-Faktor factor ist um Kamera-x · factor verschoben (Parallaxe);
// Boden-Ebene = Faktor 1. Reine Rechnung auf view.Camera, für alle Ebenen der Welt.

import (
	"math"

	"k3c/native/k3c-game/view"
)

// CullMargin ist der Rand in Units links und rechts vom Ausschnitt (400 px im Browser): breite Objekte (Burg,
// Gerüste) verschwinden nicht, solange sie noch hineinragen können.
const CullMargin = 400.0 / 32

// Span liefert den sichtbaren Bereich (Units) einer Ebene mit Scroll-Faktor factor: links, rechts. Ohne Maßstab ist
// er leer (links > rechts).
func Span(k view.Camera, factor float64) (left, right float64) {
	h := halfUnits(k)
	if h <= 0 {
		return 1, 0
	}
	mid := k.X * factor
	return mid - h, mid + h
}

// Visible meldet, ob ein Objekt auf der Boden-Ebene bei x mit Rand margin (Units) in den Ausschnitt ragt.
func Visible(k view.Camera, x, margin float64) bool {
	l, r := Span(k, 1)
	return l <= r && x >= l-margin && x <= r+margin
}

// CullMask liefert für ein Objekt bei x die Kameras (Bit i = cams[i]), in deren Ausschnitt samt Rand margin es nicht
// liegt; je Kamera getrennt, ohne Kamera 0.
func CullMask(x float64, cams []view.Camera, margin float64) uint {
	var mask uint
	for i, k := range cams {
		if !Visible(k, x, margin) {
			mask |= 1 << i
		}
	}
	return mask
}

// tileRange liefert die Kacheln (Index k, Kachel k liegt bei k·tile … (k+1)·tile) einer Ebene mit Scroll-Faktor
// factor, die in den Ausschnitt ragen: erste und letzte; leer = first > last.
func tileRange(k view.Camera, factor, tile float64) (first, last int) {
	l, r := Span(k, factor)
	if l > r || tile <= 0 {
		return 0, -1
	}
	return int(math.Floor(l / tile)), int(math.Floor(r / tile))
}
