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

// Cells baut die Zellen der Plätze in f (höchstens view.MaxPlayers; Zelle i zeigt f.Seats[i]) in dst, mit Lage und
// Maßstab; Camera.X ist 0, bis Cameras.Follow es setzt. Mit Kapazität view.MaxPlayers legt Cells nichts an.
func Cells(dst []view.Cell, win image.Rectangle, f *view.Frame) []view.Cell {
	dst = dst[:0]
	n := min(len(f.Seats), view.MaxPlayers)
	for i := range n {
		c := view.Cell{Index: i, Player: f.Seats[i].Player, Rect: CellRect(win, n, i)}
		c.Camera = view.NewCamera(c.Rect, 0, Scale(c))
		dst = append(dst, c)
	}
	return dst
}

// Place legt die Zellen neu auf das Fenster win, falls es sich seit dem Update geändert hat (Fenstergröße zwischen
// zwei Updates); das Welt-x der Kamera bleibt. Ohne Änderung bleibt die Kamera der Regel unberührt.
func Place(cells []view.Cell, win image.Rectangle) {
	for i := range cells {
		c := &cells[i]
		if r := CellRect(win, len(cells), i); r != c.Rect {
			c.Rect = r
			c.Camera = view.NewCamera(r, c.Camera.X, Scale(*c))
		}
	}
}

// Cameras führt die Kameras der Zellen über die Updates: Zustand je Spieler (Seat.Player), Regel aus dem Paket world,
// feste Werte des Szenen-Schalters --cam je Zelle.
type Cameras struct {
	Rule  view.CameraRule // nil = folgt dem Monarchen, ohne Monarchen der Burg
	Fixed []float64       // Welt-x je Zelle, NaN = Regel
	prev  [view.MaxPlayers]view.Camera
}

// Follow setzt die Kamera jeder Zelle (einmal je Update): die Regel mit der Kamera des Spielers aus dem vorigen
// Update, danach feste Werte; ein NaN- oder unendliches Ergebnis der Regel gilt nicht. Spieler ohne Zelle vergessen
// ihre Kamera (beim nächsten Auftauchen springt die Regel).
func (k *Cameras) Follow(cells []view.Cell, f *view.Frame) {
	var next [view.MaxPlayers]view.Camera
	for i := range cells {
		c := &cells[i]
		if c.Player < 0 || c.Player >= view.MaxPlayers {
			continue
		}
		prev := k.prev[c.Player]
		c.Camera.X = prev.X
		if k.Rule == nil {
			c.Camera.X = defaultX(f, *c)
		} else if cam := k.Rule.Follow(f, *c, prev); finite(cam.X) {
			c.Camera = cam
		}
		if c.Index < len(k.Fixed) && !math.IsNaN(k.Fixed[c.Index]) {
			c.Camera.X = k.Fixed[c.Index]
		}
		next[c.Player] = c.Camera
	}
	k.prev = next
}

func finite(x float64) bool { return !math.IsNaN(x) && !math.IsInf(x, 0) }

// defaultX ist das Welt-x der Kamera ohne Regel: Monarch, sonst Burg, sonst 0.
func defaultX(f *view.Frame, c view.Cell) float64 {
	if m := f.Monarch(c.Player); m != nil {
		return m.X
	}
	if w := f.World(c); w != nil {
		return w.Castle.X
	}
	return 0
}
