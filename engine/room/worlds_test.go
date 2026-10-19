package room

import (
	"bytes"
	"encoding/json"
	"errors"
	"log/slog"
	"strings"
	"testing"
	"testing/fstest"

	"k3c/engine/sim"
)

// Welt-Wahl im Raum (WZ18.4, B-488/AC-03, docs/rules/welten.md § 5/6): Der Raum legt neue Inseln in der gewählten Welt
// an (Wahl nur im Dev-Mode, sonst nur standard), lädt Spielstände mit ihrer gespeicherten Welt und lehnt unbekannte oder
// ungültige Welten ab.

// nordmeerRaw ist eine Test-Welt mit zwei Inseln (nicht aus data/worlds/).
const nordmeerRaw = `{"id":"nordmeer","name":"Nordmeer","islands":[` +
	`{"id":"island1","name":"Insel 1","depths":[0,4],"depthScaling":{"hp":1.5,"damage":1.3,"speed":1.1}},` +
	`{"id":"eisinsel","name":"Eisinsel","depths":[0,4],"depthScaling":{"hp":1.6,"damage":1.4,"speed":1.15}}]}`

// worldFixture ist ein Manager im Dev-Mode mit dem Test-Ordner der Welten und einem Log.
func worldFixture() (*fixture, *bytes.Buffer) {
	f := newFixture()
	var out bytes.Buffer
	f.m.Log = slog.New(slog.NewTextHandler(&out, nil))
	f.m.Dev = true
	f.m.Worlds = fstest.MapFS{
		"nordmeer.json": {Data: []byte(nordmeerRaw)},
		"kaputt.json":   {Data: []byte(`{"id":"kaputt","name":"Kaputt","islands":[],"size":3}`)},
	}
	return f, &out
}

func TestWeltWahlImDevMode(t *testing.T) {
	f, _ := worldFixture()
	r := need(f.m.Create("xbox", &peer{}, "nord", true, 0, []int{0}, Options{World: "nordmeer"}))(t)
	if r.isl.WorldID() != "nordmeer" || r.isl.IslandID() != "island1" || len(r.isl.Stages) != 2 || r.isl.Stages[1].Biome.Depth != 4 {
		t.Fatalf("Welt %q, Insel %q, %d Stufen", r.isl.WorldID(), r.isl.IslandID(), len(r.isl.Stages))
	}
	r.saveNow()
	s := need(sim.ParseIslandSave(f.store.data["nord"]))(t)
	if s.World != "nordmeer" || s.IslandID != "island1" {
		t.Fatalf("Spielstand: Welt %q, Insel %q", s.World, s.IslandID)
	}
}

func TestWeltStandardOhneAngabe(t *testing.T) {
	for _, world := range []string{"", "standard"} {
		f := newFixture() // ohne Dev-Mode und ohne Welten-Ordner
		r := need(f.m.Create("xbox", &peer{}, "std", true, 0, []int{0}, Options{World: world}))(t)
		if r.isl.WorldID() != sim.StandardWorldID || len(r.isl.Stages) != 5 {
			t.Fatalf("Welt %q: %q mit %d Stufen", world, r.isl.WorldID(), len(r.isl.Stages))
		}
	}
}

// Die Standard-Welt kommt auch im Dev-Mode aus der Einbettung, nie von der Platte (welten.md § 6).
func TestWeltStandardImDevModeEingebettet(t *testing.T) {
	f, _ := worldFixture()
	f.m.Worlds = fstest.MapFS{"standard.json": {Data: []byte(`{`)}}
	r := need(f.m.Create("xbox", &peer{}, "std", true, 0, []int{0}, Options{World: "standard"}))(t)
	if r.isl.WorldID() != sim.StandardWorldID || len(r.isl.Stages) != 5 {
		t.Fatalf("Welt %q mit %d Stufen", r.isl.WorldID(), len(r.isl.Stages))
	}
}

