package main

// Screenshot-Modus (`--shots <ordner>`, NV3.1): stellt die Kamera nacheinander auf zwei Positionen, wartet bis alle Bilder
// geladen und gezeichnet sind, speichert den Bildschirm als PNG und beendet. Nachweis für B-789 (AC-01, AC-02).

import (
	"image"
	"image/png"
	"os"
	"path/filepath"

	"github.com/hajimehoshi/ebiten/v2"
)

var shotList = []struct {
	file string
	cam  [2]float64 // Welt-x der beiden Monarchen (= Kamera je Sicht)
}{
	{"01-kamera-1000", [2]float64{1000, 1800}},
	{"02-kamera-2200", [2]float64{2200, 3000}},
}

const shotWait = 20 // Frames Ruhe nach dem Einstellen (Fluss läuft weiter, Sheets sind geladen)

type Shots struct {
	dir  string
	next int
	wait int
}

func NewShots(dir string) *Shots { return &Shots{dir: dir, wait: -1} }

// update stellt die Kamera des aktuellen Bilds ein (läuft vor dem Spielschritt).
func (s *Shots) update(g *Game) {
	if s.next >= len(shotList) {
		return
	}
	for i, x := range shotList[s.next].cam {
		g.level.players[i].sprite.x = x
	}
}

// capture speichert nach shotWait gezeichneten Frames den Bildschirm und schaltet zum nächsten Bild.
func (s *Shots) capture(g *Game, screen *ebiten.Image) {
	if s.next >= len(shotList) || !g.paper.settled() {
		return
	}
	s.wait++
	if s.wait < shotWait {
		return
	}
	b := screen.Bounds()
	img := image.NewRGBA(b)
	screen.ReadPixels(img.Pix)
	path := filepath.Join(s.dir, shotList[s.next].file+".png")
	if err := writePNG(path, img); err != nil {
		logf("💥 %v", err)
		g.quit = true
		return
	}
	logf("💾 %s", path)
	s.next, s.wait = s.next+1, -1
	if s.next == len(shotList) {
		logf("✅ %d Screenshots in %s", len(shotList), s.dir)
		g.quit = true
	}
}

func writePNG(path string, img image.Image) error {
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	f, err := os.Create(path)
	if err != nil {
		return err
	}
	defer f.Close()
	return png.Encode(f, img)
}
