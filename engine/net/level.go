package net

import (
	"fmt"
	"net/http"
	"slices"
	"strings"
	"sync"
	"unicode/utf8"

	"k3c/data"
	"k3c/engine/level"
)

const (
	defaultLevelSeed  = "k3c"
	defaultLevelBiome = "forest"
	maxLevelSeedRunes = 64
)

// levelBiomes sind die Biome, die der Server kennt: die eingebetteten data/biomes/*.json, nach Namen sortiert.
var levelBiomes = sync.OnceValue(func() []string {
	files, err := data.Files.ReadDir("biomes")
	if err != nil {
		return nil
	}
	ids := make([]string, 0, len(files))
	for _, f := range files {
		ids = append(ids, strings.TrimSuffix(f.Name(), ".json"))
	}
	return ids
})

// levelResponse ist das Level wie im Raum (Felder des Layouts) plus die Warnungen der Spielbarkeits-Prüfung.
type levelResponse struct {
	level.Layout
	Warnings []string `json:"warnings"`
}

// level ist GET /api/level?seed=…&biome=… (B-091): das Level, das ein Raum mit diesem Seed und Biom bekäme. Reine
// Berechnung ohne Zustand: kein Raum, keine Datei, kein Token. Ohne seed gilt "k3c", ohne biome "forest".
func (s *server) level(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet && r.Method != http.MethodHead {
		fail(w, http.StatusMethodNotAllowed, "Nur GET")
		return
	}
	q := r.URL.Query()
	seed := q.Get("seed")
	if seed == "" {
		seed = defaultLevelSeed
	}
	if utf8.RuneCountInString(seed) > maxLevelSeedRunes {
		fail(w, http.StatusBadRequest, fmt.Sprintf("Seed ist zu lang (höchstens %d Zeichen)", maxLevelSeedRunes))
		return
	}
	id := q.Get("biome")
	if id == "" {
		id = defaultLevelBiome
	}
	if !slices.Contains(levelBiomes(), id) { // nur bekannte Namen: aus der Eingabe wird nie ein Pfad
		fail(w, http.StatusBadRequest, fmt.Sprintf("Unbekanntes Biom %q (bekannt: %s)", id, strings.Join(levelBiomes(), ", ")))
		return
	}
	biome, err := level.LoadBiome(id)
	if err != nil {
		s.log.Error("level: Biom nicht ladbar", "biome", id, "error", err.Error())
		fail(w, http.StatusInternalServerError, "Biom nicht ladbar")
		return
	}
	layout, err := level.Generate(biome, seed)
	if err != nil {
		s.log.Error("level: nicht erzeugbar", "biome", id, "error", err.Error())
		fail(w, http.StatusInternalServerError, "Level nicht erzeugbar")
		return
	}
	warnings := level.Validate(layout, biome)
	if warnings == nil {
		warnings = []string{} // immer ein Array, nie null
	}
	writeJSON(w, http.StatusOK, levelResponse{Layout: layout, Warnings: warnings})
}
