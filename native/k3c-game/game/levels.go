package game

import (
	"log/slog"
	"slices"

	"k3c/native/k3c-game/menu"
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

// menuLevel ist der Schlüssel der Menü-Illustrationen ohne Level (Hauptmenü, B-818); kein Biom heißt so.
const menuLevel = "_menu"

// loadLevels lädt die Grafik, sobald sich die Level in Frame.Stages ändern; ein Fehler (fehlende Grafik) zeigt die
// Schleife im Fenster (fail, B-810) statt still zu beenden. Ohne Stufen (Hauptmenü) lädt es nur die Illustrationen
// des Menüs; fehlen sie, bleibt das Menü bei Text-Karten (Log-Zeile, kein Abbruch).
func (g *Game) loadLevels() {
	if g.parts.Art == nil {
		return
	}
	if len(g.frame.Stages) == 0 {
		g.loadMenuArt()
		return
	}
	if SameLevels(g.levels, g.frame.Stages) {
		return
	}
	want := WantedAssets(g.frame.Stages, g.parts.Assets)
	if err := g.parts.Art.SetLevels(want); err != nil {
		g.fail(err)
		return
	}
	g.menuArt = false
	g.levels = g.levels[:0]
	for lv := range want {
		g.levels = append(g.levels, lv)
	}
	slices.Sort(g.levels)
}

// loadMenuArt stellt die Grafik einmal auf die Menü-Illustrationen (gibt Level-Sheets frei); ein Fehler bleibt beim
// Log und wird nicht wiederholt.
func (g *Game) loadMenuArt() {
	if g.menuArt {
		return
	}
	g.menuArt, g.levels = true, g.levels[:0]
	if err := g.parts.Art.SetLevels(map[string][]string{menuLevel: menu.Art()}); err != nil {
		slog.Warn("🚫 Menü-Illustrationen fehlen, Karten nur mit Text", "err", err)
	}
}
