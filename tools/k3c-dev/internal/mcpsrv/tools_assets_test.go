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
	img := filepath.Join(root, "public", "grafik", "town", "house-a.png")
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

// B-298/AC-06: Tools für Slots und Zuordnung antworten; assets_set schreibt Reihenfolge und Ausschnitt.
func TestAssetsTools(t *testing.T) {
	cs := connect(t, New(Config{Version: "test", Root: assetRepo(t)}))
	steps := []struct {
		tool string
		args map[string]any
		want string
	}{
		{"assets_list", map[string]any{}, "farm gebaeude luecke · Farm → –"},
		{"assets_list", map[string]any{"art": "sound"}, "1 Asset-Slots\nsfx:coin sfx luecke"},
		{"assets_catalog", map[string]any{"query": "town"}, "grafik/town/house-a.png · town  · 64×48 · CC BY-NC 4.0 [NC]"},
		{"assets_set", map[string]any{"id": "farm", "wahl": []map[string]any{
			{"datei": "grafik/town/house-a.png", "ausschnitt": map[string]int{"x": 0, "y": 0, "w": 32, "h": 48}},
			{"datei": "grafik/town/house-a.png"},
		}}, "farm gespeichert: 2 Zuordnung(en)"},
		{"assets_get", map[string]any{"id": "farm"}, "1. grafik/town/house-a.png [0,0,32,48]\n2. grafik/town/house-a.png"},
		{"assets_list", map[string]any{"status": "zugeordnet"}, "farm gebaeude zugeordnet · Farm → grafik/town/house-a.png [0,0,32,48] (+1)"},
	}
	for _, st := range steps {
		text, isErr := callText(t, cs, st.tool, st.args)
		if isErr || !strings.Contains(text, st.want) {
			t.Errorf("%s %v: %q", st.tool, st.args, text)
		}
	}
	text, isErr := callText(t, cs, "assets_set", map[string]any{"id": "farm", "wahl": []map[string]any{{"datei": "../geheim.png"}}})
	if !isErr || !strings.Contains(text, "nicht unter public/") {
		t.Errorf("Pfad außerhalb: %q", text)
	}
}
