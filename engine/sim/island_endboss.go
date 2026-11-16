package sim

import (
	"fmt"
	"slices"
	"strings"

	"k3c/engine/level"
)

// Endboss je Insel (docs/rules/welten.md § 2 und § 3, bosse.md § 1, B-981): Das Insel-Feld `endBoss` wählt die ID aus
// data/bosses.json › endBosses, ohne Feld gilt der Endboss der tiefsten Stufe. Der Bau liegt immer in der tiefsten Stufe
// der Insel, in jedem Biom (level.Options.Lair); die Bau-Werte stehen am Endboss.

// LairDepth ist die Tiefe der Stufe mit dem Bau: die tiefste der Insel.
func (d IslandDef) LairDepth() int { return slices.Max(d.Depths) }

// EndBossID ist die ID des Endbosses der Insel (Feld `endBoss`, sonst der Endboss der tiefsten Stufe); leer, wenn keiner da ist.
func (d IslandDef) EndBossID() string {
	if b := d.endBossOf(); b != nil {
		return b.ID
	}
	return ""
}

// endBossOf ist der Endboss der Insel (nil: keiner gefunden, checkEndBoss lehnt das vorher ab).
func (d IslandDef) endBossOf() *BossData {
	if d.EndBoss != "" {
		if b := bossByID(d.EndBoss); b != nil && b.Kind == "end" {
			return b
		}
		return nil
	}
	return endBoss(d.LairDepth())
}

// endBossIDs: die IDs aller Endbosse in der Reihenfolge der Datei (für Meldungen).
func endBossIDs() string {
	ids := make([]string, len(endBosses))
	for i, b := range endBosses {
		ids[i] = b.ID
	}
	return strings.Join(ids, ", ")
}

// checkEndBoss prüft Endboss und Bau der Insel: eine unbekannte ID oder ein Miniboss in `endBoss`, kein Endboss für die
// tiefste Stufe und ein Biom, das den Bau samt Camp-Abstand nicht trägt (level.CheckLairFit), lehnen die Insel ab.
func checkEndBoss(d IslandDef) error {
	deepest := d.LairDepth()
	switch b := bossByID(d.EndBoss); {
	case d.EndBoss != "" && b == nil:
		return fmt.Errorf("endBoss %q unbekannt (erlaubt: %s)", d.EndBoss, endBossIDs())
	case d.EndBoss != "" && b.Kind != "end":
		return fmt.Errorf("endBoss %q ist ein Miniboss (erlaubt: %s)", d.EndBoss, endBossIDs())
	case d.EndBoss == "" && endBoss(deepest) == nil:
		return fmt.Errorf("kein Endboss in der tiefsten Stufe %d", deepest)
	}
	if err := level.CheckLairFit(biomeForDepth(deepest), d.countAt(deepest)); err != nil {
		return fmt.Errorf("bau in der tiefsten Stufe %d: %w", deepest, err)
	}
	return nil
}

// lairOptions: Die Stufe der Tiefe depth trägt den Bau (nur die tiefste Stufe der Insel def), und ihr Endboss.
func lairOptions(def IslandDef, depth int) (*bool, *BossData) {
	lair := depth == def.LairDepth()
	if !lair {
		return &lair, nil
	}
	return &lair, def.endBossOf()
}
