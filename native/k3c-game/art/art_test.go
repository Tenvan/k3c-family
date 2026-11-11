package art

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"golang.org/x/image/font/gofont/goregular"
)

const testGrafik = "testdata/grafik"

func TestPlanLevel(t *testing.T) {
	p, err := PlanLevel(testGrafik, "test", []string{"figuren/zwerg"})
	if err != nil {
		t.Fatal(err)
	}
	if len(p.Sheets) != 2 || p.Sheets[0].Anim != "idle" || p.Sheets[1].Anim != "lauf" {
		t.Fatalf("Sheets %+v", p.Sheets)
	}
	s := p.Sheets[0]
	if s.Frames != 3 || s.FW != 4 || s.FH != 6 || s.W != 12 || s.H != 6 || s.OriginX != 0.5 || s.OriginY != 5.0/6 || s.UnitsPerPx != 1 {
		t.Errorf("Maße %+v", s)
	}
}

func TestPlanLevelBudget(t *testing.T) {
	p, err := PlanLevel(testGrafik, "test", []string{"figuren/zwerg"})
	if err != nil {
		t.Fatal(err)
	}
	// 12×6 + 16×6 Pixel, 4 Byte je Pixel
	if p.Budget.Images != 2 || p.Budget.Bytes != (12*6+16*6)*4 {
		t.Errorf("Budget %+v", p.Budget)
	}
	if got := p.Budget.String(); got != "2 Bilder, 0.0 MiB RGBA" {
		t.Errorf("Text %q", got)
	}
	b := Budget{}
	b.Add(1024, 1024)
	b.AddBytes(1 << 20)
	if b.MiB() != 5 || b.Images != 1 {
		t.Errorf("MiB %v %+v", b.MiB(), b)
	}
}

func TestPlanLevelMissing(t *testing.T) {
	_, err := PlanLevel(testGrafik, "test", []string{"figuren/zwerg", "figuren/gibtsnicht", "gegner/auchnicht"})
	if !errors.Is(err, ErrMissing) {
		t.Fatalf("erwartet ErrMissing, bekam %v", err)
	}
	for _, want := range []string{"gibtsnicht", "auchnicht", "manifest.json", "task gen:paper"} {
		if !strings.Contains(err.Error(), want) {
			t.Errorf("Meldung ohne %q: %v", want, err)
		}
	}
}

func TestPlanLevelMissingSheet(t *testing.T) {
	dir := t.TempDir()
	asset := filepath.Join(dir, "figuren", "x")
	if err := os.MkdirAll(asset, 0o755); err != nil {
		t.Fatal(err)
	}
	m := `{"rahmen":[-1,-1,2,2],"frameW":2,"frameH":2,"animationen":{"idle":{"frames":1,"fps":1,"datei":"idle.png"}}}`
	if err := os.WriteFile(filepath.Join(asset, "manifest.json"), []byte(m), 0o644); err != nil {
		t.Fatal(err)
	}
	_, err := PlanLevel(dir, "t", []string{"figuren/x"})
	if !errors.Is(err, ErrMissing) || !strings.Contains(err.Error(), "idle.png") {
		t.Fatalf("Meldung ohne Pfad des Sheets: %v", err)
	}
}

func TestPlanLevelBadSize(t *testing.T) {
	dir := t.TempDir()
	if err := os.CopyFS(dir, os.DirFS(testGrafik)); err != nil {
		t.Fatal(err)
	}
	m := `{"rahmen":[-2,-5,4,6],"frameW":4,"frameH":6,"animationen":{"idle":{"frames":9,"fps":6,"datei":"idle.png"}}}`
	if err := os.WriteFile(filepath.Join(dir, "figuren", "zwerg", "manifest.json"), []byte(m), 0o644); err != nil {
		t.Fatal(err)
	}
	_, err := PlanLevel(dir, "t", []string{"figuren/zwerg"})
	if err == nil || !strings.Contains(err.Error(), "zu klein") {
		t.Fatalf("zu kleines Bild nicht erkannt: %v", err)
	}
}

func TestGroupAssets(t *testing.T) {
	got, err := GroupAssets(testGrafik, "figuren")
	if err != nil || len(got) != 1 || got[0] != "figuren/zwerg" {
		t.Fatalf("%v %v", got, err)
	}
	if _, err := GroupAssets(testGrafik, "nix"); !errors.Is(err, ErrMissing) || !strings.Contains(err.Error(), "index.paper.json") {
		t.Errorf("fehlender Index: %v", err)
	}
}

func TestFontMissing(t *testing.T) {
	path := filepath.Join(t.TempDir(), "fehlt.ttf")
	_, err := LoadFont(path, true)
	if !errors.Is(err, ErrMissing) || !strings.Contains(err.Error(), path) {
		t.Fatalf("strenger Modus: %v", err)
	}
	f, err := LoadFont(path, false)
	if err != nil || !f.Fallback {
		t.Fatalf("Ersatz-Schrift: %+v %v", f, err)
	}
	if w, h := f.Measure("abc", 20); w != 18 || h != 16 {
		t.Errorf("Debug-Maße %v×%v", w, h)
	}
}

func TestFontLoad(t *testing.T) {
	path := filepath.Join(t.TempDir(), "go.ttf")
	if err := os.WriteFile(path, goregular.TTF, 0o644); err != nil {
		t.Fatal(err)
	}
	f, err := LoadFont(path, true)
	if err != nil || f.Fallback {
		t.Fatalf("%+v %v", f, err)
	}
	w, h := f.Measure("Hallo", 20)
	if w <= 0 || h <= 0 {
		t.Errorf("Maße %v×%v", w, h)
	}
	if f.face(20) != f.face(20) {
		t.Error("Face je Größe nicht gecacht")
	}
	bad := filepath.Join(t.TempDir(), "kaputt.ttf")
	if err := os.WriteFile(bad, []byte("kein Font"), 0o644); err != nil {
		t.Fatal(err)
	}
	if _, err := LoadFont(bad, false); err == nil || errors.Is(err, ErrMissing) {
		t.Errorf("kaputte Schrift: %v", err)
	}
}

func TestNilLibrary(t *testing.T) {
	var l *Library
	if l.Sheet("a/b", "idle") != nil || l.Level() != "" || l.Budget().Bytes != 0 {
		t.Error("nil-Library liefert Inhalt")
	}
	if _, err := l.NewActor("figuren/monarch", "", "x"); !errors.Is(err, ErrMissing) {
		t.Errorf("NewActor ohne Library: %v", err)
	}
}

func TestSheetFrameIndex(t *testing.T) {
	s := &Sheet{}
	if s.Frame(3) != nil || s.FrameAt(1) != nil {
		t.Error("leeres Sheet liefert Bild")
	}
}

func TestLoadLevelMissingKeepsState(t *testing.T) {
	l := &Library{cfg: Config{GrafikDir: testGrafik}, sheets: map[sheetKey]*Sheet{}}
	err := l.LoadLevel("kaputt", []string{"figuren/gibtsnicht"})
	if !errors.Is(err, ErrMissing) || l.Level() != "" {
		t.Fatalf("%v, Level %q", err, l.Level())
	}
}
