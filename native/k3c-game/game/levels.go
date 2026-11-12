package game

import (
	"slices"

	"k3c/native/k3c-game/view"
)

// Loader lädt die Grafik der Level (art.Library.SetLevels): genau die Level in want, Level → Assets `gruppe/id`.
type Loader interface {
	SetLevels(want map[string][]string) error
}

// SameLevels meldet, ob die Level der Stufen genau die Menge have (sortiert, ohne Doppelte) sind; ohne Allokation.
func SameLevels(have []string, stages []view.Stage) bool {
	n := 0
	for i, st := range stages {
		if !slices.Contains(have, st.Level) {
			return false
		}
		if !slices.ContainsFunc(stages[:i], func(o view.Stage) bool { return o.Level == st.Level }) {
			n++
		}
	}
	return n == len(have)
}

// WantedAssets sammelt je Level der Stufen die Assets aller Quellen, je Level sortiert und ohne Doppelte; die Welt
// eines Levels ist die seiner ersten Stufe.
func WantedAssets(stages []view.Stage, sources []view.AssetSource) map[string][]string {
	want := map[string][]string{}
	for _, st := range stages {
		if _, ok := want[st.Level]; ok {
			continue
		}
		var assets []string
		for _, src := range sources {
			assets = append(assets, src.Assets(st.Level, st.World)...)
		}
		slices.Sort(assets)
		want[st.Level] = slices.Compact(assets)
	}
	return want
}

// loadLevels lädt die Grafik, sobald sich die Level in Frame.Stages ändern; ein Fehler (fehlende Grafik) zeigt die
// Schleife im Fenster (fail, B-810) statt still zu beenden.
func (g *Game) loadLevels() {
	if g.parts.Art == nil || SameLevels(g.levels, g.frame.Stages) {
		return
	}
	want := WantedAssets(g.frame.Stages, g.parts.Assets)
	if err := g.parts.Art.SetLevels(want); err != nil {
		g.fail(err)
		return
	}
	g.levels = g.levels[:0]
	for lv := range want {
		g.levels = append(g.levels, lv)
	}
	slices.Sort(g.levels)
}
