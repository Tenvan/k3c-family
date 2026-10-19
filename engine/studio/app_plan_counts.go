package studio

import (
	"encoding/json"
	"fmt"
	"maps"
	"strconv"
	"strings"

	"k3c/engine/sim"
)

// Chunk-Anzahl je Stufe im Editor (WZ17.5, B-485/AC-04, docs/rules/geplante-chunks.md § 7): Prüfen gegen die
// Inselgrößen (data/island-sizes.json über sim.IslandSizes) und format-stabiles Schreiben der Welt-Datei ›
// chunkCounts (nur der Wert der betroffenen Insel wird ersetzt, wie bei plannedChunks). 0 heißt „Biom-Länge“: kein
// Eintrag, die Länge kommt aus lengthUnits des Bioms.

// checkIslandSize lehnt eine Anzahl ab, die keine Inselgröße ist; 0 (Biom-Länge) ist immer gültig.
func checkIslandSize(n int) error {
	if n == 0 {
		return nil
	}
	sizes := sim.IslandSizes()
	valid := make([]string, len(sizes))
	for i, s := range sizes {
		if s.ChunkCount == n {
			return nil
		}
		valid[i] = strconv.Itoa(s.ChunkCount)
	}
	return fmt.Errorf("die Chunk-Anzahl %d ist keine Inselgröße (gültig: %s)", n, strings.Join(valid, ", "))
}

// countsOf liest die Chunk-Anzahlen der Insel id (Tiefe → Anzahl) aus dem Inhalt der Welt-Datei.
func countsOf(raw []byte, id string) (map[string]int, error) {
	var f struct {
		Islands []struct {
			ID     string         `json:"id"`
			Counts map[string]int `json:"chunkCounts"`
		} `json:"islands"`
	}
	if err := json.Unmarshal(raw, &f); err != nil {
		return nil, fmt.Errorf("Welt-Datei: %w", err)
	}
	for _, isl := range f.Islands {
		if isl.ID == id {
			return isl.Counts, nil
		}
	}
	return nil, fmt.Errorf("die Insel %q gibt es nicht in der Welt-Datei", id)
}

// encodeCounts schreibt die Anzahlen kompakt, Tiefen aufsteigend: {"0": 24, "2": 40}.
func encodeCounts(counts map[string]int) []byte {
	var b strings.Builder
	b.WriteByte('{')
	for i, key := range sortedDepths(counts) {
		if i > 0 {
			b.WriteString(", ")
		}
		fmt.Fprintf(&b, "%q: %d", key, counts[key])
	}
	b.WriteByte('}')
	return []byte(b.String())
}

// patchCounts setzt die Anzahl der Tiefe depth der Insel id (0 entfernt den Eintrag) und lässt alle übrigen Bytes
// stehen; ohne inhaltliche Änderung kommt derselbe Inhalt zurück (byte-gleich).
func patchCounts(raw []byte, id string, depth, count int) ([]byte, error) {
	old, err := countsOf(raw, id)
	if err != nil {
		return nil, err
	}
	key := strconv.Itoa(depth)
	if old[key] == count {
		return raw, nil
	}
	next := maps.Clone(old)
	if next == nil {
		next = map[string]int{}
	}
	if count == 0 {
		delete(next, key)
	} else {
		next[key] = count
	}
	return replaceField(raw, id, "chunkCounts", encodeCounts(next))
}
