package actors

import (
	"slices"
	"testing"

	"k3c/data"
	"k3c/engine/level"
	"k3c/engine/sim"
)

// Je Biom nur die Sheets der Figuren, die dort vorkommen können: der Wald keine, der Kristall Spinne, Splitterling und
// den Endboss, die Mine Rattenschwarm (auch als Beschwörung der Ratten-Königin) und Grubengeist. Jedes Sheet gehört zu
// mindestens einem Biom.
func TestLevelSheets(t *testing.T) {
	c, err := LoadCatalog(data.Files)
	if err != nil {
		t.Fatal(err)
	}
	want := map[string][]string{
		"forest":  nil,
		"cave":    {"gegner/fledermaus"},
		"mine":    {"gegner/grubengeist", "gegner/rattenschwarm"},
		"crystal": {"bosse/kristallherz-waechter", "gegner/kristallspinne", "gegner/splitterling"},
	}
	var all []string
	for _, id := range []string{"forest", "cave", "mine", "ironhold", "crystal"} {
		b, err := level.LoadBiome(id)
		if err != nil {
			t.Fatal(err)
		}
		got := c.LevelSheets(&sim.World{Biome: b})
		if w, ok := want[id]; ok && !slices.Equal(got, w) {
			t.Errorf("%s: %v, erwartet %v", id, got, w)
		}
		all = append(all, got...)
	}
	slices.Sort(all)
	if all = slices.Compact(all); !slices.Equal(all, c.SheetAssets()) {
		t.Errorf("Sheets ohne Biom: alle Biome %v, Katalog %v", all, c.SheetAssets())
	}
	if got := c.LevelSheets(nil); !slices.Equal(got, c.SheetAssets()) {
		t.Errorf("ohne Welt: %v", got)
	}
}
