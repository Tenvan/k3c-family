package art

import (
	"image"

	"github.com/hajimehoshi/ebiten/v2"
)

// Sheet ist eine Animation als Streifen auf der GPU; Frames sind Teilbilder desselben Bildes (Atlas, ein Zeichenaufruf
// je Sheet reicht für die Stapelung). Nur lesen; Laden und Freigeben macht die Library.
type Sheet struct {
	Asset, Anim string
	Frames      []*ebiten.Image
	FPS         float64
	FW, FH      int
	// Fußpunkt als Anteil des Frames (von links oben); Rahmen-Einheiten je Bild-Pixel
	OriginX, OriginY, UnitsPerPx float64

	img *ebiten.Image
}

// drawOp: lineares Filtern, keine Mipmaps (wie der Prototyp: sonst Ruckler und mehr Grafikspeicher beim ersten
// verkleinerten Zeichnen). Eine Instanz für alle Aufrufe, Zeichnen läuft im Hauptfaden.
var drawOp = ebiten.DrawImageOptions{Filter: ebiten.FilterLinear, DisableMipmaps: true}

// Frame liefert Bild i; der Index läuft zyklisch (auch negativ), ein leeres Sheet liefert nil.
func (s *Sheet) Frame(i int) *ebiten.Image {
	n := len(s.Frames)
	if n == 0 {
		return nil
	}
	return s.Frames[((i%n)+n)%n]
}

// FrameAt liefert das Bild zur Zeit t (Sekunden) bei der Bildrate des Sheets.
func (s *Sheet) FrameAt(t float64) *ebiten.Image {
	if s.FPS <= 0 {
		return s.Frame(0)
	}
	return s.Frame(int(t * s.FPS))
}

// Draw zeichnet Bild i mit der Matrix g (Bild-Pixel → Ziel) in dst, ohne Allokation.
func (s *Sheet) Draw(dst *ebiten.Image, i int, g ebiten.GeoM) {
	img := s.Frame(i)
	if img == nil {
		return
	}
	drawOp.GeoM = g
	dst.DrawImage(img, &drawOp)
}

// upload legt das Bild aus vormultiplizierten RGBA-Pixeln an und schneidet die Frames aus.
func upload(spec SheetSpec, pix []byte) *Sheet {
	img := ebiten.NewImage(spec.W, spec.H)
	img.WritePixels(pix)
	s := &Sheet{
		Asset: spec.Asset, Anim: spec.Anim, FPS: spec.FPS, FW: spec.FW, FH: spec.FH, img: img,
		OriginX: spec.OriginX, OriginY: spec.OriginY, UnitsPerPx: spec.UnitsPerPx, Frames: make([]*ebiten.Image, spec.Frames),
	}
	for i := range spec.Frames {
		s.Frames[i] = img.SubImage(image.Rect(i*spec.FW, 0, (i+1)*spec.FW, spec.FH)).(*ebiten.Image)
	}
	return s
}
