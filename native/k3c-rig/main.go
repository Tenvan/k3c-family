// K3C Rig-Prototyp (Projekt NAT, Sprint NV2, B-778): Monarch und Pferd zur Laufzeit aus Rig-Teilen statt aus Sheets,
// daneben die Sheets zum Vergleich. Zwei Monarchen (zwei Gamepads oder Tastatur), keine Spiel-Logik, kein Netz.
//
// Aufruf (über `task native:rig*`): k3c-rig [--root <repo>] [--assets <ordner>] [--shots <ordner>] [--vergleich <ordner>]
// [--measure <s>] [--warmup <s>] [--figuren <n>] [--art rig|sheet] [--report <datei>] [--novsync]
package main

import (
	"errors"
	"flag"
	"fmt"
	"image/color"
	"io"
	"math"
	"os"
	"path/filepath"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-rig/paint"
)

// Fenstergröße in Bildpunkten des Bildschirms (wie die Prototypen aus NV1)
const (
	winW = 1920
	winH = 1080
)

var (
	started = clock()
	sky     = color.RGBA{158, 199, 219, 255}
)

type Args struct {
	root, assets, shots, compare, report, kind string
	measureS, warmupS                          float64
	figures                                    int
	vsync                                      bool
}

func logf(format string, a ...any) { fmt.Printf(format+"\n", a...) }

func fatal(format string, a ...any) {
	fmt.Fprintf(os.Stderr, format+"\n", a...)
	os.Exit(1)
}

func parseArgs() Args {
	a := Args{}
	novsync := false
	fs := flag.NewFlagSet("k3c-rig", flag.ContinueOnError)
	fs.SetOutput(io.Discard)
	fs.StringVar(&a.root, "root", ".", "")
	fs.StringVar(&a.assets, "assets", "", "")
	fs.StringVar(&a.shots, "shots", "", "")
	fs.StringVar(&a.compare, "vergleich", "", "")
	fs.Float64Var(&a.measureS, "measure", 0, "")
	fs.Float64Var(&a.warmupS, "warmup", 3, "")
	fs.IntVar(&a.figures, "figuren", 2, "")
	fs.StringVar(&a.kind, "art", "rig", "")
	fs.StringVar(&a.report, "report", "", "")
	fs.BoolVar(&novsync, "novsync", false, "")
	if err := fs.Parse(os.Args[1:]); err != nil || fs.NArg() > 0 {
		fatal("🚫 Ungültige Argumente (%v %v); gültig: --root --assets --shots --vergleich --measure --warmup --figuren --art --report --novsync", err, fs.Args())
	}
	if a.kind != "rig" && a.kind != "sheet" {
		fatal("🚫 --art rig|sheet, nicht %s", a.kind)
	}
	if a.figures < 1 || a.figures > 2000 {
		fatal("🚫 --figuren 1 bis 2000, nicht %d", a.figures)
	}
	a.vsync = !novsync
	return a
}

// assetsDir: genannter Ordner, sonst native/k3c-rig/assets im Repo, sonst `assets` neben der EXE (Paket für den Mini-PC).
func assetsDir(a Args) string {
	if a.assets != "" {
		return a.assets
	}
	repo := filepath.Join(a.root, "native", "k3c-rig", "assets")
	if st, err := os.Stat(repo); err == nil && st.IsDir() {
		return repo
	}
	if exe, err := os.Executable(); err == nil {
		return filepath.Join(filepath.Dir(exe), "assets")
	}
	return repo
}

// Scene ist ein Betriebsmodus: Vorführen, Screenshots, Bildvergleich oder Messung.
type Scene interface {
	Update(g *Game, dt float64) error
	Draw(g *Game, screen *ebiten.Image)
}

// Game hält die gemeinsamen Dinge aller Modi: Rig-Bilder, Sheets (nil, wenn nicht gerendert), Zeichner, Fenstermaße.
type Game struct {
	args    Args
	assets  *Assets
	sheets  *Sheets
	painter *paint.Painter
	scene   Scene
	w, h    int
	last    float64
	quit    bool
}

func (g *Game) Update() error {
	if g.quit {
		return ebiten.Termination
	}
	now := clock()
	dt := math.Min(now-g.last, 0.25)
	g.last = now
	return g.scene.Update(g, dt)
}

func (g *Game) Draw(screen *ebiten.Image) {
	screen.Fill(sky)
	g.scene.Draw(g, screen)
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

// newScene wählt den Modus aus den Argumenten.
func newScene(g *Game) (Scene, error) {
	switch {
	case g.args.compare != "":
		return NewCompare(g)
	case g.args.measureS > 0:
		return NewMeasure(g)
	}
	demo, err := NewDemo(g)
	if err != nil || g.args.shots == "" {
		return demo, err
	}
	return NewShots(demo, g.args.shots), nil
}

func main() {
	args := parseArgs()
	dir := assetsDir(args)
	logf("🚀 K3C Rig-Prototyp, Teile aus %s", dir)
	assets, err := LoadAssets(dir)
	if err != nil {
		fatal("💥 %v", err)
	}
	logf("📂 %d Teil-Bilder geladen (%.2f MiB RGBA)", len(assets.Images), mib(assets.Bytes))
	sheets, err := LoadSheets(args.root)
	if err != nil {
		logf("📄 Ohne Sheet-Vergleich: %v", err)
	}
	g := &Game{args: args, assets: assets, sheets: sheets, painter: paint.NewPainter(assets.White), last: clock()}
	if g.scene, err = newScene(g); err != nil {
		fatal("💥 %v", err)
	}
	s := scaleFactor()
	ebiten.SetWindowSize(int(math.Round(winW/s)), int(math.Round(winH/s)))
	ebiten.SetWindowResizingMode(ebiten.WindowResizingModeEnabled)
	ebiten.SetWindowTitle("K3C Rig-Prototyp: links Sheet, rechts Rig-Teile")
	ebiten.SetVsyncEnabled(args.vsync)
	// Update im Takt der Bilder: Ohne VSync läuft Draw unbegrenzt, die Bewegung rechnet mit der echten Zeit.
	ebiten.SetTPS(ebiten.SyncWithFPS)
	ebiten.SetRunnableOnUnfocused(true)
	ebiten.SetScreenClearedEveryFrame(false) // Draw füllt den Himmel selbst
	if err := ebiten.RunGame(g); err != nil && !errors.Is(err, ebiten.Termination) {
		fatal("💥 Abbruch: %v", err)
	}
}

func mib(b int64) float64 { return float64(b) / (1024 * 1024) }
