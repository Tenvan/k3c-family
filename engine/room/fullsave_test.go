package room

import (
	"bytes"
	"encoding/json"
	"errors"
	"strings"
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
	f.m.Dev = true
	// Review SV1.5: der Name voll-<seed> muss für store.Saves gültig sein (Kleinbuchstaben, Ziffern, -, höchstens 32 Zeichen).
	for _, seed := range []string{"", "ABC", "a_b", "../x", strings.Repeat("a", 28)} {
		if _, err := f.m.FullSave(seed); !errors.Is(err, ErrBadRequest) {
			t.Errorf("Seed %q: %v, erwartet ErrBadRequest", seed, err)
		}
	}
	if _, err := f.m.FullSave(strings.Repeat("a", 27)); err != nil {
		t.Errorf("27 Zeichen: %v", err)
	}
}

// B-424/AC-01: Ein offener leerer Raum mit dem Namen wird geschlossen; der neue Start lädt den frischen Stand.
func TestVollAusgebautSchliesstLeerenRaum(t *testing.T) {
	f := newFixture()
	f.m.Dev = true
	name := need(f.m.FullSave("abc"))(t)
	x := &peer{}
	old := need(f.m.Create("xbox", x, name, false, 0, []int{0}, Options{}))(t)
	old.mu.Lock()
	old.isl.Stages[0].HubLevel = 1 // „gespielter“ Zustand, den der neue Stand ersetzen muss
	old.mu.Unlock()
	old.Leave("xbox", x)
	need(f.m.FullSave("abc"))(t)
	if len(f.m.Rooms()) != 0 || !old.Closed() {
		t.Fatalf("alter Raum noch offen: %v", roomNames(f.m))
	}
	if s := need(sim.ParseIslandSave(f.store.data[name]))(t); s.Stages[0].HubLevel != 5 {
		t.Fatalf("Stand im Store: Hub %d, erwartet 5", s.Stages[0].HubLevel)
	}
	r := need(f.m.Create("xbox", x, name, false, 0, []int{0}, Options{}))(t)
	if r == old || r.isl.Stages[0].HubLevel != 5 {
		t.Fatalf("neuer Start: Hub %d, alter Raum %v", r.isl.Stages[0].HubLevel, r == old)
	}
}

// B-424/AC-02: Mit verbundenem Gerät lehnt FullSave ab und ändert nichts.
func TestVollAusgebautLehntBeiVerbundenemGeraetAb(t *testing.T) {
	f := newFixture()
	f.m.Dev = true
	name := need(f.m.FullSave("abc"))(t)
	r := need(f.m.Create("xbox", &peer{}, name, false, 0, []int{0}, Options{}))(t)
	waitSaved(r)
	before := bytes.Clone(f.store.data[name])
	if _, err := f.m.FullSave("abc"); !errors.Is(err, ErrSaveInUse) {
		t.Fatalf("mit verbundenem Gerät: %v, erwartet ErrSaveInUse", err)
	}
	if !bytes.Equal(before, f.store.data[name]) || len(f.m.Rooms()) != 1 || r.Closed() {
		t.Fatal("Stand oder Raum verändert")
	}
}
