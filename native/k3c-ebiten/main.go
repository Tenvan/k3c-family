// K3C Ebitengine-Prototyp (Projekt NAT, Sprint NV1): derselbe Wald-Level aus Papier-Sheets wie der Bevy-Prototyp
// (native/k3c-proto), zwei Monarchen an zwei Gamepads im Split-Screen, Messmodus für FPS und Texturspeicher.
// Keine Spiel-Logik, kein Netz. Zweck: direkter Vergleich der Messwerte beider Engines.
//
// Aufruf (über `task native:*`): k3c-ebiten [--root <repo>] [--mode level|alle] [--measure <s>] [--warmup <s>]
// [--report <datei>] [--novsync] [--auto]; zusätzlich nur hier: [--shots <ordner>] (Screenshots an zwei Kamera-Positionen, NV3.1), [--unmanaged] (Gegenversuch: je Sheet eine eigene Textur statt Ebitengines Atlas).
package main

import (
	"errors"
	"flag"
	"fmt"
	"image"
	"image/color"
	"io"
	"math"
	"os"
	"path/filepath"

	"github.com/hajimehoshi/ebiten/v2"
)

// Fenstergröße in Bildpunkten des Bildschirms (wie der Bevy-Prototyp)
const (
	winW = 1920
	winH = 1080
)

var (
	started = clock()
	sky     = color.RGBA{158, 199, 219, 255} // sRGB 0.62/0.78/0.86
)

type Args struct {
	root, mode, report, shots string
	measureS, warmupS      float64
	vsync, auto, unmanaged bool
}

func logf(format string, a ...any) { fmt.Printf(format+"\n", a...) }

func fatal(format string, a ...any) {
	fmt.Fprintf(os.Stderr, format+"\n", a...)
	os.Exit(1)
}

func parseArgs() Args {
	a := Args{}
	novsync := false
	fs := flag.NewFlagSet("k3c-ebiten", flag.ContinueOnError)
	fs.SetOutput(io.Discard)
	fs.StringVar(&a.root, "root", ".", "")
	fs.StringVar(&a.mode, "mode", "level", "")
	fs.Float64Var(&a.measureS, "measure", 0, "")
	fs.Float64Var(&a.warmupS, "warmup", 5, "")
	fs.StringVar(&a.report, "report", "", "")
	fs.BoolVar(&novsync, "novsync", false, "")
	fs.BoolVar(&a.auto, "auto", false, "")
	fs.BoolVar(&a.unmanaged, "unmanaged", false, "")
	fs.StringVar(&a.shots, "shots", "", "")
	if err := fs.Parse(os.Args[1:]); err != nil || fs.NArg() > 0 {
		fatal("🚫 Ungültige Argumente (%v %v); gültig: --root --mode --measure --warmup --report --novsync --auto --shots --unmanaged", err, fs.Args())
	}
	if a.mode != "level" && a.mode != "alle" {
		fatal("🚫 --mode level|alle, nicht %s", a.mode)
	}
	a.vsync = !novsync
	return a
}

// Game hält alles zusammen: Sheets, Level, Messung und die beiden Bildschirmhälften.
type Game struct {
	args    Args
	paper   *Paper
	level   *Level
	river   *River
	shots   *Shots
	measure *Measure
	w, h    int
	views   [2]*ebiten.Image
	viewsOf *ebiten.Image
	last    float64
	elapsed float64
	quit    bool
}

// Update läuft einmal je Frame (TPS = SyncWithFPS): fertige Sheets hochladen, dann die Welt um die echte Zeit weiterdrehen.
func (g *Game) Update() error {
	if g.quit {
		return ebiten.Termination
	}
	if err := g.paper.pump(); err != nil {
		return err
	}
	now := clock()
	dt := math.Min(now-g.last, 0.25) // gekappt wie Bevys Spielzeit, nur für die Bewegung
	g.last = now
	g.elapsed += dt
	if g.shots != nil {
		g.shots.update(g)
	}
	return g.level.update(g.paper, dt, g.elapsed, g.args.auto)
}

