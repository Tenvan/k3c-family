package game

// Anzeige oben rechts in view.StatsRect: mit der Schrift des Spiels (art.Font) rechtsbündig auf einer dunklen Fläche,
// die nur so groß ist wie der Text; beide Zeilen bleiben in StatsRect (NG13: die vergrößerte Debug-Schrift verdeckte
// die Kopfzeile des HUD). Ohne Schriftdatei wie vorher die Debug-Schrift in einem eigenen Bild, vergrößert gezeichnet
// (wie native/k3c-ebiten/fps.go). Im Fenster die FPS (Text höchstens viermal je Sekunde Spielzeit neu), im
// Aufnahme-Modus der Takt statt der FPS, damit das Bild nicht von der Wanduhr abhängt. Mit Settings.Stats (F2, B-816)
// kommt eine zweite Zeile dazu: Rechenzeit von Update und Draw in ms (Mittel seit dem letzten Text). Die Wanduhr läuft
// nur dann, ausgeschaltet kostet die Anzeige keine Zeitnahme; im Aufnahme-Modus stehen an ihrer Stelle feste Striche.

import (
	"fmt"
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"

	"k3c/native/k3c-game/art"
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
	imgs      [2]*ebiten.Image // Debug-Schrift: Textbild mit einer und mit zwei Zeilen
	font      *art.Font        // Schrift des Spiels; nil oder Fallback: Debug-Schrift
	pixel     *ebiten.Image    // Fläche hinter der Schrift
	text1     [2]string        // aktueller Text (Zeile 1 und 2)
	tw        [2]float64       // gemessene Breiten der Zeilen bei px (Zwischenspeicher)
	th, px    float64          // gemessene Zeilenhöhe und die Größe, für die tw und th gelten
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

// newOverlay baut die Anzeige; font nil oder Fallback zeichnet mit der Debug-Schrift.
func newOverlay(fps bool, font *art.Font) *overlay {
	w, h, _ := view.StatsTextSize()
	w2, h2, _ := view.StatsTextSizeLines(2)
	o := &overlay{imgs: [2]*ebiten.Image{ebiten.NewImage(w, h), ebiten.NewImage(w2, h2)}, fps: fps, tick: ^uint64(0), now: clock}
	if font != nil && !font.Fallback {
		o.font = font
	}
	return o
}

// statsPad ist der Rand um den Text je px Schriftgröße.
const statsPad = 0.2

// statsPx ist die Schriftgröße der Anzeige: zwei Zeilen samt Rand passen in die Höhe von StatsRect r.
func statsPx(r rectF) float64 { return r.h / (2*art.LineSpacing + 2*statsPad) }

// rectF ist ein Rechteck in Fenster-Pixeln.
type rectF struct{ x, y, w, h float64 }

// statsBox ist die Fläche hinter dem Text: rechts oben in r, so breit wie die längste der lines Zeilen (Breiten tw,
// Zeilenhöhe th) plus Rand (rein, getestet).
func statsBox(r rectF, tw [2]float64, th float64, lines int, px float64) rectF {
	w := tw[0]
	if lines == 2 {
		w = max(w, tw[1])
	}
	pad := statsPad * px
	b := rectF{w: w + 2*pad, h: float64(lines)*th + 2*pad}
	b.x, b.y = r.x+r.w-b.w, r.y
	return b
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
		if o.font != nil {
			o.text1, o.px = [2]string{l1, l2}, 0
		} else {
			o.text(lines, l1, l2)
		}
	}
	if o.font != nil {
		o.drawFont(screen, lines)
		return
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

// drawFont zeichnet den Text mit der Schrift des Spiels rechtsbündig in StatsRect; gemessen wird nur bei neuem Text
// oder neuer Größe.
func (o *overlay) drawFont(screen *ebiten.Image, lines int) {
	sr := view.StatsRect(screen.Bounds())
	r := rectF{float64(sr.Min.X), float64(sr.Min.Y), float64(sr.Dx()), float64(sr.Dy())}
	px := statsPx(r)
	if o.px != px {
		o.px = px
		o.tw[0], o.th = o.font.Measure(o.text1[0], px)
		o.tw[1], _ = o.font.Measure(o.text1[1], px)
	}
	b := statsBox(r, o.tw, o.th, lines, px)
	if o.pixel == nil {
		o.pixel = ebiten.NewImage(1, 1)
		o.pixel.Fill(color.White)
	}
	o.op.GeoM.Reset()
	o.op.GeoM.Scale(b.w, b.h)
	o.op.GeoM.Translate(b.x, b.y)
	o.op.ColorScale.Reset()
	o.op.ColorScale.ScaleWithColor(overlayBack)
	screen.DrawImage(o.pixel, &o.op)
	pad := statsPad * px
	for i := range lines {
		o.font.Draw(screen, o.text1[i], b.x+b.w-pad-o.tw[i], b.y+pad+float64(i)*o.th, px, statsColor)
	}
}

// statsColor ist die Farbe des Texts der Anzeige.
var statsColor = color.RGBA{255, 255, 255, 255}

func (o *overlay) text(lines int, l1, l2 string) {
	img := o.imgs[lines-1]
	img.Fill(overlayBack)
	ebitenutil.DebugPrintAt(img, l1, 4, 1)
	if lines == 2 {
		ebitenutil.DebugPrintAt(img, l2, 4, 17)
	}
}
