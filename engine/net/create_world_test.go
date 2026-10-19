package net

import (
	"encoding/json"
	"net/http/httptest"
	"os"
	"testing"
	"testing/fstest"

	"k3c/engine/room"
)

// Welt im Protokoll (WZ18.5, B-488/AC-03, AC-04, docs/rules/welten.md § 6): `create.world` wählt die Welt eines neuen
// Stands (nur im Dev-Mode), `joined.world` nennt die Welt des Raums.

// nordmeerRaw ist eine Test-Welt mit zwei Inseln (nicht aus data/worlds/).
const nordmeerRaw = `{"id":"nordmeer","name":"Nordmeer","islands":[` +
	`{"id":"island1","name":"Insel 1","depths":[0,4],"depthScaling":{"hp":1.5,"damage":1.3,"speed":1.1}},` +
	`{"id":"eisinsel","name":"Eisinsel","depths":[0,4],"depthScaling":{"hp":1.6,"damage":1.4,"speed":1.15}}]}`

// worldServer ist wsServer mit Dev-Mode nach Wunsch und dem Test-Ordner der Welten (gesetzt vor der ersten Verbindung).
func worldServer(t *testing.T, dev bool) (*httptest.Server, *room.Manager) {
	t.Helper()
	srv, m := wsServer(t)
	m.Dev = dev
	m.Worlds = fstest.MapFS{"nordmeer.json": {Data: []byte(nordmeerRaw)}}
	return srv, m
}

// createWorldExample ist testdata/protocol/c2s-create-world.json, roh gesendet.
func createWorldExample(t *testing.T) string {
	t.Helper()
	raw, err := os.ReadFile("../../testdata/protocol/c2s-create-world.json")
	if err != nil {
		t.Fatal(err)
	}
	var m inMsg
	if err := json.Unmarshal(raw, &m); err != nil || m.T != "create" || m.World != "nordmeer" {
		t.Fatalf("Beispiel: %+v, %v", m, err)
	}
	return string(raw)
}

// AC-03 (Protokoll), AC-04: Das Beispiel legt im Dev-Mode den Raum in der Welt an; joined nennt sie, auch einem
// zweiten Gerät per join.
func TestCreateMitWeltImDevMode(t *testing.T) {
	srv, m := worldServer(t, true)
	x := hello(t, srv, "xbox")
	x.send(createWorldExample(t))
	if j := x.expect("joined"); j["world"] != "nordmeer" {
		t.Fatalf("joined: %v", j)
	}
	if got := m.Room("KRNZ").Opts.World; got != "nordmeer" {
		t.Fatalf("Options.World %q", got)
	}
	h := hello(t, srv, "handy")
	joinRoom(h, "KRNZ", 0)
	if j := h.expect("joined"); j["world"] != "nordmeer" {
		t.Fatalf("joined per join: %v", j)
	}
}

// Ohne world (alte Clients) und mit world standard spielt der Raum die Standard-Welt, auch ohne Dev-Mode.
func TestCreateOhneWeltIstStandard(t *testing.T) {
	for i, world := range []string{"", "standard"} {
		srv, _ := worldServer(t, false)
		x := hello(t, srv, "xbox")
		msg := map[string]any{"t": "create", "save": "std", "fresh": true, "depth": 0, "slots": []int{0}}
		if world != "" {
			msg["world"] = world
		}
		x.send(msg)
		if j := x.expect("joined"); j["world"] != "standard" {
			t.Fatalf("Fall %d: joined %v", i, j)
		}
	}
}

// Eine andere Welt ohne Dev-Mode ist forbidden, eine unbekannte oder ungültige im Dev-Mode world_unknown; kein Raum.
func TestCreateWeltAbgelehnt(t *testing.T) {
	cases := []struct {
		dev   bool
		world string
		code  string
	}{
		{false, "nordmeer", "forbidden"},
		{true, "suedsee", "world_unknown"},
		{true, "../nordmeer", "world_unknown"},
	}
	for _, c := range cases {
		srv, m := worldServer(t, c.dev)
		x := hello(t, srv, "xbox")
		x.send(map[string]any{"t": "create", "save": "weg", "fresh": true, "depth": 0, "world": c.world, "slots": []int{0}})
		x.expectError(c.code)
		if len(m.Rooms()) != 0 {
			t.Fatalf("Welt %q: Raum angelegt", c.world)
		}
	}
}
