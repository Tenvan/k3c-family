// Package menu baut die Menü-Bildschirme: Hauptmenü, Spielstände, Pause (Fach-Sprint der Menüs). Bis dahin
// Platzhalter: Hauptmenü mit „Bestätigen = neues Spiel“, Pause-Menü mit „jede Taste = weiter“.
package menu

import (
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

var (
	mainColor  = color.RGBA{40, 30, 60, 255}
	pauseColor = color.RGBA{0, 0, 0, 140}
	boxColor   = color.RGBA{230, 190, 40, 255}
	errorColor = color.RGBA{200, 50, 50, 255}
)

type menus struct{}

// New liefert die Einstiege des Menüs; lib (Grafik) darf nil sein, die Platzhalter brauchen sie nicht.
func New(_ *art.Library) view.Menus { return menus{} }

func (menus) Main() view.Screen  { return &mainScreen{} }
func (menus) Pause() view.Screen { return pauseScreen{} }

// mainScreen startet bei Bestätigen ein neues Spiel mit dem Gerät, das bestätigt hat.
type mainScreen struct{ failed bool }

func (m *mainScreen) Update(in view.Input, s view.Session) view.Step {
	d, ok := in.Has(view.ActionConfirm)
	if !ok {
		return view.Step{}
	}
	if err := s.New(view.NewGame{Devices: []view.DeviceID{d}}); err != nil {
		m.failed = true
		return view.Step{Cues: []view.Cue{{Kind: view.CueDenied, Player: -1, Text: err.Error()}}}
	}
	m.failed = false
	return view.Step{Kind: view.StepPlay, Cues: []view.Cue{{Kind: view.CueConfirm, Player: -1}}}
}

func (m *mainScreen) Draw(dst *ebiten.Image, _ *view.Frame, c view.Cell) {
	r := c.Rect
	vector.FillRect(dst, float32(r.Min.X), float32(r.Min.Y), float32(r.Dx()), float32(r.Dy()), mainColor, false)
	clr := boxColor
	if m.failed {
		clr = errorColor
	}
	drawBox(dst, c, clr)
}

// pauseScreen schließt sich bei Bestätigen, Zurück oder Menü.
type pauseScreen struct{}

func (pauseScreen) Update(in view.Input, _ view.Session) view.Step {
	for _, a := range []view.Action{view.ActionConfirm, view.ActionBack, view.ActionMenu} {
		if _, ok := in.Has(a); ok {
			return view.Step{Kind: view.StepPop, Cues: []view.Cue{{Kind: view.CueResume, Player: -1}}}
		}
	}
	return view.Step{}
}

func (pauseScreen) Draw(dst *ebiten.Image, _ *view.Frame, c view.Cell) {
	r := c.Rect
	vector.FillRect(dst, float32(r.Min.X), float32(r.Min.Y), float32(r.Dx()), float32(r.Dy()), pauseColor, false)
	drawBox(dst, c, boxColor)
}

// drawBox zeichnet ein Rechteck in der Mitte des Fensters (Platzhalter für die Auswahl).
func drawBox(dst *ebiten.Image, c view.Cell, clr color.RGBA) {
	r := c.Rect
	w, h := float32(r.Dx())/4, float32(r.Dy())/6
	vector.FillRect(dst, float32(r.Min.X)+(float32(r.Dx())-w)/2, float32(r.Min.Y)+(float32(r.Dy())-h)/2, w, h, clr, false)
}
