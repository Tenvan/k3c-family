package sim

import (
	"strings"
	"testing"
)

// B-1217: Die Art der Welt (kind) ist optional; ohne Angabe gilt spinoff mit extends, sonst new.
func TestParseWorldKind(t *testing.T) {
	const head = `{"id": "w", "name": "W", `
	const isl = `"islands": [{"id": "i", "name": "I", "depths": [0], "depthScaling": {"hp": 1, "damage": 1, "speed": 1}}]}`
	for _, c := range []struct{ extra, want, err string }{
		{``, WorldKindNew, ""},
		{`"extends": "standard", `, WorldKindSpinoff, ""},
		{`"extends": "standard", "kind": "new", `, WorldKindNew, ""},
		{`"kind": "new", `, WorldKindNew, ""},
		{`"kind": "spinoff", `, "", "kind: spinoff braucht extends"},
		{`"kind": "episode", `, "", `kind: "episode" unbekannt`},
	} {
		w, err := ParseWorld([]byte(head+c.extra+isl), "w")
		if c.err != "" {
			if err == nil || !strings.Contains(err.Error(), c.err) {
				t.Errorf("%s: %v, erwartet %q", c.extra, err, c.err)
			}
			continue
		}
		if err != nil || w.Kind != c.want {
			t.Errorf("%s: Kind %q, %v, erwartet %q", c.extra, w.Kind, err, c.want)
		}
	}
}
