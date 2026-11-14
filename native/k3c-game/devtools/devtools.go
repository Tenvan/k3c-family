// Package devtools zeichnet die Entwickler-Anzeigen des nativen Spiels (B-888): Tastenübersicht (F1) aus der Tabelle
// view.DevKeys, Cheat-Menü je Spieler (F9, view.Overlay) und Debug-Overlay (F10). Den Zustand von F1 und F10 setzt game
// im Frame (view.Frame.Dev); das Cheat-Menü führt sein Overlay selbst. Nichts davon erscheint im Aufnahme-Modus.
package devtools

import (
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// Farben der Entwickler-Anzeigen (vormultipliziert).
var (
	panelColor = color.RGBA{10, 12, 18, 220}
	textColor  = color.RGBA{240, 236, 220, 255}
	keyColor   = color.RGBA{250, 200, 90, 255}
	dimColor   = color.RGBA{150, 150, 150, 255}
)

// overlay ist der Drawer über allen Zellen (Tastenübersicht).
type overlay struct {
	lib *art.Library
}

// New liefert die Drawer des Pakets; lib darf nil sein (Tests), dann ohne Text.
func New(lib *art.Library) []view.Drawer {
	return []view.Drawer{&overlay{lib: lib}}
}

func (*overlay) Layer() view.Layer { return view.LayerOverlay }

// Draw zeichnet die Tastenübersicht, wenn F1 sie eingeschaltet hat.
func (o *overlay) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	if f.Recording || !f.Dev.Help {
		return
	}
	drawHelp(dst, fontOf(o.lib), c)
}

// fontOf liefert die Schrift der Bibliothek (nil ohne Bibliothek).
func fontOf(lib *art.Library) *art.Font {
	if lib == nil {
		return nil
	}
	return lib.Font
}

// box zeichnet eine dunkle Fläche.
func box(dst *ebiten.Image, x, y, w, h float64) {
	vector.FillRect(dst, float32(x), float32(y), float32(w), float32(h), panelColor, false)
}