func (g *Game) Draw(screen *ebiten.Image) {
	screen.Fill(sky)
	for i := range 2 {
		g.drawView(screen, i)
	}
	g.paper.prime(screen)
	if g.shots != nil {
		g.shots.capture(g, screen)
	}
	info := RunInfo{sheets: g.paper.count(), bytes: g.paper.bytes, size: [2]int{g.w, g.h}, unmanaged: g.args.unmanaged}
	if g.measure.frame(clock(), g.paper.settled(), info) {
		g.quit = true
	}
}

// Layout: Zeichenfläche in echten Bildpunkten (Fenstergröße × Skalierung des Bildschirms), damit nichts hochskaliert wird.
func (g *Game) Layout(ow, oh int) (int, int) {
	s := scaleFactor()
	g.w, g.h = int(math.Round(float64(ow)*s)), int(math.Round(float64(oh)*s))
	return g.w, g.h
}

func scaleFactor() float64 {
	if s := ebiten.Monitor().DeviceScaleFactor(); s > 0 {
		return s
	}
	return 1
}

// view liefert die Bildschirmhälfte des Spielers als Ausschnitt (oben Spieler 1, unten Spieler 2); Zeichnen wird dort beschnitten.
func (g *Game) view(screen *ebiten.Image, i int) *ebiten.Image {
	half := g.h / 2
	if g.viewsOf != screen || g.views[0].Bounds().Dx() != g.w || g.views[0].Bounds().Dy() != half {
		for k := range 2 {
			g.views[k] = screen.SubImage(image.Rect(0, k*half, g.w, (k+1)*half)).(*ebiten.Image)
		}
		g.viewsOf = screen
	}
	return g.views[i]
}

// drawView zeichnet eine Sicht: obere Hälfte Welt (Boden-Linie an der Mitte), untere Hälfte Fluss vor allem anderen.
// Sichtbare Welthöhe über dem Boden: worldViewH. Die Schicht `fern` steht (Scroll-Faktor 0), `mitte` und `nah` scrollen.
func (g *Game) drawView(screen *ebiten.Image, i int) {
	view := g.view(screen, i)
	half := float64(g.h / 2)
	s := half / 2 / worldViewH
	cam := g.level.players[i].sprite.x
	cx := float64(g.w) / 2
	groundY := float64(i)*half + half/2
	for _, sp := range g.level.sprites {
		if sp.player >= 0 && sp.player != i {
			continue
		}
		x := sp.x
		if sp.player >= 0 {
			x = sp.baseX + cam*(1-sp.scroll)
		}
		sp.draw(view, (x-cam)*s+cx, groundY, s, float64(g.w))
	}
	g.river.draw(view, groundY+bankH*s, float64(i+1)*half, float64(g.w), s, cam, g.elapsed)
}

func main() {
	args := parseArgs()
	grafik, err := filepath.Abs(filepath.Join(args.root, "public", "grafik"))
	if st, serr := os.Stat(grafik); err != nil || serr != nil || !st.IsDir() {
		fatal("💥 public/grafik fehlt unter %s. Erst `task gen:paper` ausführen.", args.root)
	}
	logf("🚀 K3C Ebitengine-Prototyp, Modus %s, Grafik %s", args.mode, grafik)
	paper := NewPaper(grafik, args.unmanaged)
	level, err := buildLevel(paper, args.mode == "alle")
	if err != nil {
		fatal("💥 %v", err)
	}
	g := &Game{args: args, paper: paper, level: level, last: clock(), measure: NewMeasure(args), river: newRiver()}
	if args.shots != "" {
		g.shots = NewShots(args.shots)
	}

	s := scaleFactor()
	ebiten.SetWindowSize(int(math.Round(winW/s)), int(math.Round(winH/s)))
	ebiten.SetWindowResizingMode(ebiten.WindowResizingModeEnabled)
	ebiten.SetWindowTitle(fmt.Sprintf("K3C Ebitengine-Prototyp (%s)", args.mode))
	ebiten.SetVsyncEnabled(args.vsync)
	// Update im Takt der Bilder: Ohne VSync läuft Draw damit unbegrenzt, die Bewegung rechnet mit der echten Zeit.
	ebiten.SetTPS(ebiten.SyncWithFPS)
	ebiten.SetRunnableOnUnfocused(true)
	ebiten.SetScreenClearedEveryFrame(false) // Draw füllt den Himmel selbst
	if err := ebiten.RunGame(g); err != nil && !errors.Is(err, ebiten.Termination) {
		fatal("💥 Abbruch: %v", err)
	}
}
