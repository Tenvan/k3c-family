package main

import (
	"bytes"
	"image"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func probe() building {
	return building{ID: "probe", Kacheln: kacheln{B: 2, H: 1}, Bausteine: []baustein{
		{Art: "pfosten", Rechteck: [4]float64{0.125, 0, 0.25, 1}, Farbe: "rope"},
		{Art: "tuch", Rechteck: [4]float64{1, 0.5, 0.5, 0.5}, Farbe: "brown"},
		{Art: "fuge", Rechteck: [4]float64{1.5, 0, 1.0 / 32, 1}, Farbe: "black", Fein: true},
	}}
}

func mustRender(t *testing.T, b building, d int) *image.Paletted {
	t.Helper()
	img, err := render(b, d)
	if err != nil {
		t.Fatal(err)
	}
	return img
}

func TestFeinereDichteIstVerlustfrei(t *testing.T) {
	b := probe()
	m, f := mustRender(t, b, 32), mustRender(t, b, 64)
	if f.Bounds().Dx() != 128 || f.Bounds().Dy() != 64 {
		t.Fatalf("64er Größe %v", f.Bounds())
	}
	for y := 0; y < 32; y++ {
		for x := 0; x < 64; x++ {
			if m.ColorIndexAt(x, y) != f.ColorIndexAt(2*x, 2*y) {
				t.Fatalf("Pixel %d,%d: 64 nach Downsampling ungleich 32", x, y)
			}
		}
	}
}

func TestGroebereDichteOhneFein(t *testing.T) {
	img := mustRender(t, probe(), 16)
	if img.Bounds().Dx() != 32 || img.Bounds().Dy() != 16 {
		t.Fatalf("16er Größe %v", img.Bounds())
	}
	if img.ColorIndexAt(24, 2) != 0 {
		t.Fatal("feine Fuge bei 16 px gezeichnet")
	}
	if img.ColorIndexAt(2, 2) == 0 {
		t.Fatal("Pfosten fehlt bei 16 px")
	}
	if mustRender(t, probe(), 32).ColorIndexAt(48, 2) == 0 {
		t.Fatal("feine Fuge fehlt bei 32 px")
	}
}

func TestUngueltigeDichte(t *testing.T) {
	_, err := render(probe(), 48)
	if err == nil || !strings.Contains(err.Error(), "16, 32, 64, 128 px") {
		t.Fatalf("Dichte 48: %v", err)
	}
}

func TestFehlerfaelle(t *testing.T) {
	cases := map[string]func(b *building){
		"fremde Farbe":     func(b *building) { b.Bausteine[0].Farbe = "lila" },
		"außerhalb":        func(b *building) { b.Bausteine[1].Rechteck[0] = 1.75 },
		"kein 1/32-Raster": func(b *building) { b.Bausteine[0].Rechteck[0] = 0.01 },
		"zu dünn bei 16":   func(b *building) { b.Bausteine[2].Fein = false },
	}
	for name, mod := range cases {
		b := probe()
		mod(&b)
		if _, err := render(b, 16); err == nil {
			t.Errorf("%s: kein Fehler", name)
		}
	}
}

func TestDeterministisch(t *testing.T) {
	var a, b bytes.Buffer
	if err := encode(&a, mustRender(t, probe(), 32)); err != nil {
		t.Fatal(err)
	}
	if err := encode(&b, mustRender(t, probe(), 32)); err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(a.Bytes(), b.Bytes()) {
		t.Fatal("zwei Läufe, ungleiche Bytes")
	}
}

func TestRunSchreibtIndexMitKlasse(t *testing.T) {
	dir := t.TempDir()
	src := `{"id":"tanne","klasse":"baum","vorlage":{"motiv":"schmale Tanne","teile":["Kegel"]},"kacheln":{"b":1,"h":1},"bausteine":[{"art":"krone","rechteck":[0,0,1,1],"farbe":"dell"}]}`
	if err := os.WriteFile(filepath.Join(dir, "tanne.json"), []byte(src), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(dir, "alt-32.png"), nil, 0o644); err != nil {
		t.Fatal(err)
	}
	if n, err := run(dir, "grafik/gebaeude/"); err != nil || n != 1 {
		t.Fatalf("run: %d, %v", n, err)
	}
	idx, err := os.ReadFile(filepath.Join(dir, "index.json"))
	if err != nil || !strings.Contains(string(idx), `"klasse": "baum"`) || !strings.Contains(string(idx), "grafik/gebaeude/tanne-16.png") || !strings.Contains(string(idx), `"motiv": "schmale Tanne"`) {
		t.Fatalf("index.json: %s %v", idx, err)
	}
	if _, err := os.Stat(filepath.Join(dir, "alt-32.png")); !os.IsNotExist(err) {
		t.Fatal("verwaiste PNG nicht entfernt")
	}
}

func TestRunLaesstOrdnerBeiFehlerUnveraendert(t *testing.T) {
	dir := t.TempDir()
	if err := os.WriteFile(filepath.Join(dir, "kaputt.json"), []byte(`{"id":"kaputt","kacheln":{"b":1,"h":1},"bausteine":[{"art":"x","rechteck":[0,0,1,1],"farbe":"lila"}]}`), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(dir, "alt-32.png"), nil, 0o644); err != nil {
		t.Fatal(err)
	}
	if _, err := run(dir, ""); err == nil {
		t.Fatal("kein Fehler bei fremder Farbe")
	}
	if _, err := os.Stat(filepath.Join(dir, "alt-32.png")); err != nil {
		t.Fatal("alte PNG trotz Fehler gelöscht")
	}
}
