package menu

import (
	"image"
	"image/color"
	"math"
)

// Lage und Farben der Menü-Seiten als reine Rechnung (ohne ebiten.Image), Vorbild src/scenes/pauseHub.ts und
// paperCard.ts: Karten aus Papier mit Schlagschatten auf Nachtblau, Beschriftung auf einem Leinen-Schild. Alle Maße
// folgen der Fensterhöhe, damit 720p und 1080p gleich aussehen.

// maxRows ist die Höchstzahl sichtbarer Zeilen einer Seite (Lobby: 7; längere Listen rollen).
const maxRows = 7

// material sind die drei Töne eines Papiers (wie MATERIAL in paperCard.ts).
type material struct{ licht, basis, schatten color.RGBA }

func rgb(v uint32) color.RGBA { return color.RGBA{uint8(v >> 16), uint8(v >> 8), uint8(v), 255} }

var (
	matGold   = material{rgb(0xffe36e), rgb(0xf2c33a), rgb(0xc48a1e)}
	matGreen  = material{rgb(0x5fd18a), rgb(0x2f9a5c), rgb(0x1d6640)}
	matBlue   = material{rgb(0x8796f2), rgb(0x4f5fd0), rgb(0x33368f)}
	matRed    = material{rgb(0xe2574c), rgb(0xb8312f), rgb(0x7c1f2a)}
	matPurple = material{rgb(0xa865b8), rgb(0x7c3a8a), rgb(0x5a2466)}
	matCoral  = material{rgb(0xe8a070), rgb(0xc0703c), rgb(0x8a4a24)}
	matCloth  = material{rgb(0xaaa4b2), rgb(0x857f8e), rgb(0x625c6b)}
	matLinen  = material{rgb(0xf3ead4), rgb(0xe0d2b0), rgb(0xbba98a)}

	colNight   = rgb(0x0b1026)
	colDim     = color.RGBA{0, 0, 0, 160} // über dem stehenden Spiel (Pause), vormultipliziert
	colLine    = rgb(0x2a1d2e)            // Schrift auf Leinen
	colLineOff = rgb(0x7a7060)            // Schrift ausgegraut
	colTitle   = rgb(0xffe36e)
	colText    = rgb(0xf3ead4)
	colBad     = rgb(0xff7a6e)
	colPicked  = rgb(0xfff4c2) // Leinen-Schild der gewählten Karte, goldhell
)

// shadowLayers ist der weiche Schlagschatten (wie SHADOW_LAYERS): Versatz in px bei 1080p und Deckkraft.
var shadowLayers = [3]struct{ d, alpha float64 }{{15, 0.1}, {9, 0.15}, {4, 0.24}}

// geom sind die Maße einer Seite im Fenster win.
type geom struct {
	win                     image.Rectangle
	k                       float64 // Maßstab gegenüber 1080p
	titleY, subY            float64
	rowX, rowW, rowTop      float64
	rowH, rowGap            float64
	noteY, hintY            float64
	titlePx, rowPx, smallPx float64
}

// geomOf rechnet die Maße der Seite für das Fenster win.
func geomOf(win image.Rectangle) geom {
	h, w := float64(win.Dy()), float64(win.Dx())
	g := geom{win: win, k: h / 1080}
	g.titleY = float64(win.Min.Y) + 0.07*h
	g.subY = float64(win.Min.Y) + 0.15*h
	g.rowW = math.Min(0.6*w, 1.4*h)
	g.rowX = float64(win.Min.X) + (w-g.rowW)/2
	g.rowTop = float64(win.Min.Y) + 0.21*h
	g.rowH, g.rowGap = 0.078*h, 0.014*h
	g.noteY = float64(win.Min.Y) + 0.865*h
	g.hintY = float64(win.Min.Y) + 0.93*h
	g.titlePx, g.rowPx, g.smallPx = 0.065*h, 0.04*h, 0.034*h
	return g
}

// rowRect ist das Rechteck der sichtbaren Zeile i (0 = oben).
func (g geom) rowRect(i int) (x, y, w, h float64) {
	return g.rowX, g.rowTop + float64(i)*(g.rowH+g.rowGap), g.rowW, g.rowH
}

// stripInset ist der Rand zwischen Karte und Leinen-Schild.
func (g geom) stripInset() float64 { return math.Max(3, 10*g.k) }

// askRect ist der Kasten der Rückfrage (Mitte des Fensters).
func (g geom) askRect() (x, y, w, h float64) {
	w, h = g.rowW*1.05, 3.6*g.rowH
	return float64(g.win.Min.X) + (float64(g.win.Dx())-w)/2, float64(g.win.Min.Y) + (float64(g.win.Dy())-h)/2, w, h
}

// textScale ist die Vergrößerung der Debug-Schrift (16 px hoch) für die Schriftgröße size, mindestens 1.
func textScale(size float64) float64 { return math.Max(1, math.Round(size/16)) }

// fade liefert c mit Deckkraft a (vormultipliziert).
func fade(c color.RGBA, a float64) color.RGBA {
	return color.RGBA{uint8(float64(c.R) * a), uint8(float64(c.G) * a), uint8(float64(c.B) * a), uint8(255 * a)}
}
