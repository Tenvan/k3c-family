// Command gebaeude rendert Gebäude aus Daten-JSON (Kachel-Einheiten, DB32) zu PNGs in den erlaubten
// Dichten 16, 32, 64 und 128 px je Kachel und schreibt index.json für die Asset-Vorschau (GEB1, B-435).
// Gleiche Daten ergeben byte-gleiche PNGs. Regeln: docs/standards/gebaeude-grafik.md.
package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	root := flag.String("root", ".", "Projektwurzel")
	dir := flag.String("dir", "public/grafik/gebaeude", "Ordner mit Daten-JSON und PNGs (relativ zur Wurzel)")
	flag.Parse()
	// Pfade in index.json gelten relativ zu public/ (so lädt Vite sie).
	prefix := strings.TrimPrefix(filepath.ToSlash(*dir), "public/") + "/"
	n, err := run(filepath.Join(*root, *dir), prefix)
	if err != nil {
		fmt.Fprintln(os.Stderr, "❌ gebaeude:", err)
		os.Exit(1)
	}
	fmt.Printf("✅ gebaeude: %d Gebäude gerendert\n", n)
}
