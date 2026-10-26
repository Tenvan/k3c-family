package sim

import (
	"bytes"
	"encoding/json"
	"slices"
	"testing"

	"k3c/data"
)

// BS3.1/AC-01 (B-635): Reguläre Goldpreise liegen auf der Münzskala (wirtschaft.md § 8); Ausnahmen stehen ausdrücklich in der Liste.

var coinScale = []int{2, 3, 4, 6, 8, 10, 12, 15, 20, 25, 30}

// scaleExceptions sind Goldpreise außerhalb der Skala mit Grund (Kleinbeträge nach § 8).
var scaleExceptions = map[string]bool{
	"troops.vagrant.recruitCost": true, // Rekrutieren: 1 Münze bleibt (Kleinbetrag)
}

func onScale(t *testing.T, name string, gold int) {
	t.Helper()
	if scaleExceptions[name] {
		return
	}
	if !slices.Contains(coinScale, gold) {
		t.Errorf("%s: %d Gold liegt nicht auf der Münzskala %v", name, gold, coinScale)
	}
}

func TestGoldpreiseAufDerMuenzskala(t *testing.T) {
	for kind, b := range buildings {
		checkBuildingScale(t, kind, b)
	}
	if w := buildings["wall"]; len(w.Levels) == 0 || w.Levels[0].Cost.Gold != 2 || w.Cost.Gold != 2 {
		t.Errorf("Mauer Stufe 1 muss 2 Gold kosten: %+v", w.Cost)
	}
	for _, lv := range hub.Levels {
		for _, st := range lv.Steps {
			onScale(t, "hub.levels.steps", st.Cost.Gold)
		}
	}
	for kind, tr := range troops {
		checkTroopScale(t, kind, tr)
	}
}

func checkBuildingScale(t *testing.T, kind string, b BuildingData) {
	t.Helper()
	if b.Cost.Gold > 0 {
		onScale(t, "buildings."+kind, b.Cost.Gold)
	}
	for _, c := range b.CostByDepth {
		if c.Gold > 0 { // Tiefen ohne Treppe haben keinen Eintrag
			onScale(t, "buildings."+kind+".costByDepth", c.Gold)
		}
	}
	for _, l := range b.Levels {
		onScale(t, "buildings."+kind+".levels", l.Cost.Gold)
	}
	for _, a := range b.Armor {
		onScale(t, "buildings."+kind+".armor", a.Cost.Gold)
	}
}

func checkTroopScale(t *testing.T, kind string, tr TroopData) {
	t.Helper()
	if tr.Cost.Gold > 0 {
		onScale(t, "troops."+kind+".cost", tr.Cost.Gold)
	}
	if tr.RecruitCost.Gold > 0 {
		onScale(t, "troops."+kind+".recruitCost", tr.RecruitCost.Gold)
	}
	for name, p := range tr.Professions {
		onScale(t, "troops."+kind+".professions."+name, p.Cost.Gold)
	}
}

// BS3.2/AC-03 (B-638): Schiffskosten je Abschnitt stehen in data/ship.json, Gold auf der Skala, Schema stimmt.
func TestSchiffskostenInDenDaten(t *testing.T) {
	raw, err := data.Files.ReadFile("ship.json")
	if err != nil {
		t.Fatal(err)
	}
	var s2 struct {
		Version int
		Notes   string
		Stages  []struct {
			ID, Name string
			Cost     Cost
		}
	}
	d2 := json.NewDecoder(bytes.NewReader(raw))
	d2.DisallowUnknownFields()
	if err := d2.Decode(&s2); err != nil {
		t.Fatalf("Schema: %v", err)
	}
	ids := []string{"keel", "hull", "sail"}
	if len(s2.Stages) != len(ids) {
		t.Fatalf("%d Abschnitte, erwartet %d", len(s2.Stages), len(ids))
	}
	for i, st := range s2.Stages {
		if st.ID != ids[i] || st.Name == "" {
			t.Errorf("Abschnitt %d: %+v", i, st)
		}
		onScale(t, "ship."+st.ID, st.Cost.Gold)
		if c := st.Cost; c.Wood+c.Stone+c.Copper+c.Iron+c.Crystal == 0 {
			t.Errorf("Abschnitt %s ohne Material", st.ID)
		}
	}
}
