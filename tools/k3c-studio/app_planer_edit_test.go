package main

import (
	"slices"
	"strings"
	"testing"
)

// WZ9.3: Prüfung eines geänderten Stands (HubCheck), Arten zum Hinzufügen, Index der Zeilen und Stand beim Laden.

func TestHubCheckPrueftDenKandidaten(t *testing.T) {
	a := &App{root: repoRoot}
	v := a.HubCheck(planerArmoryFrom3(t))
	if v.Error != "" || len(v.Findings) != 0 || v.Stand != (HubStand{}) {
		t.Fatalf("Kandidat: %+v", v)
	}
	if r := rowOf(t, v.Matrix, "armory@40"); r.HubLevel != 3 || r.Source != srcSites || r.Index != 8 {
		t.Errorf("Rüstkammer: %+v", r)
	}
	bad := a.HubCheck(strings.Replace(planerArmoryFrom3(t), armory3, `{ "kind": "armory", "offsetUnits": 40, "hubLevel": 9 }`, 1))
	if len(bad.Findings) == 0 || bad.Findings[0].Rule != "stufe" {
		t.Errorf("Stufe 9 ohne Befund: %+v", bad.Findings)
	}
	if broken := a.HubCheck("{"); broken.Error == "" {
		t.Error("kaputter Kandidat ohne Fehler")
	}
}

func TestHubPlanIndexArtenUndStand(t *testing.T) {
	root := planerCopy(t, hubFile)
	v := hubPlanFrom(root, hubFile)
	if v.Stand.Hub != stamp([]byte(readIn(t, root, hubFile))) || v.Stand.Rules != stamp([]byte(readIn(t, root, rulesFile))) {
		t.Errorf("Stand %+v", v.Stand)
	}
	if r := rowOf(t, v.Matrix, "storage@-26"); r.Source != srcIslandSites || r.Index != 0 {
		t.Errorf("Lager: %+v", r)
	}
	if r := rowOf(t, v.Matrix, "gate@3"); r.Index != 2 || r.Line != 3 {
		t.Errorf("Tor 3: %+v", r)
	}
	for _, kind := range []string{"armory", "farm", "healer", "storage", "workshop"} {
		if !slices.Contains(v.Kinds, kind) {
			t.Errorf("Art %s fehlt in %v", kind, v.Kinds)
		}
	}
	for _, kind := range []string{"castle", "wall", "tower", "gate"} {
		if slices.Contains(v.Kinds, kind) {
			t.Errorf("Art %s darf kein Platz sein: %v", kind, v.Kinds)
		}
	}
}

// Ein nicht lesbarer Kandidat (Kommazahl in einem ganzzahligen Feld) nennt den Fehler und trägt den Kandidaten als Raw,
// damit das Frontend die Antwort zuordnen kann.
func TestHubCheckNichtLesbarerKandidatBehaeltRaw(t *testing.T) {
	cand := strings.Replace(planerArmoryFrom3(t), armory3, `{ "kind": "armory", "offsetUnits": 40, "hubLevel": 1.5 }`, 1)
	v := (&App{root: repoRoot}).HubCheck(cand)
	if v.Error == "" || v.Raw != cand || len(v.Matrix.Rows) != 0 {
		t.Fatalf("Kandidat mit 1.5: Error %q, Raw gesetzt %v, %d Zeilen", v.Error, v.Raw == cand, len(v.Matrix.Rows))
	}
}
