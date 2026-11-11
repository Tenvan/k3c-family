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

// Game ist die Ebitengine-Schleife: Eingabe, Menü-Stapel, Takt der Simulation, Kameras, Grafik der Level und Zeichnen
// der Zellen in Ebenen-Reihenfolge. Alle Zeit kommt aus dem Takt (Frame.Clock), nie aus der Wanduhr.
type Game struct {
	parts  Parts
	sim    Sim
	args   Args
	src    view.Source
	stack  Stack
	mode   Mode
	frame  view.Frame
	cues   []view.Cue
	cells  []view.Cell
	win    image.Rectangle // Zeichenfläche aus Layout
	cams   Cameras
	devs   devices
	levels []string // Level, deren Grafik geladen ist (sortiert)
	full   bool     // Vollbild ist angewendet
	err    error    // beendet das Programm (fehlende Grafik)
	subs   subImages
	stats  *overlay
	shots  *shooter
	meter  *meter // Messmodus (--measure), sonst nil
	endIn  int    // Takte im Spiel seit Frame.Outcome (Spielende nach view.EndDelay)
}

// New baut die Schleife. Mit Szene (Seed oder Spielstand) startet das Spiel sofort, sonst liegt das Hauptmenü oben.
// Die Schleife läuft Scene.Tick Takte vor (wie im Spiel: Eingabe, Overlays, Listener; nur ohne Zeichnen) und legt
// danach das Menü der Szene auf (Scene.Menu, menu.go).
func New(p Parts, s Sim, a Args) (*Game, error) {
	g := &Game{parts: p, sim: s, args: a, src: p.Input, mode: ModeMenu, cells: make([]view.Cell, 0, view.MaxPlayers),
		win: image.Rect(0, 0, a.W, a.H), cams: Cameras{Rule: p.Camera, Fixed: a.Scene.Cams}}
	if a.Script {
		g.src = newScripted(a.Scene)
	}
	g.stats = newOverlay(a.Shots == "")
	if a.Shots != "" {
		sh, err := newShooter(a)
		if err != nil {
			return nil, err
		}
		g.shots = sh
	}
	if a.Measure.Seconds > 0 {
		g.meter = newMeter(a.Measure)
	}
	if a.Scene.Direct() {
		if err := g.startScene(); err != nil {
			return nil, err
		}
	} else {
		g.stack.Push(p.Menus.Main())
	}
	for range a.Scene.Tick {
		if g.advance(); g.err != nil {
			return nil, g.err
		}
	}
	if err := g.openMenu(); err != nil {
		return nil, err
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

// Update rechnet einen Takt: Eingabe, Menü bzw. Spiel, Simulation, Listener. Im Aufnahme-Modus taktet der shooter;
// im Messmodus endet das Programm, sobald der Bericht geschrieben ist.
func (g *Game) Update() error {
	if g.mode == ModeQuit {
		return ebiten.Termination
	}
	if g.shots != nil {
		return g.shots.update(g)
	}
	if g.meter.done() {
		g.mode = ModeQuit
		return g.sim.Close()
	}
	t0 := clock()
	g.advance()
	g.meter.tick(clock() - t0)
	if g.err != nil {
		return g.err
	}
	if g.mode == ModeQuit {
		return g.sim.Close()
	}
	return nil
}

// Frame liefert den Frame des letzten Updates (nur lesen, gilt bis zum nächsten Update; Ende-zu-Ende-Test).
func (g *Game) Frame() *view.Frame { return &g.frame }

// Screen liefert den obersten Screen des Menü-Stapels (nil im laufenden Spiel; Ende-zu-Ende-Test).
func (g *Game) Screen() view.Screen { return g.stack.Top() }

// advance liest die Eingabe, schaltet bei ActionFullscreen das Vollbild um, bedient Menü bzw. Spiel, taktet die Simulation einmal und bereitet den Frame vor:
// Geräte, Grafik der Level, Kameras, Vollbild, dann die Listener.
func (g *Game) advance() {
	in := g.src.Poll()
	g.cues = g.devs.observe(in, g.frame.Seats, g.cues)
	g.toggleFullscreen(in)
	if top := g.stack.Top(); top != nil {
		g.step(top.Update(in, g.sim))
	} else {
		g.play(in)
	}
	g.frame = g.sim.Step()
	g.devs.decorate(&g.frame, in)
	if len(g.cues) > 0 {
		g.frame.Cues = append(g.frame.Cues, g.cues...)
		g.cues = g.cues[:0]
	}
	g.loadLevels()
	g.cells = Cells(g.cells, g.win, &g.frame)
	g.cams.Follow(g.cells, &g.frame)
	g.frame.Cells, g.frame.Recording = g.cells, g.args.Shots != ""
	g.applyFullscreen()
	for _, l := range g.parts.Listeners {
		l.Listen(&g.frame)
	}
}

// toggleFullscreen kehrt bei ActionFullscreen eines Geräts (Menü und Spiel) das Vollbild der Einstellungen um
// (Session.SetSettings; applyFullscreen wendet es im nächsten Schritt an). Nicht im Aufnahme-Modus und nicht vor dem
// ersten Frame (dessen Settings sind noch leer, sonst gingen Lautstärken und Sprache verloren).
func (g *Game) toggleFullscreen(in view.Input) {
	if g.args.Shots != "" || g.frame.Settings.Language == "" {
		return
	}
	if _, ok := in.Has(view.ActionFullscreen); !ok {
		return
	}
	s := g.frame.Settings
	s.Fullscreen = !s.Fullscreen
	if err := g.sim.SetSettings(s); err != nil {
		slog.Warn("💥 Einstellungen nicht gespeichert", "err", err)
	}
}

// applyFullscreen setzt Vollbild nach den Einstellungen, sobald sie sich ändern; nicht im Aufnahme-Modus.
func (g *Game) applyFullscreen() {
	if g.shots != nil || g.args.Shots != "" || g.frame.Settings.Fullscreen == g.full {
		return
	}
	g.full = g.frame.Settings.Fullscreen
	ebiten.SetFullscreen(g.full)
	slog.Info("🚦 Vollbild", "an", g.full)
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
	t0 := clock()
	screen.Fill(clearColor)
	win := screen.Bounds()
	Place(g.cells, win)
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
	if g.meter != nil {
		now := clock()
		g.meter.frame(now, now-t0, g)
	}
}

// Layout: Zeichenfläche in Fenster-Bildpunkten; im Aufnahme-Modus fest (dieselben Schalter, dasselbe Bild).
func (g *Game) Layout(ow, oh int) (int, int) {
	if g.shots != nil {
		ow, oh = g.args.W, g.args.H
	}
	g.win = image.Rect(0, 0, ow, oh)
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
