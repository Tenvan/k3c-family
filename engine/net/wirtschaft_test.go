package net

import (
	"encoding/json"
	"reflect"
	"slices"
	"testing"

	"k3c/engine/sim"
)

// W5.1 (B-153/AC-03): Hub-Stufe mit Ausbaukosten, Lager mit Maximum, Wartegrund je Bauplatz und Händler im Zustand.

// wirtschaftsInsel ist eine Insel (3 Stufen, 4 Spieler, Seed fest) mit bezahltem Hub-Ausbau, gefülltem Lager und
// anwesendem Händler; geliefert wird Stufe 0.
func wirtschaftsInsel(t *testing.T) (*sim.Island, *sim.World) {
	t.Helper()
	isl, err := sim.CreateIsland("w5-wirtschaft", []int{0, 1, 2}, 1)
	if err != nil {
		t.Fatal(err)
	}
	for i := range 4 {
		sim.AddIslandPlayer(isl, i%3)
	}
	w := isl.Stages[0]
	p := w.Players[0]
	p.X, p.Gold = w.Castle.X, 60
	cmds := make([]sim.PlayerCommand, 4)
	cmds[0].Pay = true
	for range 3000 {
		if up := sim.EconomyOf(w).HubUpgrade; up != nil && up.State != "" {
			break
		}
		p.X = w.Castle.X
		sim.StepIsland(isl, cmds, 1.0/30)
	}
	*isl.Stock = sim.Stock{Wood: 40, Stone: 30, Copper: 12}
	w.Merchant = &sim.Merchant{Resource: "stone", Leaves: 3}
	for _, s := range w.Sites {
		switch s.Kind {
		case "storage":
			s.State = "built"
		case "farm":
			s.State = "waitingMaterial"
		}
	}
	for _, tr := range w.Troops {
		if tr.Kind == "peasant" {
			tr.Profession = "builder"
		}
	}
	return isl, w
}

// asJSON ist v nach einem Rundlauf durch JSON (Zahlen als float64), wie es ein Client liest.
func asJSON(t *testing.T, v any) any {
	t.Helper()
	raw, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	var out any
	if err := json.Unmarshal(raw, &out); err != nil {
		t.Fatal(err)
	}
	return out
}

func TestWirtschaftZustand(t *testing.T) {
	_, w := wirtschaftsInsel(t)
	s := asJSON(t, stateOf(w, 0, false)).(map[string]any)
	if s["hubLevel"] != 1.0 || s["stockMax"] != 1200.0 {
		t.Errorf("hubLevel/stockMax: %v %v", s["hubLevel"], s["stockMax"])
	}
	up, _ := s["hubUpgrade"].(map[string]any)
	mat, _ := up["material"].(map[string]any)
	if up["gold"] != 50.0 || up["paid"] != 50.0 || up["state"] != "waitingMaterial" || mat["stone"] != 100.0 {
		t.Errorf("hubUpgrade: %v", up)
	}
	if st := s["stock"].(map[string]any); st["wood"] != 40.0 || st["stone"] != 30.0 || st["copper"] != 12.0 {
		t.Errorf("stock: %v", st)
	}
	if m := s["merchant"].(map[string]any); m["resource"] != "stone" || m["leaves"] != 3.0 {
		t.Errorf("merchant: %v", m)
	}
	checkWartegrundUndBeruf(t, s)
	if _, ok := s["danger"]; ok && w.Cycle.Phase != "night" && len(w.Enemies) == 0 {
		t.Errorf("danger ohne Nacht und Gegner")
	}
}

// checkWartegrundUndBeruf: Jeder Bauplatz nennt seinen Wartegrund, einer wartet auf Material, ein Bauer hat einen Beruf.
func checkWartegrundUndBeruf(t *testing.T, s map[string]any) {
	t.Helper()
	waiting := 0
	for _, e := range s["sites"].([]any) {
		st, _ := e.(map[string]any)["state"].(string)
		if !slices.Contains([]string{"unpaid", "waitingMaterial", "waitingWorker", "built"}, st) {
			t.Errorf("Wartegrund fehlt: %v", e)
		}
		if st == "waitingMaterial" {
			waiting++
		}
	}
	if waiting == 0 {
		t.Errorf("kein Bauplatz wartet auf Material")
	}
	if tr := s["troops"].([]any); !slices.ContainsFunc(tr, func(e any) bool { return e.(map[string]any)["profession"] == "builder" }) {
		t.Errorf("Beruf fehlt: %v", tr)
	}
}

