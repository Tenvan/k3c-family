package gameexe

import (
	"path/filepath"
	"slices"
	"testing"
)

// RU40.3: balance gibt seine Schalter über Options.Args; --window und --speed stehen vor ihnen.
func TestArgvForSchalter(t *testing.T) {
	data := filepath.Join("r", "data")
	base := []string{"x.exe", "sim", "balance", "--input", "in.json", "--data", data, "--stop-stdin"}
	if got := argvFor("x.exe", "in.json", Options{Root: "r", Sub: "balance"}); !slices.Equal(got, base) {
		t.Errorf("headless: %v", got)
	}
	got := argvFor("x.exe", "in.json", Options{Root: "r", Sub: "balance", Window: true, Speed: 4, Args: []string{"--targets", "--seeds", "2"}})
	want := append(slices.Clone(base), "--window", "--speed", "4", "--targets", "--seeds", "2")
	if !slices.Equal(got, want) {
		t.Errorf("mit Fenster: %v", got)
	}
}

// RU40.4: Data ersetzt <Root>/data (Tests: Fixture-Daten mit der EXE der Repo-Wurzel).
func TestArgvForData(t *testing.T) {
	got := argvFor("x.exe", "in.json", Options{Root: "r", Data: "fix", Sub: "world"})
	want := []string{"x.exe", "sim", "world", "--input", "in.json", "--data", "fix", "--stop-stdin"}
	if !slices.Equal(got, want) {
		t.Errorf("Data: %v", got)
	}
}
