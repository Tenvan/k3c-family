package game

// Große Anzeige oben rechts in view.StatsRect (wie native/k3c-ebiten/fps.go): die kleine Debug-Schrift in ein eigenes Bild, vergrößert
// gezeichnet. Im Fenster die FPS (Text höchstens viermal je Sekunde Spielzeit neu), im Aufnahme-Modus der Takt statt
// der FPS, damit das Bild nicht von der Wanduhr abhängt. Mit Settings.Stats (F2, B-816) kommt eine zweite Zeile dazu:
// Rechenzeit von Update und Draw in ms (Mittel seit dem letzten Text). Die Wanduhr läuft nur dann, ausgeschaltet kostet
// die Anzeige keine Zeitnahme; im Aufnahme-Modus stehen an ihrer Stelle feste Striche.

import (
	"fmt"
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"

	"k3c/native/k3c-game/view"
)

// overlayInterval sind die Sekunden Spielzeit zwischen zwei FPS-Texten. Lage und Größe: view.StatsRectLines.
const overlayInterval = 0.25

var overlayBack = color.RGBA{0, 0, 0, 170}

// avg mittelt Zeiten in Millisekunden.
type avg struct {
	sum float64
	n   int
}

func (a *avg) add(sec float64) { a.sum, a.n = a.sum+sec*1000, a.n+1 }

// take liefert das Mittel seit dem letzten Aufruf (0 ohne Werte) und beginnt neu.
func (a *avg) take() float64 {
	if a.n == 0 {
		return 0
	}
	v := a.sum / float64(a.n)
	*a = avg{}
	return v
}

type overlay struct {
	imgs      [2]*ebiten.Image // Textbild mit einer und mit zwei Zeilen
	fps       bool             // true: FPS, false: Takt (Aufnahme-Modus)
	next      float64
	tick      uint64
	lines     int // Zeilen des aktuellen Texts (0: noch keiner)
	op        ebiten.DrawImageOptions
	w         int  // Fensterbreite, für die op gilt
	measuring bool // Messmodus: keine eigene Zeitmessung, sonst verfälscht die Leistungsanzeige die Messung

	now      func() float64 // Wanduhr in Sekunden, nur für die Leistungsanzeige (clock)
	upd, drw avg            // Rechenzeit von Update und Draw seit dem letzten Text
}

func newOverlay(fps bool) *overlay {
	w, h, _ := view.StatsTextSize()
	w2, h2, _ := view.StatsTextSizeLines(2)
	return &overlay{imgs: [2]*ebiten.Image{ebiten.NewImage(w, h), ebiten.NewImage(w2, h2)}, fps: fps, tick: ^uint64(0), now: clock}
}

// timing meldet, ob Update und Draw gemessen werden: nur mit Leistungsanzeige und nicht im Aufnahme-Modus (dort hängt
// das Bild nicht von der Wanduhr ab).
func (o *overlay) timing(f *view.Frame) bool { return f.Settings.Stats && !f.Recording && !o.measuring }

// statsText liefert die Zeilen der Anzeige: FPS bzw. Takt (recording), mit on dazu Update und Draw in ms (im
// Aufnahme-Modus Striche); l2 ist sonst leer.
func statsText(recording, on bool, fps float64, tick uint64, upd, drw float64) (l1, l2 string) {
	if recording {
		l1 = fmt.Sprintf("T %7d", tick)
	} else {
		l1 = fmt.Sprintf("%4.0f FPS", fps)
	}
	switch {
	case on && recording:
		l2 = "U --- D ---"
	case on:
		l2 = fmt.Sprintf("U%4.1f D%4.1f", upd, drw)
	}
	return l1, l2
}

// due meldet, ob der Text neu entsteht: beim Wechsel der Zeilenzahl, dann nach Spielzeit (FPS) bzw. Takt.
func (o *overlay) due(f *view.Frame, lines int) bool {
	switch {
	case lines != o.lines:
		return true
	case o.fps:
		return f.Clock >= o.next
	}
	return f.Tick != o.tick
}

// draw setzt den Text bei Bedarf neu und zeichnet ihn in view.StatsRectLines.
func (o *overlay) draw(screen *ebiten.Image, f *view.Frame) {
	lines := 1
	if f.Settings.Stats {
		lines = 2
	}
	if o.due(f, lines) {
		o.next, o.tick, o.lines = f.Clock+overlayInterval, f.Tick, lines
		l1, l2 := statsText(!o.fps, lines == 2, ebiten.ActualFPS(), f.Tick, o.upd.take(), o.drw.take())
		o.text(lines, l1, l2)
	}
	if w := screen.Bounds().Dx(); w != o.w {
		o.w = w
		_, _, k := view.StatsTextSize()
		r := view.StatsRect(screen.Bounds())
		o.op.GeoM.Reset()
		o.op.GeoM.Scale(float64(k), float64(k))
		o.op.GeoM.Translate(float64(r.Min.X), float64(r.Min.Y))
	}
	screen.DrawImage(o.imgs[lines-1], &o.op)
}

func (o *overlay) text(lines int, l1, l2 string) {
	img := o.imgs[lines-1]
	img.Fill(overlayBack)
	ebitenutil.DebugPrintAt(img, l1, 4, 1)
	if lines == 2 {
		ebitenutil.DebugPrintAt(img, l2, 4, 17)
	}
}
