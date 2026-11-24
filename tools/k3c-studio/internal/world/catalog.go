package world

import (
	"encoding/json"
	"errors"
	"fmt"
	"maps"
	"os"
	"path/filepath"
	"slices"
	"strings"
)

// Welt-Zuordnung in den Katalogen (SI7.1, B-1003): Wetter, Reittier und Einsiedler stehen nicht in der Welt-Datei,
// sondern im Feld `worlds` der drei Katalog-Dateien (docs/rules/welten.md § 2 und 3; das Rust-Spiel liest sie).
// Ein Eintrag fehlt, wenn die Welt die Art nicht hat; die Auswahl „keins“ entfernt ihn.

// catalogPick beschreibt eine Art: Katalog-Datei und die Liste mit `id`-Einträgen (leer: die Schlüssel der obersten
// Ebene mit Objekt-Wert, so die Wetter-Datei).
type catalogPick struct {
	file string
	list string
}

var catalogPicks = map[string]catalogPick{
	"weather": {file: "data/weather.json"},
	"mount":   {file: "data/mounts.json", list: "mounts"},
	"hermit":  {file: "data/hermits.json", list: "hermits"},
}

// catalogReserved sind die Felder der Katalog-Dateien, die keine Art sind.
var catalogReserved = []string{"version", "notes", "worlds"}

// catalogIDs liefert die gültigen IDs der Art in der Reihenfolge der Datei.
func (e Editor) catalogIDs(p catalogPick) ([]string, error) {
	raw, err := os.ReadFile(filepath.Join(e.Root, p.file))
	if err != nil {
		return nil, err
	}
	if p.list == "" {
		keys, vals := objectItems(raw)
		var ids []string
		for i, k := range keys {
			if !slices.Contains(catalogReserved, k) && len(vals[i]) > 0 && vals[i][0] == '{' {
				ids = append(ids, k)
			}
		}
		return ids, nil
	}
	var doc map[string]json.RawMessage
	if err := json.Unmarshal(raw, &doc); err != nil {
		return nil, fmt.Errorf("%s: %w", p.file, err)
	}
	var items []struct {
		ID string `json:"id"`
	}
	if err := json.Unmarshal(doc[p.list], &items); err != nil {
		return nil, fmt.Errorf("%s › %s: %w", p.file, p.list, err)
	}
	var ids []string
	for _, item := range items {
		ids = append(ids, item.ID)
	}
	return ids, nil
}

// worldsOf liest den Block `worlds` einer Katalog-Datei in der Reihenfolge der Datei.
func worldsOf(raw []byte) ([]string, []string, error) {
	var doc struct {
		Worlds json.RawMessage `json:"worlds"`
	}
	if err := json.Unmarshal(raw, &doc); err != nil {
		return nil, nil, err
	}
	if doc.Worlds == nil {
		return nil, nil, nil
	}
	keys, vals := objectItems(doc.Worlds)
	values := make([]string, len(vals))
	for i, v := range vals {
		if err := json.Unmarshal(v, &values[i]); err != nil {
			return nil, nil, fmt.Errorf("worlds: %w", err)
		}
	}
	return keys, values, nil
}

// Picks liefert die Zuordnung der Welt id je Art (weather, mount, hermit); fehlt ein Eintrag, steht dort "".
func (e Editor) Picks(id string) (map[string]string, error) {
	if err := checkID(id); err != nil {
		return nil, err
	}
	out := map[string]string{}
	for key, p := range catalogPicks {
		raw, err := os.ReadFile(filepath.Join(e.Root, p.file))
		if err != nil {
			return nil, err
		}
		keys, values, err := worldsOf(raw)
		if err != nil {
			return nil, fmt.Errorf("%s: %w", p.file, err)
		}
		if i := slices.Index(keys, id); i >= 0 {
			out[key] = values[i]
		} else {
			out[key] = ""
		}
	}
	return out, nil
}

