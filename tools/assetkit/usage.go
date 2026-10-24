package assets

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

// maxUsage begrenzt die Verwendungsliste.
const maxUsage = 20

// usageSkip sind data/-Dateien, die Slots nur beschreiben und keine Verwendung sind.
var usageSkip = map[string]bool{"assets.json": true, "sprites.json": true, "assets-vorschlaege.json": true}

// findUsage nennt jede Stelle in data/*.json, an der die Slot-ID als Wert oder Schlüssel steht
// ("data/biomes.json › forest.enemies", Array-Indizes entfallen); sortiert, ohne Doppelte, höchstens 20 und dann
// ein Schlusseintrag „… n weitere“.
func findUsage(root, id string) []string {
	files, _ := filepath.Glob(filepath.Join(root, "data", "*.json"))
	found := map[string]bool{}
	for _, f := range files {
		name := filepath.Base(f)
		if usageSkip[name] {
			continue
		}
		b, err := os.ReadFile(f)
		if err != nil {
			continue
		}
		var v any
		if json.Unmarshal(b, &v) != nil {
			continue
		}
		walkUsage(v, nil, id, func(p []string) {
			at := "data/" + name
			if len(p) > 0 {
				at += " › " + strings.Join(p, ".")
			}
			found[at] = true
		})
	}
	out := make([]string, 0, len(found))
	for k := range found {
		out = append(out, k)
	}
	sort.Strings(out)
	if len(out) > maxUsage {
		out = append(out[:maxUsage], fmt.Sprintf("… %d weitere", len(out)-maxUsage))
	}
	return out
}

// walkUsage läuft durch den JSON-Baum; hit bekommt den Pfad ohne Array-Indizes.
func walkUsage(v any, path []string, id string, hit func([]string)) {
	switch t := v.(type) {
	case map[string]any:
		for k, c := range t {
			if strings.HasPrefix(k, "_comment") {
				continue
			}
			p := append(append([]string{}, path...), k)
			if k == id {
				hit(p)
			}
			walkUsage(c, p, id, hit)
		}
	case []any:
		for _, c := range t {
			walkUsage(c, path, id, hit)
		}
	case string:
		if t == id {
			hit(path)
		}
	}
}
