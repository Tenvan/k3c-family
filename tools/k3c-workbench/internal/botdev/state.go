package botdev

import (
	"encoding/json"
	"fmt"

	"k3c/engine/sim"
)

// Zustand einer Stufe nach docs/protocol.md › Zustand und Delta: `snap` setzt ihn, `delta` ändert ihn. Listen mit `id`
// stehen im Delta als {set, del}, alles andere ganz; `unset` entfernt Felder. Die Bots entscheiden auf einer sim.World,
// die aus diesem Zustand dekodiert wird (dieselben JSON-Namen wie der Server sendet).

// idLists wie engine/net/delta.go › idLists (dort nicht exportiert).
var idLists = map[string]bool{
	"players": true, "coins": true, "troops": true, "nodes": true, "sites": true, "enemies": true,
	"projectiles": true, "pickups": true, "drops": true,
}

// applyDelta ändert state nach einem delta.
func applyDelta(state, d map[string]any) {
	for k, v := range d {
		switch {
		case k == "unset":
			for _, name := range asList(v) {
				if s, ok := name.(string); ok {
					delete(state, s)
				}
			}
		case idLists[k]:
			state[k] = mergeList(state[k], v)
		default:
			state[k] = v
		}
	}
}

// mergeList wendet {set, del} auf eine Liste mit `id` an: geänderte Einträge an ihrem Platz, neue hinten.
func mergeList(cur, d any) []any {
	m, _ := d.(map[string]any)
	del := map[any]bool{}
	for _, id := range asList(m["del"]) {
		del[id] = true
	}
	set := map[any]any{}
	var order []any
	for _, e := range asList(m["set"]) {
		id := idOf(e)
		set[id] = e
		order = append(order, id)
	}
	out := []any{}
	for _, e := range asList(cur) {
		id := idOf(e)
		switch {
		case del[id]:
		case set[id] != nil:
			out = append(out, set[id])
			delete(set, id)
		default:
			out = append(out, e)
		}
	}
	for _, id := range order {
		if e, ok := set[id]; ok {
			out = append(out, e)
		}
	}
	return out
}

func idOf(e any) any {
	m, _ := e.(map[string]any)
	return m["id"]
}

func asList(v any) []any {
	l, _ := v.([]any)
	return l
}

// worldOf dekodiert den Zustand in eine sim.World (nur die gesendeten Felder; Level und Biom fehlen).
func worldOf(state map[string]any) (*sim.World, error) {
	raw, err := json.Marshal(state)
	if err != nil {
		return nil, err
	}
	var w sim.World
	if err := json.Unmarshal(raw, &w); err != nil {
		return nil, fmt.Errorf("zustand nicht lesbar: %w", err)
	}
	return &w, nil
}
