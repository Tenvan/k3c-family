package assetsui

import (
	"math"
	"slices"
	"strings"
	"testing"
)

// TestAssetsGroesse: Spielrolle, Maßstab des Monarchen und 1 px je Einheit (B-733/AC-01), gegen die echten Daten.
func TestAssetsGroesse(t *testing.T) {
	t.Parallel()
	d := dienst(root, false)
	monarch, err := d.AssetsGroesse("monarch")
	if err != nil {
		t.Fatal(err)
	}
	// data/sprites.json: medieval-king height 81, scale 1.5; Spec: ausgabe.rahmen[1] = -145
	if want := 81 * 1.5 / 145; math.Abs(monarch.PxProEinheit-want) > 1e-9 || !strings.Contains(monarch.Quelle, "Spieler") {
		t.Fatalf("monarch = %+v, will %.4f", monarch, want)
	}
	boss, err := d.AssetsGroesse("goblin-anfuehrer")
	if err != nil {
		t.Fatal(err)
	}
	if boss.PxProEinheit != monarch.PxProEinheit || !strings.Contains(boss.Quelle, "Monarchen") {
		t.Fatalf("goblin-anfuehrer = %+v", boss)
	}
	burg, err := d.AssetsGroesse("burg")
	if err != nil {
		t.Fatal(err)
	}
	if burg.PxProEinheit != 1 {
		t.Fatalf("burg = %+v", burg)
	}
	goblin, err := d.AssetsGroesse("goblin")
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(goblin.Quelle, "goblin") {
		t.Fatalf("goblin = %+v", goblin)
	}
}

// TestAssetsHintergrund: Biome aus den Ordnernamen; ein fremder Name wird abgelehnt, ohne eine Datei zu lesen.
func TestAssetsHintergrund(t *testing.T) {
	t.Parallel()
	d := dienst(root, false)
	teile, err := d.AssetsHintergruende()
	if err != nil {
		t.Fatal(err)
	}
	if !slices.Contains(teile, "wald") {
		t.Fatalf("teile = %v", teile)
	}
	h, err := d.AssetsHintergrund("wald")
	if err != nil {
		t.Fatal(err)
	}
	for _, b := range []string{h.Fern, h.Mitte, h.Nah} {
		if !strings.HasPrefix(b, "data:image/png;base64,") {
			t.Fatalf("Ebene ohne PNG: %.40s", b)
		}
	}
	if _, err := d.AssetsHintergrund("../wald"); err == nil {
		t.Fatal("fremder Name nicht abgelehnt")
	}
}
