package game

// Messwerte und Auswahl für die Seite Werte des Debug-Overlays (F10, B-889): Takte je Wandsekunde (gemessen nur, solange
// die Seite offen ist) und die per Maus gewählte Stelle in der Welt einer Zelle. Nicht im Aufnahme-Modus.

import (
	"image"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/inpututil"

	"k3c/native/k3c-game/view"
)

// tpsMeter zählt die Takte der Simulation je Wandsekunde.
type tpsMeter struct {
	since float64 // Beginn des Messfensters (clock)
	tick  uint64  // Frame.Tick zu Beginn
}

// update misst mit der Uhrzeit now und dem Takt tick; nach mindestens einer Sekunde liefert es den neuen Wert.
func (m *tpsMeter) update(now float64, tick uint64) (tps float64, ok bool) {
	if m.since == 0 || tick < m.tick {
		m.since, m.tick = now, tick
		return 0, false
	}
	if dt := now - m.since; dt >= 1 {
		tps = float64(tick-m.tick) / dt
		m.since, m.tick = now, tick
		return tps, true
	}
	return 0, false
}

// devValues aktualisiert Takte je Sekunde und Maus-Auswahl, solange die Seite Werte offen ist.
func (g *Game) devValues() {
	if g.dev.Debug != view.DebugValues || g.args.Shots != "" {
		g.tps = tpsMeter{}
		return
	}
	if tps, ok := g.tps.update(clock(), g.frame.Tick); ok {
		g.dev.TPS = tps
	}
	if !inpututil.IsMouseButtonJustPressed(ebiten.MouseButtonLeft) {
		return
	}
	if p, ok := pickAt(g.cells, image.Pt(ebiten.CursorPosition())); ok {
		g.dev.Pick = p
	}
}

// pickAt liefert die Stelle unter dem Punkt pt (Fenster-Pixel) in der Zelle, die ihn enthält.
func pickAt(cells []view.Cell, pt image.Point) (view.Pick, bool) {
	for _, c := range cells {
		if pt.In(c.Rect) && c.Player >= 0 {
			return view.Pick{Picked: true, Player: c.Player, X: c.Camera.WorldX(float64(pt.X))}, true
		}
	}
	return view.Pick{}, false
}
