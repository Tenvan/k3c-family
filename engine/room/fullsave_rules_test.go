package room

import (
	"os"
	"path/filepath"
	"slices"
	"testing"

	"k3c/engine/sim"
	"k3c/engine/store"
)

// hubRules: kleinste Hub-Stufe je Gebäudeart (sites und islandSites aus data/hub.json) und die Hub-Stufe der Tore.
func hubRules(t *testing.T) (map[string]int, int) {
	t.Helper()
	type site struct {
		Kind     string `json:"kind"`
		HubLevel int    `json:"hubLevel"`
	}
	var h struct {
		Sites       []site `json:"sites"`
		IslandSites []site `json:"islandSites"`
		WallLines   struct {
			GateHubLevel int `json:"gateHubLevel"`
		} `json:"wallLines"`
	}
	if err := readJSON("hub.json", &h); err != nil {
		t.Fatal(err)
	}
	need := map[string]int{}
	for _, s := range append(h.Sites, h.IslandSites...) {
		if l, ok := need[s.Kind]; !ok || max(1, s.HubLevel) < l {
			need[s.Kind] = max(1, s.HubLevel)
		}
	}
	return need, h.WallLines.GateHubLevel
}

// B-500/AC-01, B-425/AC-01: Der Voll-Stand hat die Tiefen der Welt-Datei; kein gebauter Platz hat eine höhere Stufe als
// die Hub-Stufe seiner Inselstufe, Gebäude mit Hub-Voraussetzung und Tore stehen nur, wo die Hub-Stufe sie zulässt.
func TestVollAusgebautHaeltAusbauregeln(t *testing.T) {
	s := need(fullIsland("abc", "2026-10-25T12:00:00.000Z"))(t)
	var depths []int
	for _, h := range s.Stages {
		depths = append(depths, h.Depth)
	}
	if want := sim.StandardWorld().Islands[0].Depths; !slices.Equal(depths, want) {
		t.Fatalf("Tiefen %v, Welt-Datei %v", depths, want)
	}
	minHub, gateHub := hubRules(t)
	for _, h := range s.Stages {
		hub := max(1, h.HubLevel)
		for _, site := range h.Sites {
			if site.State != "built" {
				continue
			}
			if site.Level > hub {
				t.Errorf("Tiefe %d, Hub %d: %s@%v hat Stufe %d", h.Depth, hub, site.Kind, site.X, site.Level)
			}
			if l, ok := minHub[site.Kind]; ok && l > hub {
				t.Errorf("Tiefe %d, Hub %d: %s gebaut, braucht Hub %d", h.Depth, hub, site.Kind, l)
			}
			if site.Kind == "gate" && hub < gateHub {
				t.Errorf("Tiefe %d, Hub %d: Tor gebaut, braucht Hub %d", h.Depth, hub, gateHub)
			}
		}
	}
}

// B-426/AC-02: FullSave schreibt voll-<seed> über einen echten store.Saves (Datei im TempDir), der Stand lädt.
func TestVollAusgebautMitEchtemStore(t *testing.T) {
	saves := &store.Saves{Dir: filepath.Join(t.TempDir(), "saves")}
	m := NewManager(saves)
	m.Dev = true
	name := need(m.FullSave("abc"))(t)
	if _, err := os.Stat(filepath.Join(saves.Dir, name+".json")); err != nil {
		t.Fatalf("Datei fehlt: %v", err)
	}
	s := need(sim.ParseIslandSave(need(saves.Load(name))(t)))(t)
	if len(s.Stages) != len(sim.StandardWorld().Islands[0].Depths) || s.Stages[0].HubLevel != 5 {
		t.Fatalf("Stufen %d, Hub der ersten Stufe %d", len(s.Stages), s.Stages[0].HubLevel)
	}
	need(sim.FromIslandSave(s, 1))(t)
}
