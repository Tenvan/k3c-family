package mcpsrv

import (
	"image"
	"image/png"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// assetRepo ist ein Mini-Repo mit Slot-Register, einem Bild und einer Credit-Zeile.
func assetRepo(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	files := map[string]string{
		"data/assets.json": `{"_comment": "Test", "slots": [
  {"id": "farm", "art": "grafik", "kategorie": "gebaeude", "beschreibung": "Farm", "wahl": []},
  {"id": "sfx:coin", "art": "sound", "kategorie": "sfx", "beschreibung": "Münze", "bus": "sfx", "wahl": []}]}`,
		"public/grafik/CREDITS.md": "| Ordner | Pack | Urheber | Lizenz | Quelle |\n|---|---|---|---|---|\n| town | Town | Zuno | CC BY-NC 4.0 | https://example.org |\n",
	}
	for rel, text := range files {
		p := filepath.Join(root, filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	img := filepath.Join(root, "public", "grafik", "town", "well.png")
	if err := os.MkdirAll(filepath.Dir(img), 0o755); err != nil {
		t.Fatal(err)
	}
	f, err := os.Create(img)
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = f.Close() }()
	if err := png.Encode(f, image.NewNRGBA(image.Rect(0, 0, 64, 48))); err != nil {
		t.Fatal(err)
	}
	return root
}

// B-298: assets_catalog sucht Dateien mit Lizenz-Kennzeichen (bleibt bis MC1.4).
func TestAssetsCatalog(t *testing.T) {
	cs := connect(t, New(Config{Version: "test", Root: assetRepo(t)}))
	text, isErr := callText(t, cs, "assets_catalog", map[string]any{"query": "town"})
	if isErr || !strings.Contains(text, "grafik/town/well.png · town  · 64×48 · CC BY-NC 4.0 [NC]") {
		t.Errorf("assets_catalog: %q", text)
	}
}
