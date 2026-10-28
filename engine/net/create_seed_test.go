package net

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"net/url"
	"reflect"
	"strings"
	"testing"
)

// apiLayout holt das Level von GET /api/level ohne die Warnungen (die Raum-Nachricht level trägt sie nicht).
func apiLayout(t *testing.T, srv *httptest.Server, seed, biome string) map[string]any {
	t.Helper()
	res, err := http.Get(srv.URL + "/api/level?seed=" + url.QueryEscape(seed) + "&biome=" + biome)
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = res.Body.Close() }()
	var m map[string]any
	if err := json.NewDecoder(res.Body).Decode(&m); err != nil {
		t.Fatal(err)
	}
	delete(m, "warnings")
	return m
}

// B-095/AC-01: `create` mit seed und depth erzeugt das Level, das GET /api/level für dieselben Werte liefert;
// ohne seed bleibt der Name der Seed (B-095/AC-03, alte Clients).
func TestCreateMitSeedUndTiefe(t *testing.T) {
	cases := []struct {
		name, seed string
		depth      int
		biome      string
		seedOfAPI  string
	}{
		{"probe", "Familie-2026 ÄÖ", 1, "cave", "Familie-2026 ÄÖ"},
		{"nur-name", "", 3, "ironhold", "nur-name"}, // Tiefe 0 trägt den geplanten Hafen (B-637), /api/level kennt keine Planung
		{"mine-lang", strings.Repeat("ü", 64), 2, "mine", strings.Repeat("ü", 64)},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			srv, _ := wsServer(t)
			x := hello(t, srv, "xbox")
			msg := map[string]any{"t": "create", "save": c.name, "fresh": true, "depth": c.depth, "slots": []int{0}}
			if c.seed != "" {
				msg["seed"] = c.seed
			}
			x.send(msg)
			x.expect("joined")
			l := x.expect("level")
			if l["depth"] != float64(c.depth) {
				t.Fatalf("Tiefe %v, erwartet %d", l["depth"], c.depth)
			}
			if want := apiLayout(t, srv, c.seedOfAPI, c.biome); !reflect.DeepEqual(l["layout"], any(want)) {
				t.Fatalf("Level im Raum weicht von /api/level (seed %q, biome %s) ab", c.seedOfAPI, c.biome)
			}
		})
	}
}

// Zu langer Seed (65 Zeichen) → bad_request, kein Raum.
func TestCreateSeedZuLang(t *testing.T) {
	srv, m := wsServer(t)
	x := hello(t, srv, "xbox")
	x.send(map[string]any{"t": "create", "save": "lang", "fresh": true, "depth": 0, "seed": strings.Repeat("a", 65), "slots": []int{0}})
	x.expectError("bad_request")
	if m.Room("KRNZ") != nil {
		t.Fatal("kein Raum erwartet")
	}
}
