package sim

import (
	"fmt"
	"io/fs"
	"maps"
	"slices"
	"strconv"

	"k3c/data"
	"k3c/engine/level"
)

// Inseln als Daten (data/islands.json, B-103): Liste in fester Reihenfolge, je Insel ihre Tiefen und ihre Tabelle der
// Gegner-Skalierung. Welten ohne Insel skalieren nach waves.json › depthScaling.

// IslandDef ist eine Insel aus den Daten.
type IslandDef struct {
	ID, Name     string
	Depths       []int
	DepthScaling struct{ HP, Damage, Speed float64 }
	// PlannedChunks sind die geplanten Chunks je Tiefe (Schlüssel = Tiefe, docs/rules/geplante-chunks.md); leer = reiner Zufall.
	PlannedChunks map[string][]level.PlannedChunk
	// ChunkCounts sind die Chunk-Anzahlen je Tiefe (Schlüssel = Tiefe, Wert = Inselgröße, geplante-chunks.md § 7);
	// ohne Eintrag gilt die Länge des Bioms.
	ChunkCounts map[string]int
}

var islandDefs = loadIslandDefs()

func loadIslandDefs() []IslandDef {
	var v []IslandDef
	if err := loadIslands(data.Files, &v); err != nil {
		panic(err)
	}
	return v
}

// loadIslands liest islands.json aus fsys und prüft jede Insel; bei einem Fehler bleibt *dst unverändert.
func loadIslands(fsys fs.FS, dst *[]IslandDef) error {
	var v struct{ Islands []IslandDef }
	if err := loadFrom(fsys, "islands.json", &v); err != nil {
		return err
	}
	if len(v.Islands) == 0 {
		return fmt.Errorf("data/islands.json: keine Insel")
	}
	var ids []string
	for _, d := range v.Islands {
		if err := checkIsland(d); err != nil {
			return fmt.Errorf("data/islands.json: %s: %w", d.ID, err)
		}
		if slices.Contains(ids, d.ID) {
			return fmt.Errorf("data/islands.json: %s doppelt", d.ID)
		}
		ids = append(ids, d.ID)
	}
	*dst = v.Islands
	return nil
}

// checkIsland: Tiefen vorhanden, je Tiefe ein Miniboss, in der tiefsten der Endboss, Faktoren > 0.
func checkIsland(d IslandDef) error {
	s := d.DepthScaling
	switch {
	case d.ID == "":
		return fmt.Errorf("ohne id")
	case len(d.Depths) == 0:
		return fmt.Errorf("ohne Tiefen")
	case s.HP <= 0 || s.Damage <= 0 || s.Speed <= 0:
		return fmt.Errorf("depthScaling braucht Faktoren > 0")
	case endBoss(slices.Max(d.Depths)) == nil:
		return fmt.Errorf("kein Endboss in der tiefsten Stufe %d", slices.Max(d.Depths))
	}
	for _, depth := range d.Depths {
		if !hasDepth(depth) {
			return fmt.Errorf("unbekannte Tiefe %d", depth)
		}
		if miniBoss(depth) == nil {
			return fmt.Errorf("kein Miniboss in Tiefe %d", depth)
		}
	}
	if err := checkChunkCounts(d); err != nil {
		return err
	}
	return checkPlannedChunks(d, level.DefaultWallLines())
}

// checkPlannedChunks prüft die geplanten Chunks jeder Tiefe unabhängig vom Seed (level.ValidatePlanFor) gegen die
// Mauerlinien lines und die Chunk-Anzahl der Tiefe; Fehler nennen Tiefe und Platz (der Aufrufer ergänzt die Insel).
func checkPlannedChunks(d IslandDef, lines level.WallLines) error {
	for _, key := range slices.Sorted(maps.Keys(d.PlannedChunks)) {
		plan := d.PlannedChunks[key]
		depth, err := strconv.Atoi(key)
		if err != nil || strconv.Itoa(depth) != key || !slices.Contains(d.Depths, depth) {
			return fmt.Errorf("plannedChunks: Tiefe %q ist keine Tiefe der Insel", key)
		}
		if err := level.ValidatePlanFor(biomeForDepth(depth), plan, lines, d.countAt(depth)); err != nil {
			return fmt.Errorf("plannedChunks Tiefe %d: %w", depth, err)
		}
	}
	return nil
}

// savedIslandOptions liefert die Erzeugungs-Parameter für das Laden eines Spielstands der Insel `number`: die Planung
// der Insel aus den Daten (die Planung selbst wird nicht gespeichert).
func savedIslandOptions(number int) CreateOptions {
	if number >= 0 && number < len(islandDefs) {
		return islandDefs[number].CreateOptions()
	}
	return CreateOptions{}
}
