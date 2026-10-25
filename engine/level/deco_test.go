package level

import "testing"

// B-531: Die Höhle (`bush` in `resourcesPerChunk`) zeigt Deko-Büsche, der Wald hat je Waldabschnitt 14 bis 20 Bäume (Werte: data/biomes).
func TestHoehleMitBueschen(t *testing.T) {
	count := func(id, kind string) int {
		n := 0
		for _, seed := range []string{"b531-1", "b531-2", "b531-3"} {
			l, err := Generate(biome(t, id), seed)
			if err != nil {
				t.Fatal(err)
			}
			for _, e := range l.Entities {
				if e.Kind == kind {
					n++
				}
			}
		}
		return n
	}
	if got := count("cave", "bush"); got == 0 {
		t.Fatal("Höhle ohne Büsche")
	}
	forest, _ := biome(t, "forest").ResourcesPerChunk.Get("forest")
	trees, _ := forest.Get("tree")
	if trees != [2]int{22, 30} {
		t.Fatalf("Wald-Abschnitt: tree %v, erwartet [22 30]", trees)
	}
	if got := count("forest", "tree"); got == 0 {
		t.Fatal("Wald ohne Bäume")
	}
}
