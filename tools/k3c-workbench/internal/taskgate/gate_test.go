package taskgate

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestGateZustaendeUndUebernehmen(t *testing.T) {
	path := filepath.Join(t.TempDir(), FileName)
	if err := os.WriteFile(path, []byte(`{"allowed":["test"]}`), 0o644); err != nil {
		t.Fatal(err)
	}
	g, err := Load(path)
	if err != nil {
		t.Fatal(err)
	}
	if g.State("test") != Open || g.State("lint") != Closed {
		t.Fatalf("Dateistand: %s %s", g.State("test"), g.State("lint"))
	}
	if g.Toggle("lint") != OpenTemp || g.Toggle("test") != ClosedTemp || g.Pending() != 2 {
		t.Fatalf("Schalter: %s %s %d", g.State("lint"), g.State("test"), g.Pending())
	}
	if g.Toggle("test") != Open || g.Pending() != 1 {
		t.Fatalf("zurück auf Datei: %s %d", g.State("test"), g.Pending())
	}
	if err := g.Save(); err != nil {
		t.Fatal(err)
	}
	data, _ := os.ReadFile(path)
	if !strings.Contains(string(data), `"lint",`) || g.State("lint") != Open || g.Pending() != 0 {
		t.Errorf("übernommen: %s, %s", data, g.State("lint"))
	}
}

func TestGateOhneDatei(t *testing.T) {
	g, err := Load(filepath.Join(t.TempDir(), FileName))
	if err != nil || g.State("check").Allowed() || len(g.AllowedNames([]string{"check"})) != 0 {
		t.Errorf("ohne Datei: %v %s", err, g.State("check"))
	}
}
