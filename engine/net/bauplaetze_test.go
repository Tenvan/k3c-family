package net

import (
	"testing"

	"k3c/engine/sim"
)

// W11.2 (B-208/AC-01, AC-02, B-408): Jeder Bauplatz im Zustand trägt seine Platz-Stufe `hubLevel`; unbezahlte Plätze
// über Hub-Stufe + 1 fehlen, bezahlte, wartende und gebaute nie.

// siteLevels sind die Plätze des Zustands s als id → hubLevel (JSON-Zahl); fehlt hubLevel, ist der Wert -1.
func siteLevels(t *testing.T, s map[string]any) map[float64]float64 {
	t.Helper()
	out := map[float64]float64{}
	for _, e := range s["sites"].([]any) {
		m := e.(map[string]any)
		lv, ok := m["hubLevel"].(float64)
		if !ok {
			lv = -1
		}
		out[m["id"].(float64)] = lv
	}
	return out
}

func TestBauplaetzeNurBisNaechsteHubStufe(t *testing.T) {
	_, w := wirtschaftsInsel(t)
	if w.HubLevel != 1 {
		t.Fatalf("Hub-Stufe %d, erwartet 1", w.HubLevel)
	}
	var armory *sim.Site
	for _, site := range w.Sites {
		if site.Kind == "armory" {
			armory = site
		}
	}
	if armory == nil {
		t.Fatal("keine Rüstkammer")
	}
	armory.State = "built" // gebaut über Hub-Stufe + 1: bleibt sichtbar
	got := siteLevels(t, asJSON(t, stateOf(w, 0, false)).(map[string]any))
	hidden := 0
	for _, site := range w.Sites {
		want := sim.SiteHubLevel(w, site)
		lv, shown := got[float64(site.ID)]
		visible := site.State != "unpaid" || site.PaidGold > 0 || want <= w.HubLevel+1
		switch {
		case shown != visible:
			t.Errorf("%s (Platz-Stufe %d, %s): im Zustand %v, erwartet %v", site.Kind, want, site.State, shown, visible)
		case shown && lv != float64(want):
			t.Errorf("%s: hubLevel %v, erwartet %d", site.Kind, lv, want)
		}
		if !visible {
			hidden++
		}
	}
	if hidden == 0 || got[float64(armory.ID)] != 4 {
		t.Errorf("verborgen %d, Rüstkammer %v", hidden, got[float64(armory.ID)])
	}
}

// Hub-Stufe 1 → 2 über die Sim: Das nächste Delta trägt die Plätze mit hubLevel 3 in sites.set.
func TestBauplaetzeHubAusbauImDelta(t *testing.T) {
	isl, w := wirtschaftsInsel(t)
	prev := stateOf(w, 0, false)
	*isl.Stock = sim.Stock{Wood: 1000, Stone: 1000, Copper: 1000, Iron: 1000}
	cmds := make([]sim.PlayerCommand, 4)
	for i := 0; w.HubLevel < 2 && i < 30*600; i++ {
		sim.StepIsland(isl, cmds, 1.0/30)
		if w.HubLevel < 2 {
			prev = stateOf(w, 0, false)
		}
	}
	if w.HubLevel != 2 {
		t.Fatalf("Hub-Ausbau nicht fertig: Hub-Stufe %d, %+v", w.HubLevel, sim.EconomyOf(w).HubUpgrade)
	}
	d := asJSON(t, deltaOf(prev, stateOf(w, 0, false))).(map[string]any)
	sites, _ := d["sites"].(map[string]any)
	set, _ := sites["set"].([]any)
	got := siteLevels(t, map[string]any{"sites": set})
	n := 0
	for _, site := range w.Sites {
		if site.State == "unpaid" && sim.SiteHubLevel(w, site) == 3 {
			n++
			if got[float64(site.ID)] != 3 {
				t.Errorf("%s (Platz-Stufe 3) fehlt in sites.set", site.Kind)
			}
		}
	}
	if n == 0 {
		t.Error("kein Platz mit Platz-Stufe 3")
	}
}
