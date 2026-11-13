package game

import (
	"image"
	"image/color"
	"log/slog"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/inpututil"

	"k3c/native/k3c-game/art"
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
	parts   Parts
	sim     Sim
	args    Args
	src     view.Source
	stack   Stack
	mode    Mode
	frame   view.Frame
	cues    []view.Cue
	cells   []view.Cell
	win     image.Rectangle // Zeichenfläche aus Layout
	cams    Cameras
	devs    devices
	levels  []string // Level, deren Grafik geladen ist (sortiert)
	menuArt bool     // Grafik steht auf den Menü-Illustrationen (ohne Level, loadMenuArt)
	window  string   // angewendeter Anzeige-Modus (view.Window*; Start: Fenster)
	err     error    // beendet das Programm (nur im Messmodus; sonst fail)
	fault   *Fault   // Fehler im Fenster (fehlende Grafik, B-810); gesetzt: die Schleife zeigt nur die Meldung
	// Ladeanzeige (loading.go): loading = Stufenwechsel wartet auf das Laden, shown = Draw hat die Anzeige gezeichnet.
	loading, shown bool
	loadMsg        *notice
	pressed        func(ebiten.Key) bool // Taste frisch gedrückt (devkeys.go); nil: keine Entwickler-Tasten
	subs           subImages
	stats          *overlay
	shots          *shooter
	meter          *meter // Messmodus (--measure), sonst nil
	endIn          int    // Takte im Spiel seit Frame.Outcome (Spielende nach view.EndDelay)
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
	var font *art.Font
	if lib, ok := p.Art.(*art.Library); ok && lib != nil {
		font = lib.Font
	}
	g.stats = newOverlay(a.Shots == "", font)
	g.stats.measuring = a.Measure.Seconds > 0
	g.pressed = inpututil.IsKeyJustPressed
	g.applyDevArgs()
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
	if g.fault != nil {
		return g.faultUpdate()
	}
	if g.meter.done() {
		g.mode = ModeQuit
		return g.sim.Close()
	}
	g.advanceTimed()
	if g.err != nil {
		return g.err
	}
	if g.mode == ModeQuit {
		return g.sim.Close()
	}
	return nil
}

// advanceTimed rechnet einen Takt; die Wanduhr läuft nur im Messmodus bzw. mit Leistungsanzeige (clock() ist unter
// Windows ein Syscall mit Heap-Allokation).
func (g *Game) advanceTimed() {
	switch {
	case g.meter != nil:
		t0 := clock()
		g.advance()
		g.meter.tick(clock() - t0)
	case g.stats.timing(&g.frame):
		t0 := g.stats.now()
		g.advance()
		g.stats.upd.add(g.stats.now() - t0)
	default:
		g.advance()
	}
}

// Frame liefert den Frame des letzten Updates (nur lesen, gilt bis zum nächsten Update; Ende-zu-Ende-Test).
func (g *Game) Frame() *view.Frame { return &g.frame }

// Screen liefert den obersten Screen des Menü-Stapels (nil im laufenden Spiel; Ende-zu-Ende-Test).
func (g *Game) Screen() view.Screen { return g.stack.Top() }

// advance liest die Eingabe, schaltet bei ActionFullscreen das Vollbild um, bedient Menü bzw. Spiel, taktet die Simulation einmal und bereitet den Frame vor:
// Geräte, Grafik der Level, Kameras, Vollbild, dann die Listener.
func (g *Game) advance() {
	if g.fault != nil { // Fehler im Fenster: nichts mehr rechnen (der Aufnahme-Modus ruft advance weiter)
		return
	}
	in := g.src.Poll()
	g.cues = g.devs.observe(in, g.frame.Seats, g.cues)
	g.toggleWindow(in)
	g.devKeys()
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
	g.syncLevels()
	g.cells = Cells(g.cells, g.win, &g.frame)
	g.cams.Follow(g.cells, &g.frame)
	g.frame.Cells, g.frame.Recording = g.cells, g.args.Shots != ""
	g.applyWindow()
	for _, l := range g.parts.Listeners {
		l.Listen(&g.frame)
	}
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

// Draw zeichnet den Fehler bzw. die Ladeanzeige, sonst das Spiel (drawGame).
func (g *Game) Draw(screen *ebiten.Image) {
	switch {
	case g.fault != nil:
		g.fault.Draw(screen)
	case g.loading:
		g.drawLoading(screen)
	default:
		g.drawGame(screen)
		return
	}
	if g.shots != nil {
		g.shots.capture(g, screen)
	}
}

// drawGame zeichnet je Zelle die Zellen-Ebenen, dann das Overlay, den obersten Screen und zuletzt die Anzeige oben.
func (g *Game) drawGame(screen *ebiten.Image) {
	var t0, s0 float64
	if g.meter != nil {
		t0 = clock()
	}
	timed := g.stats.timing(&g.frame)
	if timed {
		s0 = g.stats.now()
	}
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
	if timed {
		g.stats.drw.add(g.stats.now() - s0)
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
