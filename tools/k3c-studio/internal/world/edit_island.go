package world

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"maps"
	"slices"
	"strconv"
	"strings"
)

// Inseln einer Welt bearbeiten (B-954): world_set › islands und world_chunks_set.

// islandKeys sind die Felder einer Insel im patch (id wählt die Insel, biomes ist die Kurzform von depths).
var islandKeys = []string{"name", "etappe", "depths", "depthScaling", "plannedChunks", "chunkCounts", "endBoss", "nature", "natureByDepth"}

// newIsland ist die Zeile einer neuen Insel; extra sind fertige `"schlüssel": wert`-Paare.
func newIsland(id, name, depths string, extra []string) string {
	parts := append([]string{
		`"id": ` + quote(id), `"name": ` + quote(name), `"depths": ` + depths,
		`"depthScaling": { "hp": 1.5, "damage": 1.3, "speed": 1.1 }`,
	}, extra...)
	return "{ " + strings.Join(parts, ", ") + " }"
}

// patchIslands wendet die Einträge der Liste list der Reihe nach an: Eine Insel mit bekannter id wird feldweise
// geändert (null entfernt ein Feld, depthScaling wird gemischt), eine neue id wird am Ende angehängt (Standard: Name =
// id, Tiefen 0-4, Skalierung der Standard-Welt). `biomes` (Biom-IDs) ersetzt `depths`.
func (e Editor) patchIslands(raw []byte, _ string, list json.RawMessage) ([]byte, error) {
	var entries []map[string]json.RawMessage
	if err := json.Unmarshal(list, &entries); err != nil {
		return nil, errors.New("islands im patch muss eine Liste von Objekten sein")
	}
	biomes := e.biomesByDepth()
	for i, entry := range entries {
		var err error
		if raw, err = patchIsland(raw, entry, biomes); err != nil {
			return nil, fmt.Errorf("islands[%d]: %w", i, err)
		}
	}
	return raw, nil
}

func patchIsland(raw []byte, entry map[string]json.RawMessage, biomes map[int]string) ([]byte, error) {
	var id string
	if json.Unmarshal(entry["id"], &id) != nil || !idPattern.MatchString(id) {
		return nil, errors.New("id fehlt oder ist ungültig")
	}
	fields, err := islandFields(entry, biomes)
	if err != nil {
		return nil, err
	}
	obj, _, err := islandSpan(raw, id)
	var missing noIslandError
	if errors.As(err, &missing) {
		return appendIsland(raw, id, fields)
	}
	if err != nil {
		return nil, err
	}
	for _, k := range slices.Sorted(maps.Keys(fields)) {
		if raw, err = setIslandField(raw, obj, k, fields[k]); err != nil {
			return nil, err
		}
		if obj, _, err = islandSpan(raw, id); err != nil {
			return nil, err
		}
	}
	return raw, nil
}

// setIslandField setzt das Feld k der Insel (obj); depthScaling wird gemischt, etappe steht hinter name.
func setIslandField(raw []byte, obj span, k string, v json.RawMessage) ([]byte, error) {
	switch {
	case k == "etappe":
		return setEtappe(raw, obj, v)
	case k == "depthScaling" && given(v):
		at, found, _ := valueSpan(raw, obj, k)
		cur := json.RawMessage("{}")
		if found {
			cur = raw[at.from:at.to]
		}
		merged, err := mergeScaling(cur, v)
		if err != nil {
			return nil, err
		}
		v = merged
	}
	return setField(raw, obj, k, v, false)
}

// setEtappe setzt `etappe` der Insel obj (null entfernt es); ein neues Feld steht hinter `name` (Reihenfolge nach
// docs/rules/welten.md § 2). Die Zahl prüft die Engine vor dem Schreiben (≥ 1, lückenlos, nicht fallend).
func setEtappe(raw []byte, obj span, v json.RawMessage) ([]byte, error) {
	if _, found, err := valueSpan(raw, obj, "etappe"); err != nil || found || !given(v) {
		if err != nil {
			return nil, err
		}
		return setField(raw, obj, "etappe", v, false)
	}
	var n int
	if json.Unmarshal(v, &n) != nil {
		return nil, errors.New("etappe muss eine ganze Zahl sein")
	}
	name, ok, err := valueSpan(raw, obj, "name")
	if err != nil || !ok {
		return setField(raw, obj, "etappe", v, false)
	}
	return slices.Concat(raw[:name.to], []byte(", \"etappe\": "+strconv.Itoa(n)), raw[name.to:]), nil
}