// Ein Delta nennt nur die geänderten Wirtschaftsfelder: Lager, Gefahr und den abgereisten Händler (unset).
func TestWirtschaftDelta(t *testing.T) {
	_, w := wirtschaftsInsel(t)
	w.Cycle.Phase, w.Enemies, w.Events = "day", nil, []sim.Event{}
	prev := stateOf(w, 0, false)
	w.Stock.Stone += 5
	w.Cycle.Phase = "night"
	w.Merchant = nil
	cur := stateOf(w, 0, false)
	d := deltaOf(prev, cur)
	var keys []string
	for k := range d {
		keys = append(keys, k)
	}
	slices.Sort(keys)
	if want := []string{"cycle", "danger", "stock", "unset"}; !reflect.DeepEqual(keys, want) {
		t.Errorf("Delta-Felder %v, erwartet %v", keys, want)
	}
	if !reflect.DeepEqual(d["unset"], []string{"merchant"}) || d["danger"] != true {
		t.Errorf("unset/danger: %v %v", d["unset"], d["danger"])
	}
	if !reflect.DeepEqual(asJSON(t, apply(asJSON(t, prev).(map[string]any), asJSON(t, d).(map[string]any))), asJSON(t, cur)) {
		t.Errorf("Delta auf prev ergibt nicht cur")
	}
}

// W10.2 (B-332/AC-03): Der Zustand nennt Kämpfer-Zahl und Truppen-Limit; ändert sich die Kämpfer-Zahl, nennt das Delta
// fighters, und 0 Kämpfer ist ein Wert, kein unset.
func TestKaempferZustandUndDelta(t *testing.T) {
	_, w := wirtschaftsInsel(t)
	w.Events = []sim.Event{}
	prev := stateOf(w, 0, false)
	if s := asJSON(t, prev).(map[string]any); s["fighters"] != 2.0 || s["troopLimit"] != 10.0 {
		t.Errorf("fighters/troopLimit: %v %v", s["fighters"], s["troopLimit"])
	}
	for _, tr := range w.Troops {
		if tr.Kind == "archer" {
			tr.Kind = "vagrant"
		}
	}
	d := asJSON(t, deltaOf(prev, stateOf(w, 0, false))).(map[string]any)
	if d["fighters"] != 0.0 || d["unset"] != nil {
		t.Errorf("fighters/unset: %v %v", d["fighters"], d["unset"])
	}
	if _, ok := d["troopLimit"]; ok {
		t.Errorf("troopLimit unverändert, steht aber im Delta")
	}
}

// Das Beispiel s2c-snapshot-wirtschaft.json hat die Schlüssel des Zustands (rekursiv, erste Listeneinträge) und die
// Ereignisse revived, disarmed und equipmentTaken mit ihren Feldern (AC-01, AC-06).
func TestWirtschaftBeispiel(t *testing.T) {
	_, w := wirtschaftsInsel(t)
	w.Events = []sim.Event{} // Ereignisse prüft die Schleife unten
	ex := want(t, "snapshot-wirtschaft")
	got := asJSON(t, stateMsg{T: "snap", S: stateOf(w, 0, false)})
	if d := sameKeys("wirtschaft", ex, got, true); d != "" {
		t.Error(d)
	}
	fields := map[string][]string{
		"revived":        {"player", "stage", "type", "x"},
		"disarmed":       {"cause", "kind", "stage", "type", "x"},
		"equipmentTaken": {"kind", "stage", "type", "x"},
	}
	for _, raw := range ex["s"].(map[string]any)["events"].([]any) {
		var e sim.Event
		b, _ := json.Marshal(raw)
		if err := json.Unmarshal(b, &e); err != nil {
			t.Fatal(err)
		}
		var keys []string
		for k := range e {
			keys = append(keys, k)
		}
		slices.Sort(keys)
		typ, _ := e["type"].(string)
		if !reflect.DeepEqual(keys, fields[typ]) {
			t.Errorf("%s: Felder %v, erwartet %v", typ, keys, fields[typ])
		}
		delete(fields, typ)
	}
	if len(fields) > 0 {
		t.Errorf("Ereignisse fehlen im Beispiel: %v", fields)
	}
}
