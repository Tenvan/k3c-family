package simcli

import (
	"encoding/json"
	"os"
	"reflect"
	"slices"
	"strings"
	"testing"

	"k3c/engine/sim"
)

// B-923: `k3c-sim hub` rechnet Matrix, Befunde und erlaubte Arten des Gebäude-Planers (Port der Tests aus dem früheren
// Asset-Studio, app_planer_test.go). Gelesen werden nur data/hub.json und testdata/hub/hub7.json.

func hubPlanOf(t *testing.T, raw string) hubReport {
	t.Helper()
	in, err := json.Marshal(hubInput{Op: "plan", Raw: raw})
	if err != nil {
		t.Fatal(err)
	}
	code, out, errOut := call(t, string(in), "hub")
	if code != ExitOK {
		t.Fatalf("Exit %d: %s", code, errOut)
	}
	var rep hubReport
	worldReport(t, out, &rep)
	return rep
}

func readHub(t *testing.T, rel string) string {
	t.Helper()
	raw, err := os.ReadFile("../../" + rel)
	if err != nil {
		t.Fatal(err)
	}
	return string(raw)
}

func hubRowOf(t *testing.T, m hubMatrix, key string) hubRow {
	t.Helper()
	i := slices.IndexFunc(m.Rows, func(r hubRow) bool { return r.Key == key })
	if i < 0 {
		t.Fatalf("Zeile %s fehlt", key)
	}
	return m.Rows[i]
}

