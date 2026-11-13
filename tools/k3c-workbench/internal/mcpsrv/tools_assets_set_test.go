package mcpsrv

import (
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"
)

func TestAssetsSetMaterial(t *testing.T) {
	if testing.Short() {
		t.Skip("ruft die Render-CLI (bun) gut zehnmal auf, je 0,3 s; läuft ohne -short (B-619, B-830)")
	}
	t.Parallel() // ohne t.Setenv und globale Zustände (B-830)
	root := paperRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	text, isErr := callText(t, cs, "assets_set", map[string]any{"id": "monarch", "patch": map[string]any{"umhang": map[string]any{"material": "purpur"}}})
	if isErr || !strings.HasPrefix(text, "monarch geändert: 1 Feld · umhang.material koenigsrot → purpur\n") {
		t.Fatalf("assets_set:\n%s", text)
	}
	if !regexp.MustCompile(`\nRender: \d+ PNGs? \(idle, lauf, reiten\) · Pixel-Anteil idle \d+,\d\d %, lauf \d+,\d\d %, reiten \d+,\d\d %`).MatchString(text) {
		t.Errorf("Render-Zeile:\n%s", text)
	}
	if !strings.Contains(text, "\nCheckout: Repo-Wurzel ") {
		t.Errorf("Checkout-Zeile:\n%s", text)
	}
	b, _ := os.ReadFile(filepath.Join(root, "public/grafik/figuren/monarch.paper.json"))
	if !strings.Contains(string(b), `"material": "purpur"`) {
		t.Error("Spec nicht geändert")
	}
}

// MC1/AC-02 · 005/AC-06: abgelehnter Patch ändert weder Spec noch Ausgabe.
func TestAssetsSetAbgelehnt(t *testing.T) {
	t.Parallel() // ohne t.Setenv und globale Zustände (B-830)
	root := paperRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	vorher := hashUnter(t, root, "public/grafik/figuren")
	text, isErr := callText(t, cs, "assets_set", map[string]any{"id": "monarch", "patch": map[string]any{"umhang": map[string]any{"material": "scharlach"}}})
	want := "abgelehnt, nichts geändert: 1 Fehler\numhang.material · material-unbekannt · Material „scharlach“ fehlt in der Stil-Datei\n" +
		"Erlaubt: Papier-Materialien und Einzeltöne aus assets_style_get"
	if !isErr || !strings.HasPrefix(text, want) || !strings.Contains(text, "\nCheckout: ") {
		t.Errorf("Ablehnung:\n%s", text)
	}
	if hashUnter(t, root, "public/grafik/figuren") != vorher {
		t.Error("Spec oder Ausgabe geändert")
	}
}

func TestAssetsStyle(t *testing.T) {
	t.Parallel() // ohne t.Setenv und globale Zustände (B-830)
	root := paperRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	text, isErr := callText(t, cs, "assets_style_get", map[string]any{})
	if isErr || !regexp.MustCompile(`^public/grafik/stil\.paper\.json · stilHash [0-9a-f]{64}\n`).MatchString(text) ||
		!regexp.MustCompile(`Materialien \(\d+\): haut, haar`).MatchString(text) ||!strings.Contains(text, "Einzeltöne (5): huf, auge") ||
		!strings.Contains(text, "papier: lage 0.5/0.9/0.45/0.55 #1a0f14 · auflage 0.3/0.5/0.3/0.35 · struktur 1.3/2/0.18 · falz 0.25 · licht rechts-oben") {
		t.Errorf("style_get:\n%s", text)
	}
	text, isErr = callText(t, cs, "assets_style_set", map[string]any{"patch": map[string]any{"materialien": map[string]any{"stein": map[string]any{"basis": "#a8a294"}}}})
	m := regexp.MustCompile(`\nBetroffen: 1 Asset \(mauer\) · Render als Job (j-\d+) gestartet\nWeiter: assets_render \{job: "j-\d+", wait: true\}`).FindStringSubmatch(text)
	if isErr || !strings.HasPrefix(text, "Stil geändert: materialien.stein.basis #9a9386 → #a8a294\n") || m == nil {
		t.Fatalf("style_set:\n%s", text)
	}
	// Der Job rendert im Test-Repo; vor dem Aufräumen auf sein Ende warten.
	if text, _ := callText(t, cs, "assets_render", map[string]any{"job": m[1], "wait": true}); !strings.Contains(text, "fertig: 1/1 Assets") {
		t.Errorf("Job:\n%s", text)
	}
	vorher := hashUnter(t, root, "public/grafik/stil.paper.json")
	text, isErr = callText(t, cs, "assets_style_set", map[string]any{"patch": map[string]any{"farbe": "#fff"}})
	if !isErr || !strings.HasPrefix(text, "abgelehnt, nichts geändert: 1 Fehler\nstil · schema · ") {
		t.Errorf("style_set abgelehnt:\n%s", text)
	}
	if hashUnter(t, root, "public/grafik/stil.paper.json") != vorher {
		t.Error("Stil-Datei geändert")
	}
}

