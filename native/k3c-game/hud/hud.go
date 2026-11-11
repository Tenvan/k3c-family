// Package hud zeichnet die Anzeigen je Zelle (Sprint NG6, B-799), wie der Browser (src/scenes/HudScene.ts, hud/,
// hints/, kampfAnzeige.ts): Spielerkarte mit Leben und Gold, Uhr, Kampf, Vorrat, Hub und Bürger, Ort, Meldungen,
// Boss-Leiste und Nacht-Events, Radar, Reise, Beitritts-Hinweis, Skill-Leiste, Aktions-Hinweise und die
// geführte erste Nacht über der Welt, Berufe und Händler als Labels, dazu das Skill-Menü je Spieler.
//
// Aufbau: state.go schreibt je Frame den Zustand fort (Listener), overlay.go bedient das Skill-Menü (view.Overlay),
// layout.go, values.go, banner.go, radar.go, kampf.go, hints.go, glyphs.go, skillmenu.go und skilltext.go rechnen rein
// und getestet, draw*.go und paint.go zeichnen. Draw ändert keinen Zustand außer gemessenen Texten und dem Layout-Cache.
package hud

import (
	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// hud ist der Beitrag des Pakets: Drawer (LayerHUD), Listener und Overlay (Skill-Menü).
type hud struct {
	lib    *art.Library
	seats  [view.MaxPlayers]seatState
	seen   guideSeen
	lang   string
	cells  int
	joinS  string
	mJoin  memo[joinKey]
	cueBuf []view.Cue
	paint  painter
	lay    [view.MaxPlayers]layoutCache
	spots  []labelSpot // Puffer der sichtbaren Welt-Labels in Draw
}

// New liefert den Drawer des HUD; lib (Grafik) darf nil sein, dann zeichnet es nur Flächen ohne Text.
func New(lib *art.Library) view.Drawer {
	return &hud{lib: lib, lang: view.DefaultLanguage}
}

func (*hud) Layer() view.Layer { return view.LayerHUD }

// font liefert die Schrift (nil ohne Grafik-Bibliothek).
func (h *hud) font() *art.Font {
	if h.lib == nil {
		return nil
	}
	return h.lib.Font
}

// seat liefert den Zustand des Spielers p (nil, wenn er keinen hat).
func (h *hud) seat(p int) *seatState {
	if p < 0 || p >= len(h.seats) || !h.seats[p].active {
		return nil
	}
	return &h.seats[p]
}

// Draw zeichnet die Anzeigen des Spielers der Zelle c.
func (h *hud) Draw(dst *ebiten.Image, _ *view.Frame, c view.Cell) {
	st := h.seat(c.Player)
	if st == nil {
		return
	}
	h.paint.stencils()
	k := scaleOf(c, h.cells)
	h.drawWorldTexts(dst, st, c, k)
	h.drawItems(dst, st, c, k)
	if st.menu.open {
		h.drawMenu(dst, st, c, k)
	}
}
