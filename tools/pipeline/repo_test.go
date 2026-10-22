package main

import (
	"bytes"
	"image/png"
	"os"
	"path/filepath"
	"testing"
)

// TestRepoResults prüft das eingecheckte Ergebnis (AC-02): jede Grafik der Liste liegt umgerechnet vor, ist byte-gleich
// mit einem neuen Lauf und hat nur DB32-Farben. Weicht etwas ab: `task gen:pipeline` laufen lassen.
func TestRepoResults(t *testing.T) {
	root := filepath.Join("..", "..")
	list, err := readList(filepath.Join(root, "docs", "assets", "pipeline-liste.json"))
	if err != nil {
		t.Fatal(err)
	}
	outs, err := build(list, filepath.Join(root, "public", "grafik"))
	if err != nil {
		t.Fatal(err)
	}
	outDir := filepath.Join(root, "public", "grafik-db32")
	for _, o := range outs {
		got, err := os.ReadFile(filepath.Join(outDir, filepath.FromSlash(o.rel)))
		if err != nil {
			t.Fatalf("%s fehlt im Ergebnis (task gen:pipeline): %v", o.rel, err)
		}
		if !bytes.Equal(got, o.data) {
			t.Fatalf("%s: Ergebnis veraltet oder nicht deterministisch (task gen:pipeline)", o.rel)
		}
		img, err := png.Decode(bytes.NewReader(got))
		if err != nil {
			t.Fatal(err)
		}
		assertDB32(t, o.rel, img, list[o.rel].Ausnahme != softAlpha)
	}
	if n := len(readTree(t, outDir)); n != len(outs) {
		t.Fatalf("%d Dateien im Ergebnisordner, %d in der Liste: verwaiste Ergebnisse entfernen", n, len(outs))
	}
}
