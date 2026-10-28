package sim

import "k3c/engine/level"

// Schiff (BAS, B-637, docs/rules/geplante-chunks.md § 10, wirtschaft.md § 8): Der Hafen-Chunk (level.HarborKind) trägt
// den Platz `ship`. Er wird wie jeder Bau bezahlt (Gold, Material aus dem Vorrat, ein Bauer baut); die Abschnitte aus
// data/ship.json sind seine Ausbau-Stufen (Kiel = Bau, Rumpf und Segel = Stufe 2 und 3, hub_level.go), unabhängig von
// der Hub-Stufe. Gegner greifen den Platz nicht an. Fertig ist das Schiff mit der letzten Stufe; mit besiegtem Endboss
// öffnet es den Inselwechsel (island_switch.go).

const shipKind = "ship"

// shipFile ist data/ship.json.
type shipFile struct {
	Stages []struct {
		Cost         Cost
		HP           float64
		BuildSeconds float64
	}
}

// withShip trägt den Platz `ship` aus data/ship.json in die Gebäude ein: Kosten und Werte des ersten Abschnitts am
// Platz, die Abschnitte als Stufen.
func withShip(b map[string]BuildingData, f shipFile) map[string]BuildingData {
	var levels []LevelData
	for _, st := range f.Stages {
		levels = append(levels, LevelData{Cost: st.Cost, HP: st.HP, BuildSeconds: st.BuildSeconds})
	}
	if len(levels) == 0 {
		panic("ship.json: keine Abschnitte")
	}
	first := levels[0]
	b[shipKind] = BuildingData{HP: first.HP, BuildSeconds: first.BuildSeconds, Cost: first.Cost, Levels: levels}
	return b
}

// addShipSite legt den Schiffs-Platz an den Hafen der Stufe (letzter Platz der Liste, die Plätze davor behalten ihre IDs).
func addShipSite(w *World) {
	for _, e := range w.Level.Entities {
		if e.Kind == level.HarborKind {
			w.Sites = append(w.Sites, emptySite(w, shipKind, e.X))
		}
	}
}

// harbor ist die Stufe mit dem Hafen und ihr Schiffs-Platz; nil ohne Hafen.
func (isl *Island) harbor() (*World, *Site) {
	for _, w := range isl.Stages {
		for _, s := range w.Sites {
			if s.Kind == shipKind {
				return w, s
			}
		}
	}
	return nil, nil
}

// shipComplete: Alle Abschnitte des Schiffs sind gebaut.
func shipComplete(w *World, s *Site) bool {
	return s.State == "built" && s.Upgrade == "" && levelOf(w, s) >= len(buildings[shipKind].Levels)
}
