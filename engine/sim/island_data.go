package sim

import (
	"fmt"
	"io/fs"
	"slices"

	"k3c/data"
)

// Inseln als Daten (data/islands.json, B-103): Liste in fester Reihenfolge, je Insel ihre Tiefen und ihre Tabelle der
// Gegner-Skalierung. Welten ohne Insel skalieren nach waves.json › depthScaling.

// IslandDef ist eine Insel aus den Daten.
type IslandDef struct {
	ID, Name     string
	Depths       []int
	DepthScaling struct{ HP, Damage, Speed float64 }
}

var islandDefs = loadIslandDefs()

func loadIslandDefs() []IslandDef {
	var v []IslandDef
	if err := loadIslands(data.Files, &v); err != nil {
		panic(err)
	}
	return v
}

// loadIslands liest islands.json aus fsys und prüft jede Insel; bei einem Fehler bleibt *dst unverändert.
func loadIslands(fsys fs.FS, dst *[]IslandDef) error {
	var v struct{ Islands []IslandDef }
	if err := loadFrom(fsys, "islands.json", &v); err != nil {
		return err
	}
	if len(v.Islands) == 0 {
		return fmt.Errorf("data/islands.json: keine Insel")
	}
	var ids []string
	for _, d := range v.Islands {
		if err := checkIsland(d); err != nil {
			return fmt.Errorf("data/islands.json: %s: %w", d.ID, err)
		}
		if slices.Contains(ids, d.ID) {
			return fmt.Errorf("data/islands.json: %s doppelt", d.ID)
		}
		ids = append(ids, d.ID)
	}
	*dst = v.Islands
	return nil
}

// checkIsland: Tiefen vorhanden, je Tiefe ein Miniboss, in der tiefsten der Endboss, Faktoren > 0.
func checkIsland(d IslandDef) error {
	s := d.DepthScaling
	switch {
	case d.ID == "":
		return fmt.Errorf("ohne id")
	case len(d.Depths) == 0:
		return fmt.Errorf("ohne Tiefen")
	case s.HP <= 0 || s.Damage <= 0 || s.Speed <= 0:
		return fmt.Errorf("depthScaling braucht Faktoren > 0")
	case endBoss(slices.Max(d.Depths)) == nil:
		return fmt.Errorf("kein Endboss in der tiefsten Stufe %d", slices.Max(d.Depths))
	}
	for _, depth := range d.Depths {
		if !hasDepth(depth) {
			return fmt.Errorf("unbekannte Tiefe %d", depth)
		}
		if miniBoss(depth) == nil {
			return fmt.Errorf("kein Miniboss in Tiefe %d", depth)
		}
	}
	return nil
}
