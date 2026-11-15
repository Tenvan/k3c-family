package hud

// Aufnahme der Seekarte im Fenster (B-964, GS2.2): zwei Zellen wie bei zwei Spielern, jede mit dem Diorama einer festen
// Karte. Schreibt ein Bild für die Sichtprüfung und zählt die Allokationen eines Aufrufs von drawSea für beide Zellen
// (R6), für Diorama und Formen, dazu die eines einzelnen Zeichenbefehls. Im Fenster allokiert Ebitengine je
// Zeichenbefehl selbst (DrawImage reiht ihn ein); das HUD darf darüber nichts hinzufügen: Die Zahl je Aufruf muss ein
// ganzes Vielfaches der Zahl je Zeichenbefehl sein. Ebitengine braucht den Hauptfaden, deshalb läuft die Aufnahme in
// TestMain statt der Tests:
//
//	K3C_SEA_SHOT=<bild.png> go test ./hud/   (im Ordner native/k3c-game; Grafik vorher mit `task gen:paper`)

import (
	"errors"
	"fmt"
	"image"
	"image/color"
	"image/png"
	"math"
	"os"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

const (
	shotEnv      = "K3C_SEA_SHOT"
	shotW, shotH = 1920, 1080
	shotWarm     = 30 // Bilder, bis Texte, Vorlagen und Puffer von Ebitengine stehen
	repoFont     = "../../../public/schrift/k3c.ttf"
)

func TestMain(m *testing.M) {
	if out := os.Getenv(shotEnv); out != "" {
		os.Exit(runSeaShot(out))
	}
	if out := os.Getenv(voyageShotEnv); out != "" {
		os.Exit(runVoyageShot(out))
	}
	if out := os.Getenv(storyShotEnv); out != "" {
		os.Exit(runStoryShot(out))
	}
	if out := os.Getenv(glyphShotEnv); out != "" {
		os.Exit(runGlyphShot(out))
	}
	os.Exit(m.Run())
}

// Karte der Aufnahme: jede Welt-Miniatur, alle Zustände, drei Reihen in der mittleren Etappe.
var shotRoute = []view.RouteIsland{
	{ID: "gruenhain", Name: "Grünhain", Etappe: 1, World: "standard", Stages: 2, Chunks: 24},
	{ID: "kyoto", Name: "Kyoto", Etappe: 2, World: "sengoku", Stages: 3, Chunks: 40},
	{ID: "theben", Name: "Theben", Etappe: 2, World: "nil", Stages: 5, Chunks: 100},
	{ID: "avalon", Name: "Avalon", Etappe: 2, World: "britannien", Stages: 1, Chunks: 16},
	{ID: "nebelkap", Name: "Nebelkap", Etappe: 3, World: "standard", Stages: 4, Chunks: 64},
	{ID: "edo", Name: "Edo", Etappe: 3, World: "sengoku", Stages: 2},
}

type shotGame struct {
	h     *hud
	cells [2]view.Cell
	shot  *ebiten.Image
	frame int
	paper *art.SeaMap
	// Allokationen je Aufruf von drawSea für beide Zellen (Diorama, Formen) und je einzelnem Zeichenbefehl.
	paperN, shapesN, callN float64
	out                    string
	err                    error
}

func (g *shotGame) Layout(int, int) (int, int) { return shotW, shotH }

func (g *shotGame) Update() error {
	if g.frame <= shotWarm {
		return nil
	}
	g.err = writeSeaShot(g.shot, g.out)
	return ebiten.Termination
}

func (g *shotGame) Draw(screen *ebiten.Image) {
	g.frame++
	g.shot.Fill(color.RGBA{0x3c, 0x6e, 0x47, 0xff}) // Wiese statt Spielwelt, damit die Tafel durchscheint
	g.h.paint.stencils()
	draw := func() {
		for i := range g.cells {
			g.h.drawSea(g.shot, &g.h.seats[i], g.cells[i], scaleOf(g.cells[i], g.h.cells))
		}
	}
	if g.frame == shotWarm { // messen, dann das Bild neu aufbauen
		g.paperN = testing.AllocsPerRun(20, draw)
		g.h.seaMap = nil
		draw()
		g.shapesN = testing.AllocsPerRun(20, draw)
		g.h.seaMap = g.paper
		g.callN = testing.AllocsPerRun(200, func() { g.h.paint.fill(g.shot, rect{0, 0, 2, 2}, white) })
		g.shot.Fill(color.RGBA{0x3c, 0x6e, 0x47, 0xff})
	}
	draw()
	screen.DrawImage(g.shot, nil)
}

// runSeaShot zeichnet die Karte, schreibt das Bild nach out und prüft die Allokationen; Rückgabe ist der Exit-Code.
func runSeaShot(out string) int {
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
	h.cells, h.lang = 2, "de"
	sw := &sim.SwitchState{Open: true, Choice: "theben", Completed: []string{"gruenhain"}, Targets: []sim.SwitchTarget{
		{ID: "kyoto", Name: "Kyoto", Etappe: 2}, {ID: "theben", Name: "Theben", Etappe: 2}, {ID: "avalon", Name: "Avalon", Etappe: 2}}}
	g := &shotGame{h: h, paper: h.seaMap, out: out, shot: ebiten.NewImage(shotW, shotH)}
	for i, profile := range []string{view.ProfileWASD, view.ProfileGamepad} {
		g.cells[i] = view.Cell{Index: i, Player: i, Rect: image.Rect(0, i*shotH/2, shotW, (i+1)*shotH/2)}
		h.seats[i].sea.refresh(h.lang, profile, sw, shotRoute)
	}
	h.seats[1].sea.step([]view.Action{view.ActionLeft}) // Spieler 1 blättert: Cursor und Wahl auf verschiedenen Inseln
	h.seats[1].sea.refresh(h.lang, view.ProfileGamepad, sw, shotRoute)
	ebiten.SetVsyncEnabled(false)
	ebiten.SetWindowSize(shotW/2, shotH/2)
	ebiten.SetWindowTitle("K3C Seekarte")
	if err := ebiten.RunGame(g); err != nil && !errors.Is(err, ebiten.Termination) || g.err != nil {
		fmt.Println("❌ Aufnahme:", err, g.err)
		return 1
	}
	fmt.Printf("💾 Bild %s; Allokationen je Aufruf (beide Zellen): Diorama %.0f, Formen %.0f, je Zeichenbefehl %.0f\n",
		out, g.paperN, g.shapesN, g.callN)
	if g.callN == 0 && g.paperN != 0 || g.callN > 0 && math.Mod(g.paperN, g.callN) != 0 {
		fmt.Println("❌ Zeichnen der Seekarte allokiert über die Zeichenbefehle hinaus")
		return 1
	}
	return 0
}

// writeSeaShot schreibt das Bild img als PNG nach path.
func writeSeaShot(img *ebiten.Image, path string) error {
	b := img.Bounds()
	rgba := image.NewRGBA(b)
	img.ReadPixels(rgba.Pix)
	f, err := os.Create(path)
	if err != nil {
		return err
	}
	if err := png.Encode(f, rgba); err != nil {
		_ = f.Close()
		return err
	}
	return f.Close()
}
