package world

import (
	"image"
	"math"
	"slices"
	"testing"

	"k3c/native/k3c-game/view"
)

// layerCell ist eine Zelle 1920 × 540 px, Maßstab 32 px je Unit, Kamera bei x.
func layerCell(x float64) view.Cell {
	c := view.Cell{Rect: image.Rect(0, 540, 1920, 1080)}
	c.Camera = view.NewCamera(c.Rect, x, 32)
	return c
}

// tileX ist das linke Fenster-x der Kachel k einer Schicht (Kulissen-Kachel 768 × 480, Fuß links unten).
func tileX(c view.Cell, factor float64, k int) rectF {
	return tileRect(c, factor, k, 768, 480, 0, 1, 1)
}

// AC-02: Bei zwei Kamera-Positionen steht `fern`, `mitte` und `nah` verschieben sich um ΔX · Faktor · Maßstab.
func TestLayerOffsets(t *testing.T) {
	forest := buildLooks("hintergruende", 2, nil)["forest"]
	if forest.scroll[0] != 0 {
		t.Fatalf("fern scrollt: %v", forest.scroll[0])
	}
	a, b := layerCell(40), layerCell(60)
	for i, factor := range forest.scroll {
		ra, rb := tileX(a, factor, 2), tileX(b, factor, 2)
		want := -20 * factor * 32
		if math.Abs((rb.x-ra.x)-want) > 1 {
			t.Errorf("%s: Versatz %v, erwartet %v", layerNames[i], rb.x-ra.x, want)
		}
	}
	if forest.scroll[1] <= 0 || forest.scroll[2] <= forest.scroll[1] {
		t.Errorf("mitte und nah müssen scrollen, nah schneller: %v", forest.scroll)
	}
}

// Kacheln: Fuß auf der Boden-Linie, 24 Units breit, Nachbarn stoßen ohne Lücke aneinander.
func TestTileRect(t *testing.T) {
	c := layerCell(13.3)
	r0, r1 := tileX(c, 0.35, 0), tileX(c, 0.35, 1)
	if r0.y+r0.h != float64(c.GroundY()) {
		t.Errorf("Fuß bei %v, Boden-Linie %d", r0.y+r0.h, c.GroundY())
	}
	if r0.x+r0.w != r1.x || r0.x != math.Round(r0.x) {
		t.Errorf("Naht: %v + %v ≠ %v", r0.x, r0.w, r1.x)
	}
	if math.Abs(r0.w-tileUnits*32) > 1 {
		t.Errorf("Breite %v", r0.w)
	}
}

// Asset-Quelle: alle Varianten aller Schichten des Levels, unbekanntes Level ohne Grafik; Varianten laufen reihum.
func TestAssetsAndVariants(t *testing.T) {
	ls := buildLooks("hintergruende", 2, map[string]palette{"forest": {Sky: "#8ecae6", Ground: "#588157"}})
	got := ls.assets("forest")
	if len(got) != 6 || !slices.Contains(got, "hintergruende/hg-wald-fern-v1") || !slices.Contains(got, "hintergruende/hg-wald-nah-v2") {
		t.Errorf("Assets: %v", got)
	}
	if ls.assets("nirgends") != nil {
		t.Error("unbekanntes Level mit Grafik")
	}
	if g := ls.of("forest").ground; g.R != 0x58 || g.G != 0x81 || g.B != 0x57 {
		t.Errorf("Boden-Farbe %v", g)
	}
	cases := []struct {
		seq     []int
		k, n, v int
	}{
		{nil, 3, 2, 1}, {[]int{1, 2, 3}, 1, 2, 2}, {[]int{1, 2, 3}, 2, 2, 1}, {[]int{1, 2, 3}, -1, 2, 1}, {[]int{4, 5}, 4, 5, 4},
	}
	for _, tc := range cases {
		if v := variant(tc.seq, tc.k, tc.n); v != tc.v {
			t.Errorf("variant(%v, %d, %d) = %d, erwartet %d", tc.seq, tc.k, tc.n, v, tc.v)
		}
	}
}

// Die Daten im Repo: jedes Biom mit Kulisse hat eine Palette, paper.json nennt Gruppe und Varianten.
func TestLoadLooks(t *testing.T) {
	group, n, err := backdropGroup()
	if err != nil || group != "hintergruende" || n < 1 {
		t.Fatalf("paper.json: %q %d %v", group, n, err)
	}
	pal, err := readPalettes()
	if err != nil {
		t.Fatal(err)
	}
	for id := range biomeLayers {
		if _, ok := pal[id]; !ok {
			t.Errorf("Biom %s ohne Palette", id)
		}
	}
}
