package menu

import (
	"slices"
	"testing"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// B-818 AC-01/AC-02: Jeder Eintrag von Haupt- und Pause-Menü hat eine Illustration aus vorhandener Spielgrafik
// (Manifest mit der Animation artAnim unter public/grafik); Art nennt jede genau einmal und sortiert.
func TestMenuArtForEveryEntry(t *testing.T) {
	for i, a := range slices.Concat(mainArt[:], pauseArt[:]) {
		if a == "" {
			t.Errorf("Eintrag %d ohne Illustration", i)
			continue
		}
		m, err := art.ReadManifest("../../../public/grafik", a)
		if err != nil {
			t.Errorf("%s: %v", a, err)
			continue
		}
		if !slices.Contains(m.AnimNames(), artAnim) {
			t.Errorf("%s ohne Animation %q: %v", a, artAnim, m.AnimNames())
		}
	}
	got := Art()
	if !slices.IsSorted(got) || len(slices.Compact(slices.Clone(got))) != len(got) {
		t.Errorf("Art nicht sortiert oder doppelt: %v", got)
	}
	if a := (&menus{}).Assets("wald", nil); !slices.Equal(a, got) {
		t.Errorf("Assets je Level %v, erwartet %v", a, got)
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
