// Package data bettet die Balancing-Daten ein. Dieselben JSON-Dateien importiert der Client direkt,
// sie sind die einzige Quelle für Werte (docs/arbeitsweise.md, Entscheidung 001).
package data

import "embed"

// Files enthält alle JSON-Dateien aus data/ und data/biomes/.
//
//go:embed *.json biomes/*.json
var Files embed.FS