// Beispiel 1 aus B-462: Zuordnung Hub-Stufe × Bauplatz mit Platz und Tiefe der heutigen data/hub.json.
func TestHubPlanMatrixDerHeutigenDatei(t *testing.T) {
	rep := hubPlanOf(t, readHub(t, "data/hub.json"))
	if rep.ParseError != "" || len(rep.Findings) != 0 {
		t.Fatalf("Fehler %q, Befunde %v", rep.ParseError, rep.Findings)
	}
	m := rep.Matrix
	want := map[string]int{
		"workshop@-16": 1, "barracks@24": 2, "tavern@-32": 2, "storage@-24": 2, "bowyer@-28": 1,
		"stairsDown@8": 2, "stairsUp@16": 2, "smithy@28": 3, "healer@-40": 3, "armory@40": 4,
		"zone1": 1, "merchantBuy@16": 1, "merchantSell@20": 1,
	}
	if m.Levels != 5 || len(m.Rows) != len(want) {
		t.Fatalf("Stufen %d, Zeilen %d, erwartet 5 und %d", m.Levels, len(m.Rows), len(want))
	}
	for key, level := range want {
		r := hubRowOf(t, m, key)
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

func TestHubPlanPlatzTiefeBaugrundUndHaendler(t *testing.T) {
	m := hubPlanOf(t, readHub(t, "data/hub.json")).Matrix
	if c := hubRowOf(t, m, "armory@40").Cells; c[2].Available || c[3] != (hubCell{Available: true, OffsetUnits: 40, OnlyDepth: -1}) {
		t.Errorf("Rüstkammer: %+v", c)
	}
	if c := hubRowOf(t, m, "stairsUp@16").Cells[1]; c.FromDepth != 1 || c.NeedsDeeper {
		t.Errorf("Treppe hoch: %+v", c)
	}
	if c := hubRowOf(t, m, "stairsDown@8").Cells[4]; !c.NeedsDeeper || c.OffsetUnits != 8 {
		t.Errorf("Treppe runter: %+v", c)
	}
	if r := hubRowOf(t, m, "storage@-24"); r.Source != hubSrcIslandSites || r.Index != 0 {
		t.Errorf("Lager: %+v", r)
	}
}

func TestHubPlanBaugrundUndHaendler(t *testing.T) {
	m := hubPlanOf(t, readHub(t, "data/hub.json")).Matrix
	zone := hubRowOf(t, m, "zone1")
	if !zone.BothSides || zone.Source != hubSrcPlacement || zone.TowerUnits != 36 || zone.Line != 1 || zone.Cells[0].OffsetUnits != 44 {
		t.Errorf("Baugrund Zone 1: %+v", zone)
	}
	if buy := hubRowOf(t, m, "merchantBuy@16"); buy.Cells[0].OnlyDepth != 0 || buy.Source != hubSrcMerchant {
		t.Errorf("Händler: %+v", buy)
	}
}

// AC-08: Die 7-Stufen-Fixture liefert 7 Spalten, ohne feste 5 im Code.
func TestHubPlanSiebenStufen(t *testing.T) {
	rep := hubPlanOf(t, readHub(t, "testdata/hub/hub7.json"))
	if rep.ParseError != "" || len(rep.Findings) != 0 || rep.Matrix.Levels != 7 {
		t.Fatalf("Fehler %q, Befunde %v, Stufen %d", rep.ParseError, rep.Findings, rep.Matrix.Levels)
	}
	for _, r := range rep.Matrix.Rows {
		if len(r.Cells) != 7 {
			t.Errorf("%s: %d Zellen", r.Key, len(r.Cells))
		}
	}
	if r := hubRowOf(t, rep.Matrix, "armory@40"); r.HubLevel != 7 || !r.Cells[6].Available || r.Cells[5].Available {
		t.Errorf("Rüstkammer: %+v", r)
	}
	if r := hubRowOf(t, rep.Matrix, "zone1"); r.HubLevel != 1 || r.Cells[0].OffsetUnits != 60 || r.TowerUnits != 52 {
		t.Errorf("Baugrund der Fixture: %+v", r)
	}
}

func TestHubPlanBefundeFehlerUndArten(t *testing.T) {
	for _, bad := range []string{"{kaputt", `{"levels": "fünf"}`, `{"levels": [], "sites": [{"kind": "armory", "hubLevel": 1.5}]}`} {
		if rep := hubPlanOf(t, bad); rep.ParseError == "" || len(rep.Matrix.Rows) != 0 {
			t.Errorf("%q: Fehler %q, %d Zeilen", bad, rep.ParseError, len(rep.Matrix.Rows))
		}
	}
	raw := readHub(t, "data/hub.json")
	rep := hubPlanOf(t, strings.Replace(raw, `"kind": "workshop"`, `"kind": "smithy"`, 1))
	if rep.ParseError != "" || len(rep.Matrix.Rows) == 0 || !slices.ContainsFunc(rep.Findings, func(f hubFinding) bool { return f.Rule == sim.RuleRequired }) {
		t.Fatalf("Kandidat ohne Werkstatt: %q, %v", rep.ParseError, rep.Findings)
	}
	if len(rep.Notes) != 0 {
		t.Errorf("Hinweise %v, erwartet keine (geplante Chunks werden mitgeprüft, B-479)", rep.Notes)
	}
	for _, kind := range []string{"armory", "nursery", "healer", "storage", "workshop"} {
		if !slices.Contains(rep.Kinds, kind) {
			t.Errorf("Art %s fehlt in %v", kind, rep.Kinds)
		}
	}
	for _, kind := range []string{"castle", "wall", "tower"} {
		if slices.Contains(rep.Kinds, kind) {
			t.Errorf("Art %s darf kein Platz sein: %v", kind, rep.Kinds)
		}
	}
}

// Der Mock der Workbench (frontend/src/api/hubFixture.json) ist die Antwort für die 7-Stufen-Fixture: Ändert sich das
// Matrix-Modell, muss die Datei neu erzeugt werden, sonst laufen die Frontend-Tests gegen ein veraltetes Format.
func TestHubFixtureDesMocksIstDieAntwortDerEngine(t *testing.T) {
	var fixture struct {
		Plan struct {
			Raw      string          `json:"raw"`
			Matrix   json.RawMessage `json:"matrix"`
			Findings json.RawMessage `json:"findings"`
			Notes    json.RawMessage `json:"notes"`
			Kinds    json.RawMessage `json:"kinds"`
		} `json:"plan"`
	}
	if err := json.Unmarshal([]byte(readHub(t, "tools/k3c-workbench/frontend/src/api/hubFixture.json")), &fixture); err != nil {
		t.Fatal(err)
	}
	raw := strings.ReplaceAll(readHub(t, "testdata/hub/hub7.json"), "\r\n", "\n")
	if fixture.Plan.Raw != raw {
		t.Error("hubFixture.json: raw weicht von testdata/hub/hub7.json ab")
	}
	rep := hubPlanOf(t, raw)
	for name, pair := range map[string][2]any{
		"matrix": {fixture.Plan.Matrix, rep.Matrix}, "findings": {fixture.Plan.Findings, rep.Findings},
		"notes": {fixture.Plan.Notes, rep.Notes}, "kinds": {fixture.Plan.Kinds, rep.Kinds},
	} {
		want, _ := json.Marshal(pair[1])
		var a, b any
		_ = json.Unmarshal(pair[0].(json.RawMessage), &a)
		_ = json.Unmarshal(want, &b)
		if !reflect.DeepEqual(a, b) {
			t.Errorf("hubFixture.json: %s weicht von k3c-sim hub ab (Datei neu erzeugen)", name)
		}
	}
}

func TestHubUnbekannteOp(t *testing.T) {
	if code, _, _ := call(t, `{"op":"fly"}`, "hub"); code != ExitUsage {
		t.Errorf("unbekannte op: Exit %d", code)
	}
}
