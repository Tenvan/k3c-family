package game

// Große Anzeige oben rechts (wie native/k3c-ebiten/fps.go): die kleine Debug-Schrift in ein eigenes Bild, vergrößert
// gezeichnet. Im Fenster die FPS (Text höchstens viermal je Sekunde Spielzeit neu), im Aufnahme-Modus der Takt statt
// der FPS, damit das Bild nicht von der Wanduhr abhängt.

import (
	"fmt"
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"

	"k3c/native/k3c-game/view"
)

const (
	overlayScale    = 5.0  // Vergrößerung der Debug-Schrift
	overlayInterval = 0.25 // Sekunden Spielzeit zwischen zwei FPS-Texten
	overlayW        = 76   // Breite des Textbilds in px vor der Vergrößerung
	overlayH        = 18
	overlayMargin   = 16
)

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
	return &overlay{img: ebiten.NewImage(overlayW, overlayH), fps: fps, tick: ^uint64(0)}
}

// draw setzt den Text bei Bedarf neu und zeichnet ihn oben rechts.
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
		o.op.GeoM.Reset()
		o.op.GeoM.Scale(overlayScale, overlayScale)
		o.op.GeoM.Translate(float64(screen.Bounds().Max.X)-overlayW*overlayScale-overlayMargin, overlayMargin)
	}
	screen.DrawImage(o.img, &o.op)
}

func (o *overlay) text(s string) {
	o.img.Fill(overlayBack)
	ebitenutil.DebugPrintAt(o.img, s, 4, 1)
}
