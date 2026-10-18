package room

import (
	"fmt"

	"k3c/engine/level"
)

// Seed-Inhalt eines Chunks (WZ17.3, B-485/AC-06, geplante-chunks.md § 7): Die Umwandlung eines Seed-Chunks in feste
// Objekte übernimmt den Inhalt aus dem Vorschau-Level, kein neuer Generator-Code.

// PreviewChunkObjects liefert die Objekte des Chunks auf Platz slot im Vorschau-Level der Optionen (Biom, Seed,
// Planung, Chunk-Anzahl wie BuildPreview): Art und Lage `at` in Units ab Chunk-Anfang, sortiert nach Lage, im Format
// von plannedChunks.objects. Bei einem geplanten Chunk sind das sein Seed- bzw. Strom-Inhalt oder seine festen
// Objekte; bei einem ungeplanten Chunk alles, was dort steht (auch Skill-Punkte und Adern, die nicht planbar sind;
// die Lair läge nur bei einem Biom mit Lair und Ausgang links in einem Platz, das gibt es heute nicht).
// Platz 0 (Hub), Ausgang, Rand und Plätze außerhalb des Levels sind ein Fehler. Kein Paketzustand.
func PreviewChunkObjects(o PreviewOptions, slot int) ([]level.PlannedObject, error) {
	if slot == 0 {
		return nil, fmt.Errorf("vorschau: es gibt keinen Platz 0 (Hub)")
	}
	w, err := BuildPreview(o)
	if err != nil {
		return nil, err
	}
	c, err := chunkAt(w.Level, slot)
	if err != nil {
		return nil, err
	}
	end := c.StartUnits + w.Level.ChunkWidthUnits
	out := []level.PlannedObject{}
	for _, e := range w.Level.Entities {
		if e.X >= c.StartUnits && e.X < end {
			// Exakt: Anfang ≥ eine Chunk-Breite und e.X < 2·Anfang, also ist die Differenz ohne Rundung (Sterbenz);
			// Anfang + at ergibt wieder genau e.X, die Umwandlung ändert kind@x nicht.
			out = append(out, level.PlannedObject{Kind: e.Kind, At: e.X - c.StartUnits})
		}
	}
	return out, nil
}

// chunkAt sucht den Chunk zum Platz: +n ist der n-te Chunk rechts der Hub-Chunks, −n der n-te links davon.
func chunkAt(lv level.Layout, slot int) (level.Chunk, error) {
	hubStart, hubChunks := -1, 0
	for i, c := range lv.Chunks {
		if c.Kind == "hub" {
			if hubStart < 0 {
				hubStart = i
			}
			hubChunks++
		}
	}
	if hubStart < 0 {
		return level.Chunk{}, fmt.Errorf("vorschau: Level ohne Hub")
	}
	i := hubStart + slot
	if slot > 0 {
		i = hubStart + hubChunks + slot - 1
	}
	if i < 0 || i >= len(lv.Chunks) {
		return level.Chunk{}, fmt.Errorf("vorschau: Platz %+d liegt außerhalb des Levels (%d Chunks)", slot, len(lv.Chunks))
	}
	if k := lv.Chunks[i].Kind; k == "exit" || k == "edge" {
		return level.Chunk{}, fmt.Errorf("vorschau: Platz %+d ist Ausgang oder Rand (%s)", slot, k)
	}
	return lv.Chunks[i], nil
}
