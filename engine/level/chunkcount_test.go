package level

import (
	"encoding/json"
	"slices"
	"strconv"
	"strings"
	"testing"

	"k3c/data"
	"k3c/engine/rng"
)

// WZ17.2 (B-485/AC-02, AC-03): Chunk-Anzahl je Stufe (docs/rules/geplante-chunks.md § 7).

// sizeCounts liest die Chunk-Anzahlen der Inselgrößen aus data/island-sizes.json (einzige Quelle der Tabelle).
func sizeCounts(t *testing.T) []int {
	t.Helper()
	raw, err := data.Files.ReadFile("island-sizes.json")
	if err != nil {
		t.Fatal(err)
	}
	var v struct {
		Sizes []struct {
			ChunkCount int `json:"chunkCount"`
		} `json:"sizes"`
	}
	if err := json.Unmarshal(raw, &v); err != nil {
		t.Fatal(err)
	}
	out := make([]int, len(v.Sizes))
	for i, s := range v.Sizes {
		out[i] = s.ChunkCount
	}
	if !slices.Equal(out, []int{16, 24, 40, 64, 100, 160}) {
		t.Fatalf("Inselgrößen %v, laut Regel 16·24·40·64·100·160", out)
	}
	return out
}

// Mit gesetzter Anzahl würfelt frame() die Länge nicht: Der Haupt-Strom steht danach noch am Anfang.
func TestFrameMitAnzahlWuerfeltNicht(t *testing.T) {
	b := biome(t, "forest")
	g := &gen{b: b, r: rng.New("forest:x"), cw: b.ChunkWidthUnits, count: 20}
	g.frame()
	if got, want := g.r.Next(), rng.New("forest:x").Next(); got != want {
		t.Errorf("frame() hat mit Anzahl gewürfelt: nächster Wert %v, erwartet %v", got, want)
	}
	if len(g.kinds) != 20 {
		t.Errorf("%d Chunks, erwartet 20", len(g.kinds))
	}
}

// Ohne Anzahl bleibt der Wurf: gleiches Level wie Generate (die Golden-Dateien prüfen die Bytes).
func TestOhneAnzahlWieBisher(t *testing.T) {
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		for seed := range 20 {
			want, _ := Generate(b, strconv.Itoa(seed))
			got, err := GenerateWith(b, strconv.Itoa(seed), Options{ChunkCount: 0})
			if err != nil || !slices.Equal(got.Chunks, want.Chunks) || !slices.Equal(got.Entities, want.Entities) {
				t.Fatalf("Biom %s, Seed %d: ohne Anzahl anders als Generate (%v)", id, seed, err)
			}
		}
	}
}

// Je Inselgröße und Biom ein Einzeltest (ein Seed): genau diese Chunks, gerade, Hub mittig, Validate ohne Befund.
// Kein Nachweis über viele Seeds; geht eine Größe nicht, meldet GenerateWith einen Fehler.
func TestChunkAnzahlJeGroesse(t *testing.T) {
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		for _, n := range sizeCounts(t) {
			o := Options{ChunkCount: n}
			l, err := GenerateWith(b, "groesse", o)
			if err != nil {
				t.Fatalf("Biom %s, %d Chunks: %v", id, n, err)
			}
			checkFrame(t, l, b, n)
			if errs := ValidateWith(l, b, o); len(errs) > 0 {
				t.Errorf("Biom %s, %d Chunks: %v", id, n, errs)
			}
		}
	}
}

// checkFrame: n Chunks, Breite n × Chunk-Breite, Hub-Chunks und Burg genau in der Mitte.
func checkFrame(t *testing.T, l Layout, b Biome, n int) {
	t.Helper()
	hubChunks := max(2, int(b.HubWidthUnits/b.ChunkWidthUnits/2+0.5)*2)
	if len(l.Chunks) != n || l.WidthUnits != float64(n)*b.ChunkWidthUnits || l.HubCenterUnits != l.WidthUnits/2 {
		t.Fatalf("Biom %s: %d Chunks, Breite %v, Hub %v; erwartet %d Chunks, Hub mittig", b.ID, len(l.Chunks), l.WidthUnits, l.HubCenterUnits, n)
	}
	for i, c := range l.Chunks {
		inHub := i >= n/2-hubChunks/2 && i < n/2+hubChunks/2
		if inHub != (c.Kind == "hub") {
			t.Fatalf("Biom %s, %d Chunks: Chunk %d ist %q", b.ID, n, i, c.Kind)
		}
	}
}

