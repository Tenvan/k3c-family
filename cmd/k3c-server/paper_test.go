package main

import (
	"os"
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
