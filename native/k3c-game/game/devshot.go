package game

// Screenshot der Entwickler-Taste F12 (B-888): das nächste gezeichnete Bild als PNG nach reports/ (Arbeitsordner, wie
// task native:game), Name nach Datum und Uhrzeit. Ein Fehler steht im Log, das Spiel läuft weiter.

import (
	"fmt"
	"image"
	"log/slog"
	"os"
	"path/filepath"
	"time"

	"github.com/hajimehoshi/ebiten/v2"
)

// ReportsDir ist der Ordner der Berichte und Screenshots (relativ zum Arbeitsordner).
const ReportsDir = "reports"

// devShot schreibt das Bild screen, wenn F12 es verlangt hat.
func (g *Game) devShot(screen *ebiten.Image) {
	if !g.shotNext {
		return
	}
	g.shotNext = false
	img := image.NewRGBA(screen.Bounds())
	screen.ReadPixels(img.Pix)
	path, err := saveShot(ReportsDir, time.Now(), img)
	if err != nil {
		slog.Warn("💥 Screenshot nicht geschrieben", "err", err)
		return
	}
	slog.Info("📄 Screenshot geschrieben", "datei", path)
}

// saveShot schreibt img als shot-<Datum>-<Zeit>.png in dir (legt dir an) und liefert den Pfad.
func saveShot(dir string, now time.Time, img image.Image) (string, error) {
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return "", err
	}
	path := filepath.Join(dir, fmt.Sprintf("shot-%s.png", now.Format("20060102-150405.000")))
	return path, writePNG(path, img)
}
