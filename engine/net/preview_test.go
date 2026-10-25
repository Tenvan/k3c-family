package net

import (
	"encoding/json"
	"os"
	"reflect"
	"slices"
	"testing"

	"k3c/engine/level"
	"k3c/engine/room"
	"k3c/engine/sim"
)

// WZ7.1 (B-460/AC-03, AC-08): Preview liefert Level und Zustand im Format des Spiels.

func previewOpts() room.PreviewOptions {
	return room.PreviewOptions{Seed: "abc", Source: room.PreviewSource{Biome: "forest"}, HubLevel: 3, Phase: "dusk", Progress: 0.25}
}

func previewJSON(t *testing.T, o room.PreviewOptions) string {
	t.Helper()
	res, err := Preview(o)
	if err != nil {
		t.Fatal(err)
	}
	raw, err := json.Marshal(res)
	if err != nil {
		t.Fatal(err)
	}
	return string(raw)
}

func TestPreviewLevelIstDasLevelDesSpiels(t *testing.T) {
	res, err := Preview(previewOpts())
	if err != nil {
		t.Fatal(err)
	}
	biome, err := level.LoadBiome("forest")
	if err != nil {
		t.Fatal(err)
	}
	want, err := level.Generate(biome, "abc")
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(res.Level, want) {
		t.Error("Level weicht von level.Generate(biome, seed) ab")
	}
}

// Der Zustand hat dieselben Felder wie der `snap` eines Raums für dieselbe Welt (stateOf), ohne seed und widthUnits.
func TestPreviewZustandHatDasFormatVonSnap(t *testing.T) {
	res, err := Preview(previewOpts())
	if err != nil {
		t.Fatal(err)
	}
	biome, _ := level.LoadBiome("forest")
	w, err := sim.CreateWorld(biome, "abc", sim.Options{CycleSpeed: 1})
	if err != nil {
		t.Fatal(err)
	}
	var want, got []string
	for k := range stateOf(w, 0, false) {
		want = append(want, k)
	}
	for k := range res.State {
		got = append(got, k)
	}
	slices.Sort(want)
	slices.Sort(got)
	if !slices.Equal(want, got) {
		t.Errorf("Felder des Zustands %v, erwartet wie stateOf %v", got, want)
	}
	for _, key := range []string{"seed", "widthUnits", "devTimescale", "devPaused"} {
		if _, has := res.State[key]; has {
			t.Errorf("Zustand enthält %q", key)
		}
	}
	cycle, _ := res.State["cycle"].(map[string]any)
	if cycle["phase"] != "dusk" || cycle["progress"] != 0.25 {
		t.Errorf("Tageszeit im Zustand: %v", cycle)
	}
	for _, key := range []string{"troops", "enemies"} {
		if list, ok := res.State[key].([]any); !ok || len(list) != 0 {
			t.Errorf("%s im Zustand: %v, erwartet leere Liste", key, res.State[key])
		}
	}
}

func TestPreviewIstDeterministischUndSchreibtNichts(t *testing.T) {
	dir := t.TempDir()
	t.Chdir(dir)
	other := previewOpts()
	other.Seed, other.HubLevel = "xyz", 5
	first := previewJSON(t, previewOpts())
	_ = previewJSON(t, other)
	if first != previewJSON(t, previewOpts()) {
		t.Error("gleiche Optionen liefern nach einem anderen Aufruf ein anderes Ergebnis")
	}
	if entries, err := os.ReadDir(dir); err != nil || len(entries) != 0 {
		t.Errorf("Preview hat Dateien angelegt: %v (%v)", entries, err)
	}
}

func TestPreviewLehntUngueltigeOptionenAb(t *testing.T) {
	o := previewOpts()
	o.Phase = "mittag"
	if _, err := Preview(o); err == nil {
		t.Error("mittag wurde nicht abgelehnt")
	}
	o = previewOpts()
	o.Progress = 1
	if _, err := Preview(o); err == nil {
		t.Error("Fortschritt 1 wurde nicht abgelehnt")
	}
}
