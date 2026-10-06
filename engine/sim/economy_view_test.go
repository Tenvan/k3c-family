package sim

import (
	"encoding/json"
	"testing"
)

// W9.1 (B-323/AC-01, Sprint W9 AC-01): Lager-Maximum, Hub-Ausbau mit Kosten und Wartegrund „Gefahr“ im JSON.

// economyJSON ist EconomyOf(w) als JSON-Objekt, wie W5.1 es in den Zustand übernimmt.
func economyJSON(t *testing.T, w *World) map[string]any {
	t.Helper()
	raw, err := json.Marshal(EconomyOf(w))
	if err != nil {
		t.Fatal(err)
	}
	var m map[string]any
	if err := json.Unmarshal(raw, &m); err != nil {
		t.Fatal(err)
	}
	return m
}

func TestEconomyInselMitLagerAusbauUndNacht(t *testing.T) {
	isl := mustIsland(t, "w9-wirtschaft", []int{0, 1, 2})
	w := isl.Stages[0]
	storageSite(w).State = "built"
	for upgradePayable(w, w.hubSite) {
		payUpgrade(w, w.hubSite)
	}
	w.Cycle.Phase = "night"
	got := economyJSON(t, w)
	if got["stockMax"] != 1200.0 || got["hubLevel"] != 1.0 || got["danger"] != true {
		t.Errorf("stockMax/hubLevel/danger: %v", got)
	}
	up, _ := got["hubUpgrade"].(map[string]any)
	mat, _ := up["material"].(map[string]any)
	if up["gold"] != 50.0 || up["paid"] != 50.0 || up["state"] != "waitingMaterial" || mat["stone"] != 100.0 {
		t.Errorf("hubUpgrade: %v", up)
	}
}

func TestEconomyOhneInselUndStufe5(t *testing.T) {
	w := quietWorld(t)
	w.Cycle.Phase = "day"
	w.Enemies = nil
	got := economyJSON(t, w)
	if _, ok := got["stockMax"]; ok {
		t.Errorf("ohne Insel kein Maximum: %v", got)
	}
	if _, ok := got["danger"]; ok {
		t.Errorf("am Tag ohne Gegner keine Gefahr: %v", got)
	}
	if up, _ := got["hubUpgrade"].(map[string]any); up["gold"] != 50.0 || up["paid"] != 0.0 || up["state"] != nil {
		t.Errorf("Stufe 1 ohne Zahlung: %v", up)
	}
	w.HubLevel = len(hub.Levels)
	if got := economyJSON(t, w); got["hubLevel"] != 5.0 || got["hubUpgrade"] != nil {
		t.Errorf("Stufe 5 ohne Kosten: %v", got)
	}
}

// W10.1 (B-332, Sprint W10 AC-02): Kämpfer-Zahl und Truppen-Limit aus fighters und troopLimit, noch nicht im JSON.
func TestEconomyKaempferUndLimit(t *testing.T) {
	w := quietWorld(t)
	l := buildings["barracks"].TroopLimit
	if e := EconomyOf(w); e.Fighters != 0 || e.TroopLimit != l.Base {
		t.Errorf("ohne Kämpfer und Kaserne: %d/%d, erwartet 0/%d", e.Fighters, e.TroopLimit, l.Base)
	}
	makeArcher(w, spawnVagrant(w, w.HubX, w.HubX))
	makeArcher(w, spawnVagrant(w, w.HubX, w.HubX))
	spawnVagrant(w, w.HubX+30, w.HubX+30) // zählt nicht
	barracks := buildSite(t, w, "barracks")
	if e := EconomyOf(w); e.Fighters != 2 || e.TroopLimit != l.Base+l.PerBuilding {
		t.Errorf("2 Bogenschützen, Kaserne: %d/%d, erwartet 2/%d", e.Fighters, e.TroopLimit, l.Base+l.PerBuilding)
	}
	destroySite(w, barracks)
	if e := EconomyOf(w); e.TroopLimit != l.Base {
		t.Errorf("zerstörte Kaserne zählt nicht: %d", e.TroopLimit)
	}
	if j := economyJSON(t, w); j["fighters"] != 2.0 || j["troopLimit"] != float64(l.Base) {
		t.Errorf("JSON fighters/troopLimit: %v %v (W10.2)", j["fighters"], j["troopLimit"])
	}
}
