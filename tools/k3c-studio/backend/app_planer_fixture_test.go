package backend

import (
	"encoding/json"
	"os"
	"reflect"
	"strings"
	"testing"
)

// planerFixture ist der Mock des Gebäude-Planers im Frontend (frontend/src/api/hubFixture.json): Planer-Stand und
// Abweichung der Regeltabelle für die 7-Stufen-Fixture testdata/hub/hub7.json (7 Spalten, Regeltabelle mit 5 Zeilen
// weicht ab).
type planerFixture struct {
	Plan  HubPlanView  `json:"plan"`
	Rules HubRulesView `json:"rules"`
}

func freshPlanerFixture(t *testing.T) planerFixture {
	t.Helper()
	plan := hubPlanFrom(repoRoot, "testdata/hub/hub7.json")
	plan.Raw = strings.ReplaceAll(plan.Raw, "\r\n", "\n") // Arbeitsbaum mit CRLF (Windows) oder LF (CI): gleiche Fixture
	plan.Stand = HubStand{}                               // hängt an den Zeilenenden; der Mock speichert nicht
	return planerFixture{Plan: plan, Rules: hubRulesFrom(repoRoot, []byte(plan.Raw))}
}

// Ändert sich das Matrix-Modell, die Fixture oder die Regeltabelle, muss hubFixture.json neu erzeugt werden, sonst
// laufen die Frontend-Tests gegen ein veraltetes Format.
func TestMockPlanerIstDerPlanerDesBackends(t *testing.T) {
	raw, err := os.ReadFile("../frontend/src/api/hubFixture.json")
	if err != nil {
		t.Fatal(err)
	}
	var fixture, want any
	if err := json.Unmarshal(raw, &fixture); err != nil {
		t.Fatal(err)
	}
	fresh, err := json.Marshal(freshPlanerFixture(t))
	if err != nil {
		t.Fatal(err)
	}
	if err := json.Unmarshal(fresh, &want); err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(fixture, want) {
		t.Error("hubFixture.json weicht vom Backend ab: neu erzeugen (json.Marshal von freshPlanerFixture in die Datei schreiben)")
	}
}
