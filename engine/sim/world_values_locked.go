package sim

import (
	"encoding/json"
	"fmt"
	"maps"
	"slices"
)

// Nicht übersteuerbare Felder (docs/rules/welten.md § 1.3, B-692): Der Level-Generator (engine/level) liest diese Werte
// aus den eingebetteten Dateien, nicht aus dem Wertesatz der Welt. Ein Override darauf ließe Level (Lage der Plätze,
// Zahlziele, Ruinen-Planung) und Simulation auseinanderlaufen; es wird beim Laden mit Pfad abgelehnt (fail closed).

// lockedFields sind je Basis-Datei die Felder der obersten Ebene, die keine Welt übersteuert.
var lockedFields = map[string][]string{
	"economy": {"payRangeUnits"},
	"hub":     {"sites", "islandSites", "sitePlacement"},
}

// lockedPlantation: `buildings.<id>.plantation.stripUnits` prüft die Ruinen-Planung gegen die Basis (level/ruins.go).
const lockedPlantation = "stripUnits"

// checkLocked lehnt ein Override der Datei name auf ein nicht übersteuerbares Feld ab; der Pfad folgt den Meldungen von
// mergeFile. Formfehler meldet mergeFile vorher.
func checkLocked(name string, ov json.RawMessage) error {
	var obj map[string]json.RawMessage
	if json.Unmarshal(ov, &obj) != nil {
		return nil
	}
	for _, f := range lockedFields[name] {
		if _, ok := obj[f]; ok {
			return lockedErr(f)
		}
	}
	if name != "buildings" {
		return nil
	}
	for _, id := range slices.Sorted(maps.Keys(obj)) {
		var b struct {
			Plantation map[string]json.RawMessage `json:"plantation"`
		}
		if json.Unmarshal(obj[id], &b) == nil && b.Plantation[lockedPlantation] != nil {
			return lockedErr(id + ".plantation." + lockedPlantation)
		}
	}
	return nil
}

func lockedErr(path string) error {
	return fmt.Errorf("%s: nicht übersteuerbar (der Level-Generator liest die Basis, welten.md § 1.3)", path)
}
