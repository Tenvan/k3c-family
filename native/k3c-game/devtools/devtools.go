// Package devtools zeichnet die Entwickler-Anzeigen des nativen Spiels (B-888): Tastenübersicht (F1) aus der Tabelle
// view.DevKeys, Cheat-Menü je Spieler (F9, view.Overlay) und Debug-Overlay (F10). Den Zustand von F1 und F10 setzt game
// im Frame (view.Frame.Dev); das Cheat-Menü führt sein Overlay selbst. Gezeichnet wird alles auf LayerOverlay über allen
// Zellen (je Zelle in deren Rechteck aus Frame.Cells). Nichts davon erscheint im Aufnahme-Modus.
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
	badColor   = color.RGBA{240, 110, 90, 255}
	selColor   = color.RGBA{70, 60, 20, 230}
)

// overlay ist der Beitrag des Pakets: Drawer (LayerOverlay) und Overlay (Cheat-Menü).
type overlay struct {
	lib     *art.Library
	menus   [view.MaxPlayers]cheatState
	toggles toggles
	cues    []view.Cue
	acts    []view.Action // Puffer von actionsOf
}

// New liefert die Drawer des Pakets; lib darf nil sein (Tests), dann ohne Text.
func New(lib *art.Library) []view.Drawer {
	return []view.Drawer{&overlay{lib: lib, toggles: toggles{}}}
}

func (*overlay) Layer() view.Layer { return view.LayerOverlay }

// Draw zeichnet die Cheat-Menüs in ihren Zellen und darüber die Tastenübersicht, wenn F1 sie eingeschaltet hat.
func (o *overlay) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	if f.Recording {
		return
	}
	font := fontOf(o.lib)
	for _, cell := range f.Cells {
		if cell.Player >= 0 && cell.Player < view.MaxPlayers && o.menus[cell.Player].open {
			o.drawCheats(dst, font, cell, &o.menus[cell.Player])
		}
	}
	if f.Dev.Help {
		drawHelp(dst, font, c)
	}
}

// fontOf liefert die Schrift der Bibliothek (nil ohne Bibliothek).
func fontOf(lib *art.Library) *art.Font {
	if lib == nil {
		return nil
	}
	return lib.Font
}

// fill zeichnet eine Fläche.
func fill(dst *ebiten.Image, x, y, w, h float64, clr color.RGBA) {
	vector.FillRect(dst, float32(x), float32(y), float32(w), float32(h), clr, false)
}

// box zeichnet eine dunkle Fläche.
func box(dst *ebiten.Image, x, y, w, h float64) { fill(dst, x, y, w, h, panelColor) }