// SetPick setzt die Art key der Welt id auf die Katalog-ID value; "" entfernt den Eintrag. Die ID muss im Katalog
// stehen, die Welt muss es geben. Geschrieben wird nur der Block `worlds` der Katalog-Datei, Reihenfolge der Datei bleibt.
func (e Editor) SetPick(id, key, value string) error {
	pick, ok := catalogPicks[key]
	if !ok {
		return fmt.Errorf("unbekannte Art %q (erlaubt: %s)", key, strings.Join(slices.Sorted(maps.Keys(catalogPicks)), ", "))
	}
	if err := checkID(id); err != nil {
		return err
	}
	if _, err := os.Stat(e.file(id)); err != nil {
		return fmt.Errorf("welt %q gibt es nicht", id)
	}
	if value != "" {
		ids, err := e.catalogIDs(pick)
		if err != nil {
			return err
		}
		if !slices.Contains(ids, value) {
			return fmt.Errorf("unbekannte ID %q für %s (gültig: %s)", value, key, strings.Join(ids, ", "))
		}
	}
	writeMu.Lock()
	defer writeMu.Unlock()
	path := filepath.Join(e.Root, pick.file)
	raw, err := os.ReadFile(path)
	if err != nil {
		return err
	}
	block, found, err := valueSpan(raw, span{0, len(raw)}, "worlds")
	if err != nil {
		return err
	}
	if !found {
		return errors.New(pick.file + ": Feld worlds fehlt")
	}
	keys, values, err := worldsOf(raw)
	if err != nil {
		return fmt.Errorf("%s: %w", pick.file, err)
	}
	if i := slices.Index(keys, id); i >= 0 {
		if value == "" {
			keys, values = slices.Delete(keys, i, i+1), slices.Delete(values, i, i+1)
		} else {
			values[i] = value
		}
	} else if value != "" {
		keys, values = append(keys, id), append(values, value)
	}
	out := slices.Concat(raw[:block.from], []byte(renderWorlds(keys, values)), raw[block.to:])
	if string(out) == string(raw) {
		return nil
	}
	return writeAtomic(path, out)
}

// renderWorlds schreibt den Block `worlds` im Stil der Katalog-Dateien: je Welt eine Zeile, Reihenfolge wie übergeben.
func renderWorlds(keys, values []string) string {
	if len(keys) == 0 {
		return "{}"
	}
	lines := make([]string, len(keys))
	for i, k := range keys {
		lines[i] = "    " + quote(k) + ": " + quote(values[i])
	}
	return "{\n" + strings.Join(lines, ",\n") + "\n  }"
}

// pickValue liest den Wert einer Art aus dem Patch: Text oder null (= keins, entfernt den Eintrag).
func pickValue(v json.RawMessage) (string, error) {
	if !given(v) {
		return "", nil
	}
	var s string
	if err := json.Unmarshal(v, &s); err != nil {
		return "", errors.New("Katalog-ID als Text oder null")
	}
	return s, nil
}

// applyPicks schreibt die Arten (weather, mount, hermit) des Patches in die Katalog-Dateien, vor den Welt-Dateien.
func (e Editor) applyPicks(id string, p map[string]json.RawMessage) error {
	for k, v := range p {
		if _, pick := catalogPicks[k]; !pick {
			continue
		}
		value, err := pickValue(v)
		if err != nil {
			return fmt.Errorf("%s: %w", k, err)
		}
		if err := e.SetPick(id, k, value); err != nil {
			return err
		}
	}
	return nil
}

// checkPatchKeys prüft die Schlüssel eines world_set-Patches: Arten, Kopf-Felder und islands; story gehört zu world_story_set.
func checkPatchKeys(p map[string]json.RawMessage) error {
	for k := range p {
		if k == "story" {
			return errStoryTool
		}
		if _, pick := catalogPicks[k]; pick {
			continue
		}
		if !slices.Contains(headerKeys, k) && k != "islands" {
			return fmt.Errorf("unbekanntes Feld %q im patch (erlaubt: %s, islands, weather, mount, hermit)", k, strings.Join(headerKeys, ", "))
		}
	}
	return nil
}
