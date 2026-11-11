// Package art lädt die Grafik des nativen Spiels: Papier-Sheets je Level (public/grafik über data/paper.json), die
// Rig-Bibliothek (native/k3c-rig/paint) und die Schrift. Fachpakete holen hier ihre Bilder; kein Fachpaket liest Dateien selbst.
//
// Aufbau: Planen (Pfade, Maße, Texturbudget; ohne GPU, testbar) getrennt vom Hochladen (Ebitengine-Bilder).
package art

import "fmt"

// bytesPerPixel: Sheets liegen als RGBA8 (vormultipliziert) auf der GPU.
const bytesPerPixel = 4

// Budget ist das Texturbudget: Summe aus Breite × Höhe × 4 Byte je Bild.
type Budget struct {
	Images int
	Bytes  int64
}

// Add zählt ein Bild der Größe w × h dazu.
func (b *Budget) Add(w, h int) {
	b.Images++
	b.Bytes += int64(w) * int64(h) * bytesPerPixel
}

// AddBytes zählt bereits berechnete Bytes dazu (z. B. die Teil-Bilder der Rigs).
func (b *Budget) AddBytes(n int64) { b.Bytes += n }

// MiB liefert die Summe in Mebibyte.
func (b Budget) MiB() float64 { return float64(b.Bytes) / (1 << 20) }

func (b Budget) String() string { return fmt.Sprintf("%d Bilder, %.1f MiB RGBA", b.Images, b.MiB()) }
