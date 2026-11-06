package paperassets

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestAnlegen(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	ctx := context.Background()
	r, err := Anlegen(ctx, cli, "fass", "requisite", "Ein Fass", "requisiten")
	if err != nil {
		t.Fatal(err)
	}
	if r.Asset.Pfad != "public/grafik/requisiten/fass.paper.json" || len(r.Fehler) != 0 || r.Render.PNGs != 1 {
		t.Errorf("Anlegen: %+v", r)
	}
	if _, err := os.Stat(filepath.Join(root, "public/grafik/requisiten/fass.paper.json")); err != nil {
		t.Fatal(err)
	}
	if _, err := Anlegen(ctx, cli, "fass", "requisite", "x", "andere"); err == nil || !strings.Contains(err.Error(), "fass gibt es schon") {
		t.Errorf("doppelt: %v", err)
	}
	for _, f := range [][4]string{{"Fass!", "requisite", "x", "g"}, {"a", "drache", "x", "g"}, {"a", "requisite", "", "g"}, {"a", "requisite", "x", "../g"}} {
		if _, err := Anlegen(ctx, cli, f[0], f[1], f[2], f[3]); err == nil {
			t.Errorf("%v ohne Fehler", f)
		}
	}
}

func TestLoesche(t *testing.T) {
	t.Parallel()
	root, _ := testRepo(t)
	schreibe(t, root, "public/grafik/gebaeude/mauer/stand.png", "x")
	schreibe(t, root, "public/grafik/gebaeude/mauer/manifest.json", "{}")
	schreibe(t, root, "public/grafik/gebaeude/index.paper.json",
		`[{"id": "mauer", "rig": "gebaeude", "manifest": "mauer/manifest.json"}, {"id": "turm", "rig": "gebaeude", "manifest": "turm/manifest.json"}]`)
	if n, err := AusgabePNGs(root, "mauer"); err != nil || n != 1 {
		t.Errorf("AusgabePNGs: %d %v", n, err)
	}
	r, err := Loesche(root, "mauer")
	if err != nil {
		t.Fatal(err)
	}
	if r.PNGs != 1 || !r.Index || r.Asset.Gruppe != "gebaeude" {
		t.Errorf("Loesche: %+v", r)
	}
	for _, p := range []string{"public/grafik/gebaeude/mauer.paper.json", "public/grafik/gebaeude/mauer"} {
		if _, err := os.Stat(filepath.Join(root, p)); !os.IsNotExist(err) {
			t.Errorf("%s noch da", p)
		}
	}
	b, _ := os.ReadFile(filepath.Join(root, "public/grafik/gebaeude/index.paper.json"))
	if strings.Contains(string(b), `"mauer"`) || !strings.Contains(string(b), `"turm"`) {
		t.Errorf("Index: %s", b)
	}
	if _, err := Loesche(root, "mauer"); err == nil {
		t.Error("zweites Löschen ohne Fehler")
	}
}

func TestGueltigkeit(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	schreibe(t, root, "public/grafik/requisiten/banner.paper.json", bannerUngueltig)
	assets, _ := Assets(root)
	g := NewGueltigkeit()
	f := g.Pruefe(context.Background(), cli, assets)
	if len(f["banner"]) != 1 || f["banner"][0].Regel != "material-unbekannt" || len(f["monarch"]) != 0 || len(f) != 4 {
		t.Errorf("Gueltigkeit: %+v", f)
	}
}

// Review MC1.2: gemischter Cache (Treffer und neue Prüfungen) in einem Aufruf.
func TestGueltigkeitGemischterCache(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	assets, _ := Assets(root)
	g := NewGueltigkeit()
	g.Pruefe(context.Background(), cli, assets[:1])
	schreibe(t, root, "public/grafik/requisiten/banner.paper.json", bannerUngueltig)
	alle, _ := Assets(root)
	f := g.Pruefe(context.Background(), cli, alle)
	if len(f) != 4 || len(f["banner"]) != 1 {
		t.Errorf("gemischt: %+v", f)
	}
}

// Review MC1.2: ein Aufruf- oder Abbruchfehler wird nicht gecacht.
func TestGueltigkeitCachtKeineCLIFehler(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	assets, _ := Assets(root)
	g := NewGueltigkeit()
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	if f := g.Pruefe(ctx, cli, assets[:1]); len(f[assets[0].ID]) != 1 || f[assets[0].ID][0].Regel != "cli" {
		t.Fatalf("Abbruch: %+v", f)
	}
	if f := g.Pruefe(context.Background(), cli, assets[:1]); len(f[assets[0].ID]) != 0 {
		t.Errorf("Abbruch gecacht: %+v", f)
	}
}

// Review MC1.2: index ist als id reserviert (Gruppen-Index).
func TestAnlegenIndexReserviert(t *testing.T) {
	t.Parallel()
	_, cli := testRepo(t)
	if _, err := Anlegen(context.Background(), cli, "index", "requisite", "x", "requisiten"); err == nil {
		t.Error("id index angenommen")
	}
}
