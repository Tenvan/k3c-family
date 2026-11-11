package game

// Große Anzeige oben rechts in view.StatsRect (wie native/k3c-ebiten/fps.go): die kleine Debug-Schrift in ein eigenes Bild, vergrößert
// gezeichnet. Im Fenster die FPS (Text höchstens viermal je Sekunde Spielzeit neu), im Aufnahme-Modus der Takt statt
// der FPS, damit das Bild nicht von der Wanduhr abhängt.

import (
	"fmt"
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"

	"k3c/native/k3c-game/view"
)

// overlayInterval sind die Sekunden Spielzeit zwischen zwei FPS-Texten. Lage und Größe: view.StatsRect.
const overlayInterval = 0.25

var overlayBack = color.RGBA{0, 0, 0, 170}

type overlay struct {
	img  *ebiten.Image
	fps  bool // true: FPS, false: Takt (Aufnahme-Modus)
	next float64
	tick uint64
	op   ebiten.DrawImageOptions
	w    int // Fensterbreite, für die op gilt
}

func newOverlay(fps bool) *overlay {
	w, h, _ := view.StatsTextSize()
	return &overlay{img: ebiten.NewImage(w, h), fps: fps, tick: ^uint64(0)}
}

// draw setzt den Text bei Bedarf neu und zeichnet ihn in view.StatsRect.
func (o *overlay) draw(screen *ebiten.Image, f *view.Frame) {
	switch {
	case o.fps && f.Clock >= o.next:
		o.next = f.Clock + overlayInterval
		o.text(fmt.Sprintf("%4.0f FPS", ebiten.ActualFPS()))
	case !o.fps && f.Tick != o.tick:
		o.tick = f.Tick
		o.text(fmt.Sprintf("T %7d", f.Tick))
	}
	if w := screen.Bounds().Dx(); w != o.w {
		o.w = w
		_, _, k := view.StatsTextSize()
		r := view.StatsRect(screen.Bounds())
		o.op.GeoM.Reset()
		o.op.GeoM.Scale(float64(k), float64(k))
		o.op.GeoM.Translate(float64(r.Min.X), float64(r.Min.Y))
	}
	screen.DrawImage(o.img, &o.op)
}

func (o *overlay) text(s string) {
	o.img.Fill(overlayBack)
	ebitenutil.DebugPrintAt(o.img, s, 4, 1)
}
