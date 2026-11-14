package actors

// Sheets je Level (view.AssetSource): nur die requisite-Figuren, die in der Welt vorkommen können, statt aller
// (B-797-Review: der Kristallherz-Boss allein sind 6 MiB). Quelle sind die Daten: Gegner des Bioms (Portal, Nacht),
// alle Gegner der Tiefe (enemies.json › depth: Elite, Händler-Überfall), Bosse der Tiefe (bosses.json, wie
// engine/sim/boss_data.go nach World.Biome.Depth) und ihre Beschwörungen. Dev-Spawns beliebiger Arten deckt das nicht.

import (
	"encoding/json"
	"slices"

	"k3c/engine/sim"
)

// byDepth ordnet die Gegner-Arten (Rohdaten aus enemies.json, Schlüssel ohne depth wie "notes" fallen weg) und die
// Bosse samt Beschwörungen ihrer Tiefe zu.
func byDepth(enemies map[string]json.RawMessage, bosses []bossEntry) (kinds, ids map[int][]string) {
	kinds, ids = map[int][]string{}, map[int][]string{}
	for kind, raw := range enemies {
		var v struct {
			Depth *int `json:"depth"`
		}
		if json.Unmarshal(raw, &v) == nil && v.Depth != nil {
			kinds[*v.Depth] = append(kinds[*v.Depth], kind)
		}
	}
	for _, b := range bosses {
		ids[b.Depth] = append(ids[b.Depth], b.ID)
		for _, a := range append([]*bossAbility{b.Ability}, phaseAbilities(b)...) {
			if a != nil && a.Enemy != "" {
				kinds[b.Depth] = append(kinds[b.Depth], a.Enemy)
			}
		}
	}
	return kinds, ids
}

func phaseAbilities(b bossEntry) []*bossAbility {
	out := make([]*bossAbility, 0, len(b.Phases))
	for _, p := range b.Phases {
		out = append(out, p.Ability)
	}
	return out
}

// LevelSheets nennt die Sheet-Assets der Figuren, die in der Welt w vorkommen können, sortiert und je einmal. Ohne
// Welt alle (SheetAssets).
func (c *Catalog) LevelSheets(w *sim.World) []string {
	if w == nil {
		return c.SheetAssets()
	}
	depth := w.Biome.Depth
	kinds := slices.Concat(w.Biome.Enemies.Portal, w.Biome.Enemies.Night, c.depthKinds[depth])
	var out []string
	for _, k := range kinds {
		if l, ok := c.Enemies[k]; ok && l.Sheet {
			out = append(out, l.Asset)
		}
	}
	for _, id := range c.depthBosses[depth] {
		if l, ok := c.Bosses[id]; ok && l.Sheet {
			out = append(out, l.Asset)
		}
	}
	for _, l := range c.Troops {
		if l.Sheet {
			out = append(out, l.Asset)
		}
	}
	slices.Sort(out)
	return slices.Compact(out)
}
