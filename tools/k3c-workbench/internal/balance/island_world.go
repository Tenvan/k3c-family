package balance

import (
	"fmt"
	"io/fs"

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
	first := world.Islands[0]
	return sim.CreateIslandIn(world, sc.Seed, first.Depths, 1, first.CreateOptions())
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
