package assets

import (
	"path/filepath"
	"testing"
)

// katalog liest den Katalog des Mini-Repos mit einem Pferd (NC-SA), Gruppen und Credits.
func katalog(t *testing.T) []Asset {
	t.Helper()
	root := repo(t)
	writePNG(t, filepath.Join(root, "public", "sprites", "horse", "idle.png"), 100, 65)
	writeFile(t, filepath.Join(root, "public", "grafik", "index.json"), `[{"pack":"town","group":"props","file":"house-a.png","w":64,"h":48}]`)
	writeFile(t, filepath.Join(root, "public", "grafik", "CREDITS.md"), "# Grafik\n\n| Ordner | Pack | Urheber | Lizenz | Quelle |\n|---|---|---|---|---|\n"+
		"| town, `city` | GothicVania Town | Luis Zuno | CC0 1.0 | https://example.org/town |\n")
	writeFile(t, filepath.Join(root, "public", "sprites", "CREDITS.md"), "| Ordner | Pack | Urheber | Lizenz | Quelle |\n|---|---|---|---|---|\n"+
		"| horse | Horse | Jemand | CC BY-NC-SA 4.0 | https://example.org/horse |\n")
	list, err := Catalog(root)
	if err != nil {
		t.Fatal(err)
	}
	return list
}

func TestKatalogMitCredits(t *testing.T) {
	list := katalog(t)
	if len(list) != 4 {
		t.Fatalf("Katalog: %+v", list)
	}
	byFile := map[string]Asset{}
	for _, a := range list {
		byFile[a.File] = a
	}
	house := byFile["grafik/town/house-a.png"]
	if house.Pack != "town" || house.Group != "props" || house.W != 64 || house.H != 48 || house.Artist != "Luis Zuno" || house.Flag != "" {
		t.Errorf("house-a: %+v", house)
	}
	if h := byFile["sprites/horse/idle.png"]; h.Flag != "NC" || h.W != 100 {
		t.Errorf("horse: %+v", h)
	}
	if c := byFile["audio/coin.ogg"]; c.Kind != KindSound || c.License != selfMade {
		t.Errorf("coin: %+v", c)
	}
}

// B-437: eigene Gebäude-Grafik (tools/gebaeude) ohne Credit-Zeile hat eine eigene Lizenz, fremde Packs ohne Credit bleiben „unbekannt“.
func TestKatalogEigeneGebaeude(t *testing.T) {
	root := repo(t)
	writePNG(t, filepath.Join(root, "public", "grafik", "gebaeude", "taverne-32.png"), 192, 128)
	writePNG(t, filepath.Join(root, "public", "grafik", "fremd", "x.png"), 16, 16)
	list, err := Catalog(root)
	if err != nil {
		t.Fatal(err)
	}
	byFile := map[string]Asset{}
	for _, a := range list {
		byFile[a.File] = a
	}
	if g := byFile["grafik/gebaeude/taverne-32.png"]; g.License != ownBuilding || g.Flag != "" {
		t.Errorf("gebaeude: %+v", g)
	}
	if f := byFile["grafik/fremd/x.png"]; f.License != "unbekannt" {
		t.Errorf("fremd: %+v", f)
	}
}

func TestKatalogSuche(t *testing.T) {
	list := katalog(t)
	if got := Search(list, "TOWN props", "", 0); len(got) != 1 || got[0].File != "grafik/town/house-a.png" {
		t.Errorf("Suche town props: %+v", got)
	}
	if got := Search(list, "nc-sa", "", 0); len(got) != 1 {
		t.Errorf("Suche Lizenz: %+v", got)
	}
	if got := Search(list, "", KindSound, 0); len(got) != 1 {
		t.Errorf("Filter Art: %+v", got)
	}
	if got := Search(list, "", "", 2); len(got) != 2 {
		t.Errorf("limit: %d", len(got))
	}
}

// Q70: ND nicht übernehmbar vor NC vor SA; Wörter wie „Sample“ zählen nicht.
func TestLicenseFlag(t *testing.T) {
	cases := map[string]string{
		"CC0 1.0": "", "CC BY 3.0": "", "OGA-BY 3.0": "", "CC BY-SA 4.0": "SA", "CC-BY-SA 3.0": "SA",
		"CC BY-NC 4.0": "NC", "CC BY-NC-SA 4.0": "NC", "CC BY-NC-ND 4.0": "ND", "Sample Pack": "", selfMade: "", ownBuilding: "",
	}
	for lic, want := range cases {
		if got := LicenseFlag(lic); got != want {
			t.Errorf("%q: %q, erwartet %q", lic, got, want)
		}
	}
}
