package main

import (
	"bytes"
	"encoding/json"
	"image"
	"image/color"
	"image/png"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// fixture legt eine Mini-Wurzel mit einem genutzten Sheet (idle 3, run 2 Frames à 8x8) und einem ungenutzten an.
func fixture(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	must(t, os.MkdirAll(filepath.Join(root, "data"), 0o755))
	must(t, os.MkdirAll(filepath.Join(root, "public", "sprites", "hero"), 0o755))
	must(t, os.WriteFile(filepath.Join(root, "data", "sprites.json"), []byte(`{
	  "sheets": {"hero": {"frameWidth": 8, "frameHeight": 8, "anims": {"idle": {"frames": 3}, "run": {"frames": 2}}},
	             "unused": {"frameWidth": 8, "frameHeight": 8, "anims": {"idle": {"frames": 1}}}},
	  "players": [{"sheet": "hero"}], "troops": {}, "enemies": {}}`), 0o644))
	strip(t, filepath.Join(root, "public", "sprites", "hero", "idle.png"), 3)
	strip(t, filepath.Join(root, "public", "sprites", "hero", "run.png"), 2)
	return root
}

func strip(t *testing.T, path string, n int) {
	t.Helper()
	img := image.NewNRGBA(image.Rect(0, 0, 8*n, 8))
	for i := 0; i < n; i++ {
		img.Set(i*8+1, 1, color.NRGBA{R: uint8(40 * (i + 1)), G: 9, B: 9, A: 255})
	}
	var b bytes.Buffer
	must(t, png.Encode(&b, img))
	must(t, os.WriteFile(path, b.Bytes(), 0o644))
}

func must(t *testing.T, err error) {
	t.Helper()
	if err != nil {
		t.Fatal(err)
	}
}

func read(t *testing.T, dir string) map[string][]byte {
	t.Helper()
	files, _ := filepath.Glob(filepath.Join(dir, "atlas*"))
	m := map[string][]byte{}
	for _, f := range files {
		b, err := os.ReadFile(f)
		must(t, err)
		m[filepath.Base(f)] = b
	}
	return m
}

func TestDeterministisch(t *testing.T) {
	root := fixture(t)
	n, err := run(root, "out1", 4096)
	must(t, err)
	if n != 1 {
		t.Fatalf("Atlanten = %d, erwartet 1", n)
	}
	_, err = run(root, "out2", 4096)
	must(t, err)
	a, b := read(t, filepath.Join(root, "out1")), read(t, filepath.Join(root, "out2"))
	if len(a) != 2 || len(a) != len(b) {
		t.Fatalf("Dateien: %d und %d, erwartet je 2 (PNG + JSON)", len(a), len(b))
	}
	for k := range a {
		if !bytes.Equal(a[k], b[k]) {
			t.Errorf("%s unterscheidet sich zwischen zwei Läufen", k)
		}
	}
	var desc struct{ Textures []texture }
	must(t, json.Unmarshal(a["atlas.json"], &desc))
	if got := len(desc.Textures[0].Frames); got != 5 {
		t.Errorf("Frames = %d, erwartet 5 (das ungenutzte Sheet gehört nicht dazu)", got)
	}
}

func TestFehlendesBildBrichtMitNamenAb(t *testing.T) {
	root := fixture(t)
	must(t, os.Remove(filepath.Join(root, "public", "sprites", "hero", "run.png")))
	_, err := run(root, "out", 4096)
	if err == nil || !strings.Contains(err.Error(), "hero/run.png") {
		t.Fatalf("Fehler = %v, erwartet Abbruch mit hero/run.png", err)
	}
}

func TestKleineGrenzeErzwingtZweiAtlanten(t *testing.T) {
	root := fixture(t)
	n, err := run(root, "out", 16) // 16x16 fasst höchstens 4 Frames à 8x8 samt Abstand, 5 Frames brauchen 2 Atlanten
	must(t, err)
	if n < 2 {
		t.Fatalf("Atlanten = %d, erwartet mindestens 2", n)
	}
	if _, err := run(root, "out", 4); err == nil {
		t.Fatal("Frame größer als die Grenze muss einen Fehler liefern")
	}
}
