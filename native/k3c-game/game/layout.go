package game

import (
	"image"
	"math"

	"k3c/native/k3c-game/view"
)

// ViewUnits ist die sichtbare Welthöhe über dem Boden in Units (wie NV3: 420 px Welt bei 32 px je Unit). Der Maßstab
// einer Zelle folgt daraus: Höhe des Welt-Teils / ViewUnits.
const ViewUnits = 420.0 / 32

// CellRect liefert die Lage der Zelle i von n im Fenster win: eine Zelle füllt das Fenster, zwei liegen übereinander
// (wie der Browser, src/scenes/layout.ts); die untere bekommt bei ungerader Höhe den Rest.
func CellRect(win image.Rectangle, n, i int) image.Rectangle {
	if n <= 1 {
		return win
	}
	mid := win.Min.Y + win.Dy()/2
	if i == 0 {
		return image.Rect(win.Min.X, win.Min.Y, win.Max.X, mid)
	}
	return image.Rect(win.Min.X, mid, win.Max.X, win.Max.Y)
}

// Scale liefert den Maßstab der Zelle c in Bildpunkten je Unit.
func Scale(c view.Cell) float64 { return float64(c.WorldRect().Dy()) / ViewUnits }

// Cells baut die Zellen der Plätze in f (höchstens view.MaxPlayers, nach Player) in dst. cams[i] setzt die Kamera der
// Zelle i fest (Aufnahme-Modus); fehlt der Eintrag oder ist er NaN, folgt die Kamera dem Monarchen, ohne Monarchen
// steht sie an der Burg. Mit Kapazität view.MaxPlayers legt Cells nichts an.
func Cells(dst []view.Cell, win image.Rectangle, f *view.Frame, cams []float64) []view.Cell {
	dst = dst[:0]
	n := min(len(f.Seats), view.MaxPlayers)
	for i := range n {
		c := view.Cell{Index: i, Player: f.Seats[i].Player, Rect: CellRect(win, n, i)}
		c.Camera = view.NewCamera(c.Rect, cameraX(f, c, cams), Scale(c))
		dst = append(dst, c)
	}
	return dst
}

// cameraX ist das Welt-x der Kamera der Zelle c.
func cameraX(f *view.Frame, c view.Cell, cams []float64) float64 {
	if c.Index < len(cams) && !math.IsNaN(cams[c.Index]) {
		return cams[c.Index]
	}
	if m := f.Monarch(c.Player); m != nil {
		return m.X
	}
	if w := f.World(c); w != nil {
		return w.Castle.X
	}
	return 0
}
