package balance

import (
	"os"
	"slices"
	"strings"
	"testing"
)

// Seed-Modus (B-634, BS1.1): k3c-dev steht vorn, dev rechnet ohne Angabe nur ihn, ausdrückliche Seeds gehen vor.

func TestDevSeedStehtVorn(t *testing.T) {
	ts, err := DefaultTargets()
	if err != nil {
		t.Fatal(err)
	}
	if ts.Seeds[0] != DevSeed {
		t.Fatalf("seeds[0] = %q, erwartet %q", ts.Seeds[0], DevSeed)
	}
}

func TestSeedModus(t *testing.T) {
	ts := &Targets{Seeds: []string{DevSeed, "1", "2", "3"}, SeedMode: SeedModeDev}
	if got := ts.seedList(0); !slices.Equal(got, []string{DevSeed}) {
		t.Errorf("dev ohne Angabe: %v", got)
	}
	if got := ts.seedList(3); len(got) != 3 {
		t.Errorf("dev mit --seeds 3: %v", got)
	}
	if got := ts.seedModeOf(ts.seedList(0)); got != SeedModeDev {
		t.Errorf("Modus %q, erwartet dev", got)
	}
	ts.SeedMode = SeedModeAll
	if got := ts.seedList(0); len(got) != 4 {
		t.Errorf("all ohne Angabe: %v", got)
	}
	if got := ts.seedModeOf(ts.seedList(2)); got != "auswahl" {
		t.Errorf("Modus %q, erwartet auswahl", got)
	}
	ts.SeedMode = ""
	if got := ts.seedList(0); len(got) != 4 {
		t.Errorf("ohne Modus wie all: %v", got)
	}
}

func TestWithSeeds(t *testing.T) {
	ts := &Targets{Seeds: []string{DevSeed, "1"}, SeedMode: SeedModeDev}
	got, err := ts.WithSeeds([]string{"7", DevSeed})
	if err != nil {
		t.Fatal(err)
	}
	if got.seedModeOf(got.seedList(0)) != seedModePick {
		t.Errorf("Modus %q, erwartet auswahl", got.seedModeOf(got.seedList(0)))
	}
	if !slices.Equal(got.seedList(0), []string{"7", DevSeed}) || ts.Seeds[0] != DevSeed {
		t.Errorf("Seeds %v, Original %v", got.seedList(0), ts.Seeds)
	}
	for _, bad := range [][]string{nil, {"7", "7"}, {""}} {
		if _, err := ts.WithSeeds(bad); err == nil {
			t.Errorf("%q angenommen", bad)
		}
	}
}

func TestSeedModusUnbekannt(t *testing.T) {
	raw, err := os.ReadFile("../../../../data/balance-targets.json")
	if err != nil {
		t.Fatal(err)
	}
	bad := strings.Replace(string(raw), `"seedMode": "all"`, `"seedMode": "raten"`, 1)
	if bad == string(raw) {
		t.Fatal("seedMode nicht in der Datei")
	}
	if _, err := LoadTargets([]byte(bad)); err == nil {
		t.Error("seedMode raten angenommen")
	}
}

func TestBerichtNenntSeedModus(t *testing.T) {
	s := Summary{Seeds: 1, SeedMode: SeedModeDev, SeedNames: []string{DevSeed}}
	if got := s.seedText(); got != "1 Seed, Seed-Modus dev (k3c-dev)" {
		t.Errorf("Kopf %q", got)
	}
	if !strings.Contains(s.MarkdownWith(nil, nil), "Seed-Modus dev (k3c-dev)") {
		t.Error("Markdown ohne Seed-Modus")
	}
	if got := (Summary{Seeds: 100}).seedText(); got != "100 Seeds" {
		t.Errorf("alter Bericht: %q", got)
	}
}
