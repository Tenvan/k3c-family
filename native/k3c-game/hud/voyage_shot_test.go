package hud

// Aufnahme der Schiffsfahrt im Vollbild (B-977, GS2.3): die Fahrt von Grünhain nach Theben über die Karte der Aufnahme
// der Seekarte, zur Mitte der Fahrzeit. Schreibt ein Bild für die Sichtprüfung und zählt die Allokationen eines Aufrufs
// von DrawVoyage (AC-04): wie bei der Seekarte ein ganzes Vielfaches der Zahl je Zeichenbefehl, das HUD legt darüber
// nichts an. Läuft wie die Aufnahme der Seekarte in TestMain (seekarte_shot_test.go):
//
//	K3C_VOYAGE_SHOT=<bild.png> go test ./hud/   (im Ordner native/k3c-game; Grafik vorher mit `task gen:paper`)

import (
	"errors"
	"fmt"
	"math"
	"os"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

const (
	voyageShotEnv = "K3C_VOYAGE_SHOT"
	voyageShotAt  = 1.6 // Sekunden seit dem Ablegen im Bild
)

type voyageShotGame struct {
	h     *hud
	f     view.Frame
	v     *view.Voyage
	shot  *ebiten.Image
	frame int
	drawN float64 // Allokationen je Aufruf von DrawVoyage
	callN float64 // je einzelnem Zeichenbefehl
	drawn bool
	out   string
	err   error
	flag  bool
}

func (g *voyageShotGame) Layout(int, int) (int, int) { return shotW, shotH }

func (g *voyageShotGame) Update() error {
	if g.frame <= shotWarm {
		return nil
	}
	g.err = writeSeaShot(g.shot, g.out)
	return ebiten.Termination
}

func (g *voyageShotGame) Draw(screen *ebiten.Image) {
	g.frame++
	draw := func() { g.drawn = g.h.DrawVoyage(g.shot, &g.f, g.v) }
	if g.frame == shotWarm {
		g.v.Elapsed = 0 // am Liegeplatz: noch keine Fahne auf der verlassenen Insel
		draw()
		g.flag = g.h.voy.node(g.h.voy.from).kind == seaTarget
		g.v.Elapsed = voyageShotAt
		g.drawN = testing.AllocsPerRun(20, draw)
		g.flag = g.flag && g.h.voy.node(g.h.voy.from).kind == seaDone
		g.callN = testing.AllocsPerRun(200, func() { g.h.paint.fill(g.shot, rect{0, 0, 2, 2}, white) })
	}
	draw()
	screen.DrawImage(g.shot, nil)
}

// runVoyageShot zeichnet die Fahrt, schreibt das Bild nach out und prüft die Allokationen; Rückgabe ist der Exit-Code.
func runVoyageShot(out string) int {
	cfg := art.Config{GrafikDir: repoGrafik}
	if _, err := os.Stat(repoFont); err == nil {
		cfg.FontPath = repoFont
	}
	lib, err := art.New(cfg)
	if err != nil {
		fmt.Println("❌ Grafik:", err)
		return 1
	}
	h := New(lib).(*hud)
	if h.seaMap == nil {
		fmt.Println("❌ Assets der Seekarte fehlen (erst `task gen:paper`)")
		return 1
	}
	if err := lib.SetLevels(map[string][]string{"forest": h.Assets("forest", nil)}); err != nil {
		fmt.Println("❌ Grafik:", err)
		return 1
	}
	h.lang = "de"
	g := &voyageShotGame{h: h, f: view.Frame{Route: shotRoute}, v: voyageOf(), out: out, shot: ebiten.NewImage(shotW, shotH)}
	g.v.Elapsed = voyageShotAt
	if !h.VoyageReady(&g.f, g.v) {
		fmt.Println("❌ Fahrt nicht bereit, obwohl alle Assets geladen sind")
		return 1
	}
	ebiten.SetVsyncEnabled(false)
	ebiten.SetWindowSize(shotW/2, shotH/2)
	ebiten.SetWindowTitle("K3C Schiffsfahrt")
	if err := ebiten.RunGame(g); err != nil && !errors.Is(err, ebiten.Termination) || g.err != nil {
		fmt.Println("❌ Aufnahme:", err, g.err)
		return 1
	}
	fmt.Printf("💾 Bild %s; Allokationen je Aufruf von DrawVoyage %.0f, je Zeichenbefehl %.0f\n", out, g.drawN, g.callN)
	switch {
	case !g.drawn:
		fmt.Println("❌ Fahrt nicht gezeichnet")
	case !g.flag:
		fmt.Println("❌ Fahne der verlassenen Insel: am Liegeplatz schon da oder nach dem Ablegen nicht")
	case g.callN == 0 && g.drawN != 0 || g.callN > 0 && math.Mod(g.drawN, g.callN) != 0:
		fmt.Println("❌ Zeichnen der Fahrt allokiert über die Zeichenbefehle hinaus")
	default:
		return 0
	}
	return 1
}
