package main

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
	"testing/fstest"
)

// Gruppen-Index: jedes Manifest und jedes Sheet muss da sein (US1/AC-02).
func TestCheckPaperFehlendeDatei(t *testing.T) {
	ok := fstest.MapFS{
		"grafik/truppen/index.paper.json":      {Data: []byte(`[{"id":"krieger","rig":"figur","manifest":"krieger/manifest.json"}]`)},
		"grafik/truppen/krieger/manifest.json": {Data: []byte(`{"animationen":{"idle":{"datei":"idle.png"},"lauf":{"datei":"lauf.png"}}}`)},
		"grafik/truppen/krieger/idle.png":      {Data: []byte("png")},
		"grafik/truppen/krieger/lauf.png":      {Data: []byte("png")},
	}
	if errs := checkPaper(ok); len(errs) != 0 {
		t.Fatalf("gültiger Stand: %v", errs)
	}
	delete(ok, "grafik/truppen/krieger/lauf.png")
	errs := checkPaper(ok)
	if len(errs) != 1 || !strings.Contains(errs[0].Error(), "grafik/truppen/krieger/lauf.png") {
		t.Fatalf("fehlendes Sheet: %v", errs)
	}
	delete(ok, "grafik/truppen/krieger/manifest.json")
	if errs := checkPaper(ok); len(errs) != 1 || !strings.Contains(errs[0].Error(), "manifest.json") {
		t.Fatalf("fehlendes Manifest: %v", errs)
	}
	ok["grafik/truppen/index.paper.json"] = &fstest.MapFile{Data: []byte(`kaputt`)}
	if errs := checkPaper(ok); len(errs) != 1 || !strings.Contains(errs[0].Error(), "index.paper.json") {
		t.Fatalf("kaputter Index: %v", errs)
	}
}

// Der eingecheckte Stand unter public/ ist vollständig.
func TestCheckPaperRepoStand(t *testing.T) {
	if errs := checkPaper(os.DirFS("../../public")); len(errs) != 0 {
		t.Fatalf("public/: %v", errs)
	}
}

// Nur Gruppen, die data/paper.json nutzt, blockieren; ein fehlendes Figuren-Asset im Index auch (Review US1, W3).
func TestCheckPaperNurGenutzteGruppen(t *testing.T) {
	public := fstest.MapFS{
		"grafik/reittiere/index.paper.json": {Data: []byte(`[{"id":"pferd","manifest":"pferd/manifest.json"}]`)},
		"grafik/truppen/index.paper.json":   {Data: []byte(`[]`)},
	}
	data := fstest.MapFS{"paper.json": {Data: []byte(`{"spieler":[],"beritten":"truppen/krieger","truppen":{},"gegner":{},"gebaeude":"gebaeude-paper","hintergruende":{"gruppe":"hintergruende","varianten":2}}`)}}
	errs, warns := checkPaperUse(public, data)
	if len(warns) != 1 || !strings.Contains(warns[0].Error(), "reittiere") {
		t.Fatalf("ungenutzte Gruppe soll nur warnen: %v", warns)
	}
	if len(errs) != 1 || !strings.Contains(errs[0].Error(), "truppen/krieger") {
		t.Fatalf("fehlendes Asset der Rolle: %v", errs)
	}
}

// Eingecheckter Stand: was data/paper.json nutzt, ist vollständig; checkData mit public endet mit 0.
func TestCheckPaperUseRepoStand(t *testing.T) {
	errs, _ := checkPaperUse(os.DirFS("../../public"), os.DirFS("../../data"))
	if len(errs) != 0 {
		t.Fatalf("public/: %v", errs)
	}
}

// Fehlt ein Sheet einer genutzten Gruppe, endet checkData mit 1 (alter Stand bleibt aktiv).
func TestCheckDataPaperFehlerExit1(t *testing.T) {
	public := t.TempDir()
	dir := filepath.Join(public, "grafik", "gebaeude-paper", "mauer")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	_ = os.WriteFile(filepath.Join(public, "grafik", "gebaeude-paper", "index.paper.json"), []byte(`[{"id":"mauer","manifest":"mauer/manifest.json"}]`), 0o644)
	_ = os.WriteFile(filepath.Join(dir, "manifest.json"), []byte(`{"animationen":{"stand":{"datei":"stand.png"}}}`), 0o644)
	var out strings.Builder
	if code := checkData(config{public: public}, &out); code != 1 || !strings.Contains(out.String(), "stand.png") {
		t.Fatalf("code %d, %s", code, out.String())
	}
}
