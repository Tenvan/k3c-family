package room

import (
	"bytes"
	"encoding/json"
	"errors"
	"testing"
	"time"

	"k3c/engine/sim"
)

// B-315/AC-01: Jeder Bauplatz aller Stufen ist gebaut, jede Ausbaustufe (Hub, Mauer, Turm) kommt mindestens einmal vor,
// und der Stand lädt.
func TestVollAusgebauterSpielstand(t *testing.T) {
	f := newFixture()
	f.m.Dev = true
	name := need(f.m.FullSave("abc"))(t)
	if name != "voll-abc" {
		t.Fatalf("Name %q", name)
	}
	s := need(sim.ParseIslandSave(f.store.data[name]))(t)
	if len(s.Stages) != 5 {
		t.Fatalf("Stufen: %d", len(s.Stages))
	}
	seen := map[string]map[int]bool{"hub": {}, "wall": {}, "tower": {}}
	for _, h := range s.Stages {
		seen["hub"][max(1, h.HubLevel)] = true
		for _, site := range h.Sites {
			if site.State != "built" || site.HP <= 0 {
				t.Fatalf("Tiefe %d: %s@%v ist %s mit HP %v", h.Depth, site.Kind, site.X, site.State, site.HP)
			}
			if seen[site.Kind] != nil {
				seen[site.Kind][max(1, site.Level)] = true
			}
		}
	}
	for kind, levels := range seen {
		for l := 1; l <= 5; l++ {
			if !levels[l] {
				t.Errorf("%s: Stufe %d fehlt (%v)", kind, l, levels)
			}
		}
	}
	if s.Stages[0].HubLevel != 5 {
		t.Errorf("Hub der ersten Stufe: %d, erwartet 5", s.Stages[0].HubLevel)
	}
	loadsBuilt(t, s)
}

// loadsBuilt: Der Stand lädt, jeder Platz ist danach gebaut und die erste Stufe hat Hub 5.
func loadsBuilt(t *testing.T, s sim.IslandSave) {
	t.Helper()
	isl := need(sim.FromIslandSave(s, 1))(t)
	for _, w := range isl.Stages {
		for _, site := range w.Sites {
			if site.State != "built" {
				t.Fatalf("nach Laden: Tiefe %d %s ist %s", w.Biome.Depth, site.Kind, site.State)
			}
		}
	}
	if isl.Stages[0].HubLevel != 5 {
		t.Fatalf("Hub nach Laden: %d", isl.Stages[0].HubLevel)
	}
}

// B-315/AC-02: Gleicher Seed ergibt denselben Stand (bis auf den Zeitstempel savedAt).
func TestVollAusgebautDeterministisch(t *testing.T) {
	f := newFixture()
	f.m.Dev = true
	first := bytes.Clone(f.store.data[need(f.m.FullSave("abc"))(t)])
	f.now = f.now.Add(time.Hour)
	second := f.store.data[need(f.m.FullSave("abc"))(t)]
	strip := func(raw []byte) sim.IslandSave { s := need(sim.ParseIslandSave(raw))(t); s.SavedAt = ""; return s }
	a, b := need(json.Marshal(strip(first)))(t), need(json.Marshal(strip(second)))(t)
	if !bytes.Equal(a, b) {
		t.Fatal("gleicher Seed, anderer Spielstand")
	}
}

func TestVollAusgebautNurImDevMode(t *testing.T) {
	f := newFixture()
	if _, err := f.m.FullSave("abc"); !errors.Is(err, ErrForbidden) {
		t.Fatalf("ohne Dev-Mode: %v", err)
	}
	if _, err := (func() (string, error) { f.m.Dev = true; return f.m.FullSave("") })(); !errors.Is(err, ErrBadRequest) {
		t.Fatalf("leerer Seed: %v", err)
	}
}
