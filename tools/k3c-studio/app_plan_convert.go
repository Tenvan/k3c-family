package main

import (
	"fmt"
	"unicode/utf8"

	"k3c/engine/level"
	"k3c/engine/room"
)

// Umwandlung eines Seed-Chunks in feste Objekte (WZ17.5, B-485/AC-06, geplante-chunks.md § 7): Der Inhalt kommt aus
// dem Vorschau-Level des Servers (room.PreviewChunkObjects) zur selben Quelle wie Chunk- und Live-Vorschau; das
// Frontend setzt die Objekte als feste Objekte des Chunks und entfernt den Seed, das Bild bleibt gleich.

// PlanObjectsView sind die Objekte eines Chunks (Art und Lage in Units ab Chunk-Anfang) oder ein Hinweis in Error.
type PlanObjectsView struct {
	Objects []level.PlannedObject `json:"objects"`
	Error   string                `json:"error"`
}

// PlanChunkObjects liefert den Seed-Inhalt des geplanten Chunks am Platz slot im Level zu Seed und Quelle (Binding).
// Nur ein geplanter Chunk mit Chunk-Seed und ohne feste Objekte lässt sich umwandeln; sonst nennt Error den Grund.
func (a *App) PlanChunkObjects(seed string, src WorldSource, slot int) PlanObjectsView {
	if n := utf8.RuneCountInString(seed); n == 0 || n > maxWorldSeedRunes {
		return PlanObjectsView{Objects: []level.PlannedObject{}, Error: fmt.Sprintf("Seed braucht 1 bis %d Zeichen", maxWorldSeedRunes)}
	}
	r, err := a.resolveSource(src)
	if err == nil {
		err = convertible(r.planned, slot)
	}
	var objects []level.PlannedObject
	if err == nil {
		objects, err = room.PreviewChunkObjects(room.PreviewOptions{
			Seed: seed, Source: room.PreviewSource{Biome: r.biome}, Planned: r.planned, ChunkCount: r.count,
		}, slot)
	}
	if err != nil {
		return PlanObjectsView{Objects: []level.PlannedObject{}, Error: err.Error()}
	}
	return PlanObjectsView{Objects: objects}
}

// convertible prüft, ob am Platz ein geplanter Chunk mit Chunk-Seed und ohne feste Objekte steht.
func convertible(planned []level.PlannedChunk, slot int) error {
	for _, c := range planned {
		if c.Slot != slot {
			continue
		}
		switch {
		case c.Seed == "":
			return fmt.Errorf("der Platz %+d hat keinen Chunk-Seed, es gibt nichts umzuwandeln", slot)
		case len(c.Objects) > 0:
			return fmt.Errorf("der Platz %+d hat schon feste Objekte", slot)
		}
		return nil
	}
	return fmt.Errorf("der Platz %+d ist nicht geplant", slot)
}
