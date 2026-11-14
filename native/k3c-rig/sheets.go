package main

// Papier-Sheets des Monarchen zum Vergleich (Entscheidung 005): public/grafik/figuren/monarch/manifest.json und die
// PNG-Streifen daneben (entstehen durch `task gen:paper`, nicht eingecheckt). Fehlen sie, läuft der Prototyp ohne.

import (
	"encoding/json"
	"fmt"
	"image"
	"os"
	"path/filepath"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-rig/paint"
)

type sheetAnim struct {
	Frames int         `json:"frames"`
	Datei  string      `json:"datei"`
	Rahmen *[4]float64 `json:"rahmen"`
	FrameW *int        `json:"frameW"`
	FrameH *int        `json:"frameH"`
}

type sheetManifest struct {
	Rahmen      [4]float64           `json:"rahmen"`
	FrameW      int                  `json:"frameW"`
	FrameH      int                  `json:"frameH"`
	Animationen map[string]sheetAnim `json:"animationen"`
}

// Sheet ist ein Streifen aus Frames mit seinem Rahmen in Einheiten.
type Sheet struct {
	pix    *image.RGBA // vormultipliziert, für den Bildvergleich
	frames []*ebiten.Image
	frame  [4]float64 // Rahmen in Einheiten: x, y, Breite, Höhe
	fw, fh int
	bytes  int64
}

type Sheets struct {
	dir      string
	manifest *sheetManifest
	sheets   map[string]*Sheet
	bytes    int64
}

// LoadSheets liest das Manifest des Monarchen; ohne Manifest gibt es nil und den Grund.
func LoadSheets(root string) (*Sheets, error) {
	dir := filepath.Join(root, "public", "grafik", "figuren", "monarch")
	text, err := os.ReadFile(filepath.Join(dir, "manifest.json"))
	if err != nil {
		return nil, fmt.Errorf("Sheets fehlen unter %s (für den Vergleich erst `task gen:paper`): %w", dir, err)
	}
	m := &sheetManifest{}
	if err := json.Unmarshal(text, m); err != nil {
		return nil, fmt.Errorf("Manifest kaputt: %s: %w", dir, err)
	}
	return &Sheets{dir: dir, manifest: m, sheets: map[string]*Sheet{}}, nil
}

// sheet lädt den Streifen einer Animation in einer Variante (`trab@spieler2`) beim ersten Zugriff.
func (s *Sheets) sheet(anim, variant string) (*Sheet, error) {
	key := anim + "@" + variant
	if sh, ok := s.sheets[key]; ok {
		return sh, nil
	}
	a, ok := s.manifest.Animationen[key]
	if !ok {
		return nil, fmt.Errorf("Animation %s fehlt im Manifest des Monarchen", key)
	}
	sh := &Sheet{frame: s.manifest.Rahmen, fw: s.manifest.FrameW, fh: s.manifest.FrameH}
	if a.Rahmen != nil {
		sh.frame = *a.Rahmen
	}
	if a.FrameW != nil {
		sh.fw = *a.FrameW
	}
	if a.FrameH != nil {
		sh.fh = *a.FrameH
	}
	pix, err := paint.ReadPNG(filepath.Join(s.dir, a.Datei))
	if err != nil {
		return nil, fmt.Errorf("Sheet %s nicht geladen: %w", a.Datei, err)
	}
	if pix.Bounds().Dx() < a.Frames*sh.fw || pix.Bounds().Dy() < sh.fh || a.Frames < 1 {
		return nil, fmt.Errorf("Sheet %s ist zu klein für %d Frames à %d×%d", a.Datei, a.Frames, sh.fw, sh.fh)
	}
	img := ebiten.NewImageFromImage(pix)
	for i := range a.Frames {
		sh.frames = append(sh.frames, img.SubImage(image.Rect(i*sh.fw, 0, (i+1)*sh.fw, sh.fh)).(*ebiten.Image))
	}
	sh.pix, sh.bytes = pix, int64(len(pix.Pix))
	s.bytes += sh.bytes
	s.sheets[key] = sh
	return sh, nil
}

// draw zeichnet Frame n so, dass der Ursprung der Figur (Füße) bei (x, y) liegt; scale = Bildpunkte je Einheit.
func (sh *Sheet) draw(dst *ebiten.Image, n int, x, y, scale float64, left bool) {
	op := &ebiten.DrawImageOptions{Filter: ebiten.FilterLinear}
	sx, sy := scale*sh.frame[2]/float64(sh.fw), scale*sh.frame[3]/float64(sh.fh)
	op.GeoM.Scale(sx, sy)
	op.GeoM.Translate(scale*sh.frame[0], scale*sh.frame[1])
	if left {
		op.GeoM.Scale(-1, 1)
	}
	op.GeoM.Translate(x, y)
	dst.DrawImage(sh.frames[n%len(sh.frames)], op)
}
