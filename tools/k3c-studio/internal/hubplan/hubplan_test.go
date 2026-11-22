package hubplan

import (
	"context"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
)

// WZ9.2 (B-462/AC-01, AC-08): Matrix-Modell des Gebäude-Planers aus data/hub.json bzw. der 7-Stufen-Fixture. Die
// Tests lesen die Dateien des Checkouts nur; geändert wird höchstens eine Kopie in t.TempDir().

// repoRoot ist die Wurzel des Checkouts, von tools/k3c-studio/internal/hubplan aus.
const repoRoot = "../../../.."

func rowOf(t *testing.T, m HubMatrix, key string) HubRow {
	t.Helper()
	i := slices.IndexFunc(m.Rows, func(r HubRow) bool { return r.Key == key })
	if i < 0 {
		t.Fatalf("Zeile %s fehlt", key)
	}
	return m.Rows[i]
}

// AC-08: Die 7-Stufen-Fixture liefert 7 Spalten, ohne feste 5 im Code.
func TestHubPlanSiebenStufen(t *testing.T) {
	view := editor(repoRoot).planFrom(context.Background(), "testdata/hub/hub7.json")
	if view.Error != "" || len(view.Findings) != 0 {
		t.Fatalf("Error %q, Befunde %v", view.Error, view.Findings)
	}
	if view.Matrix.Levels != 7 {
		t.Fatalf("Spalten %d, erwartet 7", view.Matrix.Levels)
	}
}

func TestHubPlanBefundeUndFehler(t *testing.T) {
	root := t.TempDir()
	if v := editor(root).planFrom(context.Background(), hubFile); !strings.Contains(v.Error, hubFile) || len(v.Matrix.Rows) != 0 {
		t.Errorf("fehlende Datei: %+v", v)
	}
	write(t, filepath.Join(root, "data", "hub.json"), "{kaputt")
	if v := editor(root).planFrom(context.Background(), hubFile); !strings.Contains(v.Error, "nicht lesbar als hub.json") {
		t.Errorf("ungültiges JSON: %q", v.Error)
	}
	write(t, filepath.Join(root, "data", "hub.json"), `{"levels": "fünf"}`)
	if v := editor(root).planFrom(context.Background(), hubFile); !strings.Contains(v.Error, "nicht lesbar als hub.json") || len(v.Matrix.Rows) != 0 {
		t.Errorf("Typfehler: %q", v.Error)
	}
	raw, err := os.ReadFile(filepath.Join(repoRoot, "data", "hub.json"))
	if err != nil {
		t.Fatal(err)
	}
	write(t, filepath.Join(root, "data", "hub.json"), strings.Replace(string(raw), `"kind": "workshop"`, `"kind": "smithy"`, 1))
	v := editor(root).planFrom(context.Background(), hubFile)
	if v.Error != "" || len(v.Findings) == 0 || len(v.Matrix.Rows) == 0 {
		t.Fatalf("Kandidat ohne Werkstatt: Error %q, Befunde %v", v.Error, v.Findings)
	}
	if !slices.ContainsFunc(v.Findings, func(f HubFindingView) bool { return f.Rule == "pflichtplatz" }) {
		t.Errorf("Befund pflichtplatz fehlt: %v", v.Findings)
	}
	if len(v.Notes) != 0 {
		t.Errorf("Hinweise %v, erwartet keine (geplante Chunks werden mitgeprüft, B-479)", v.Notes)
	}
}

func write(t *testing.T, path, text string) {
	t.Helper()
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(path, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
}
