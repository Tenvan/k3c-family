package balance

import (
	"fmt"
	"io/fs"
	"strings"

	"k3c/data"
	"k3c/engine/sim"
)

// createIsland baut die Insel des Szenarios: ohne World eine Stufe der Tiefe Depth, mit World alle Tiefen der ersten
// Insel der Welt (BS11, B-642). Die Stufe 0 ist die erste Tiefe der Insel.
func createIsland(sc Scenario) (*sim.Island, error) {
	if sc.World == "" {
		return sim.CreateIsland(sc.Seed, []int{sc.Depth}, 1, sim.CreateOptions{})
	}
	world, err := loadWorld(sc.World)
	if err != nil {
		return nil, err
	}
	if len(world.Islands) == 0 {
		return nil, fmt.Errorf("welt %q hat keine Insel", sc.World)
	}
	return createIn(world, sc.Seed)
}

// createIn löst die Welt samt `extends`-Kette und Overrides auf (sim.ResolveWorld, Eltern aus data/worlds) und baut ihre
// erste Insel; ein Fehler der Kette nennt die Welt (welten.md § 1.2).
func createIn(world sim.WorldDef, seed string) (*sim.Island, error) {
	world, err := sim.ResolveWorld(world, loadWorld)
	if err != nil {
		return nil, err
	}
	first := world.Islands[0]
	return sim.CreateIslandIn(world, seed, first.Depths, 1, first.CreateOptions())
}

// Worlds liefert die IDs der Welt-Dateien in data/worlds (sortiert), die ein Lauf als World nehmen darf.
func Worlds() ([]string, error) {
	worlds, err := fs.Sub(data.Files, "worlds")
	if err != nil {
		return nil, err
	}
	entries, err := fs.ReadDir(worlds, ".")
	if err != nil {
		return nil, err
	}
	var ids []string
	for _, e := range entries {
		if id, ok := strings.CutSuffix(e.Name(), ".json"); ok && !e.IsDir() {
			ids = append(ids, id)
		}
	}
	return ids, nil
}

func loadWorld(id string) (sim.WorldDef, error) {
	if id == sim.StandardWorldID {
		return sim.StandardWorld(), nil
	}
	worlds, err := fs.Sub(data.Files, "worlds")
	if err != nil {
		return sim.WorldDef{}, err
	}
	return sim.LoadWorld(worlds, id)
}
