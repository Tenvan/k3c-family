package menu

import (
	"os"
	"slices"
	"testing"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// B-818 AC-01/AC-02: Jeder Eintrag von Haupt- und Pause-Menü hat eine Illustration aus vorhandener Spielgrafik
// (Spec unter public/grafik, gerendert mit der Animation artAnim); Art nennt jede genau einmal und sortiert.
func TestMenuArtForEveryEntry(t *testing.T) {
	for i, a := range slices.Concat(mainArt[:], pauseArt[:]) {
		if a == "" {
			t.Errorf("Eintrag %d ohne Illustration", i)
			continue
		}
		// Eingecheckt ist nur die Spec; manifest.json entsteht erst mit gen:paper (B-737) und fehlt z. B. in der CI.
		if _, err := os.Stat("../../../public/grafik/" + a + ".paper.json"); err != nil {
			t.Errorf("%s: Spec fehlt: %v", a, err)
			continue
		}
		m, err := art.ReadManifest("../../../public/grafik", a)
		if err != nil {
			continue // nicht gerendert: Animation prüft der Lauf mit gen:paper
		}
		if !slices.Contains(m.AnimNames(), artAnim) {
			t.Errorf("%s ohne Animation %q: %v", a, artAnim, m.AnimNames())
		}
	}
	got := artList(defaultWorlds, true, func(string) bool { return true })
	if !slices.IsSorted(got) || len(slices.Compact(slices.Clone(got))) != len(got) {
		t.Errorf("Art nicht sortiert oder doppelt: %v", got)
	}
	// Im Level nur die kleinen Weltbilder, die großen nur im Hauptmenü (B-937).
	m := newMenus()
	m.usable = func(string) bool { return true }
	lvl := m.Assets("wald", nil)
	for w, img := range m.worlds {
		if !slices.Contains(got, img.Big) || !slices.Contains(got, img.Small) {
			t.Errorf("Art ohne Weltbilder von %s: %v", w, got)
		}
		if slices.Contains(lvl, img.Big) || !slices.Contains(lvl, img.Small) {
			t.Errorf("Assets je Level für %s: %v", w, lvl)
		}
	}
}

// Ohne Grafik-Bibliothek oder ohne Sheet zeichnet die Karte nur Text (artBox meldet false, nichts wird gezeichnet).
func TestMenuArtMissingIsTextCard(t *testing.T) {
	var p painter
	if p.artBox(nil, "objekte/portal", 0, 0, 10, 10) {
		t.Error("ohne Bibliothek gezeichnet")
	}
	lib, err := art.New(art.Config{GrafikDir: "../../../public/grafik", FontPath: "fehlt.ttf"})
	if err != nil {
		t.Fatal(err)
	}
	p.lib = lib
	if p.artBox(nil, "objekte/portal", 0, 0, 10, 10) {
		t.Error("ohne geladenes Sheet gezeichnet")
	}
}

// Haupt- und Pause-Menü geben ihre Illustration an die Zeilen weiter.
func TestMenuRowsCarryArt(t *testing.T) {
	m := newMenus()
	sc := newMain(m)
	sc.fill(view.DefaultLanguage)
	for i := range mainCount {
		if sc.pg.rows[i].art != mainArt[i] {
			t.Errorf("Hauptmenü Zeile %d: %q", i, sc.pg.rows[i].art)
		}
	}
}
