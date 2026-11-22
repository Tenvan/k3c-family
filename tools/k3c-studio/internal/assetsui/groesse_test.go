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

// B-853: Größenvergleich auch für das Vorher-Bild. Spec und Maßstab kommen aus dem Stand der Basis, die Rolle aus dem
// Arbeitsstand; ein Asset, das es in der Basis nicht gibt, und eine leere Basis sind Fehler.
func TestAssetsGroesseBasis(t *testing.T) {
	t.Parallel()
	repo := testRepo(t)
	paper := `{"spieler": ["figuren/monarch"], "figuren": {"spieler": [{"hoehe": 120}]}}`
	if err := os.MkdirAll(filepath.Join(repo, "data"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(repo, "data", "paper.json"), []byte(paper), 0o644); err != nil {
		t.Fatal(err)
	}
	d := dienst(repo, false)
	monarch, err := d.AssetsGroesseBasis("monarch", "HEAD~1")
	if err != nil {
		t.Fatal(err)
	}
	ausgabe, _ := monarch.Spec["ausgabe"].(map[string]any)
	rahmen, _ := ausgabe["rahmen"].([]any)
	if len(rahmen) != 4 || monarch.Spec["$schema"] != nil {
		t.Fatalf("Spec der Basis = %v", monarch.Spec)
	}
	if want := 120 / -rahmen[1].(float64); math.Abs(monarch.PxProEinheit-want) > 1e-9 || !strings.Contains(monarch.Quelle, "Spieler") {
		t.Fatalf("monarch (HEAD~1) = %+v, will %.4f", monarch.Groesse, want)
	}
	// fass gibt es nur in der Basis HEAD~1 (im Arbeitsstand gelöscht): Requisiten zeichnet das Spiel 1:1.
	if fass, err := d.AssetsGroesseBasis("fass", "HEAD~1"); err != nil || fass.PxProEinheit != 1 {
		t.Fatalf("fass = %+v, %v", fass, err)
	}
	if _, err := d.AssetsGroesseBasis("pferd", "HEAD~1"); err == nil {
		t.Fatal("pferd fehlt in HEAD~1, kein Fehler")
	}
	if _, err := d.AssetsGroesseBasis("monarch", ""); err == nil {
		t.Fatal("leere Basis angenommen")
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