func TestValidateMitAnzahlMeldetFalscheLaenge(t *testing.T) {
	b := biome(t, "forest")
	l, err := GenerateWith(b, "laenge", Options{ChunkCount: 16})
	if err != nil {
		t.Fatal(err)
	}
	if errs := Validate(l, b); !slices.Contains(errs, "Level zu kurz") {
		t.Errorf("Validate ohne Anzahl = %q, erwartet „Level zu kurz“ (16 Chunks Wald)", errs)
	}
	want := []string{"Level hat 16 Chunks, erwartet 24"}
	if got := ValidateWith(l, b, Options{ChunkCount: 24}); !slices.Equal(got, want) {
		t.Errorf("ValidateWith = %q, erwartet %q", got, want)
	}
}

func TestGenerateLehntUngueltigeAnzahlAb(t *testing.T) {
	for _, n := range []int{-2, 2, 4, 17} {
		if _, err := GenerateWith(biome(t, "forest"), "x", Options{ChunkCount: n}); err == nil || !strings.Contains(err.Error(), "Chunk-Anzahl") {
			t.Errorf("Anzahl %d: Fehler %v", n, err)
		}
	}
}

// AC-03: Ein geplanter Platz außerhalb der Anzahl wird mit Platz abgelehnt; die Prüfung rechnet mit der Anzahl.
func TestPlanungGegenDieAnzahl(t *testing.T) {
	cases := []struct {
		biome string
		count int
		slot  int
	}{{"forest", 16, 7}, {"forest", 16, -7}, {"forest", 20, 9}, {"ironhold", 16, -6}, {"crystal", 16, 6}}
	for _, c := range cases {
		err := ValidatePlanFor(biome(t, c.biome), []PlannedChunk{p(c.slot, "chest")}, c.count)
		want := "auf Platz " + strconv.FormatInt(int64(c.slot), 10)
		if c.slot > 0 {
			want = "auf Platz +" + strconv.Itoa(c.slot)
		}
		if err == nil || !strings.Contains(err.Error(), want) || !strings.Contains(err.Error(), "Chunk-Anzahl "+strconv.Itoa(c.count)) {
			t.Errorf("%s, %d Chunks, Platz %+d: Fehler %v", c.biome, c.count, c.slot, err)
		}
	}
	// Jenseits der kürzesten Länge (Wald ±7), aber innerhalb von 48 Chunks: gültig und im Level an seinem Platz.
	b := biome(t, "forest")
	plan := []PlannedChunk{p(20, "clearing"), p(-6, "chest")}
	if err := ValidatePlan(b, plan); err == nil {
		t.Error("Platz +20 ohne Anzahl angenommen")
	}
	o := Options{ChunkCount: 48, Planned: plan}
	l, err := GenerateWith(b, "weit", o)
	if err != nil {
		t.Fatal(err)
	}
	if k := l.Chunks[24+20].Kind; k != "clearing" {
		t.Errorf("Platz +20 bei 48 Chunks ist %q", k)
	}
	if errs := ValidateWith(l, b, o); len(errs) > 0 {
		t.Errorf("Validate: %v", errs)
	}
}

// Mit Planung und Anzahl deterministisch: gleicher Seed, gleiches Level.
func TestAnzahlUndPlanungDeterministisch(t *testing.T) {
	b := biome(t, "cave")
	o := Options{ChunkCount: 32, Planned: []PlannedChunk{p(-10, "cavern"), p(8, "chest")}}
	first, err := GenerateWith(b, "Käse🧀", o)
	if err != nil {
		t.Fatal(err)
	}
	again, _ := GenerateWith(b, "Käse🧀", o)
	if !slices.Equal(first.Chunks, again.Chunks) || !slices.Equal(first.Entities, again.Entities) {
		t.Error("gleicher Seed, anderes Level")
	}
}