// B-710: teile als Objekt nach ID ändert nur das Teil; B-706: assets_set auf ein Reittier nennt die Reiter.
func TestAssetsSetTeilPatchUndReiter(t *testing.T) {
	root := paperRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	text, isErr := callText(t, cs, "assets_set", map[string]any{"id": "monarch", "patch": map[string]any{"teile": map[string]any{"kopf": map[string]any{"pivot": []int{1, -91}}}}})
	b, _ := os.ReadFile(filepath.Join(root, "public/grafik/figuren/monarch.paper.json"))
	if isErr || !strings.Contains(string(b), `"pivot": [1, -91]`) || !strings.Contains(string(b), `"id": "rumpf"`) {
		t.Fatalf("Teil-Patch:\n%s", text)
	}
	text, isErr = callText(t, cs, "assets_set", map[string]any{"id": "monarch", "patch": map[string]any{"teile": map[string]any{"nase": map[string]any{}}}})
	if !isErr || !strings.Contains(text, "teile.nase · teil-unbekannt · Teil „nase“ fehlt; vorhanden: ") {
		t.Errorf("unbekanntes Teil:\n%s", text)
	}
	text, isErr = callText(t, cs, "assets_set", map[string]any{"id": "pferd", "patch": map[string]any{"version": 2}})
	if isErr || !strings.Contains(text, "\nReiter veraltet: monarch · Weiter: assets_render {id} je Reiter") {
		t.Errorf("Reiter:\n%s", text)
	}
}

func TestAssetsDelete(t *testing.T) {
	t.Parallel() // ohne t.Setenv und globale Zustände (B-830)
	root := paperRepo(t)
	schreibeDatei(t, root, "public/grafik/gebaeude/mauer/stand.png", "x")
	schreibeDatei(t, root, "public/grafik/gebaeude/index.paper.json", `[{"id": "mauer", "rig": "gebaeude", "manifest": "mauer/manifest.json"}]`)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	text, isErr := callText(t, cs, "assets_delete", map[string]any{"id": "mauer"})
	if isErr || !strings.HasPrefix(text, "nicht gelöscht: assets_delete braucht confirm=true (mauer, 1 PNG)") {
		t.Errorf("ohne confirm:\n%s", text)
	}
	if _, err := os.Stat(filepath.Join(root, "public/grafik/gebaeude/mauer.paper.json")); err != nil {
		t.Fatal("Spec ohne confirm gelöscht")
	}
	text, isErr = callText(t, cs, "assets_delete", map[string]any{"id": "mauer", "confirm": true})
	want := "mauer gelöscht: public/grafik/gebaeude/mauer.paper.json und public/grafik/gebaeude/mauer/ (1 PNG); Gruppen-Index gebaeude nachgezogen"
	if isErr || !strings.HasPrefix(text, want) {
		t.Errorf("mit confirm:\n%s", text)
	}
	if _, err := os.Stat(filepath.Join(root, "public/grafik/gebaeude/mauer")); !os.IsNotExist(err) {
		t.Error("Ausgabe-Ordner noch da")
	}
}
