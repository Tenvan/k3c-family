package sim

import (
	"strings"
	"testing"
	"testing/fstest"
)

// WZ17.2 (B-485/AC-02, AC-03): Inselgrößen und Chunk-Anzahl je Stufe (docs/rules/geplante-chunks.md § 7).

func islandsWith(fields string) fstest.MapFS {
	raw := `{"islands":[{"id":"a","depths":[0,1,2,3,4],"depthScaling":{"hp":1,"damage":1,"speed":1}` + fields + `}]}`
	return fstest.MapFS{"islands.json": {Data: []byte(raw)}}
}

func TestInselgroessenAusDenDaten(t *testing.T) {
	want := []IslandSize{{"sehr klein", 16}, {"klein", 24}, {"mittel", 40}, {"groß", 64}, {"sehr groß", 100}, {"mega groß", 160}}
	got := IslandSizes()
	if len(got) != len(want) {
		t.Fatalf("Inselgrößen %v, erwartet %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Errorf("Größe %d: %v, erwartet %v", i, got[i], want[i])
		}
	}
	got[0].ChunkCount = 99 // Kopie: der Aufrufer ändert die Tabelle nicht
	if IslandSizes()[0].ChunkCount != 16 {
		t.Error("IslandSizes liefert keine Kopie")
	}
}

func TestInselgroessenWerdenGeprueft(t *testing.T) {
	bad := map[string]string{
		"leer":      `{"sizes":[]}`,
		"ungerade":  `{"sizes":[{"name":"x","chunkCount":17}]}`,
		"doppelt":   `{"sizes":[{"name":"x","chunkCount":16},{"name":"y","chunkCount":16}]}`,
		"ohne Name": `{"sizes":[{"name":"","chunkCount":16}]}`,
		"zu klein":  `{"sizes":[{"name":"x","chunkCount":0}]}`,
	}
	for name, raw := range bad {
		var dst []IslandSize
		err := loadIslandSizes(fstest.MapFS{"island-sizes.json": {Data: []byte(raw)}}, &dst)
		if err == nil || dst != nil || !strings.Contains(err.Error(), "data/island-sizes.json") {
			t.Errorf("%s: Fehler %v, Ergebnis %v", name, err, dst)
		}
	}
}

func TestChunkAnzahlWirdBeimLadenGeprueft(t *testing.T) {
	var dst []IslandDef
	if err := loadIslands(islandsWith(`,"chunkCounts":{"0":24,"4":160}`), &dst); err != nil {
		t.Fatalf("gültige Anzahl abgelehnt: %v", err)
	}
	if o := dst[0].CreateOptions(); dst[0].ChunkCounts["0"] != 24 || o.ChunkCounts[0] != 24 || o.ChunkCounts[4] != 160 || len(o.ChunkCounts) != 2 {
		t.Errorf("Anzahl nicht geladen: %v, CreateOptions %v", dst[0].ChunkCounts, o.ChunkCounts)
	}
	bad := map[string][]string{
		`,"chunkCounts":{"0":18}`:  {"data/islands.json: a:", "Tiefe 0", "18", "16, 24, 40, 64, 100, 160"},
		`,"chunkCounts":{"3":12}`:  {"data/islands.json: a:", "Tiefe 3", "12", "16, 24, 40, 64, 100, 160"},
		`,"chunkCounts":{"7":24}`:  {"data/islands.json: a:", "Tiefe \"7\""},
		`,"chunkCounts":{"x":24}`:  {"data/islands.json: a:", "Tiefe \"x\""},
		`,"chunkCounts":{"01":24}`: {"data/islands.json: a:", "Tiefe \"01\""},
	}
	for fields, want := range bad {
		var out []IslandDef
		err := loadIslands(islandsWith(fields), &out)
		if err == nil || out != nil {
			t.Errorf("%s: Fehler %v, Ergebnis %v", fields, err, out)
			continue
		}
		for _, w := range want {
			if !strings.Contains(err.Error(), w) {
				t.Errorf("%s: Fehler %q nennt %q nicht", fields, err, w)
			}
		}
	}
}

// AC-03: Die Planung wird gegen die eingestellte Anzahl geprüft (Platz +9 im Wald: jenseits der kürzesten Länge ±7).
func TestPlanungGegenChunkAnzahl(t *testing.T) {
	plan := `,"plannedChunks":{"0":[{"slot":9,"kind":"forest"}]}`
	var dst []IslandDef
	if err := loadIslands(islandsWith(plan+`,"chunkCounts":{"0":24}`), &dst); err != nil {
		t.Errorf("Platz +9 bei 24 Chunks abgelehnt: %v", err)
	}
	for _, counts := range []string{``, `,"chunkCounts":{"0":16}`} {
		err := loadIslands(islandsWith(plan+counts), &dst)
		if err == nil || !strings.Contains(err.Error(), "Tiefe 0") || !strings.Contains(err.Error(), "Platz +9") {
			t.Errorf("%q: Fehler %v", counts, err)
		}
	}
}

func TestInselMitChunkAnzahl(t *testing.T) {
	isl, err := CreateIsland("groesse", []int{0, 1, 4}, 1, CreateOptions{ChunkCounts: map[int]int{0: 16, 4: 160}})
	if err != nil {
		t.Fatal(err)
	}
	plain, err := CreateIsland("groesse", []int{1}, 1, CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	for i, want := range []float64{16 * 50, plain.Stages[0].WidthUnits, 160 * 25} {
		if got := isl.Stages[i].WidthUnits; got != want {
			t.Errorf("Stufe %d: Breite %v, erwartet %v", i, got, want)
		}
	}
	if _, err := CreateIsland("groesse", []int{0}, 1, CreateOptions{ChunkCounts: map[int]int{0: 18}}); err == nil || !strings.Contains(err.Error(), "16, 24, 40, 64, 100, 160") {
		t.Errorf("Anzahl 18 angenommen: %v", err)
	}
}

func TestEingebetteteInselnOhneChunkAnzahl(t *testing.T) {
	for _, d := range islandDefs {
		if len(d.ChunkCounts) != 0 {
			t.Errorf("%s: eingebettete Daten enthalten eine Chunk-Anzahl", d.ID)
		}
	}
}
