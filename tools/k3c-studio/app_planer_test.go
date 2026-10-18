package main

import (
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"

	"k3c/engine/sim"
)

// WZ9.2 (B-462/AC-01, AC-08): Matrix-Modell des Gebäude-Planers aus data/hub.json bzw. der 7-Stufen-Fixture. Die
// Tests lesen die Dateien des Checkouts nur; geändert wird höchstens eine Kopie in t.TempDir().

// repoRoot ist die Wurzel des Checkouts, von tools/k3c-studio aus.
const repoRoot = "../.."

func rowOf(t *testing.T, m HubMatrix, key string) HubRow {
	t.Helper()
	i := slices.IndexFunc(m.Rows, func(r HubRow) bool { return r.Key == key })
	if i < 0 {
		t.Fatalf("Zeile %s fehlt", key)
	}
	return m.Rows[i]
}

// Beispiel 1 aus B-462: Zuordnung Hub-Stufe × Bauplatz mit Platz und Tiefe der heutigen data/hub.json.
func TestHubPlanMatrixDerHeutigenDatei(t *testing.T) {
	view := (&App{root: repoRoot}).HubPlan()
	if view.Error != "" || len(view.Findings) != 0 {
		t.Fatalf("Error %q, Befunde %v", view.Error, view.Findings)
	}
	m := view.Matrix
	if m.Levels != 5 || view.Path != hubFile || view.Raw == "" {
		t.Fatalf("Stufen %d, Pfad %q", m.Levels, view.Path)
	}
	want := map[string]int{
		"workshop@-16": 1, "farm@-52": 1, "farm@52": 1, "barracks@24": 2, "tavern@-32": 2, "storage@-26": 2,
		"stairsDown@8": 2, "stairsUp@16": 2, "smithy@28": 3, "healer@-40": 3, "armory@40": 4,
		"line@1": 1, "line@2": 2, "line@3": 3, "line@4": 4, "line@5": 5,
		"gate@1": 2, "gate@2": 2, "gate@3": 3, "gate@4": 4, "gate@5": 5, "merchantBuy@16": 1, "merchantSell@20": 1,
	}
	if len(m.Rows) != len(want) {
		t.Errorf("Zeilen %d, erwartet %d", len(m.Rows), len(want))
	}
	for key, level := range want {
		r := rowOf(t, m, key)
		if r.HubLevel != level || len(r.Cells) != m.Levels {
			t.Errorf("%s: ab Stufe %d, %d Zellen; erwartet ab %d, %d Zellen", key, r.HubLevel, len(r.Cells), level, m.Levels)
		}
		for i, c := range r.Cells {
			if c.Available != (i+1 >= level) {
				t.Errorf("%s: Stufe %d verfügbar %v", key, i+1, c.Available)
			}
		}
	}
}

func TestHubPlanPlatzUndTiefeJeZelle(t *testing.T) {
	m := (&App{root: repoRoot}).HubPlan().Matrix
	if c := rowOf(t, m, "armory@40").Cells; c[2].Available || c[3] != (HubCell{Available: true, OffsetUnits: 40, OnlyDepth: -1}) {
		t.Errorf("Rüstkammer: %+v", c)
	}
	if c := rowOf(t, m, "stairsUp@16").Cells[1]; c.FromDepth != 1 || c.NeedsDeeper {
		t.Errorf("Treppe hoch: %+v", c)
	}
	if c := rowOf(t, m, "stairsDown@8").Cells[4]; !c.NeedsDeeper || c.OffsetUnits != 8 {
		t.Errorf("Treppe runter: %+v", c)
	}
	if r := rowOf(t, m, "storage@-26"); r.Source != srcIslandSites {
		t.Errorf("Lager aus %q", r.Source)
	}
}

