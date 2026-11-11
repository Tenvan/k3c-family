package game

import (
	"image"
	"image/color"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// viewUnits ist die sichtbare Welthöhe über dem Boden in Units (vorläufig; Lage und Ausschnitt legt NG1.4 fest).
const viewUnits = 12.0

var clearColor = color.RGBA{0, 0, 0, 255}

// Game ist die Ebitengine-Schleife: Eingabe, Menü-Stapel, Frame und Zeichnen in Ebenen-Reihenfolge. Vorläufig eine
// Zelle über das ganze Fenster; Simulation (NG1.2) und Zellen im Split-Screen (NG1.4) folgen.
type Game struct {
	parts   Parts
	session view.Session
	stack   Stack
	mode    Mode
	frame   view.Frame
	cues    []view.Cue
}

// New baut die Schleife mit dem Hauptmenü oben.
func New(p Parts, s view.Session) *Game {
	g := &Game{parts: p, session: s, mode: ModeMenu}
	g.stack.Push(p.Menus.Main())
	return g
}

// Update liest die Eingabe, bedient Menü bzw. Spiel und reicht den Frame an die Listener.
func (g *Game) Update() error {
	if g.mode == ModeQuit {
		return ebiten.Termination
	}
	in := g.parts.Input.Poll()
	if top := g.stack.Top(); top != nil {
		g.step(top.Update(in, g.session))
	} else if _, ok := in.Has(view.ActionMenu); ok {
		g.session.Pause(true)
		g.stack.Push(g.parts.Menus.Pause())
		g.mode = ModeMenu
		g.cues = append(g.cues, view.Cue{Kind: view.CuePause, Player: -1})
	}
	g.frame.Clock += 1 / float64(ebiten.TPS())
	g.frame.Paused = g.mode == ModeMenu && g.session.Active()
	g.frame.Cues, g.cues = g.cues, nil
	for _, l := range g.parts.Listeners {
		l.Listen(&g.frame)
	}
	if g.mode == ModeQuit {
		return g.session.Close()
	}
	return nil
}

// step wendet das Ergebnis des obersten Screens an.
func (g *Game) step(st view.Step) {
	g.cues = append(g.cues, st.Cues...)
	was := g.mode
	g.mode = g.stack.Apply(st, g.session.Active(), g.parts.Menus.Main)
	if was == ModeMenu && g.mode == ModePlay {
		g.session.Pause(false)
	}
}

// Draw zeichnet je Zelle die Zellen-Ebenen, dann das Overlay und zuletzt den obersten Screen.
func (g *Game) Draw(screen *ebiten.Image) {
	screen.Fill(clearColor)
	win := screen.Bounds()
	for _, c := range g.cells(win) {
		dst := screen.SubImage(c.Rect).(*ebiten.Image)
		for _, d := range g.parts.Drawers {
			if d.Layer().PerCell() {
				d.Draw(dst, &g.frame, c)
			}
		}
	}
	wc := view.WindowCell(win)
	for _, d := range g.parts.Drawers {
		if d.Layer() == view.LayerOverlay {
			d.Draw(screen, &g.frame, wc)
		}
	}
	if top := g.stack.Top(); top != nil {
		top.Draw(screen, &g.frame, wc)
	}
}

// cells liefert vorläufig eine Zelle über das ganze Fenster für Spieler 0, Kamera auf seinem Monarchen.
func (g *Game) cells(win image.Rectangle) []view.Cell {
	c := view.Cell{Index: 0, Player: 0, Rect: win}
	x := 0.0
	if m := g.frame.Monarch(0); m != nil {
		x = m.X
	}
	c.Camera = view.NewCamera(win, x, float64(c.WorldRect().Dy())/viewUnits)
	return []view.Cell{c}
}

// Layout: Zeichenfläche in Fenster-Bildpunkten.
func (g *Game) Layout(ow, oh int) (int, int) { return ow, oh }