// rejected prüft, dass Create mit der Welt scheitert, kein Raum und kein Spielstand entsteht und das Log 🚫 meldet.
func rejected(t *testing.T, f *fixture, out *bytes.Buffer, world string, want error, msg string) {
	t.Helper()
	_, err := f.m.Create("xbox", &peer{}, "nord", true, 0, []int{0}, Options{World: world})
	if !errors.Is(err, want) || !strings.Contains(err.Error(), msg) {
		t.Fatalf("Welt %q: %v, erwartet %v mit %q", world, err, want, msg)
	}
	if len(f.m.Rooms()) != 0 || f.store.data["nord"] != nil {
		t.Fatalf("Welt %q: Raum oder Spielstand angelegt", world)
	}
	if !strings.Contains(out.String(), "🚫 Beitritt abgelehnt") {
		t.Fatalf("Log ohne Ablehnung %q:\n%s", msg, out)
	}
}

func TestWeltWahlOhneDevModeAbgelehnt(t *testing.T) {
	f, out := worldFixture()
	f.m.Dev = false
	rejected(t, f, out, "nordmeer", ErrForbidden, `Welt "nordmeer" nur im Dev-Mode`)
}

func TestWeltUnbekanntOderUngueltigAbgelehnt(t *testing.T) {
	for world, msg := range map[string]string{
		"suedsee":       `Welt "suedsee" unbekannt`,
		"../saves/nord": `Welt-ID "../saves/nord" ungültig`,
		"kaputt":        `data/worlds/kaputt.json: unbekanntes Feld "size"`,
		"Nordmeer":      `Welt-ID "Nordmeer" ungültig`,
		"nordmeer/../x": `ungültig`,
	} {
		f, out := worldFixture()
		rejected(t, f, out, world, ErrWorldUnknown, msg)
	}
}

// nordmeerSave ist ein Spielstand auf Insel 2 der Test-Welt nordmeer (Version 8).
func nordmeerSave(t *testing.T) []byte {
	t.Helper()
	w := need(sim.LoadWorld(fstest.MapFS{"nordmeer.json": {Data: []byte(nordmeerRaw)}}, "nordmeer"))(t)
	isl := need(sim.CreateIslandIn(w, "nord", []int{0, 4}, 1, sim.CreateOptions{}))(t)
	sim.AddIslandPlayer(isl, 0)
	sim.AddIslandPlayer(isl, 0)
	next := need(sim.NextIsland(isl, w.Islands[1]))(t)
	return need(json.Marshal(next.ToSave("2026-10-19T20:00:00Z")))(t)
}

func TestWeltLadenMitGespeicherterWelt(t *testing.T) {
	f, _ := worldFixture()
	f.store.data["nord"] = nordmeerSave(t)
	r := need(f.m.Create("xbox", &peer{}, "nord", false, 0, []int{0, 1}, Options{}))(t)
	if r.isl.WorldID() != "nordmeer" || r.isl.IslandID() != "eisinsel" || r.isl.Number != 1 {
		t.Fatalf("Welt %q, Insel %q, Nummer %d", r.isl.WorldID(), r.isl.IslandID(), r.isl.Number)
	}
}

func TestWeltLadenAbgelehntStandUnveraendert(t *testing.T) {
	cases := []struct {
		name string
		dev  bool
		fs   fstest.MapFS
		want error
		msg  string
	}{
		{"ohne Dev-Mode", false, nil, ErrForbidden, `spielstand: Welt "nordmeer" nur im Dev-Mode`},
		{"Datei fehlt", true, fstest.MapFS{}, ErrWorldUnknown, `spielstand: Welt "nordmeer" unbekannt`},
		{"Datei ungültig", true, fstest.MapFS{"nordmeer.json": {Data: []byte(`{`)}}, ErrWorldUnknown, `data/worlds/nordmeer.json: kein gültiges JSON`},
	}
	for _, c := range cases {
		f, out := worldFixture()
		f.m.Dev, f.m.Worlds = c.dev, c.fs
		save := nordmeerSave(t)
		f.store.data["nord"] = bytes.Clone(save)
		_, err := f.m.Create("xbox", &peer{}, "nord", false, 0, []int{0}, Options{})
		if !errors.Is(err, c.want) || !strings.Contains(err.Error(), c.msg) {
			t.Fatalf("%s: %v, erwartet %q", c.name, err, c.msg)
		}
		if !bytes.Equal(f.store.data["nord"], save) || len(f.m.Rooms()) != 0 || !strings.Contains(out.String(), "🚫") {
			t.Fatalf("%s: Stand geändert, Raum angelegt oder kein 🚫 im Log", c.name)
		}
	}
}
