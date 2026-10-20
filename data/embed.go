// Package data bettet die Balancing-Daten ein. Dieselben JSON-Dateien importiert der Client direkt,
// sie sind die einzige Quelle für Werte (docs/arbeitsweise.md, Entscheidung 001).
package data

import (
	"embed"
	"io/fs"
	"os"
	"time"
)

// embedded enthält alle JSON-Dateien aus data/ und data/biomes/ und von den Welten nur die Standard-Welt
// (docs/rules/welten.md § 6: eigene Welten bleiben lokal und kommen nie in einen Build).
//
//go:embed *.json biomes/*.json worlds/standard.json
var embedded embed.FS

// FS ist, was Files kann: Dateien und Ordner lesen (embed.FS und os.DirFS).
type FS interface {
	fs.ReadFileFS
	fs.ReadDirFS
}

// DirEnv nennt den Ordner, aus dem Files beim Start des Prozesses gelesen wird (B-507: der Spielserver liest data/ von
// der Platte, damit ein Neustart Änderungen des Studios übernimmt). Leer oder ein fehlender Ordner: eingebettet.
const DirEnv = "K3C_DATA_DIR"

// LoadedAt ist die Zeit vor dem ersten Lesen der Daten (vor Files, das bestimmt die Reihenfolge der Initialisierung):
// Eine Datei, die danach geändert wurde, hat der Prozess vielleicht schon im alten Stand gelesen (Datei-Watch, B-507).
var LoadedAt = time.Now()

// Files sind die Daten: aus DirEnv, sonst eingebettet. Gewählt wird einmal beim Laden des Pakets, vor allen Paketen,
// die beim Start Werte daraus lesen.
var Files = choose(os.Getenv(DirEnv))

// choose liefert den Ordner dir als FS, wenn es ihn gibt, sonst die eingebetteten Dateien.
func choose(dir string) FS {
	if dir != "" {
		if info, err := os.Stat(dir); err == nil && info.IsDir() {
			if fsys, ok := os.DirFS(dir).(FS); ok {
				return fsys
			}
		}
	}
	return embedded
}
