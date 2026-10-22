package room

import (
	"bytes"
	"errors"
	"os"
	"strings"
	"testing"

	"k3c/data"
	"k3c/engine/level"
	"k3c/engine/sim"
)

// WZ9.1 (B-462/AC-04, AC-08, AC-09): Vorschau aus einer Kandidaten-hub.json. Alle Kandidaten sind Kopien im Speicher.

// embeddedHub liefert eine Kopie der eingebetteten data/hub.json.
func embeddedHub(t *testing.T) []byte {
	t.Helper()
	raw, err := data.Files.ReadFile("hub.json")
	if err != nil {
		t.Fatal(err)
	}
	return bytes.Clone(raw)
}

// armoryFrom3: die eingebettete hub.json, Rüstkammer ab Hub-Stufe 3 statt 4.
func armoryFrom3(t *testing.T) []byte {
	t.Helper()
	const from, to = `{ "kind": "armory", "offsetUnits": 40, "hubLevel": 4 }`, `{ "kind": "armory", "offsetUnits": 40, "hubLevel": 3 }`
	raw := embeddedHub(t)
	if bytes.Count(raw, []byte(from)) != 1 {
		t.Fatalf("data/hub.json enthält %s nicht genau einmal; Test an die Datei anpassen", from)
	}
	return bytes.Replace(raw, []byte(from), []byte(to), 1)
}

func TestBuildPreviewKandidatGleichEingebettet(t *testing.T) {
	for _, biome := range []string{"forest", "cave", "crystal"} {
		for level := 0; level <= hubLevels(t); level++ {
			o := previewOpts()
			o.Source.Biome, o.HubLevel = biome, level
			pw := mustPreview(t, o)
			o.HubCandidate = embeddedHub(t)
			cw := mustPreview(t, o)
			if jsonOf(t, cw) != jsonOf(t, pw) {
				t.Errorf("%s, Hub %d: Kandidat = eingebettete hub.json ergibt eine andere Vorschau", biome, level)
			}
			// EconomyOf schreibt net.Preview in den Zustand (stockMax hängt an der Insel): muss ebenfalls gleich sein
			if jsonOf(t, sim.EconomyOf(cw)) != jsonOf(t, sim.EconomyOf(pw)) {
				t.Errorf("%s, Hub %d: Wirtschaft mit Kandidat %+v, ohne %+v", biome, level, sim.EconomyOf(cw), sim.EconomyOf(pw))
			}
		}
	}
}

func TestBuildPreviewRuestkammerAbStufe3NurMitKandidat(t *testing.T) {
	o := previewOpts()
	o.HubLevel = 3
	if n := builtOf(mustPreview(t, o), "armory"); n != 0 {
		t.Errorf("ohne Kandidat: Rüstkammer in Hub-Stufe 3 gebaut (%d), erwartet erst ab 4", n)
	}
	o.HubCandidate = armoryFrom3(t)
	if n := builtOf(mustPreview(t, o), "armory"); n != 1 {
		t.Errorf("mit Kandidat: Rüstkammer in Hub-Stufe 3 %d-mal gebaut, erwartet 1", n)
	}
	o.HubLevel = 2
	if n := builtOf(mustPreview(t, o), "armory"); n != 0 {
		t.Errorf("mit Kandidat: Rüstkammer schon in Hub-Stufe 2 gebaut (%d)", n)
	}
}

func TestBuildPreviewLehntUngueltigenKandidatenAb(t *testing.T) {
	noWorkshop := strings.Replace(string(embeddedHub(t)), `"kind": "workshop"`, `"kind": "nirgendwo"`, 1)
	cases := map[string]string{
		"kein JSON":      `{"levels": [`,
		"leere Stufen":   `{"levels": []}`,
		"unbekannte Art": noWorkshop,
		"Stufe zu hoch":  strings.Replace(string(armoryFrom3(t)), `"hubLevel": 3 }`, `"hubLevel": 99 }`, 1),
	}
	for name, raw := range cases {
		o := previewOpts()
		o.HubCandidate = []byte(raw)
		w, err := BuildPreview(o)
		var ce *CandidateError
		if w != nil || !errors.As(err, &ce) || len(ce.Findings) == 0 {
			t.Errorf("%s: Welt %v, Fehler %v; erwartet CandidateError mit Befunden", name, w != nil, err)
		}
	}
}

