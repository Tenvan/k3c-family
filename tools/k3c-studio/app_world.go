package main

import (
	"fmt"
	"regexp"
	"unicode/utf8"

	"k3c/engine/level"
)

// Welt-Betrachter (B-460, Sprint WZ7): die Chunk-Vorschau rechnet das Level des Spiels im Studio selbst, ohne Spielserver.

// maxWorldSeedRunes ist die Länge des Seeds wie bei GET /api/level.
const maxWorldSeedRunes = 64

var worldBiomeID = regexp.MustCompile(`^[a-z0-9-]{1,32}$`)

// WorldLevelView ist das Level wie im Spiel (Felder des Layouts) plus die Warnungen der Spielbarkeits-Prüfung; Error
// ist der Hinweis für die Seite, dann ist das Layout leer.
type WorldLevelView struct {
	level.Layout
	Warnings []string `json:"warnings"`
	Error    string   `json:"error"`
}

// WorldSource ist die Quelle des Levels als Wert. In WZ7 wertet das Studio nur Biome aus; Island und Stage (Insel +
// Stufe) trägt die Schnittstelle schon, WZ8.5 stellt die Chunk-Vorschau darauf um.
type WorldSource struct {
	Biome  string `json:"biome"`
	Island string `json:"island"`
	Stage  int    `json:"stage"`
}

// WorldLevel berechnet das Level zu Seed und Quelle (Binding). Ein ungültiger Seed, eine fehlende oder unbekannte
// Quelle ergibt Error, nie einen Pfadzugriff.
func (a *App) WorldLevel(seed string, src WorldSource) WorldLevelView {
	biome := src.Biome
	if src.Island != "" || src.Stage != 0 {
		return WorldLevelView{Warnings: []string{}, Error: "Quelle Insel + Stufe folgt mit WZ8.5, bisher nur das Biom"}
	}
	if n := utf8.RuneCountInString(seed); n == 0 || n > maxWorldSeedRunes {
		return WorldLevelView{Warnings: []string{}, Error: fmt.Sprintf("Seed braucht 1 bis %d Zeichen", maxWorldSeedRunes)}
	}
	if !worldBiomeID.MatchString(biome) {
		return WorldLevelView{Warnings: []string{}, Error: fmt.Sprintf("Unbekanntes Biom %q", biome)}
	}
	b, err := level.LoadBiome(biome)
	if err != nil {
		return WorldLevelView{Warnings: []string{}, Error: fmt.Sprintf("Unbekanntes Biom %q", biome)}
	}
	layout, err := level.Generate(b, seed)
	if err != nil {
		return WorldLevelView{Warnings: []string{}, Error: "Level nicht erzeugbar: " + err.Error()}
	}
	warnings := level.Validate(layout, b)
	if warnings == nil {
		warnings = []string{} // immer ein Array, nie null
	}
	return WorldLevelView{Layout: layout, Warnings: warnings}
}
