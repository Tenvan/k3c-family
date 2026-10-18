package room

import (
	"slices"
	"strconv"
	"strings"
	"testing"

	"k3c/engine/level"
	"k3c/engine/sim"
)

// WZ17.3 (B-485/AC-04): Die Vorschau nimmt die Chunk-Anzahl als Option, ohne globalen Zustand (wie WZ8.4).

func countOpts(n int, plan ...level.PlannedChunk) PreviewOptions {
	o := plannedOpts(plan...)
	o.ChunkCount = n
	return o
}

func TestVorschauMitChunkAnzahlHatGenauDieseChunks(t *testing.T) {
	for _, size := range sim.IslandSizes() {
		w := mustPreview(t, countOpts(size.ChunkCount))
		if got := len(w.Level.Chunks); got != size.ChunkCount {
			t.Errorf("%s: %d Chunks, erwartet %d", size.Name, got, size.ChunkCount)
		}
		if want := float64(size.ChunkCount/2) * w.Level.ChunkWidthUnits; w.HubX != want {
			t.Errorf("%s: Hub bei %v, erwartet mittig bei %v", size.Name, w.HubX, want)
		}
	}
}

func TestVorschauMitChunkAnzahlIstDeterministisch(t *testing.T) {
	a, b := mustPreview(t, countOpts(24)), mustPreview(t, countOpts(24))
	if jsonOf(t, a) != jsonOf(t, b) {
		t.Error("gleiche Anzahl und gleicher Seed, verschiedene Welt")
	}
}

func TestVorschauChunkAnzahlNullIstHeutigesLevel(t *testing.T) {
	biome, _ := level.LoadBiome("forest")
	want, err := level.Generate(biome, "abc")
	if err != nil {
		t.Fatal(err)
	}
	w := mustPreview(t, countOpts(0))
	if !slices.Equal(w.Level.Entities, want.Entities) || len(w.Level.Chunks) != len(want.Chunks) {
		t.Error("Anzahl 0: das Level weicht von level.Generate ab")
	}
	if jsonOf(t, w) != jsonOf(t, mustPreview(t, previewOpts())) {
		t.Error("Anzahl 0: andere Welt als ohne Option")
	}
}

func TestVorschauLehntAnzahlOhneInselgroesseAb(t *testing.T) {
	var valid []string
	for _, s := range sim.IslandSizes() {
		valid = append(valid, strconv.Itoa(s.ChunkCount))
	}
	for _, n := range []int{-16, 2, 18, 22, 50} {
		_, err := BuildPreview(countOpts(n))
		if err == nil || !strings.Contains(err.Error(), "keine Inselgröße") || !strings.Contains(err.Error(), strings.Join(valid, ", ")) {
			t.Errorf("Anzahl %d: Fehler %v, erwartet „keine Inselgröße“ mit den gültigen Werten", n, err)
		}
	}
}

// Kann das Biom die Größe mit dieser Planung nicht erzeugen (kein Platz für die Portale), meldet die Erzeugung den
// Fehler; die Vorschau gerät nicht in Panik.
func TestVorschauMeldetFehlerDerErzeugung(t *testing.T) {
	var plan []level.PlannedChunk
	for slot := 3; slot <= 6; slot++ {
		plan = append(plan, level.PlannedChunk{Slot: slot, Kind: "clearing"}, level.PlannedChunk{Slot: -slot, Kind: "clearing"})
	}
	w, err := BuildPreview(countOpts(16, plan...))
	if err == nil || w != nil || !strings.Contains(err.Error(), "vorschau: Level nicht erzeugbar") {
		t.Fatalf("Welt %v, Fehler %v; erwartet der Fehler der Erzeugung", w != nil, err)
	}
}

func TestVorschauAufeinanderfolgendeAufrufeMitVerschiedenerAnzahl(t *testing.T) {
	first := jsonOf(t, mustPreview(t, countOpts(24)))
	other := mustPreview(t, countOpts(40))
	plain := mustPreview(t, countOpts(0))
	again := jsonOf(t, mustPreview(t, countOpts(24)))
	if first != again {
		t.Error("zweiter Aufruf mit Anzahl 24 liefert einen anderen Snapshot")
	}
	if len(other.Level.Chunks) != 40 || first == jsonOf(t, other) || first == jsonOf(t, plain) {
		t.Errorf("Anzahl 40 hat %d Chunks oder gleicht der Welt mit Anzahl 24 bzw. 0", len(other.Level.Chunks))
	}
}

// sim.CreateWorldWith nimmt noch keine Chunk-Anzahl: mit Kandidat wird sie abgelehnt statt still übergangen.
func TestVorschauChunkAnzahlMitKandidatAbgelehnt(t *testing.T) {
	o := countOpts(24)
	o.HubCandidate = embeddedHub(t)
	if _, err := BuildPreview(o); err == nil || !strings.Contains(err.Error(), "Kandidat") {
		t.Errorf("Fehler %v, erwartet die Ablehnung der Anzahl mit Kandidat", err)
	}
	o.ChunkCount = 0
	mustPreview(t, o)
}

// Die Planung wird gegen die eingestellte Anzahl geprüft, nicht gegen die kürzeste Länge des Bioms (Wald: ±7).
func TestVorschauPrueftPlanungGegenAnzahl(t *testing.T) {
	far := level.PlannedChunk{Slot: 8, Kind: "clearing", Seed: "lichtung"}
	if _, err := BuildPreview(plannedOpts(far)); err == nil || !strings.Contains(err.Error(), "Platz +8") {
		t.Errorf("ohne Anzahl: Fehler %v, erwartet Platz +8 jenseits der kürzesten Länge", err)
	}
	w := mustPreview(t, countOpts(24, far))
	if len(w.Level.Chunks) != 24 {
		t.Errorf("Anzahl 24 mit Planung: %d Chunks", len(w.Level.Chunks))
	}
	_, err := BuildPreview(countOpts(16, level.PlannedChunk{Slot: -7, Kind: "clearing"}))
	if err == nil || !strings.Contains(err.Error(), "Platz -7") || !strings.Contains(err.Error(), "Chunk-Anzahl 16") {
		t.Errorf("Anzahl 16: Fehler %v, erwartet Platz -7 jenseits der Chunk-Anzahl 16", err)
	}
}
