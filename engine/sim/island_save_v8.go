package sim

import (
	"fmt"
	"slices"
)

// Spielstand Version 8 (WZ18.3, B-488/AC-03, docs/rules/welten.md § 5): Der Stand nennt die Welt (`world`) und die
// aktuelle Insel (`islandId`) statt des Index `island`. Stände bis Version 7 kommen in die Standard-Welt, auf die Insel
// am alten Index; die Datei wird dabei nicht umgeschrieben (Version 8 entsteht beim nächsten Speichern). Planung und
// Erzeugungs-Optionen stehen nicht im Stand, sie kommen beim Laden aus der Welt.

// migrateWorld setzt Welt und Insel eines Stands bis Version 7: Standard-Welt, Insel am alten Index.
func migrateWorld(s IslandSave, index int) (IslandSave, error) {
	id, err := legacyIslandID(standardWorld, index)
	if err != nil {
		return IslandSave{}, err
	}
	s.World, s.IslandID = standardWorld.ID, id
	return s, nil
}

// legacyIslandID ist die ID der Insel am Index index der Welt w; ein Index außerhalb der Liste ist ein Fehler.
func legacyIslandID(w WorldDef, index int) (string, error) {
	if index < 0 || index >= len(w.Islands) {
		return "", fmt.Errorf("spielstand: Insel %d, die Daten kennen %d", index, len(w.Islands))
	}
	return w.Islands[index].ID, nil
}

// islandIn liefert den Index der Insel des Stands in der Welt w. Fehler (welten.md § 5): Welt des Stands ist nicht w,
// Insel-ID gibt es in w nicht.
func islandIn(w WorldDef, s IslandSave) (int, error) {
	if s.World != w.ID {
		return 0, fmt.Errorf("spielstand: Welt %q unbekannt", s.World)
	}
	i := slices.IndexFunc(w.Islands, func(d IslandDef) bool { return d.ID == s.IslandID })
	if i < 0 {
		return 0, fmt.Errorf("spielstand: Insel %q gibt es in Welt %q nicht", s.IslandID, s.World)
	}
	return i, nil
}