// islandFields prüft die Felder des Eintrags und macht aus `biomes` die `depths`.
func islandFields(entry map[string]json.RawMessage, biomes map[int]string) (map[string]json.RawMessage, error) {
	fields := map[string]json.RawMessage{}
	for k, v := range entry {
		switch {
		case k == "id":
		case k == "biomes":
			depths, err := biomeDepths(v, biomes)
			if err != nil {
				return nil, err
			}
			if _, both := entry["depths"]; both {
				return nil, errors.New("depths oder biomes, nicht beides")
			}
			fields["depths"] = depths
		case slices.Contains(islandKeys, k):
			fields[k] = v
		case k == "story":
			return nil, errStoryTool
		default:
			return nil, fmt.Errorf("unbekanntes Feld %q (erlaubt: id, biomes, %s)", k, strings.Join(islandKeys, ", "))
		}
	}
	return fields, nil
}

// biomeDepths übersetzt Biom-IDs in die Tiefen (Feld depth der Biome unter data/biomes/).
func biomeDepths(list json.RawMessage, biomes map[int]string) (json.RawMessage, error) {
	var ids []string
	if err := json.Unmarshal(list, &ids); err != nil {
		return nil, errors.New("biomes muss eine Liste von Biom-IDs sein")
	}
	depths := []int{}
	for _, id := range ids {
		found := false
		for d, b := range biomes {
			if b == id {
				depths, found = append(depths, d), true
			}
		}
		if !found {
			known := slices.Sorted(maps.Values(biomes))
			return nil, fmt.Errorf("unbekanntes Biom %q (bekannt: %s)", id, strings.Join(known, ", "))
		}
	}
	return json.Marshal(depths)
}

// mergeScaling legt die Felder von next über cur (hp, damage, speed, in dieser Reihenfolge).
func mergeScaling(cur, next json.RawMessage) (json.RawMessage, error) {
	var a, b map[string]json.RawMessage
	if err := json.Unmarshal(cur, &a); err != nil {
		return nil, fmt.Errorf("depthScaling: %w", err)
	}
	if err := json.Unmarshal(next, &b); err != nil {
		return nil, errors.New("depthScaling muss ein Objekt sein (hp, damage, speed)")
	}
	maps.Copy(a, b)
	keys := []string{"hp", "damage", "speed"}
	for _, k := range slices.Sorted(maps.Keys(a)) {
		if !slices.Contains(keys, k) {
			keys = append(keys, k)
		}
	}
	var parts []string
	for _, k := range keys {
		if v, ok := a[k]; ok {
			parts = append(parts, quote(k)+": "+string(v))
		}
	}
	return json.RawMessage("{" + strings.Join(parts, ", ") + "}"), nil
}

// appendIsland hängt eine neue Insel an die Liste an.
func appendIsland(raw []byte, id string, fields map[string]json.RawMessage) ([]byte, error) {
	line, err := islandLine(id, fields)
	if err != nil {
		return nil, err
	}
	arr, found, err := valueSpan(raw, span{0, len(raw)}, "islands")
	if err != nil || !found {
		return nil, errors.New("islands fehlt in der Welt-Datei")
	}
	at := insertSpan(raw, arr)
	sep := ","
	if raw[at.from-1] == '[' {
		sep = ""
	}
	return slices.Concat(raw[:at.from], eol(raw, sep+"\n    "+line), raw[at.from:]), nil
}

