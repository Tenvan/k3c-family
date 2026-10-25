package level

import (
	"reflect"
	"testing"
)

// TestBackdrops: gleiche Eingabe gleiche Folge, Varianten 1 … n, Nachbarn verschieden, deckt die Weltbreite (GEB18, B-618/AC-03).
func TestBackdrops(t *testing.T) {
	a := backdrops("forest", "abc", 5, 480)
	if !reflect.DeepEqual(a, backdrops("forest", "abc", 5, 480)) {
		t.Fatal("gleicher Seed, andere Folge")
	}
	if len(a) != 21 {
		t.Fatalf("Länge %d, erwartet 480/24+1 = 21", len(a))
	}
	seen := map[int]bool{}
	for i, v := range a {
		if v < 1 || v > 5 {
			t.Fatalf("Kachel %d: Variante %d außerhalb 1 … 5", i, v)
		}
		if i > 0 && v == a[i-1] {
			t.Fatalf("Kachel %d: gleiche Variante %d wie der Nachbar", i, v)
		}
		seen[v] = true
	}
	if len(seen) < 4 {
		t.Errorf("nur %d verschiedene Varianten in 21 Kacheln: %v", len(seen), a)
	}
	if reflect.DeepEqual(a, backdrops("forest", "xyz", 5, 480)) || reflect.DeepEqual(a, backdrops("cave", "abc", 5, 480)) {
		t.Error("anderer Seed oder anderes Biom ergibt dieselbe Folge")
	}
	if got := backdrops("forest", "abc", 1, 100); len(got) != 6 || got[0] != 1 || got[5] != 1 {
		t.Errorf("eine Variante: %v", got)
	}
	if got := backdrops("forest", "abc", 0, 100); got != nil {
		t.Errorf("ohne Varianten: %v", got)
	}
}

// TestLevelBackdrops: das Level trägt die Folge aus den Biom-Daten, Chunks und Entities bleiben wie ohne sie.
func TestLevelBackdrops(t *testing.T) {
	for _, id := range []string{"forest", "cave", "mine", "ironhold", "crystal"} {
		b, err := LoadBiome(id)
		if err != nil {
			t.Fatal(err)
		}
		if b.BackdropVariants != 5 {
			t.Errorf("%s: backdropVariants %d, erwartet 5", id, b.BackdropVariants)
		}
		l, err := Generate(b, "golden")
		if err != nil {
			t.Fatal(err)
		}
		if !reflect.DeepEqual(l.Backdrops, backdrops(id, "golden", 5, l.WidthUnits)) {
			t.Errorf("%s: Folge im Level weicht ab", id)
		}
		b.BackdropVariants = 0
		ohne, err := Generate(b, "golden")
		if err != nil {
			t.Fatal(err)
		}
		if !reflect.DeepEqual(ohne.Chunks, l.Chunks) || !reflect.DeepEqual(ohne.Entities, l.Entities) {
			t.Errorf("%s: die Hintergrund-Folge verändert das Level", id)
		}
	}
}
