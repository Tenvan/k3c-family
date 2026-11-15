package hubplan

import (
	"context"
	"strings"
	"testing"
)

// WZ9.3: Prüfung eines geänderten Stands (Check), Arten zum Hinzufügen, Index der Zeilen und Stand beim Laden.

func TestHubCheckPrueftDenKandidaten(t *testing.T) {
	a := editor(repoRoot)
	v := a.Check(context.Background(), planerArmoryFrom3(t))
	if v.Error != "" || len(v.Findings) != 0 || v.Stand != (HubStand{}) {
		t.Fatalf("Kandidat: %+v", v)
	}
	if r := rowOf(t, v.Matrix, "armory@40"); r.HubLevel != 3 || r.Source != "sites" || r.Index != 7 {
		t.Errorf("Rüstkammer: %+v", r)
	}
	bad := a.Check(context.Background(), strings.Replace(planerArmoryFrom3(t), armory3, `{ "kind": "armory", "offsetUnits": 40, "hubLevel": 9 }`, 1))
	if len(bad.Findings) == 0 || bad.Findings[0].Rule != "stufe" {
		t.Errorf("Stufe 9 ohne Befund: %+v", bad.Findings)
	}
	if broken := a.Check(context.Background(), "{"); broken.Error == "" {
		t.Error("kaputter Kandidat ohne Fehler")
	}
}

func TestHubPlanIndexArtenUndStand(t *testing.T) {
	root := planerCopy(t, hubFile)
	v := editor(root).planFrom(context.Background(), hubFile)
	if v.Stand.Hub != stamp([]byte(readIn(t, root, hubFile))) || v.Stand.Rules != stamp([]byte(readIn(t, root, rulesFile))) {
		t.Errorf("Stand %+v", v.Stand)
	}
	if r := rowOf(t, v.Matrix, "storage@-26"); r.Source != "islandSites" || r.Index != 0 {
		t.Errorf("Lager: %+v", r)
	}
	if r := rowOf(t, v.Matrix, "zone1"); r.Index != 0 || r.Line != 1 || r.Source != "sitePlacement" {
		t.Errorf("Baugrund Zone 1: %+v", r)
	}
}

// Ein nicht lesbarer Kandidat (Kommazahl in einem ganzzahligen Feld) nennt den Fehler und trägt den Kandidaten als Raw,
// damit das Frontend die Antwort zuordnen kann.
func TestHubCheckNichtLesbarerKandidatBehaeltRaw(t *testing.T) {
	cand := strings.Replace(planerArmoryFrom3(t), armory3, `{ "kind": "armory", "offsetUnits": 40, "hubLevel": 1.5 }`, 1)
	v := editor(repoRoot).Check(context.Background(), cand)
	if v.Error == "" || v.Raw != cand || len(v.Matrix.Rows) != 0 {
		t.Fatalf("Kandidat mit 1.5: Error %q, Raw gesetzt %v, %d Zeilen", v.Error, v.Raw == cand, len(v.Matrix.Rows))
	}
}
