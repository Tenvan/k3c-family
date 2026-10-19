package sim

import (
	"fmt"
	"io/fs"
	"maps"
	"slices"
	"strconv"
	"strings"

	"k3c/data"
)

// Inselgrößen (docs/rules/geplante-chunks.md § 7, B-485): Die Chunk-Anzahl einer Stufe steht je Insel und Tiefe in
// der Welt-Datei (data/worlds/<id>.json) › chunkCounts; erlaubt sind nur die Anzahlen der Tabelle data/island-sizes.json (für alle Biome).

// IslandSize ist eine Inselgröße: Name und Chunk-Anzahl.
type IslandSize struct {
	Name       string `json:"name"`
	ChunkCount int    `json:"chunkCount"`
}

var islandSizes = mustIslandSizes()

func mustIslandSizes() []IslandSize {
	var v []IslandSize
	if err := loadIslandSizes(data.Files, &v); err != nil {
		panic(err)
	}
	return v
}

// loadIslandSizes liest island-sizes.json aus fsys und prüft die Tabelle (nicht leer, Name gesetzt, Anzahl gerade,
// größer 0 und eindeutig); bei einem Fehler bleibt *dst unverändert.
func loadIslandSizes(fsys fs.FS, dst *[]IslandSize) error {
	var v struct{ Sizes []IslandSize }
	if err := loadFrom(fsys, "island-sizes.json", &v); err != nil {
		return err
	}
	if len(v.Sizes) == 0 {
		return fmt.Errorf("data/island-sizes.json: keine Inselgröße")
	}
	for i, s := range v.Sizes {
		switch {
		case s.Name == "":
			return fmt.Errorf("data/island-sizes.json: Größe %d ohne Namen", i+1)
		case s.ChunkCount <= 0 || s.ChunkCount%2 != 0:
			return fmt.Errorf("data/island-sizes.json: %s: Chunk-Anzahl %d ist nicht gerade und größer 0", s.Name, s.ChunkCount)
		case slices.ContainsFunc(v.Sizes[:i], func(o IslandSize) bool { return o.ChunkCount == s.ChunkCount }):
			return fmt.Errorf("data/island-sizes.json: Chunk-Anzahl %d doppelt", s.ChunkCount)
		}
	}
	*dst = v.Sizes
	return nil
}

// IslandSizes liefert eine Kopie der Inselgrößen in der Reihenfolge der Daten (für Editor und Vorschau).
func IslandSizes() []IslandSize { return slices.Clone(islandSizes) }

// checkChunkCount lehnt eine Chunk-Anzahl ab, die keine Inselgröße ist; der Fehler nennt die gültigen Werte.
func checkChunkCount(n int) error {
	valid := make([]string, len(islandSizes))
	for i, s := range islandSizes {
		if s.ChunkCount == n {
			return nil
		}
		valid[i] = strconv.Itoa(s.ChunkCount)
	}
	return fmt.Errorf("die Chunk-Anzahl %d ist keine Inselgröße (gültig: %s)", n, strings.Join(valid, ", "))
}

// checkChunkCounts prüft chunkCounts einer Insel: Schlüssel ist eine Tiefe der Insel, Wert eine Inselgröße. Fehler
// nennen Tiefe und die gültigen Werte (der Aufrufer ergänzt die Insel).
func checkChunkCounts(d IslandDef) error {
	for _, key := range slices.Sorted(maps.Keys(d.ChunkCounts)) {
		depth, err := strconv.Atoi(key)
		if err != nil || strconv.Itoa(depth) != key || !slices.Contains(d.Depths, depth) {
			return fmt.Errorf("chunkCounts: Tiefe %q ist keine Tiefe der Insel", key)
		}
		if err := checkChunkCount(d.ChunkCounts[key]); err != nil {
			return fmt.Errorf("chunkCounts Tiefe %d: %w", depth, err)
		}
	}
	return nil
}

// countAt liefert die Chunk-Anzahl der Insel für eine Tiefe (0 = Länge des Bioms).
func (d IslandDef) countAt(depth int) int { return d.ChunkCounts[strconv.Itoa(depth)] }
