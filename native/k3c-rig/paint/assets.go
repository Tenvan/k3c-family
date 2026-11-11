// Package paint lädt die Bilder der Rig-Figuren und zeichnet sie mit Ebitengine (Rig-Prototyp k3c-rig und
// Level-Prototyp k3c-ebiten): Teil-PNGs, Bänder und Elemente aus native/k3c-rig/assets (eingecheckt, kein Gen-Lauf nötig).
package paint

import (
	"fmt"
	"image"
	"image/color"
	"image/draw"
	"image/png"
	"os"
	"path/filepath"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-rig/rig"
)

// Skin sind die Bilder eines Rigs in einer Variante: je Teil und je Band (nil = ohne Bild).
type Skin struct {
	rig      *rig.Rig
	parts    []*ebiten.Image
	bands    []*ebiten.Image
	bandPart []int // Teil, in dessen Rahmen die Stützpunkte des Bands liegen (-1 = Welt)
}

// ElementImage ist ein optionales Element mit seinem Bild.
type ElementImage struct {
	rig.Element
	img *ebiten.Image
}

// Assets hält Rigs, Elemente und Bilder; jedes Bild wird nur einmal geladen.
type Assets struct {
	Dir      string
	Images   map[string]*ebiten.Image
	Elements map[string]*ElementImage
	Weapons  []string // Auswahl am Platz `waffe`, "" = leer (zuletzt)
	Armors   []string // Auswahl am Platz `ruestung`, "" = leer (zuerst)
	White    *ebiten.Image
	Bytes    int64 // Summe aller Bilder als RGBA8
	rigs     map[string]*rig.Rig
	skins    map[string]*Skin
}

// LoadAssets liest die Elemente samt Bildern; Rigs lädt Rig beim ersten Zugriff. Fehlt etwas, kommt eine klare Meldung.
func LoadAssets(dir string) (*Assets, error) {
	a := &Assets{Dir: dir, Images: map[string]*ebiten.Image{}, Elements: map[string]*ElementImage{}, Armors: []string{""},
		rigs: map[string]*rig.Rig{}, skins: map[string]*Skin{}}
	elements, err := rig.LoadElements(filepath.Join(dir, "elemente"))
	if err != nil {
		return nil, err
	}
	for _, e := range elements {
		img, err := a.Image(filepath.Join("elemente", e.File))
		if err != nil {
			return nil, err
		}
		a.Elements[e.ID] = &ElementImage{e, img}
		switch e.Slot {
		case "waffe":
			a.Weapons = append(a.Weapons, e.ID)
		case "ruestung":
			a.Armors = append(a.Armors, e.ID)
		}
	}
	a.Weapons = append(a.Weapons, "")
	white := ebiten.NewImage(3, 3)
	white.Fill(color.White)
	a.White = white.SubImage(image.Rect(1, 1, 2, 2)).(*ebiten.Image)
	return a, nil
}

// Rig lädt das Rig `<gruppe>/<id>` einmal (Ordner relativ zum Assets-Ordner).
func (a *Assets) Rig(rel string) (*rig.Rig, error) {
	if r, ok := a.rigs[rel]; ok {
		return r, nil
	}
	r, err := rig.Load(filepath.Join(a.Dir, filepath.FromSlash(rel)))
	if err != nil {
		return nil, err
	}
	a.rigs[rel] = r
	return r, nil
}

// Image lädt ein PNG einmal (Pfad relativ zum Assets-Ordner) als vormultipliziertes RGBA.
func (a *Assets) Image(rel string) (*ebiten.Image, error) {
	if img, ok := a.Images[rel]; ok {
		return img, nil
	}
	path := filepath.Join(a.Dir, rel)
	rgba, err := ReadPNG(path)
	if err != nil {
		return nil, fmt.Errorf("Teil-Bild fehlt oder ist kaputt: %s (erst `task native:rig:export` ausführen): %w", path, err)
	}
	img := ebiten.NewImageFromImage(rgba)
	a.Images[rel] = img
	a.Bytes += int64(len(rgba.Pix))
	return img, nil
}

// Skin stellt die Bilder eines Rigs in einer Variante zusammen (Varianten ersetzen einzelne Bilder). Schlüssel und
// Bildpfade gehen über den Ordner des Rigs (<gruppe>/<id>), weil dieselbe id in zwei Gruppen vorkommt.
func (a *Assets) Skin(r *rig.Rig, variant string) (*Skin, error) {
	dir, err := filepath.Rel(a.Dir, r.Dir)
	if err != nil {
		return nil, err
	}
	key := dir + "@" + variant
	if s, ok := a.skins[key]; ok {
		return s, nil
	}
	s := &Skin{rig: r, parts: make([]*ebiten.Image, len(r.Parts)), bands: make([]*ebiten.Image, len(r.Bands)), bandPart: make([]int, len(r.Bands))}
	for i, p := range r.Parts {
		if p.File == "" {
			continue
		}
		img, err := a.Image(filepath.Join(dir, r.ImageFile(p.ID, p.File, variant)))
		if err != nil {
			return nil, err
		}
		s.parts[i] = img
	}
	for i, b := range r.Bands {
		img, err := a.Image(filepath.Join(dir, r.ImageFile(b.ID, b.File, variant)))
		if err != nil {
			return nil, err
		}
		s.bands[i], s.bandPart[i] = img, r.Index(b.Part)
	}
	a.skins[key] = s
	return s, nil
}

// ReadPNG liest ein PNG als vormultipliziertes RGBA (das Format, das Ebitengine erwartet).
func ReadPNG(path string) (*image.RGBA, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer f.Close()
	src, err := png.Decode(f)
	if err != nil {
		return nil, err
	}
	b := src.Bounds()
	rgba := image.NewRGBA(image.Rect(0, 0, b.Dx(), b.Dy()))
	draw.Draw(rgba, rgba.Bounds(), src, b.Min, draw.Src)
	return rgba, nil
}

// WritePNG schreibt ein vormultipliziertes RGBA-Bild als PNG.
func WritePNG(path string, img *image.RGBA) error {
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
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