// islandLine ist die Zeile einer neuen Insel: Name = id, Tiefen 0-4 und die Skalierung der Standard-Welt, soweit
// fields nichts anderes nennt; übrige Felder folgen nach Namen sortiert.
func islandLine(id string, fields map[string]json.RawMessage) (string, error) {
	name, depths, scaling := quote(id), "[0, 1, 2, 3, 4]", json.RawMessage(defaultScaling)
	var err error
	if v, ok := fields["name"]; ok {
		var text string
		if json.Unmarshal(v, &text) != nil || text == "" {
			return "", errors.New("name muss ein Text sein")
		}
		name = quote(text)
	}
	if v, ok := fields["depths"]; ok {
		if depths, err = inline(v); err != nil {
			return "", err
		}
	}
	if v, ok := fields["depthScaling"]; ok {
		if scaling, err = mergeScaling(scaling, v); err != nil {
			return "", err
		}
	}
	sc, err := inline(scaling)
	if err != nil {
		return "", err
	}
	extra, err := islandExtras(fields)
	if err != nil {
		return "", err
	}
	parts := []string{`"id": ` + quote(id), `"name": ` + name}
	if v, ok := fields["etappe"]; ok && given(v) {
		var n int
		if json.Unmarshal(v, &n) != nil {
			return "", errors.New("etappe muss eine ganze Zahl sein")
		}
		parts = append(parts, `"etappe": `+strconv.Itoa(n))
	}
	parts = append(append(parts, `"depths": `+depths, `"depthScaling": `+sc), extra...)
	return "{ " + strings.Join(parts, ", ") + " }", nil
}

// islandExtras sind die Felder neben Name, Tiefen und Skalierung als fertige `"schlüssel": wert`-Paare.
func islandExtras(fields map[string]json.RawMessage) ([]string, error) {
	extra := []string{}
	for _, k := range slices.Sorted(maps.Keys(fields)) {
		if k == "name" || k == "etappe" || k == "depths" || k == "depthScaling" || !given(fields[k]) {
			continue
		}
		text, err := inline(fields[k])
		if err != nil {
			return nil, fmt.Errorf("%s: %w", k, err)
		}
		extra = append(extra, quote(k)+": "+text)
	}
	return extra, nil
}

// SetChunks setzt die geplanten Chunks (plannedChunks) und/oder die Chunk-Anzahl (chunkCounts) der Tiefe depth der Insel
// island. chunks nil lässt sie, null oder [] entfernt sie; count nil lässt die Anzahl, 0 entfernt sie.
func (e Editor) SetChunks(ctx context.Context, id, island string, depth int, chunks json.RawMessage, count *int) ([]string, error) {
	if chunks == nil && count == nil {
		return nil, errors.New("chunks oder count fehlt")
	}
	if count != nil && *count < 0 {
		return nil, errors.New("count darf nicht negativ sein")
	}
	writeMu.Lock()
	defer writeMu.Unlock()
	raw, err := e.read(id)
	if err != nil {
		return nil, err
	}
	_, head, err := islandSpan(raw, island)
	if err != nil {
		return nil, err
	}
	if !slices.Contains(head.Depths, depth) {
		return nil, fmt.Errorf("die Tiefe %d gehört nicht zur Insel %q (Tiefen: %v)", depth, island, head.Depths)
	}
	key, out := strconv.Itoa(depth), raw
	if chunks != nil {
		if out, err = setDepthEntry(out, island, "plannedChunks", key, chunks); err != nil {
			return nil, err
		}
	}
	if count != nil {
		v := json.RawMessage(strconv.Itoa(*count))
		if *count == 0 {
			v = nil
		}
		if out, err = setDepthEntry(out, island, "chunkCounts", key, v); err != nil {
			return nil, err
		}
	}
	return e.commitChange(ctx, id, raw, out)
}

// setDepthEntry setzt den Eintrag key im Tiefen-Objekt field der Insel (nil, null und [] entfernen ihn; ein leeres
// Objekt entfällt ganz). Die Schlüssel stehen nach Tiefe sortiert.
func setDepthEntry(raw []byte, island, field, key string, v json.RawMessage) ([]byte, error) {
	obj, _, err := islandSpan(raw, island)
	if err != nil {
		return nil, err
	}
	m := map[string]json.RawMessage{}
	if at, found, _ := valueSpan(raw, obj, field); found {
		if err := json.Unmarshal(raw[at.from:at.to], &m); err != nil {
			return nil, fmt.Errorf("%s: %w", field, err)
		}
	}
	if t := bytes.TrimSpace(v); given(t) && string(compactJSON(t)) != "[]" {
		m[key] = t
	} else {
		delete(m, key)
	}
	text := json.RawMessage("null")
	if len(m) > 0 {
		keys := numericKeys(m)
		var parts []string
		for _, k := range keys {
			parts = append(parts, quote(k)+": "+string(m[k]))
		}
		text = json.RawMessage("{" + strings.Join(parts, ", ") + "}")
	}
	return setField(raw, obj, field, text, false)
}
