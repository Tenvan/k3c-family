package mcpsrv

import (
	"context"
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

// MC1/AC-06: Register-Varianten von assets_list/get/set und assets_catalog sind entfernt; die Recherche bleibt.
func TestAlteRegisterToolsEntfernt(t *testing.T) {
	cs := connect(t, New(Config{Version: "test", Root: assetRepo(t)}))
	res, err := cs.ListTools(context.Background(), nil)
	if err != nil {
		t.Fatal(err)
	}
	da := map[string]bool{}
	for _, tool := range res.Tools {
		da[tool.Name] = true
		if tool.Name == "assets_list" || tool.Name == "assets_get" || tool.Name == "assets_set" {
			if strings.Contains(tool.Description, "data/assets.json") || strings.Contains(tool.Description, "Slot") {
				t.Errorf("%s beschreibt noch das Register", tool.Name)
			}
			props, _ := tool.InputSchema.(map[string]any)["properties"].(map[string]any)
			for _, alt := range []string{"art", "kategorie", "status", "wahl"} {
				if _, ok := props[alt]; ok {
					t.Errorf("%s hat noch das Feld %s", tool.Name, alt)
				}
			}
		}
	}
	if da["assets_catalog"] {
		t.Error("assets_catalog ist noch registriert")
	}
	for _, bleibt := range []string{"assets_research", "assets_hits", "assets_hit_add", "assets_hit_drop", "assets_import"} {
		if !da[bleibt] {
			t.Errorf("%s fehlt", bleibt)
		}
	}
	if strings.Contains(Instructions(), "assets_catalog") {
		t.Error("instructions.md nennt assets_catalog")
	}
}
