package game

import (
	"slices"
	"strings"
	"testing"
)

// --flow startet im Hauptmenü ohne Szenen-Spiel und ohne Standard-Folge; --at setzt Vorlauf und Zahl der Bilder.
func TestParseArgsFlowAt(t *testing.T) {
	a, err := ParseArgs([]string{"--flow", "--shots", "o", "--at", "1,20,300", "--input", "0@2=a:confirm"}, "s")
	if err != nil {
		t.Fatal(err)
	}
	if a.Scene.Direct() || !a.Flow || !a.Script || len(a.Scene.Script) != 1 {
		t.Errorf("Ablauf: %+v", a.Scene)
	}
	if !slices.Equal(a.At, []int{1, 20, 300}) || a.Scene.Tick != 1 || a.Count != 3 {
		t.Errorf("--at: %v, Vorlauf %d, Bilder %d", a.At, a.Scene.Tick, a.Count)
	}
	sh := &shooter{every: 3, at: a.At, taken: 1}
	if g := sh.gap(); g != 19 {
		t.Errorf("Abstand nach Bild 1: %d", g)
	}
	if sh.taken = 2; sh.gap() != 280 {
		t.Errorf("Abstand nach Bild 2: %d", sh.gap())
	}
	if a, _ = ParseArgs([]string{"--flow", "--shots", "o"}, "s"); a.Scene.Seed != "" || len(a.Scene.Script) != 0 {
		t.Errorf("--flow ohne Folge: %+v", a.Scene)
	}
}

func TestParseArgsMeasure(t *testing.T) {
	a, err := ParseArgs([]string{"--seed", "k3c", "--measure", "20", "--warmup", "5", "--report", "r.json", "--novsync"}, "s")
	if err != nil {
		t.Fatal(err)
	}
	if a.Measure != (Measure{Seconds: 20, Warmup: 5, Report: "r.json", NoVsync: true}) || a.Shots != "" {
		t.Errorf("Messmodus: %+v", a.Measure)
	}
}

func TestParseArgsModesReject(t *testing.T) {
	bad := [][]string{
		{"--flow", "--seed", "x"},
		{"--flow", "--menu", "main"},
		{"--at", "1,2"},
		{"--shots", "o", "--at", "5,5"},
		{"--shots", "o", "--at", "9,3"},
		{"--shots", "o", "--at", "a"},
		{"--measure", "-1"},
		{"--measure", "5", "--shots", "o"},
	}
	for _, argv := range bad {
		if _, err := ParseArgs(argv, "s"); err == nil {
			t.Errorf("angenommen: %s", strings.Join(argv, " "))
		}
	}
}

// --assets nennt je Level die Grafik aller Asset-Quellen: Wald-Natur, Höhlen-Natur, Gebäude und Kulissen.
func TestAssetDirs(t *testing.T) {
	levels, err := Levels()
	if err != nil || !slices.Contains(levels, "forest") || !slices.Contains(levels, "cave") {
		t.Fatalf("Level %v %v", levels, err)
	}
	p, err := Assemble(Packages(nil))
	if err != nil {
		t.Fatal(err)
	}
	dirs, err := AssetDirs(p.Assets)
	if err != nil {
		t.Fatal(err)
	}
	for _, want := range []string{"objekte/tanne", "objekte/wurzel"} {
		if !slices.Contains(dirs, want) {
			t.Errorf("%s fehlt in %v", want, dirs)
		}
	}
	if !slices.IsSorted(dirs) || len(slices.Compact(slices.Clone(dirs))) != len(dirs) {
		t.Errorf("nicht sortiert oder doppelt: %v", dirs)
	}
}

// Die Kennwerte der Messung: Perzentil, 1-%-Low über die langsamsten Bilder.
func TestPercentile(t *testing.T) {
	v := []float64{1, 2, 3, 4, 100}
	if percentile(v, 0.5) != 3 || percentile(v, 0.99) != 100 || percentile(nil, 0.5) != 0 || sum(v) != 110 {
		t.Errorf("Perzentile falsch")
	}
}
