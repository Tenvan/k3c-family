package sim

import (
	"fmt"
	"maps"
	"slices"
	"strconv"

	"k3c/engine/level"
)

// Inseln als Daten (Welt-Datei data/worlds/<id>.json, B-103, B-488): Liste in fester Reihenfolge, je Insel ihre Tiefen
// und ihre Tabelle der Gegner-Skalierung. Welten ohne Insel skalieren nach waves.json › depthScaling. Laden und
// strenges Lesen: island_worlds.go.

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

// checkIsland prüft eine Insel in der Reihenfolge aus docs/rules/welten.md § 3: Name, Tiefen (vorhanden, bekannt,
// einmalig), je Tiefe ein Miniboss, in der tiefsten der Endboss, Faktoren > 0, chunkCounts, plannedChunks. ID und
// Eindeutigkeit prüft checkWorld.
func checkIsland(d IslandDef) error {
	if d.Name == "" {
		return fmt.Errorf("name ist leer")
	}
	if len(d.Depths) == 0 {
		return fmt.Errorf("ohne Tiefen")
	}
	for i, depth := range d.Depths {
		switch {
		case !hasDepth(depth):
			return fmt.Errorf("unbekannte Tiefe %d", depth)
		case slices.Contains(d.Depths[:i], depth):
			return fmt.Errorf("doppelte Tiefe %d", depth)
		}
	}
	for _, depth := range d.Depths {
		if miniBoss(depth) == nil {
			return fmt.Errorf("kein Miniboss in Tiefe %d", depth)
		}
	}
	if deepest := slices.Max(d.Depths); endBoss(deepest) == nil {
		return fmt.Errorf("kein Endboss in der tiefsten Stufe %d", deepest)
	}
	if s := d.DepthScaling; s.HP <= 0 || s.Damage <= 0 || s.Speed <= 0 {
		return fmt.Errorf("depthScaling braucht Faktoren > 0")
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
// der Insel aus der Standard-Welt (die Planung selbst wird nicht gespeichert).
func savedIslandOptions(number int) CreateOptions {
	if number >= 0 && number < len(standardWorld.Islands) {
		return standardWorld.Islands[number].CreateOptions()
	}
	return CreateOptions{}
}
