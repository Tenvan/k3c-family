// Package enginetools rechnet Level und Simulationen in-process mit engine/level und engine/sim (M6, B-047).
// Gleiche Eingabe ergibt denselben Text; nichts davon spricht mit einem laufenden Server.
package enginetools

import (
	"fmt"
	"slices"
	"strings"

	"k3c/engine/level"
)

// Biomes sind die Biome in data/biomes/.
var Biomes = []string{"forest", "cave", "mine"}

// loadBiome prüft den Namen (leer = forest) und lädt das Biom.
func loadBiome(id string) (level.Biome, error) {
	if id == "" {
		id = "forest"
	}
	if !slices.Contains(Biomes, id) {
		return level.Biome{}, fmt.Errorf("unbekanntes Biom %q (bekannt: %s)", id, strings.Join(Biomes, ", "))
	}
	return level.LoadBiome(id)
}

// Level erzeugt ein Level und beschreibt es: Kopfzeile, eine Zeile je Abschnitt, Objekte nach Art, Warnungen der Prüfung.
func Level(biomeID, seed string) (string, error) {
	b, err := loadBiome(biomeID)
	if err != nil {
		return "", err
	}
	l, err := level.Generate(b, seed)
	if err != nil {
		return "", fmt.Errorf("level nicht erzeugbar: %w", err)
	}
	lines := []string{fmt.Sprintf("Level %s · Seed %q · Breite %g Units · Hub-Mitte %g · %d Abschnitte à %g Units",
		l.BiomeID, l.Seed, l.WidthUnits, l.HubCenterUnits, len(l.Chunks), l.ChunkWidthUnits)}
	for _, c := range l.Chunks {
		lines = append(lines, fmt.Sprintf("%3d %s @%g", c.Index, c.Kind, c.StartUnits))
	}
	lines = append(lines, entityLines(l)...)
	if warn := level.Validate(l, b); len(warn) > 0 {
		lines = append(lines, "Warnungen: "+strings.Join(warn, "; "))
	}
	return strings.Join(lines, "\n"), nil
}

// entityLines zählt die Objekte je Art (Art-Reihenfolge alphabetisch) und nennt die ersten Positionen.
func entityLines(l level.Layout) []string {
	xs := map[string][]float64{}
	for _, e := range l.Entities {
		xs[e.Kind] = append(xs[e.Kind], e.X)
	}
	kinds := make([]string, 0, len(xs))
	for k := range xs {
		kinds = append(kinds, k)
	}
	slices.Sort(kinds)
	lines := []string{fmt.Sprintf("Objekte: %d", len(l.Entities))}
	for _, k := range kinds {
		pos := xs[k]
		first := make([]string, 0, 3)
		for _, x := range pos[:min(3, len(pos))] {
			first = append(first, fmt.Sprintf("%g", x))
		}
		lines = append(lines, fmt.Sprintf("  %s ×%d · erste x: %s", k, len(pos), strings.Join(first, ", ")))
	}
	return lines
}
