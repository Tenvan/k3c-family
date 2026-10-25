package net

import (
	"testing"

	"k3c/engine/sim"
)

// W11.2 (B-208, B-408; Sprint W11 AC-02): Bauplätze tragen die Platz-Stufe `hubLevel`, unbezahlte Plätze über der
// nächsten Hub-Stufe fehlen im Zustand, bezahlte und gebaute immer.

// siteLevels sind die Plätze aus dem Zustand: id → hubLevel.
func siteLevels(t *testing.T, s map[string]any) map[float64]float64 {
	t.Helper()
	out := map[float64]float64{}
	for _, e := range asJSON(t, s["sites"]).([]any) {
		m := e.(map[string]any)
		lvl, ok := m["hubLevel"].(float64)
		if !ok {
			t.Fatalf("Platz ohne hubLevel: %v", m)
		}
		out[m["id"].(float64)] = lvl
	}
	return out
}

func TestBauplaetzeBisNaechsteHubStufe(t *testing.T) {
	_, w := wirtschaftsInsel(t)
	if w.HubLevel != 1 {
		t.Fatalf("Hub-Stufe %d, erwartet 1", w.HubLevel)
	}
	got := siteLevels(t, stateOf(w, 0, false))
	hidden := 0
	for _, site := range w.Sites {
		lvl, shown := got[float64(site.ID)]
		want := sim.SiteHubLevel(w, site)
		switch {
		case shown && lvl != float64(want):
			t.Errorf("%s %d: hubLevel %v, Sim %d", site.Kind, site.ID, lvl, want)
		case shown && want > 2 && site.State == "unpaid" && site.PaidGold == 0:
			t.Errorf("%s %d: unbezahlt mit Platz-Stufe %d sichtbar", site.Kind, site.ID, want)
		case !shown && (want <= 2 || site.State != "unpaid"):
			t.Errorf("%s %d (Stufe %d, %s) fehlt", site.Kind, site.ID, want, site.State)
		case !shown:
			hidden++
		}
	}
	if hidden == 0 {
		t.Error("kein Platz über Hub-Stufe 2 verborgen")
	}
}

// Ein bezahlter oder gebauter Platz bleibt sichtbar, auch über der nächsten Hub-Stufe.
func TestBauplaetzeBezahltImmerSichtbar(t *testing.T) {
	_, w := wirtschaftsInsel(t)
	var armory *sim.Site
	for _, s := range w.Sites {
		if s.Kind == "armory" {
			armory = s
		}
	}
	if armory == nil || sim.SiteHubLevel(w, armory) <= 2 {
		t.Fatalf("Rüstkammer fehlt oder liegt nicht über Hub-Stufe 2: %+v", armory)
	}
	if _, ok := siteLevels(t, stateOf(w, 0, false))[float64(armory.ID)]; ok {
		t.Fatal("Rüstkammer unbezahlt sichtbar")
	}
	for _, mod := range []func(){func() { armory.PaidGold = 1 }, func() { armory.PaidGold, armory.State = 0, "built" }} {
		mod()
		if _, ok := siteLevels(t, stateOf(w, 0, false))[float64(armory.ID)]; !ok {
			t.Errorf("Rüstkammer %s (Gold %d) fehlt", armory.State, armory.PaidGold)
		}
	}
}

// Hub-Stufe 1 → 2: Das nächste Delta trägt die Plätze der Stufe 3 in sites.set (B-208/AC-02). Den Ausbau in Schritten
// mit Material-Tor prüft engine/sim/hub_step_test.go (GEB17), hier zählt nur das Delta danach.
func TestBauplaetzeDeltaNachHubAusbau(t *testing.T) {
	isl, w := wirtschaftsInsel(t)
	sim.DevSetPhase(isl, "day")
	prev := stateOf(w, 0, false)
	if !sim.DevBuildAll(isl, 2) || w.HubLevel != 2 {
		t.Fatalf("Hub-Stufe %d nach dem Ausbau, erwartet 2", w.HubLevel)
	}
	d := asJSON(t, deltaOf(prev, stateOf(w, 0, false))).(map[string]any)
	sites, _ := d["sites"].(map[string]any)
	set, _ := sites["set"].([]any)
	before := siteLevels(t, prev)
	var neu int
	for _, e := range set {
		m := e.(map[string]any)
		if _, old := before[m["id"].(float64)]; !old {
			if m["hubLevel"] != 3.0 {
				t.Errorf("neu sichtbar mit hubLevel %v: %v", m["hubLevel"], m)
			}
			neu++
		}
	}
	if neu == 0 {
		t.Errorf("keine Plätze der Hub-Stufe 3 im Delta: %v", sites)
	}
}
