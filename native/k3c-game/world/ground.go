package world

// Boden der Stufe (B-795/AC-02): ein Uferstreifen in der Boden-Farbe des Bioms unter der Boden-Linie, oben im
// Fluss-Drittel der Zelle (NV3: native/k3c-ebiten/river.go › bankH); darunter fließt der Fluss. Lava-Streifen
// (engine/sim/world_lava.go) liegen glühend im Uferstreifen.

import (
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// bankUnits ist die Höhe des Uferstreifens unter der Boden-Linie in Units (20 px wie NV3).
const bankUnits = 20.0 / 32

// lavaColor ist die Farbe eines Lava-Streifens (nur Darstellung).
var lavaColor = color.RGBA{232, 96, 32, 255}

// bankHeight ist die Höhe des Uferstreifens der Zelle in px, höchstens das Fluss-Drittel.
func bankHeight(c view.Cell) float64 {
	return min(bankUnits*c.Camera.Scale, float64(c.RiverRect().Dy()))
}

// lavaSpan liefert links und rechts (Units) des Lava-Streifens im Chunk i der Welt; ok = false ohne Lava.
func lavaSpan(w *sim.World, i int) (left, right float64, ok bool) {
	ch := w.Level.Chunks[i]
	half := w.Biome.Lava.WidthUnits / 2
	if ch.Kind != "lava" || half <= 0 {
		return 0, 0, false
	}
	mid := ch.StartUnits + w.Level.ChunkWidthUnits/2
	return mid - half, mid + half, true
}

// drawGround zeichnet den Uferstreifen und die sichtbaren Lava-Streifen.
func (w *World) drawGround(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	st := stageOf(f, c)
	if st == nil {
		return
	}
	k := c.Camera
	top, h := float64(c.GroundY()), bankHeight(c)
	w.fill(dst, float64(c.Rect.Min.X), top, float64(c.Rect.Dx()), h, w.looks.of(st.Level).ground, 1, 1)
	wd := st.World
	for i := range wd.Level.Chunks {
		l, r, ok := lavaSpan(wd, i)
		if !ok || !Visible(k, (l+r)/2, (r-l)/2) {
			continue
		}
		x0, x1 := k.ScreenX(l), k.ScreenX(r)
		w.fill(dst, x0, top, x1-x0, h, lavaColor, 1, 1)
	}
}
