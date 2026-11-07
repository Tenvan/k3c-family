package mcpsrv

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// Werkzeuge des Vertrags (paper-mcp.md › 2).
var vertragTools = []string{"assets_guide", "assets_list", "assets_get", "assets_create", "assets_set", "assets_style_get",
	"assets_style_set", "assets_render", "assets_delete"}

// MC1/AC-01 · paper-mcp.md › 1: jedes Asset-Tool nimmt checkout; schreibende nennen den Checkout.
func TestAssetsToolsNehmenCheckout(t *testing.T) {
	root := paperRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	schemas := toolSchemas(t, cs)
	for _, name := range vertragTools {
		props, _ := schemas[name]["properties"].(map[string]any)
		if _, ok := props["checkout"]; !ok {
			t.Errorf("%s: kein checkout im Schema (%v)", name, props)
		}
	}
	for _, w := range []string{"assets_create", "assets_set", "assets_style_set", "assets_render", "assets_delete"} {
		if !writeTools[w] {
			t.Errorf("%s fehlt in writeTools", w)
		}
	}
	text, _ := callText(t, cs, "assets_render", map[string]any{"id": "monarch"})
	if !strings.Contains(text, "\nCheckout: ") {
		t.Errorf("assets_render ohne Checkout-Zeile: %q", text)
	}
}

func TestAssetsGuide(t *testing.T) {
	cs := connect(t, New(Config{Version: "test", Root: paperRepo(t)}))
	text, isErr := callText(t, cs, "assets_guide", map[string]any{"rig": "figur"})
	for _, want := range []string{"Papierschnitt · Rig figur · Standard docs/standards/paper-spec.md",
		"Papier-Materialien (licht/basis/schatten): haut, haar, gold, koenigsblau",
		"Einzeltöne (ohne Ton): huf, auge, weiss, linie, edelstein",
		"Pflicht-Teile figur: rumpf, kopf, oberarm-v", "Pivots: Hüfte (0,-50)",
		"Proportion: Kopfhöhe / Rahmenhöhe = 0.286 ± 0.15", "Lagen: flaeche {form, material, ton?, auflage?}",
		"Ablauf: assets_get → assets_set (Merge-Patch) → assets_preview → assets_review"} {
		if isErr || !strings.Contains(text, want) {
			t.Errorf("guide figur ohne %q:\n%s", want, text)
		}
	}
	alle, _ := callText(t, cs, "assets_guide", map[string]any{})
	for _, rig := range []string{"figur", "vierbeiner", "gebaeude", "requisite", "hintergrund"} {
		if !strings.Contains(alle, "Pflicht-Teile "+rig+": ") {
			t.Errorf("guide ohne rig: %s fehlt", rig)
		}
	}
	if text, isErr := callText(t, cs, "assets_guide", map[string]any{"rig": "drache"}); !isErr || !strings.Contains(text, "figur, vierbeiner") {
		t.Errorf("unbekannter Rig: %q", text)
	}
}

func TestAssetsListPaper(t *testing.T) {
	root := paperRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	text, isErr := callText(t, cs, "assets_list", map[string]any{})
	if isErr || !strings.HasPrefix(text, "3 Assets · 0 ungültig · 0 geändert gegen HEAD\n") ||
		!strings.Contains(text, "monarch · figur · public/grafik/figuren/monarch.paper.json · gültig\n") {
		t.Errorf("assets_list:\n%s", text)
	}
	schreibeDatei(t, root, "public/grafik/figuren/monarch/idle.png", "neu")
	text, _ = callText(t, cs, "assets_list", map[string]any{"geaendert": true})
	if !strings.HasPrefix(text, "1 Asset · 0 ungültig · 1 geändert gegen HEAD\n") || !strings.Contains(text, "monarch · figur · public/grafik/figuren/monarch.paper.json · gültig · geändert") {
		t.Errorf("geaendert:\n%s", text)
	}
	text, _ = callText(t, cs, "assets_list", map[string]any{"rig": "gebaeude"})
	if !strings.Contains(text, "mauer · gebaeude") || strings.Contains(text, "monarch") {
		t.Errorf("rig:\n%s", text)
	}
	text, _ = callText(t, cs, "assets_list", map[string]any{"gruppe": "reittiere"})
	if !strings.Contains(text, "pferd · vierbeiner") || strings.Contains(text, "mauer") {
		t.Errorf("gruppe:\n%s", text)
	}
}

func TestAssetsGetFelder(t *testing.T) {
	cs := connect(t, New(Config{Version: "test", Root: paperRepo(t)}))
	text, isErr := callText(t, cs, "assets_get", map[string]any{"id": "monarch", "felder": []string{"umhang", "ausgabe"}})
	if isErr || !strings.HasPrefix(text, "public/grafik/figuren/monarch.paper.json · Felder umhang, ausgabe\n{\n  \"umhang\": {") ||
		!strings.Contains(text, `"ausgabe": {`) || strings.Contains(text, `"teile"`) {
		t.Errorf("felder:\n%s", text)
	}
	text, _ = callText(t, cs, "assets_get", map[string]any{"id": "monarch"})
	if !strings.HasPrefix(text, "public/grafik/figuren/monarch.paper.json\n{") || strings.Contains(text, "$schema") || !strings.Contains(text, `"teile"`) {
		t.Errorf("ganz:\n%.300s", text)
	}
	if text, isErr := callText(t, cs, "assets_get", map[string]any{"id": "x"}); !isErr || text != "Asset „x“ fehlt" {
		t.Errorf("unbekannt: %q", text)
	}
	if text, isErr := callText(t, cs, "assets_get", map[string]any{"id": "monarch", "felder": []string{"farbe"}}); !isErr || !strings.Contains(text, "umhang") {
		t.Errorf("unbekanntes Feld: %q", text)
	}
}

func TestAssetsCreate(t *testing.T) {
	root := paperRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	args := map[string]any{"id": "fass", "rig": "requisite", "beschreibung": "Ein Fass", "gruppe": "requisiten"}
	text, isErr := callText(t, cs, "assets_create", args)
	for _, want := range []string{"fass angelegt: public/grafik/requisiten/fass.paper.json (Rig requisite, Vorlage mit Teil korpus)\n",
		"validate: ok\n", "Render: 1 PNG · ", "Nächster Schritt: assets_set {id: \"fass\", patch: …}, dann assets_preview"} {
		if isErr || !strings.Contains(text, want) {
			t.Errorf("create ohne %q:\n%s", want, text)
		}
	}
	vorher, _ := os.ReadFile(filepath.Join(root, "public/grafik/requisiten/fass.paper.json"))
	if text, isErr := callText(t, cs, "assets_create", args); !isErr || !strings.Contains(text, "fass gibt es schon") {
		t.Errorf("doppelt: %q", text)
	}
	if nachher, _ := os.ReadFile(filepath.Join(root, "public/grafik/requisiten/fass.paper.json")); string(nachher) != string(vorher) {
		t.Error("vorhandene Datei geändert")
	}
	if _, isErr := callText(t, cs, "assets_create", map[string]any{"id": "Fass!", "rig": "requisite", "beschreibung": "x", "gruppe": "g"}); !isErr {
		t.Error("ungültige id angenommen")
	}
}
