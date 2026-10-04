package net

import "reflect"

// Delta nach docs/protocol.md › Zustand und Delta: nur geänderte Felder. Listen mit `id` stehen als
// {"set": [geänderte oder neue Einträge], "del": [entfernte ids]}, alles andere bei einer Änderung ganz. `null` ist ein
// Wert. `events` fehlt, wenn es im Tick keine gab.

// idLists sind die Listen, deren Einträge eine `id` haben.
var idLists = map[string]bool{
	"players": true, "coins": true, "troops": true, "nodes": true, "sites": true, "enemies": true,
	"projectiles": true, "pickups": true,
}

// deltaOf liefert die Änderungen von prev zu cur (beide aus stateOf).
func deltaOf(prev, cur map[string]any) map[string]any {
	d := map[string]any{}
	for k, v := range cur {
		switch {
		case k == "events":
			if l, _ := v.([]any); len(l) > 0 {
				d[k] = v
			}
		case idLists[k]:
			if set, del := listDelta(prev[k], v); len(set)+len(del) > 0 {
				d[k] = map[string]any{"set": set, "del": del}
			}
		case !reflect.DeepEqual(prev[k], v):
			d[k] = v
		}
	}
	return d
}

func listDelta(prev, cur any) (set []any, del []any) {
	old := map[any]any{}
	for _, e := range asList(prev) {
		old[e.(map[string]any)["id"]] = e
	}
	set, del = []any{}, []any{}
	seen := map[any]bool{}
	for _, e := range asList(cur) {
		id := e.(map[string]any)["id"]
		seen[id] = true
		if !reflect.DeepEqual(old[id], e) {
			set = append(set, e)
		}
	}
	for _, e := range asList(prev) {
		if id := e.(map[string]any)["id"]; !seen[id] {
			del = append(del, id)
		}
	}
	return set, del
}

func asList(v any) []any {
	l, _ := v.([]any)
	return l
}
