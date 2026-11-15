package game

// Ladebild im Papierschnitt-Stil (B-949): Papier-Hintergrund mit zwei Hügel-Streifen, der Krieger aus den Menü-
// Illustrationen (immer geladen, wenn ein Stufenwechsel anliegt), Titel, Fortschrittsbalken und Prozent. Ohne Schrift oder
// ohne Krieger zeichnet drawLoading die schlichte Meldung (notice.go). Nach dem ersten Bild entsteht nichts mehr, nur der
// Prozent-Text je Prozentschritt.

import (
	"image/color"
	"math"
	"strconv"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"

	"k3c/native/k3c-game/art"
)

// loadFigure ist die Papierfigur im Ladebild (Menü-Illustration, Animation idle).
const loadFigure = "truppen/krieger"

// loadTitle steht über dem Ladebild.
const loadTitle = "Family Three Crowns"

var (
	loadPaper  = color.RGBA{238, 226, 198, 255}
	loadHillA  = color.RGBA{136, 160, 112, 255}
	loadHillB  = color.RGBA{88, 121, 84, 255}
	loadShade  = color.RGBA{0, 0, 0, 56}
	loadInk    = color.RGBA{74, 56, 36, 255}
	loadTrack  = color.RGBA{214, 196, 160, 255}
	loadGold   = color.RGBA{226, 176, 52, 255}
	loadLights = color.RGBA{255, 255, 255, 90}
)

// loadScreen hält, was das Ladebild über die Bilder merkt.
type loadScreen struct {
	op    ebiten.DrawImageOptions
	tick  int
	pct   int
	label string // „NN %“, neu bei jedem Prozentschritt
}

// percent rundet den Anteil p auf ganze Prozent (0 … 100).
func percent(p float64) int { return int(math.Round(math.Min(1, math.Max(0, p)) * 100)) }

// draw zeichnet das Ladebild mit Fortschritt p (0 … 1); false = nichts gezeichnet (keine Schrift oder keine Figur),
// der Aufrufer nimmt die Meldung.
func (s *loadScreen) draw(dst *ebiten.Image, lib *art.Library, p float64) bool {
	if lib == nil || lib.Font == nil || lib.Font.Fallback {
		return false
	}
	sh := lib.Sheet(loadFigure, "idle")
	if sh == nil || sh.Frame(0) == nil {
		return false
	}
	if n := percent(p); s.label == "" || s.pct != n {
		s.pct, s.label = n, strconv.Itoa(n)+" %"
	}
	s.tick++
	b := dst.Bounds()
	x0, y0, w, h := float32(b.Min.X), float32(b.Min.Y), float32(b.Dx()), float32(b.Dy())
	dst.Fill(loadPaper)
	// Hügel als Papierstreifen mit Schatten, hinten hell, vorn dunkel
	vector.FillRect(dst, x0, y0+h*0.66, w, h*0.34, loadHillA, false)
	vector.FillRect(dst, x0, y0+h*0.66, w, h*0.012, loadLights, false)
	vector.FillRect(dst, x0, y0+h*0.80-h*0.012, w, h*0.012, loadShade, false)
	vector.FillRect(dst, x0, y0+h*0.80, w, h*0.20, loadHillB, false)
	// Figur: Fuß auf dem hinteren Hügel, wippt leicht
	img := sh.Frame(0)
	ib := img.Bounds()
	k := float64(h) * 0.34 / float64(ib.Dy())
	bob := math.Sin(float64(s.tick)/9) * float64(h) * 0.008
	s.op.GeoM.Reset()
	s.op.GeoM.Scale(k, k)
	s.op.GeoM.Translate(float64(x0)+(float64(w)-float64(ib.Dx())*k)/2, float64(y0)+float64(h)*0.72-float64(ib.Dy())*k+bob)
	s.op.Filter = ebiten.FilterLinear
	dst.DrawImage(img, &s.op)
	// Titel und Balken
	f, size := lib.Font, float64(h)*0.07
	tw, _ := f.Measure(loadTitle, size)
	tx, ty := float64(x0)+(float64(w)-tw)/2, float64(y0)+float64(h)*0.07
	f.Draw(dst, loadTitle, tx+3, ty+3, size, loadShade)
	f.Draw(dst, loadTitle, tx, ty, size, loadInk)
	bw, bh := w*0.5, h*0.04
	bx, by := x0+(w-bw)/2, y0+h*0.86
	vector.FillRect(dst, bx+4, by+4, bw, bh, loadShade, false)
	vector.FillRect(dst, bx-4, by-4, bw+8, bh+8, loadInk, false)
	vector.FillRect(dst, bx, by, bw, bh, loadTrack, false)
	vector.FillRect(dst, bx, by, bw*float32(s.pct)/100, bh, loadGold, false)
	ps := float64(h) * 0.035
	lw, _ := f.Measure(s.label, ps)
	f.Draw(dst, s.label, float64(bx)+(float64(bw)-lw)/2, float64(by+bh)+float64(h)*0.015, ps, loadPaper)
	return true
}
