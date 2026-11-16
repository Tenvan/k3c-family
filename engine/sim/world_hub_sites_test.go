package sim

import (
	"encoding/json"
	"reflect"
	"strconv"
	"strings"
	"testing"

	"k3c/engine/level"
)

// B-983: hub.sites und hub.islandSites sind übersteuerbar; Level, Prüfung und Stufe rechnen mit dem Wertesatz der Welt.

func siteX(w *World, kind string) (float64, bool) {
	for _, s := range w.Sites {
		if s.Kind == kind && !s.Ruin {
			return s.X, true
		}
	}
	return 0, false
}

// AC-01: Welt mit geänderten Plätzen: Level und Simulation zeigen denselben Platz, die Basis bleibt.
func TestWeltPlaetzeWirkenInLevelUndSim(t *testing.T) {
	world := overrideWorld(t, "plaetze", map[string]string{
		"hub": `{"sites": {"1": {"offsetUnits": -32}, "5": {"offsetUnits": -28}}}`})
	isl := need(CreateIslandIn(world, "plaetze", world.Islands[0].Depths, 1, CreateOptions{}))(t)
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 1)
	w := isl.Stages[0]
	for kind, want := range map[string]float64{"bowyer": -32, "tavern": -28} {
		if x, ok := siteX(w, kind); !ok || x != w.HubX+want {
			t.Fatalf("%s: Lage %v (gefunden %v), erwartet %v", kind, x, ok, w.HubX+want)
		}
	}
	// Der Generator rechnet mit denselben Plätzen wie die Stufe: gleiches Level mit den Plätzen der Welt.
	want, err := level.GenerateWith(w.Biome, w.Seed, level.Options{Planned: w.planned, ChunkCount: w.chunkCount,
		Nature: w.Level.Nature, Lair: nil, HubSites: world.values.Hub.LevelSites()})
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(want.Sites, w.Level.Sites) || want.HubCenterUnits != w.HubX {
		t.Error("Level der Stufe weicht vom Level mit den Plätzen der Welt ab")
	}
	base := quietWorld(t)
	if x, _ := siteX(base, "bowyer"); x != base.HubX-28 {
		t.Errorf("Basis: Bogner bei %v, erwartet %v", x, base.HubX-28)
	}
}

// AC-02: Fehlerfälle beim Laden: außerhalb, Überlappung, Pflichtplatz, unbekannte Art; Meldung nennt Welt und Datei.
func TestWeltPlaetzeFehlerfaelle(t *testing.T) {
	for override, want := range map[string]string{
		`{"sites": {"7": {"offsetUnits": 55}}}`:        "hub-breite",
		`{"sites": {"7": {"offsetUnits": -50}}}`:       "hub-breite",
		`{"sites": {"7": {"offsetUnits": 25}}}`:        "abstand", // barracks bei 24
		`{"islandSites": {"0": {"offsetUnits": -17}}}`: "abstand", // workshop bei -16
		`{"sites": {"0": {"kind": "bowyer"}}}`:         "workshop",
		`{"sites": {"7": {"kind": "volcano"}}}`:        "unbekannte Art",
		`{"sites": {"7": {"hubLevel": 9}}}`:            "hubLevel 9",
	} {
		w := StandardWorld()
		w.ID, w.Extends = "plaetze", StandardWorldID
		w.Overrides = map[string]json.RawMessage{"hub": json.RawMessage(override)}
		_, err := ResolveWorld(w, standardLookup)
		if err == nil || !strings.Contains(err.Error(), want) || !strings.Contains(err.Error(), "data/hub.json") || !strings.Contains(err.Error(), "plaetze") {
			t.Errorf("%s: Fehler %v, enthält %q, Datei und Welt erwartet", override, err, want)
		}
	}
}

// AC-02: sitePlacement bleibt gesperrt, auch neben erlaubten Plätzen.
func TestWeltSitePlacementBleibtGesperrt(t *testing.T) {
	w := StandardWorld()
	w.ID, w.Extends = "plaetze", StandardWorldID
	w.Overrides = map[string]json.RawMessage{"hub": json.RawMessage(`{"sites": {"7": {"offsetUnits": 32}}, "sitePlacement": {"zone1WallUnits": 45}}`)}
	_, err := ResolveWorld(w, standardLookup)
	if err == nil || !strings.Contains(err.Error(), "sitePlacement: nicht übersteuerbar") {
		t.Errorf("sitePlacement: Ablehnung erwartet, war %v", err)
	}
}

// Die Insel-Plätze gehen mit den Hub-Plätzen an den Generator (Dateireihenfolge: sites, dann islandSites).
func TestLevelSitesMitInselPlaetzen(t *testing.T) {
	h := HubData{Sites: []HubSite{{Kind: "workshop", OffsetUnits: -16}}, IslandSites: []HubSite{{Kind: "storage", OffsetUnits: -24}}}
	want := []level.HubSite{{Kind: "workshop", OffsetUnits: -16}, {Kind: "storage", OffsetUnits: -24}}
	if got := h.LevelSites(); !reflect.DeepEqual(got, want) {
		t.Errorf("LevelSites %v, erwartet %v", got, want)
	}
}

// Review B-983: Der Gebäude-Planer prüft Kandidaten wie die Welt: Hub-Breite als Befund, Planung gegen die Kandidaten-Plätze.
func TestKandidatPlaetzeHubBreiteUndPlanung(t *testing.T) {
	cand := cloneHub(&hub)
	cand.Sites[7].OffsetUnits = 60 // smithy außerhalb der Hub-Breite
	found := func(r HubReport, rule string) bool {
		for _, f := range r.Findings {
			if f.Rule == rule {
				return true
			}
		}
		return false
	}
	if r := ValidateHubData(cand); !found(r, RuleBounds) {
		t.Errorf("Kandidat mit Platz bei 60: Befund %q erwartet, war %v", RuleBounds, r.Findings)
	}
	world := StandardWorld()
	d := world.Islands[0]
	depth := d.Depths[0]
	d.PlannedChunks = map[string][]level.PlannedChunk{strconv.Itoa(depth): {{Slot: 1, Kind: "forest", Buildings: []level.PlannedBuilding{{Kind: "smithy", At: 10}}}}}
	world.Islands = []IslandDef{d}
	if r := ValidateHubDataIn(&hub, world); found(r, RulePlanned) {
		t.Errorf("Basis: Ruine bei 60 ist gültig, war %v", r.Findings)
	}
	if r := ValidateHubDataIn(cand, world); !found(r, RulePlanned) {
		t.Errorf("Kandidat mit Platz auf der Ruinen-Lage: Befund %q erwartet, war %v", RulePlanned, r.Findings)
	}
}
