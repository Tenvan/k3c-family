package sim

import (
	"encoding/json"
	"testing"

	"k3c/engine/level"
)

// WZ24.2 (B-489/AC-02): Ein Stand lädt seine Stufen mit der Planung und Chunk-Anzahl, mit denen sie gebaut wurden, auch
// wenn die Daten der Welt sich danach ändern.
func TestStandLaedtMitGespeicherterPlanungUndAnzahl(t *testing.T) {
	plan := []level.PlannedChunk{{Slot: 9, Kind: "forest"}}
	opts := CreateOptions{Planned: map[int][]level.PlannedChunk{0: plan}, ChunkCounts: map[int]int{0: 24}}
	isl, err := CreateIsland("stand", []int{0, 1}, 1, opts)
	if err != nil {
		t.Fatal(err)
	}
	raw, err := json.Marshal(isl.ToSave("2026-10-22T00:00:00Z"))
	if err != nil {
		t.Fatal(err)
	}
	save, err := ParseIslandSave(raw)
	if err != nil {
		t.Fatal(err)
	}
	// Die Standard-Welt hat weder Planung noch Anzahl: wer aus den aktuellen Daten baut, bekäme eine andere Stufe 0.
	if cur := StandardWorld().Islands[0].CreateOptions(); cur.ChunkCounts[0] != 0 || len(cur.Planned[0]) != 1 || cur.Planned[0][0].Kind != level.HarborKind {
		t.Fatalf("Voraussetzung verletzt: die Daten der Standard-Welt tragen Planung oder Anzahl: %+v", cur)
	}
	loaded, err := FromIslandSave(save, 1)
	if err != nil {
		t.Fatal(err)
	}
	for i := range isl.Stages {
		if a, b := isl.Stages[i].WidthUnits, loaded.Stages[i].WidthUnits; a != b {
			t.Errorf("Stufe %d: Breite %v nach dem Laden, gebaut mit %v", i, b, a)
		}
	}
	if got := loaded.Stages[0].chunkCount; got != 24 {
		t.Errorf("Chunk-Anzahl nach dem Laden %d, erwartet 24", got)
	}
	if len(loaded.Stages[0].planned) != 1 || loaded.Stages[0].planned[0].Slot != 9 {
		t.Errorf("Planung nach dem Laden %+v", loaded.Stages[0].planned)
	}
	if len(loaded.Stages[1].planned) != 0 || loaded.Stages[1].chunkCount != 0 {
		t.Errorf("Stufe 1 hatte keine Planung: %+v %d", loaded.Stages[1].planned, loaded.Stages[1].chunkCount)
	}
}

// Ein Stand ohne die Felder lädt in der Länge des Bioms ohne Planung, wie ältere Stände gebaut wurden.
func TestStandOhneFelderLaedtReinenZufall(t *testing.T) {
	isl, err := CreateIsland("alt", []int{1}, 1, CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	raw, _ := json.Marshal(isl.ToSave("2026-10-22T00:00:00Z"))
	var generic map[string]any
	if err := json.Unmarshal(raw, &generic); err != nil {
		t.Fatal(err)
	}
	for _, st := range generic["stages"].([]any) {
		if _, ok := st.(map[string]any)["planned"]; ok {
			t.Error("planned ohne Planung geschrieben (omitempty)")
		}
	}
	save, err := ParseIslandSave(raw)
	if err != nil {
		t.Fatal(err)
	}
	loaded, err := FromIslandSave(save, 1)
	if err != nil || loaded.Stages[0].WidthUnits != isl.Stages[0].WidthUnits {
		t.Errorf("Laden: %v, Breite %v statt %v", err, loaded.Stages[0].WidthUnits, isl.Stages[0].WidthUnits)
	}
}
