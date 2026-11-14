package hud

import (
	"encoding/json"
	"reflect"
	"slices"
	"strings"
	"testing"

	"k3c/data"
	"k3c/engine/sim"
)

// paperGroups liefert je Asset-ID aus data/paper.json die Gruppe (alle Ebenen durchsucht).
func paperGroups(t *testing.T) map[string]string {
	t.Helper()
	raw, err := data.Files.ReadFile("paper.json")
	if err != nil {
		t.Fatal(err)
	}
	var v any
	if err := json.Unmarshal(raw, &v); err != nil {
		t.Fatal(err)
	}
	out := map[string]string{}
	var walk func(any)
	walk = func(x any) {
		m, ok := x.(map[string]any)
		if !ok {
			return
		}
		for k, sub := range m {
			if sm, ok := sub.(map[string]any); ok {
				if g, ok := sm["gruppe"].(string); ok {
					out[k] = g
				}
			}
			walk(sub)
		}
	}
	walk(v)
	return out
}

// AC-01 (B-890): Jeder Rohstoff der Sim (Felder von sim.Stock) hat ein Icon; jedes Icon des HUD steht in
// data/paper.json in der Gruppe icons und wird als Asset gemeldet.
func TestEveryResourceHasIcon(t *testing.T) {
	st := reflect.TypeOf(sim.Stock{})
	if st.NumField() != len(resourceOrder) {
		t.Fatalf("sim.Stock hat %d Rohstoffe, HUD kennt %d", st.NumField(), len(resourceOrder))
	}
	for i := range st.NumField() {
		tag, _, _ := strings.Cut(st.Field(i).Tag.Get("json"), ",")
		if resourceOrder[i] != tag || resourceIcons[tag] == "" {
			t.Errorf("Rohstoff %s: Reihenfolge %s, Icon %q", tag, resourceOrder[i], resourceIcons[tag])
		}
	}
	groups := paperGroups(t)
	icons := append([]string{goldIcon}, slices.Collect(func(yield func(string) bool) {
		for _, a := range resourceIcons {
			if !yield(a) {
				return
			}
		}
	})...)
	assets := (&hud{}).Assets("forest", nil)
	for _, a := range icons {
		group, id, _ := strings.Cut(a, "/")
		if groups[id] != group || !slices.Contains(assets, a) {
			t.Errorf("%s: Gruppe %q in paper.json, gemeldet %v", a, groups[id], slices.Contains(assets, a))
		}
	}
	if !slices.IsSorted(assets) || len(assets) != len(icons) {
		t.Errorf("Assets %v", assets)
	}
}

// AC-02 (B-890): Die Vorrats-Anzeige hat je Rohstoff mit Vorrat (oder dem Primärrohstoff) Icon und Zahl statt Namen,
// mit Maximum „n/max“ und voll markiert; Skill-Punkte stehen als Text dahinter.
func TestStockParts(t *testing.T) {
	k := stockKey{lang: "de", primary: "stone", stock: [5]int{12, 0, 0, 3, 0}, max: 12, points: 2}
	got := stockParts(nil, k)
	if len(got) != 4 || got[0] != (stockPart{res: "wood", text: "12/12", full: true}) || got[1].res != "stone" ||
		got[1].text != "0/12" || got[2] != (stockPart{res: "iron", text: "3/12"}) || got[3].res != "" || got[3].text == "" {
		t.Fatalf("Teile %+v", got)
	}
	k.max, k.points = 0, 0
	if got = stockParts(got, k); len(got) != 3 || got[0].text != "12" || got[0].full {
		t.Errorf("ohne Maximum: %+v", got)
	}
	for _, p := range got {
		if strings.Contains(p.text, "Holz") || strings.Contains(p.text, "Stein") {
			t.Errorf("Name statt Icon: %+v", p)
		}
	}
}