func TestHubPlanLinienToreUndHaendler(t *testing.T) {
	m := (&App{root: repoRoot}).HubPlan().Matrix
	line := rowOf(t, m, "line@2")
	if !line.BothSides || line.TowerUnits != 56 || line.JitterUnits != 4 || line.Cells[1].OffsetUnits != 64 {
		t.Errorf("Linie 2: %+v", line)
	}
	if fixed := rowOf(t, m, "line@1"); fixed.JitterUnits != 0 {
		t.Errorf("Linie 1 streut %v", fixed.JitterUnits)
	}
	if gate := rowOf(t, m, "gate@1"); gate.Cells[1].OffsetUnits != 48 || gate.Line != 1 {
		t.Errorf("Tor Linie 1: %+v", gate)
	}
	if buy := rowOf(t, m, "merchantBuy@16"); buy.Cells[0].OnlyDepth != 0 || buy.Source != srcMerchant {
		t.Errorf("Händler: %+v", buy)
	}
}

// AC-08: Die 7-Stufen-Fixture liefert 7 Spalten, ohne feste 5 im Code.
func TestHubPlanSiebenStufen(t *testing.T) {
	view := hubPlanFrom(repoRoot, "testdata/hub/hub7.json")
	if view.Error != "" || len(view.Findings) != 0 {
		t.Fatalf("Error %q, Befunde %v", view.Error, view.Findings)
	}
	if view.Matrix.Levels != 7 {
		t.Fatalf("Spalten %d, erwartet 7", view.Matrix.Levels)
	}
	for _, r := range view.Matrix.Rows {
		if len(r.Cells) != 7 {
			t.Errorf("%s: %d Zellen", r.Key, len(r.Cells))
		}
	}
	if r := rowOf(t, view.Matrix, "armory@40"); r.HubLevel != 7 || !r.Cells[6].Available || r.Cells[5].Available {
		t.Errorf("Rüstkammer: %+v", r)
	}
	if r := rowOf(t, view.Matrix, "line@7"); r.HubLevel != 7 {
		t.Errorf("Linie 7 ab %d", r.HubLevel)
	}
}

func TestHubPlanBefundeUndFehler(t *testing.T) {
	root := t.TempDir()
	if v := hubPlanFrom(root, hubFile); !strings.Contains(v.Error, hubFile) || len(v.Matrix.Rows) != 0 {
		t.Errorf("fehlende Datei: %+v", v)
	}
	write(t, filepath.Join(root, "data", "hub.json"), "{kaputt")
	if v := hubPlanFrom(root, hubFile); !strings.Contains(v.Error, "nicht lesbar als hub.json") {
		t.Errorf("ungültiges JSON: %q", v.Error)
	}
	write(t, filepath.Join(root, "data", "hub.json"), `{"levels": "fünf"}`)
	if v := hubPlanFrom(root, hubFile); !strings.Contains(v.Error, "nicht lesbar als hub.json") || len(v.Matrix.Rows) != 0 {
		t.Errorf("Typfehler: %q", v.Error)
	}
	raw, err := os.ReadFile(filepath.Join(repoRoot, "data", "hub.json"))
	if err != nil {
		t.Fatal(err)
	}
	write(t, filepath.Join(root, "data", "hub.json"), strings.Replace(string(raw), `"kind": "workshop"`, `"kind": "smithy"`, 1))
	v := hubPlanFrom(root, hubFile)
	if v.Error != "" || len(v.Findings) == 0 || len(v.Matrix.Rows) == 0 {
		t.Fatalf("Kandidat ohne Werkstatt: Error %q, Befunde %v", v.Error, v.Findings)
	}
	if !slices.ContainsFunc(v.Findings, func(f HubFindingView) bool { return f.Rule == sim.RuleRequired }) {
		t.Errorf("Befund %s fehlt: %v", sim.RuleRequired, v.Findings)
	}
	if !slices.Contains(v.Notes, sim.NotePlannedChunksUnchecked) {
		t.Errorf("Hinweis fehlt: %v", v.Notes)
	}
}
