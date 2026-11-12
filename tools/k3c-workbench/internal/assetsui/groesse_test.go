package assetsui

import (
	"math"
	"os"
	"path/filepath"
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
	// data/paper.json › figuren › spieler[0]: hoehe 121,5 (= 81 × 1,5); Spec: ausgabe.rahmen[1] = -145
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
// Die Ordner baut der Test selbst: die gerenderten PNG sind nicht eingecheckt (B-737), ein frischer Checkout hat sie
// erst nach gen:paper (B-738).
func TestAssetsHintergrund(t *testing.T) {
	t.Parallel()
	repo := t.TempDir()
	for _, ordner := range []string{"hg-wald-fern-v1", "hg-wald-mitte-v1", "hg-wald-nah-v1", "hg-wald-nah-v2", "hg-hoehle-fern-v1"} {
		dir := filepath.Join(repo, filepath.FromSlash(hintergrundOrdner), ordner)
		if err := os.MkdirAll(dir, 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(filepath.Join(dir, "stand.png"), []byte("\x89PNG"), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	d := dienst(repo, false)
	teile, err := d.AssetsHintergruende()
	if err != nil {
		t.Fatal(err)
	}
	if !slices.Equal(teile, []string{"wald"}) {
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
