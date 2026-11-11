package game

// Aufnahme-Modus (`--shots <ordner>`, B-792/AC-05, Vorbild native/k3c-ebiten/shots.go): Die Szene läuft in New bis
// Scene.Tick vor, dann schreibt der shooter Count Bilder im Abstand von Every Takten als shot-01.png … und zusätzlich
// verkleinert nebeneinander als strip.png, und beendet das Programm. Die Zeit folgt dem Takt: Update rechnet die Takte
// bis zum nächsten Bild und wartet, bis Draw es geschrieben hat; dieselben Schalter ergeben dieselben Bilder.

import (
	"errors"
	"fmt"
	"image"
	"image/color"
	"image/png"
	"log/slog"
	"os"
	"path/filepath"

	"github.com/hajimehoshi/ebiten/v2"
)

const (
	thumbWidth = 320 // höchste Breite eines Bilds im Streifen in px
	stripGap   = 4   // Abstand der Bilder im Streifen in px
	stripName  = "strip.png"
)

var stripBack = color.RGBA{255, 0, 255, 255} // Lücken im Streifen, auffällig

type shooter struct {
	dir     string
	count   int
	every   int
	pending bool // ein Bild ist fällig, Draw schreibt es
	taken   int
	thumbs  []*image.RGBA
	err     error
}

// newShooter legt den Ordner an und räumt Bilder eines früheren Laufs weg.
func newShooter(a Args) (*shooter, error) {
	if err := os.MkdirAll(a.Shots, 0o755); err != nil {
		return nil, err
	}
	old, _ := filepath.Glob(filepath.Join(a.Shots, "shot-*.png"))
	for _, p := range append(old, filepath.Join(a.Shots, stripName)) {
		if err := os.Remove(p); err != nil && !errors.Is(err, os.ErrNotExist) {
			return nil, err
		}
	}
	return &shooter{dir: a.Shots, count: a.Count, every: a.Every, pending: true}, nil
}

// update rechnet die Takte bis zum nächsten Bild; nach dem letzten Bild schreibt es den Streifen und beendet.
func (s *shooter) update(g *Game) error {
	switch {
	case s.err != nil:
		return s.err
	case s.pending:
		return nil
	case s.taken == s.count:
		return s.finish(g)
	}
	for range s.every {
		g.advance()
	}
	if !g.sim.Active() {
		return errors.New("💥 Aufnahme: Spiel nicht mehr aktiv (Absturz oder Niederlage)")
	}
	s.pending = true
	return nil
}

// capture schreibt das fällige Bild aus dem fertig gezeichneten Bildschirm.
func (s *shooter) capture(g *Game, screen *ebiten.Image) {
	if !s.pending || s.err != nil {
		return
	}
	img := image.NewRGBA(screen.Bounds())
	screen.ReadPixels(img.Pix)
	path := filepath.Join(s.dir, fmt.Sprintf("shot-%02d.png", s.taken+1))
	if err := writePNG(path, img); err != nil {
		s.err = err
		return
	}
	slog.Info("💾 Bild geschrieben", "datei", path, "takt", g.frame.Tick)
	s.thumbs = append(s.thumbs, Shrink(img, (img.Bounds().Dx()+thumbWidth-1)/thumbWidth))
	s.taken++
	s.pending = false
}

// finish schreibt den Streifen, beendet das Spiel und das Programm.
func (s *shooter) finish(g *Game) error {
	path := filepath.Join(s.dir, stripName)
	if err := writePNG(path, Strip(s.thumbs, stripGap)); err != nil {
		return err
	}
	if err := g.sim.Close(); err != nil {
		slog.Warn("💥 Spiel nicht sauber beendet", "err", err)
	}
	slog.Info("✅ Aufnahme fertig", "bilder", s.taken, "streifen", path)
	g.mode = ModeQuit
	return ebiten.Termination
}

// Shrink verkleinert src um den ganzzahligen Faktor k (Mittel über k×k Bildpunkte; ein Rest am Rand fällt weg).
func Shrink(src *image.RGBA, k int) *image.RGBA {
	b := src.Bounds()
	if k <= 1 {
		out := image.NewRGBA(image.Rect(0, 0, b.Dx(), b.Dy()))
		copy(out.Pix, src.Pix)
		return out
	}
	out := image.NewRGBA(image.Rect(0, 0, b.Dx()/k, b.Dy()/k))
	for y := range out.Rect.Dy() {
		for x := range out.Rect.Dx() {
			out.SetRGBA(x, y, average(src, b.Min.X+x*k, b.Min.Y+y*k, k))
		}
	}
	return out
}

// average ist das Mittel des k×k-Blocks ab (x0, y0).
func average(src *image.RGBA, x0, y0, k int) color.RGBA {
	var sum [4]int
	for y := y0; y < y0+k; y++ {
		for x := x0; x < x0+k; x++ {
			c := src.RGBAAt(x, y)
			sum[0], sum[1], sum[2], sum[3] = sum[0]+int(c.R), sum[1]+int(c.G), sum[2]+int(c.B), sum[3]+int(c.A)
		}
	}
	n := k * k
	return color.RGBA{uint8(sum[0] / n), uint8(sum[1] / n), uint8(sum[2] / n), uint8(sum[3] / n)}
}

// Strip legt die Bilder nebeneinander, oben bündig, mit gap px Abstand.
func Strip(imgs []*image.RGBA, gap int) *image.RGBA {
	w, h := 0, 0
	for i, m := range imgs {
		if i > 0 {
			w += gap
		}
		w, h = w+m.Bounds().Dx(), max(h, m.Bounds().Dy())
	}
	out := image.NewRGBA(image.Rect(0, 0, max(w, 1), max(h, 1)))
	for i := range out.Pix {
		out.Pix[i] = [4]uint8{stripBack.R, stripBack.G, stripBack.B, stripBack.A}[i%4]
	}
	x := 0
	for _, m := range imgs {
		b := m.Bounds()
		for y := range b.Dy() {
			copy(out.Pix[out.PixOffset(x, y):], m.Pix[m.PixOffset(b.Min.X, b.Min.Y+y):m.PixOffset(b.Max.X, b.Min.Y+y)])
		}
		x += b.Dx() + gap
	}
	return out
}

func writePNG(path string, img image.Image) error {
	f, err := os.Create(path)
	if err != nil {
		return err
	}
	if err := png.Encode(f, img); err != nil {
		f.Close()
		return err
	}
	return f.Close()
}
