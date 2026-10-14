package main

import (
	"image"
	"image/png"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	assetreg "k3c/tools/k3c-dev/internal/assets"
)

func assetApp(t *testing.T) *App {
	t.Helper()
	root := t.TempDir()
	reg := `{"_comment": "", "slots": [{"id": "farm", "art": "grafik", "kategorie": "gebaeude", "beschreibung": "Farm", "wahl": []}]}`
	if err := os.MkdirAll(filepath.Join(root, "data"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(root, assetreg.RegisterPath), []byte(reg), 0o644); err != nil {
		t.Fatal(err)
	}
	for _, name := range []string{"house-a.png", "house-b.png"} {
		p := filepath.Join(root, "public", "grafik", "town", name)
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		f, err := os.Create(p)
		if err != nil {
			t.Fatal(err)
		}
		if err := png.Encode(f, image.NewNRGBA(image.Rect(0, 0, 64, 48))); err != nil {
			t.Fatal(err)
		}
		_ = f.Close()
	}
	return &App{root: root}
}

// B-298/AC-02, AC-03 über die Oberfläche: Reihenfolge und Ausschnitt landen im Register, die Liste zeigt sie mit Status.
func TestAssetsBindings(t *testing.T) {
	app := assetApp(t)
	if v := app.AssetsList(); v.Error != "" || len(v.Slots) != 1 || v.Slots[0].Status != assetreg.StatusGap || v.Preview != "" {
		t.Fatalf("Liste vorher: %+v", v)
	}
	wahl := []assetreg.Choice{{File: "grafik/town/house-b.png", Cut: &assetreg.Rect{X: 0, Y: 0, W: 32, H: 32}}, {File: "grafik/town/house-a.png"}}
	got, err := app.AssetsSet("farm", wahl)
	if err != nil || got.Status != assetreg.StatusAssigned {
		t.Fatalf("AssetsSet: %+v %v", got, err)
	}
	v := app.AssetsList()
	c := v.Slots[0].Choices
	if len(c) != 2 || c[0].File != "grafik/town/house-b.png" || c[0].Cut == nil || c[0].Cut.W != 32 || c[1].File != "grafik/town/house-a.png" {
		t.Errorf("Register nach AssetsSet: %+v", c)
	}
}

func TestAssetsKatalogUndAblehnung(t *testing.T) {
	app := assetApp(t)
	hits, err := app.AssetsCatalog("house", "grafik", 0)
	if err != nil || len(hits) != 2 {
		t.Errorf("Katalog: %+v %v", hits, err)
	}
	if _, err := app.AssetsSet("farm", []assetreg.Choice{{File: "grafik/town/house-a.png", Cut: &assetreg.Rect{X: 60, Y: 0, W: 32, H: 48}}}); err == nil || !strings.Contains(err.Error(), "außerhalb") {
		t.Errorf("Ausschnitt außerhalb: %v", err)
	}
}

func TestAssetsListKaputt(t *testing.T) {
	app := assetApp(t)
	if err := os.WriteFile(filepath.Join(app.root, assetreg.RegisterPath), []byte("{"), 0o644); err != nil {
		t.Fatal(err)
	}
	if v := app.AssetsList(); !strings.Contains(v.Error, "ungültig") || len(v.Slots) != 0 {
		t.Errorf("kaputtes Register: %+v", v)
	}
}

func TestAssetsHitsBindings(t *testing.T) {
	app := assetApp(t)
	if hits, err := app.AssetsHits("farm"); err != nil || hits == nil || len(hits) != 0 {
		t.Fatalf("ohne Treffer: %v %v", hits, err)
	}
	if _, err := assetreg.AddHits(app.root, "farm", []assetreg.Hit{{Title: "Farm", Link: "https://x/farm"}}, time.Now()); err != nil {
		t.Fatal(err)
	}
	hits, _ := app.AssetsHits("farm")
	if len(hits) != 1 || hits[0].LicenseStatus != assetreg.LicenseCheck {
		t.Fatalf("Treffer: %+v", hits)
	}
	if err := app.AssetsHitDrop("farm", hits[0].ID); err != nil {
		t.Fatal(err)
	}
	if hits, _ := app.AssetsHits("farm"); len(hits) != 0 {
		t.Errorf("nach Verwerfen: %+v", hits)
	}
}

func TestAssetsLicenseOK(t *testing.T) {
	app := assetApp(t)
	if _, err := assetreg.AddHits(app.root, "farm", []assetreg.Hit{{Title: "Farm", Link: "https://x/farm"}}, time.Now()); err != nil {
		t.Fatal(err)
	}
	hits, _ := app.AssetsHits("farm")
	if err := app.AssetsLicenseOK("farm", hits[0].ID); err != nil {
		t.Fatal(err)
	}
	if hits, _ := app.AssetsHits("farm"); hits[0].LicenseStatus != assetreg.LicenseConfirmed {
		t.Errorf("Status: %+v", hits[0])
	}
}
