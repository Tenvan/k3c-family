package art

// Level-Plan: welche Sheets ein Level braucht, mit Pfaden, Maßen und Texturbudget. Liest nur Manifeste und PNG-Köpfe
// (image.DecodeConfig), keine Pixel und keine GPU; alle fehlenden Dateien werden gesammelt gemeldet.

import (
	"errors"
	"fmt"
	"image"
	_ "image/png" // Decoder für DecodeConfig
	"os"
	"path/filepath"
)

// SheetSpec ist ein geplantes Sheet: eine Animation eines Assets als Streifen.
type SheetSpec struct {
	Asset, Anim string
	Path        string
	Frames      int
	FPS         float64
	FW, FH      int // Größe eines Frames
	W, H        int // Größe des PNG
	// Fußpunkt als Anteil des Frames (von links oben); Rahmen-Einheiten je Bild-Pixel (für den Maßstab)
	OriginX, OriginY, UnitsPerPx float64
}

// Plan ist die Liste aller Sheets eines Levels samt Budget.
type Plan struct {
	Level  string
	Sheets []SheetSpec
	Budget Budget
}

// PlanLevel plant alle Animationen der Assets (`gruppe/id`) unter dem Grafik-Ordner dir. Fehlt etwas, kommt ein Fehler
// (ErrMissing) mit allen fehlenden Pfaden; es entsteht nie ein halber Plan.
func PlanLevel(dir, level string, assets []string) (*Plan, error) {
	p := &Plan{Level: level}
	var errs []error
	for _, asset := range assets {
		m, err := ReadManifest(dir, asset)
		if err != nil {
			errs = append(errs, err)
			continue
		}
		for _, name := range m.AnimNames() {
			spec, err := planSheet(dir, asset, name, m)
			if err != nil {
				errs = append(errs, err)
				continue
			}
			p.Sheets = append(p.Sheets, spec)
			p.Budget.Add(spec.W, spec.H)
		}
	}
	if err := errors.Join(errs...); err != nil {
		return nil, err
	}
	return p, nil
}

// planSheet prüft eine Animation: Maße im Manifest, PNG vorhanden und groß genug.
func planSheet(dir, asset, anim string, m *Manifest) (SheetSpec, error) {
	a := m.Anims[anim]
	fw, fh, r := m.resolve(a)
	path := filepath.Join(dir, filepath.FromSlash(asset), a.File)
	if a.Frames < 1 || fw < 1 || fh < 1 || r[2] == 0 || r[3] == 0 {
		return SheetSpec{}, fmt.Errorf("Animation %s in %s hat ungültige Maße (%s)", anim, asset, path)
	}
	f, err := os.Open(path)
	if err != nil {
		return SheetSpec{}, fmt.Errorf("%w: Sheet %s (%v); erst `task gen:paper` ausführen", ErrMissing, path, err)
	}
	defer f.Close()
	cfg, _, err := image.DecodeConfig(f)
	if err != nil {
		return SheetSpec{}, fmt.Errorf("Sheet kaputt: %s (%w)", path, err)
	}
	if cfg.Width < a.Frames*fw || cfg.Height < fh {
		return SheetSpec{}, fmt.Errorf("Sheet %s: Bild %d×%d zu klein für %d Frames à %d×%d", path, cfg.Width, cfg.Height, a.Frames, fw, fh)
	}
	return SheetSpec{
		Asset: asset, Anim: anim, Path: path, Frames: a.Frames, FPS: a.Fps, FW: fw, FH: fh, W: cfg.Width, H: cfg.Height,
		OriginX: -r[0] / r[2], OriginY: -r[1] / r[3], UnitsPerPx: r[3] / float64(fh),
	}, nil
}
