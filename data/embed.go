// Package data bettet die Balancing-Daten ein. Dieselben JSON-Dateien importiert der Client direkt,
// sie sind die einzige Quelle für Werte (docs/arbeitsweise.md, Entscheidung 001).
package data

import "embed"

// Files enthält alle JSON-Dateien aus data/ und data/biomes/ und von den Welten nur die Standard-Welt
// (docs/rules/welten.md § 6: eigene Welten bleiben lokal und kommen nie in einen Build).
//
//go:embed *.json biomes/*.json worlds/standard.json
var Files embed.FS