func TestBuildPreviewKandidatDeterministischOhneGlobalenZustand(t *testing.T) {
	plain := previewOpts()
	plain.HubLevel = 3
	before := jsonOf(t, mustPreview(t, plain))
	cand := plain
	cand.HubCandidate, cand.Phase, cand.Progress = armoryFrom3(t), "dusk", 0.25
	first := jsonOf(t, mustPreview(t, cand))
	if second := jsonOf(t, mustPreview(t, cand)); first != second {
		t.Error("zweimal derselbe Aufruf mit Kandidat liefert verschiedene Welten")
	}
	if after := jsonOf(t, mustPreview(t, plain)); after != before {
		t.Error("ein Aufruf mit Kandidat ändert die Vorschau ohne Kandidat (globaler Zustand?)")
	}
	if first == jsonOf(t, mustPreview(t, PreviewOptions{Seed: cand.Seed, Source: cand.Source, HubLevel: 3, Phase: "dusk", Progress: 0.25})) {
		t.Error("Kandidat mit Rüstkammer ab 3 und eingebettete hub.json liefern dieselbe Welt")
	}
}

func TestBuildPreviewSiebenStufenKandidat(t *testing.T) {
	raw, err := os.ReadFile("../../testdata/hub/hub7.json")
	if err != nil {
		t.Fatal(err)
	}
	o := previewOpts()
	o.HubCandidate = raw
	if w := mustPreview(t, o); w.HubLevel != 7 {
		t.Errorf("Standard-Stufe mit 7-Stufen-Kandidat: %d, erwartet 7", w.HubLevel)
	}
	for level := 1; level <= 7; level++ {
		o.HubLevel = level
		w := mustPreview(t, o)
		smithy := 0
		if level >= 6 { // Schmiede steht in hub7.json ab Hub-Stufe 6
			smithy = 1
		}
		if w.HubLevel != level || builtOf(w, "smithy") != smithy {
			t.Errorf("Hub %d: HubLevel %d, Schmiede %d (erwartet %d)", level, w.HubLevel, builtOf(w, "smithy"), smithy)
		}
	}
	o.HubLevel = 8
	if _, err := BuildPreview(o); err == nil {
		t.Error("Hub-Stufe 8 mit 7-Stufen-Kandidat ohne Fehler")
	}
	o.HubCandidate, o.HubLevel = nil, hubLevels(t)+1
	if _, err := BuildPreview(o); err == nil {
		t.Errorf("Hub-Stufe %d ohne Kandidat ohne Fehler (über der eingebetteten hub.json)", o.HubLevel)
	}
}

// WZ24.3 (B-479, B-462/AC-09): Sind geplante Chunks und Kandidat zugleich gesetzt, gelten die Linien des Kandidaten, und
// die Planung wird dagegen geprüft: Linie 5 bei 126 macht die Ader bei Hub +136 ungültig, bei 124 ist sie gültig.
func TestVorschauPlanungGegenKandidatenLinien(t *testing.T) {
	plan := []level.PlannedChunk{{Slot: 2, Kind: "cavern", Objects: []level.PlannedObject{{Kind: "stoneVein", At: 36}}}}
	opts := func(last string) PreviewOptions {
		o := previewOpts()
		o.Source.Biome = "cave"
		o.Planned = plan
		o.HubCandidate = []byte(strings.Replace(string(embeddedHub(t)), `"wallUnits": [44, 64, 84, 104, 124]`, `"wallUnits": [44, 64, 84, 104, `+last+`]`, 1))
		return o
	}
	mustPreview(t, opts("124"))
	if _, err := BuildPreview(opts("126")); err == nil || !strings.Contains(err.Error(), "Bauplatz") {
		t.Errorf("Linie 5 bei 126: Fehler %v, erwartet die Ablehnung der Ader bei Hub +136", err)
	}
}
