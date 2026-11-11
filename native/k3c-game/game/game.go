package game

import (
	"image"
	"image/color"
	"log/slog"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// TPS ist der Takt von Update: ein Simulationstakt je Update (30 Hz, Entscheidung B-793), gezeichnet wird mit Bildrate.
const TPS = 30

var clearColor = color.RGBA{0, 0, 0, 255}

// Sim ist die Simulation aus Sicht der Schleife: die Session der Menüs plus Takt und Befehle (local erfüllt sie).
type Sim interface {
	view.Session
	Command(player int, c view.Command) error
	Step() view.Frame
}

// Game ist die Ebitengine-Schleife: Eingabe, Menü-Stapel, Takt der Simulation und Zeichnen der Zellen in
// Ebenen-Reihenfolge. Alle Zeit kommt aus dem Takt (Frame.Clock), nie aus der Wanduhr.
type Game struct {
	parts Parts
	sim   Sim
	args  Args
	src   view.Source
	stack Stack
	mode  Mode
	frame view.Frame
	cues  []view.Cue
	cells []view.Cell
	subs  subImages
	stats *overlay
	shots *shooter
}

// New baut die Schleife. Mit Szene (Seed oder Spielstand) startet das Spiel sofort, sonst liegt das Hauptmenü oben.
// Im Aufnahme-Modus läuft die Simulation bis Scene.Tick vor.
func New(p Parts, s Sim, a Args) (*Game, error) {
	g := &Game{parts: p, sim: s, args: a, src: p.Input, mode: ModeMenu, cells: make([]view.Cell, 0, view.MaxPlayers)}
	if a.Script {
		g.src = newScripted(a.Scene)
	}
	g.stats = newOverlay(a.Shots == "")
	if !a.Scene.Direct() {
		g.stack.Push(p.Menus.Main())
		return g, nil
	}
	if err := g.startScene(); err != nil {
		return nil, err
	}
	if a.Shots != "" {
		sh, err := newShooter(a)
		if err != nil {
			return nil, err
		}
		g.shots = sh
	}
	for range a.Scene.Tick {
		g.advance()
	}
	return g, nil
}

// startScene startet das Spiel der Szene ohne Menü.
func (g *Game) startScene() error {
	sc := g.args.Scene
	var err error
	if sc.Load != "" {
		err = g.sim.Load(sc.Load, sc.Devices())
	} else {
		err = g.sim.New(view.NewGame{Seed: sc.Seed, Depth: sc.Depth, Devices: sc.Devices()})
	}
	if err != nil {
		return err
	}
	g.mode = ModePlay
	slog.Info("🚀 Szene gestartet", "seed", sc.Seed, "load", sc.Load, "tiefe", sc.Depth, "spieler", sc.Players, "vorlauf", sc.Tick)
	return nil
}

// Update rechnet einen Takt: Eingabe, Menü bzw. Spiel, Simulation, Listener. Im Aufnahme-Modus taktet der shooter.
func (g *Game) Update() error {
	if g.mode == ModeQuit {
		return ebiten.Termination
	}
	if g.shots != nil {
		return g.shots.update(g)
	}
	g.advance()
	if g.mode == ModeQuit {
		return g.sim.Close()
	}
	return nil
}

// advance liest die Eingabe, bedient Menü bzw. Spiel und taktet die Simulation einmal.
func (g *Game) advance() {
	in := g.src.Poll()
	if top := g.stack.Top(); top != nil {
		g.step(top.Update(in, g.sim))
	} else {
		g.play(in)
	}
	g.frame = g.sim.Step()
	if len(g.cues) > 0 {
		g.frame.Cues = append(g.frame.Cues, g.cues...)
		g.cues = g.cues[:0]
	}
	for _, l := range g.parts.Listeners {
		l.Listen(&g.frame)
	}
}

// play gibt im laufenden Spiel die Befehle der Geräte an ihre Spieler, nimmt neue Geräte bei Bestätigen auf und
// öffnet bei ActionMenu das Pause-Menü. ActionBack (B) ist im Spiel unbelegt.
func (g *Game) play(in view.Input) {
	if !g.sim.Active() {
		g.stack.Push(g.parts.Menus.Main())
		g.mode = ModeMenu
		return
	}
	if _, ok := in.Has(view.ActionMenu); ok {
		g.sim.Pause(true)
		g.stack.Push(g.parts.Menus.Pause())
		g.mode = ModeMenu
		return
	}
	for _, s := range g.frame.Seats {
		di, _ := in.Of(s.Device)
		if err := g.sim.Command(s.Player, di.Command); err != nil {
			slog.Warn("🚫 Befehl abgelehnt", "player", s.Player, "err", err)
		}
	}
	for _, di := range in.Devices {
		if !g.seated(di.Device) && hasAction(di, view.ActionConfirm) {
			if p, err := g.sim.Join(di.Device); err != nil {
				g.cues = append(g.cues, view.Cue{Kind: view.CueDenied, Player: -1, Text: err.Error()})
			} else {
				slog.Info("👑 Gerät spielt mit", "device", di.Device.String(), "player", p)
			}
		}
	}
}

// seated meldet, ob das Gerät d einen Platz hat.
func (g *Game) seated(d view.DeviceID) bool {
	for _, s := range g.frame.Seats {
		if s.Device == d {
			return true
		}
	}
	return false
}

func hasAction(di view.DeviceInput, a view.Action) bool {
	for _, x := range di.Actions {
		if x == a {
			return true
		}
	}
	return false
}

// step wendet das Ergebnis des obersten Screens an.
func (g *Game) step(st view.Step) {
	g.cues = append(g.cues, st.Cues...)
	was := g.mode
	g.mode = g.stack.Apply(st, g.sim.Active(), g.parts.Menus.Main)
	if was == ModeMenu && g.mode == ModePlay {
		g.sim.Pause(false)
	}
}

// Draw zeichnet je Zelle die Zellen-Ebenen, dann das Overlay, den obersten Screen und zuletzt die Anzeige oben.
func (g *Game) Draw(screen *ebiten.Image) {
	screen.Fill(clearColor)
	win := screen.Bounds()
	g.cells = Cells(g.cells, win, &g.frame, g.args.Scene.Cams)
	for i, c := range g.cells {
		dst := g.subs.of(screen, i, c.Rect)
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
	g.stats.draw(screen, &g.frame)
	if g.shots != nil {
		g.shots.capture(g, screen)
	}
}

// Layout: Zeichenfläche in Fenster-Bildpunkten; im Aufnahme-Modus fest (dieselben Schalter, dasselbe Bild).
func (g *Game) Layout(ow, oh int) (int, int) {
	if g.shots != nil {
		return g.args.W, g.args.H
	}
	return ow, oh
}

// subImages hält die Ausschnitte der Zellen, damit Draw nicht je Bild neue anlegt.
type subImages struct {
	src   *ebiten.Image
	rects [view.MaxPlayers]image.Rectangle
	imgs  [view.MaxPlayers]*ebiten.Image
}

// of liefert den Ausschnitt r des Bilds screen für Zelle i.
func (s *subImages) of(screen *ebiten.Image, i int, r image.Rectangle) *ebiten.Image {
	if s.src != screen {
		s.src, s.imgs = screen, [view.MaxPlayers]*ebiten.Image{}
	}
	if s.imgs[i] == nil || s.rects[i] != r {
		s.imgs[i], s.rects[i] = screen.SubImage(r).(*ebiten.Image), r
	}
	return s.imgs[i]
}
