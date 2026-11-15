package hud

// Aufnahme der Geschichtszeile (B-974, GS1.3): zwei Spielerhälften bei 1920 × 1080, oben ein Eintrag aus zwei Zeilen,
// unten derselbe; das HUD darüber wie im Spiel, ohne Welt (einfarbiger Grund). Schreibt ein Bild für die Sichtprüfung
// und zählt die Allokationen des Zeichnens einer Zelle mit laufender Einblendung: wie bei der Seekarte ein ganzes
// Vielfaches der Zahl je Zeichenbefehl. Läuft wie die Aufnahme der Seekarte in TestMain (seekarte_shot_test.go):
//
//	K3C_STORY_SHOT=<bild.png> go test ./hud/   (im Ordner native/k3c-game)

import (
	"errors"
	"fmt"
	"image"
	"image/color"
	"math"
	"os"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

const (
	storyShotEnv = "K3C_STORY_SHOT"
	storyShotAt  = 2.0 // Sekunden seit Beginn der Einblendung im Bild (steht)
)

type storyShotGame struct {
	h     *hud
	f     *view.Frame
	cells [2]view.Cell
	shot  *ebiten.Image
	frame int
	drawN float64 // Allokationen je Aufruf von Draw (beide Zellen)
	callN float64 // je einzelnem Zeichenbefehl
	out   string
	err   error
}

func (g *storyShotGame) Layout(int, int) (int, int) { return shotW, shotH }

func (g *storyShotGame) Update() error {
	if g.frame <= shotWarm {
		return nil
	}
	g.err = writeSeaShot(g.shot, g.out)
	return ebiten.Termination
}

func (g *storyShotGame) Draw(screen *ebiten.Image) {
	g.frame++
	draw := func() {
		for i := range g.cells {
			g.h.Draw(g.shot, g.f, g.cells[i])
		}
	}
	g.shot.Fill(color.RGBA{0x2c, 0x4a, 0x3a, 255})
	if g.frame == shotWarm {
		g.drawN = testing.AllocsPerRun(20, draw)
		g.callN = testing.AllocsPerRun(200, func() { g.h.paint.fill(g.shot, rect{0, 0, 2, 2}, white) })
		g.shot.Fill(color.RGBA{0x2c, 0x4a, 0x3a, 255})
	}
	draw()
	screen.DrawImage(g.shot, nil)
}

// storyShotFrame: zwei Spieler auf einer Insel, neue Kampagne der Testwelt.
func storyShotFrame() (*view.Frame, error) {
	isl, err := sim.CreateIsland("wald", []int{0}, 1, sim.CreateOptions{})
	if err != nil {
		return nil, err
	}
	a, b := sim.AddIslandPlayer(isl, 0), sim.AddIslandPlayer(isl, 0)
	return &view.Frame{
		Stages:   []view.Stage{{Index: 0, World: isl.Stages[0]}},
		Seats:    []view.Seat{{Player: 0, Device: keys1, Monarch: a.Index}, {Player: 1, Device: pad, Monarch: b.Index}},
		Campaign: view.Campaign{Run: 1, World: storyWorld(), Island: "a", Fresh: true},
	}, nil
}

// runStoryShot zeichnet die Einblendung, schreibt das Bild nach out und prüft die Allokationen; Rückgabe ist der
// Exit-Code.
func runStoryShot(out string) int {
	cfg := art.Config{GrafikDir: repoGrafik}
	if _, err := os.Stat(repoFont); err == nil {
		cfg.FontPath = repoFont
	}
	lib, err := art.New(cfg)
	if err != nil {
		fmt.Println("❌ Grafik:", err)
		return 1
	}
	f, err := storyShotFrame()
	if err != nil {
		fmt.Println("❌ Insel:", err)
		return 1
	}
	h := New(lib).(*hud)
	h.Listen(f)
	f.Campaign.Fresh = false
	f.Clock = storyShotAt
	h.Listen(f)
	g := &storyShotGame{h: h, f: f, out: out, shot: ebiten.NewImage(shotW, shotH)}
	for i := range g.cells {
		g.cells[i] = view.Cell{Index: i, Player: i, Rect: image.Rect(0, i*shotH/2, shotW, (i+1)*shotH/2)}
	}
	ebiten.SetVsyncEnabled(false)
	ebiten.SetWindowSize(shotW/2, shotH/2)
	ebiten.SetWindowTitle("K3C Geschichtszeile")
	if err := ebiten.RunGame(g); err != nil && !errors.Is(err, ebiten.Termination) || g.err != nil {
		fmt.Println("❌ Aufnahme:", err, g.err)
		return 1
	}
	fmt.Printf("💾 Bild %s; Allokationen je Draw (beide Zellen) %.0f, je Zeichenbefehl %.0f\n", out, g.drawN, g.callN)
	switch {
	case h.seats[0].story.showing.b == "" || h.seats[0].story.showing != h.seats[1].story.showing:
		fmt.Println("❌ Einblendung fehlt oder ist je Spieler verschieden")
	case g.callN == 0 && g.drawN != 0 || g.callN > 0 && math.Mod(g.drawN, g.callN) != 0:
		fmt.Println("❌ Zeichnen der Geschichtszeile allokiert über die Zeichenbefehle hinaus")
	default:
		return 0
	}
	return 1
}
