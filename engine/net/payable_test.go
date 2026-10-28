package net

import (
	"testing"

	"k3c/engine/sim"
)

// B-587/AC-01, B-588: `sites[].payable` gleich sim.SitePayable, `nodes[].minable` gleich sim.NodeMarkable; beide fehlen
// bei false, auch nachdem ein Platz bezahlt bzw. ein Knoten markiert ist (Snapshot und Delta).
func TestPayableUndMinable(t *testing.T) {
	_, w := wirtschaftsInsel(t)
	prev := stateOf(w, 0, false)
	paid := checkPayable(t, w, prev)
	node := w.Nodes[0]
	if prev["nodes"].([]any)[0].(map[string]any)["minable"] != sim.NodeMarkable(node) || !sim.NodeMarkable(node) {
		t.Fatalf("erster Knoten (%s) soll abbaubar sein", node.Kind)
	}

	paid.State = "waitingMaterial" // Gold bezahlt: nimmt keine Münzen mehr
	node.Marked = true
	d := asJSON(t, deltaOf(prev, stateOf(w, 0, false))).(map[string]any)
	if e := entryOf(t, d, "sites", paid.ID); e["state"] != "waitingMaterial" || e["payable"] != nil {
		t.Errorf("bezahlter Platz im Delta: %v", e)
	}
	if e := entryOf(t, d, "nodes", node.ID); e["marked"] != true || e["minable"] != nil {
		t.Errorf("markierter Knoten im Delta: %v", e)
	}
}

// checkPayable prüft `payable` aller Plätze gegen sim.SitePayable und liefert den ersten bezahlbaren unbezahlten Platz.
func checkPayable(t *testing.T, w *sim.World, s map[string]any) *sim.Site {
	t.Helper()
	var paid *sim.Site
	yes, no := 0, 0
	byID := map[float64]*sim.Site{}
	for _, site := range w.Sites {
		byID[float64(site.ID)] = site
	}
	for _, raw := range s["sites"].([]any) {
		site := byID[raw.(map[string]any)["id"].(float64)] // filterSites lässt Plätze weg: über die ID zuordnen
		got, has := raw.(map[string]any)["payable"]
		want := sim.SitePayable(w, site)
		if want != (got == true) || (!want && has) {
			t.Errorf("Platz %d (%s): payable %v, erwartet %v", site.ID, site.Kind, got, want)
		}
		if !want {
			no++
			continue
		}
		yes++
		if paid == nil && site.State == "unpaid" {
			paid = site
		}
	}
	if yes == 0 || no == 0 || paid == nil {
		t.Fatalf("Insel braucht bezahlbare und gesperrte Plätze: %d bezahlbar, %d gesperrt", yes, no)
	}
	return paid
}

// entryOf ist der Eintrag mit id aus `set` der Liste key im Delta d.
func entryOf(t *testing.T, d map[string]any, key string, id int) map[string]any {
	t.Helper()
	list, _ := d[key].(map[string]any)
	set, _ := list["set"].([]any)
	for _, raw := range set {
		if e := raw.(map[string]any); e["id"] == float64(id) {
			return e
		}
	}
	t.Fatalf("%s %d fehlt im Delta", key, id)
	return nil
}

// B-636: `sites[].clearing` ist wahr für einen Platz, der seine Hub-Stufe erreicht hat, aber im Freiraum noch Bäume hat; er ist dann nicht `payable`.
func TestClearingImSnapshot(t *testing.T) {
	_, w := wirtschaftsInsel(t)
	var target *sim.Site
	for _, s := range w.Sites {
		if _, ok := sim.SiteZone(w, s); ok && s.State == "unpaid" {
			target = s
			break
		}
	}
	if target == nil {
		t.Fatal("kein Mauer-, Turm- oder Baumschul-Platz in der Insel")
	}
	w.HubLevel = 5
	w.Nodes = append(w.Nodes, &sim.ResourceNode{ID: 99999, Kind: "tree", X: target.X})
	for _, raw := range stateOf(w, 0, false)["sites"].([]any) {
		e := raw.(map[string]any)
		if e["id"] == float64(target.ID) && (e["clearing"] != true || e["payable"] != nil) {
			t.Fatalf("Platz %s@%v: clearing %v payable %v", target.Kind, target.X, e["clearing"], e["payable"])
		} else if e["id"] == float64(target.ID) {
			return
		}
	}
	t.Fatal("Platz nicht im Snapshot")
}
