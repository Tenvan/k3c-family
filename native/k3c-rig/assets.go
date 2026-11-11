package main

// Bilder der Rig-Figuren: Teil-PNGs, Bänder und Elemente aus native/k3c-rig/assets (eingecheckt, kein Gen-Lauf nötig).

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

type Assets struct {
	dir      string
	monarch  *rig.Rig
	horse    *rig.Rig
	images   map[string]*ebiten.Image
	skins    map[string]*Skin
	elements map[string]*ElementImage
	weapons  []string // Auswahl am Platz `waffe`, "" = leer
	armors   []string // Auswahl am Platz `ruestung`
	white    *ebiten.Image
	bytes    int64 // Summe aller Bilder als RGBA8
}

// LoadAssets liest beide Rigs, die Elemente und alle Bilder; fehlt etwas, bricht es mit klarer Meldung ab.
func LoadAssets(dir string) (*Assets, error) {
	a := &Assets{dir: dir, images: map[string]*ebiten.Image{}, skins: map[string]*Skin{}, elements: map[string]*ElementImage{},
		weapons: []string{}, armors: []string{""}}
	var err error
	if a.monarch, err = rig.Load(filepath.Join(dir, "monarch")); err != nil {
		return nil, err
	}
	if a.horse, err = rig.Load(filepath.Join(dir, "pferd")); err != nil {
		return nil, err
	}
	elements, err := rig.LoadElements(filepath.Join(dir, "elemente"))
	if err != nil {
		return nil, err
	}
	for _, e := range elements {
		img, err := a.image(filepath.Join("elemente", e.File))
		if err != nil {
			return nil, err
		}
		a.elements[e.ID] = &ElementImage{e, img}
		switch e.Slot {
		case "waffe":
			a.weapons = append(a.weapons, e.ID)
		case "ruestung":
			a.armors = append(a.armors, e.ID)
		}
	}
	a.weapons = append(a.weapons, "")
	for _, v := range variants {
		for _, r := range []*rig.Rig{a.monarch, a.horse} {
			if _, err := a.skin(r, v); err != nil {
				return nil, err
			}
		}
	}
	white := ebiten.NewImage(3, 3)
	white.Fill(color.White)
	a.white = white.SubImage(image.Rect(1, 1, 2, 2)).(*ebiten.Image)
	return a, nil
}

// image lädt ein PNG einmal (Pfad relativ zum Assets-Ordner) als vormultipliziertes RGBA.
func (a *Assets) image(rel string) (*ebiten.Image, error) {
	if img, ok := a.images[rel]; ok {
		return img, nil
	}
	path := filepath.Join(a.dir, rel)
	rgba, err := readPNG(path)
	if err != nil {
		return nil, fmt.Errorf("Teil-Bild fehlt oder ist kaputt: %s (erst `task native:rig:export` ausführen): %w", path, err)
	}
	img := ebiten.NewImageFromImage(rgba)
	a.images[rel] = img
	a.bytes += int64(len(rgba.Pix))
	return img, nil
}

// skin stellt die Bilder eines Rigs in einer Variante zusammen (Varianten ersetzen einzelne Bilder).
func (a *Assets) skin(r *rig.Rig, variant string) (*Skin, error) {
	key := r.ID + "@" + variant
	if s, ok := a.skins[key]; ok {
		return s, nil
	}
	s := &Skin{rig: r, parts: make([]*ebiten.Image, len(r.Parts)), bands: make([]*ebiten.Image, len(r.Bands)), bandPart: make([]int, len(r.Bands))}
	for i, p := range r.Parts {
		if p.File == "" {
			continue
		}
		img, err := a.image(filepath.Join(r.ID, r.ImageFile(p.ID, p.File, variant)))
		if err != nil {
			return nil, err
		}
		s.parts[i] = img
	}
	for i, b := range r.Bands {
		img, err := a.image(filepath.Join(r.ID, r.ImageFile(b.ID, b.File, variant)))
		if err != nil {
			return nil, err
		}
		s.bands[i], s.bandPart[i] = img, r.Index(b.Part)
	}
	a.skins[key] = s
	return s, nil
}

// readPNG liest ein PNG als vormultipliziertes RGBA (das Format, das Ebitengine erwartet).
func readPNG(path string) (*image.RGBA, error) {
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

// writePNG schreibt ein vormultipliziertes RGBA-Bild als PNG.
func writePNG(path string, img *image.RGBA) error {
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
