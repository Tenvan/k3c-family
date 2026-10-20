package backend

import (
	"encoding/json"
	"reflect"
	"testing"

	"k3c/engine/net"
	"k3c/engine/room"
)

// WZ7.3 (B-460/AC-04): Das Binding liefert dieselbe Vorschau-Welt wie net.Preview, fehlerfrei und deterministisch.

func previewArgs() WorldPreviewArgs {
	return WorldPreviewArgs{Seed: "abc", Source: WorldSource{Biome: "forest"}, HubLevel: 3, Phase: "dusk", Progress: 0.25}
}

func TestWorldPreviewIstDieVorschauDesServers(t *testing.T) {
	view := (&App{}).WorldPreview(previewArgs())
	if view.Error != "" {
		t.Fatal(view.Error)
	}
	want, err := net.Preview(room.PreviewOptions{Seed: "abc", Source: room.PreviewSource{Biome: "forest"}, HubLevel: 3, Phase: "dusk", Progress: 0.25})
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(view.PreviewResult, want) {
		t.Error("Level oder Zustand weichen von net.Preview ab")
	}
	again := (&App{}).WorldPreview(previewArgs())
	if !reflect.DeepEqual(view, again) {
		t.Error("gleiche Optionen liefern verschiedene Vorschau-Welten")
	}
}

func TestWorldPreviewJSONHatDieFelderDerSeite(t *testing.T) {
	raw, err := json.Marshal((&App{}).WorldPreview(previewArgs()))
	if err != nil {
		t.Fatal(err)
	}
	var m map[string]any
	if err := json.Unmarshal(raw, &m); err != nil {
		t.Fatal(err)
	}
	for _, key := range []string{"level", "state", "error"} {
		if _, has := m[key]; !has {
			t.Errorf("Feld %q fehlt im JSON", key)
		}
	}
	state, _ := m["state"].(map[string]any)
	for _, key := range []string{"sites", "troops", "enemies", "cycle", "hubLevel"} {
		if _, has := state[key]; !has {
			t.Errorf("Feld %q fehlt im Zustand", key)
		}
	}
}

func TestWorldPreviewGebaeudeAuswahlUndFehler(t *testing.T) {
	a := &App{}
	all := a.WorldPreview(previewArgs())
	sites, _ := all.State["sites"].([]any)
	first, _ := sites[0].(map[string]any)
	args := previewArgs()
	args.Off = map[string]bool{room.PreviewSiteKey(first["kind"].(string), first["x"].(float64)): false}
	part := a.WorldPreview(args)
	if got := len(part.State["sites"].([]any)); got != len(sites)-1 {
		t.Errorf("Plätze %d, erwartet %d", got, len(sites)-1)
	}
	for name, mutate := range map[string]func(*WorldPreviewArgs){
		"dawn":          func(x *WorldPreviewArgs) { x.Phase = "dawn" },
		"Fortschritt 1": func(x *WorldPreviewArgs) { x.Progress = 1 },
		"Hub-Stufe 99":  func(x *WorldPreviewArgs) { x.HubLevel = 99 },
		"Biom":          func(x *WorldPreviewArgs) { x.Source.Biome = "nirgendwo" },
		"Insel":         func(x *WorldPreviewArgs) { x.Source = WorldSource{Island: "k3c", Stage: 1} },
	} {
		x := previewArgs()
		mutate(&x)
		if view := a.WorldPreview(x); view.Error == "" || view.State != nil {
			t.Errorf("%s: Error %q, Zustand %v", name, view.Error, view.State != nil)
		}
	}
}
