package room

import (
	"encoding/json"
	"reflect"
	"strings"
	"testing"

	"k3c/engine/level"
	"k3c/engine/sim"
)

// WZ7.1 (B-460/AC-03, AC-08): Vorschau-Welt für den Welt-Betrachter.

func previewOpts() PreviewOptions {
	return PreviewOptions{Seed: "abc", Source: PreviewSource{Biome: "forest"}}
}

func mustPreview(t *testing.T, o PreviewOptions) *sim.World {
	t.Helper()
	w, err := BuildPreview(o)
	if err != nil {
		t.Fatal(err)
	}
	return w
}

func jsonOf(t *testing.T, v any) string {
	t.Helper()
	raw, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	return string(raw)
}

func builtOf(w *sim.World, kind string) (n int) {
	for _, s := range w.Sites {
		if s.Kind == kind && s.State == "built" {
			n++
		}
	}
	return n
}

func TestBuildPreviewLehntUngueltigeOptionenAb(t *testing.T) {
	cases := map[string]func(o *PreviewOptions){
		"Seed leer":         func(o *PreviewOptions) { o.Seed = "" },
		"Seed zu lang":      func(o *PreviewOptions) { o.Seed = strings.Repeat("a", 65) },
		"Quelle fehlt":      func(o *PreviewOptions) { o.Source = PreviewSource{} },
		"Biom unbekannt":    func(o *PreviewOptions) { o.Source.Biome = "nirgendwo" },
		"Biom mit Pfad":     func(o *PreviewOptions) { o.Source.Biome = "../hub" },
		"Hub-Stufe -1":      func(o *PreviewOptions) { o.HubLevel = -1 },
		"Hub-Stufe zu hoch": func(o *PreviewOptions) { o.HubLevel = hubLevels(t) + 1 },
		"Phase dawn":        func(o *PreviewOptions) { o.Phase = "dawn" },
		"Phase unbekannt":   func(o *PreviewOptions) { o.Phase = "mittag" },
		"Fortschritt -0.1":  func(o *PreviewOptions) { o.Progress = -0.1 },
		"Fortschritt 1":     func(o *PreviewOptions) { o.Progress = 1 },
		"Fortschritt 1.5":   func(o *PreviewOptions) { o.Progress = 1.5 },
	}
	for name, mutate := range cases {
		o := previewOpts()
		mutate(&o)
		if _, err := BuildPreview(o); err == nil {
			t.Errorf("%s: kein Fehler", name)
		}
	}
}

// hubLevels ist die Zahl der Hub-Stufen aus data/hub.json.
func hubLevels(t *testing.T) int {
	t.Helper()
	d, err := fullData()
	if err != nil {
		t.Fatal(err)
	}
	return d.hub
}

func TestBuildPreviewHatKeineBuergerKeineGegner(t *testing.T) {
	biome, err := level.LoadBiome("forest")
	if err != nil {
		t.Fatal(err)
	}
	if raw, err := sim.CreateWorld(biome, "abc", sim.Options{CycleSpeed: 1}); err != nil || len(raw.Troops) == 0 {
		t.Fatalf("Ausgangswelt hat keine Truppen (%v), der Test prüfte sonst ins Leere", err)
	}
	w := mustPreview(t, previewOpts())
	if len(w.Troops) != 0 || len(w.Enemies) != 0 || len(w.SpawnQueue) != 0 || len(w.Players) != 0 {
		t.Errorf("Truppen %d, Gegner %d, Warteschlange %d, Spieler %d", len(w.Troops), len(w.Enemies), len(w.SpawnQueue), len(w.Players))
	}
	js := jsonOf(t, w)
	for _, key := range []string{`"troops":[]`, `"enemies":[]`, `"spawnQueue":[]`} {
		if !strings.Contains(js, key) {
			t.Errorf("JSON enthält %s nicht (Listen müssen [] sein, nicht null)", key)
		}
	}
	if len(w.Nodes) == 0 || len(w.Sites) == 0 {
		t.Errorf("Umgebung fehlt: Ressourcen %d, Plätze %d", len(w.Nodes), len(w.Sites))
	}
}

// builtKinds zählt die gebauten Plätze je Art.
func builtKinds(w *sim.World) map[string]int {
	out := map[string]int{}
	for _, s := range w.Sites {
		if s.State == "built" {
			out[s.Kind]++
		}
	}
	return out
}

// Feste Erwartungen aus data/hub.json (healer ab Hub 3, armory ab Hub 4); Standard (0) ist die höchste Stufe.
// Dazu je Stufe alle Arten wie bei sim.PreviewHubLevel auf einer frischen Welt (genau die freigeschalteten Plätze).
func TestBuildPreviewHubStufen(t *testing.T) {
	top := hubLevels(t)
	biome, err := level.LoadBiome("forest")
	if err != nil {
		t.Fatal(err)
	}
	for level := 0; level <= top; level++ {
		expected := level
		if level == 0 {
			expected = top
		}
		o := previewOpts()
		o.HubLevel = level
		w := mustPreview(t, o)
		ref, err := sim.CreateWorld(biome, "abc", sim.Options{CycleSpeed: 1})
		if err != nil {
			t.Fatal(err)
		}
		if err := sim.PreviewHubLevel(ref, expected); err != nil {
			t.Fatal(err)
		}
		if w.HubLevel != expected || !reflect.DeepEqual(builtKinds(w), builtKinds(ref)) {
			t.Errorf("Hub %d: HubLevel %d, gebaut %v; erwartet %d und %v", level, w.HubLevel, builtKinds(w), expected, builtKinds(ref))
		}
		healer, armory := 0, 0
		if expected >= 3 {
			healer = 1
		}
		if expected >= 4 {
			armory = 1
		}
		if builtOf(w, "healer") != healer || builtOf(w, "armory") != armory {
			t.Errorf("Hub %d: healer %d (erwartet %d), armory %d (erwartet %d)", level, builtOf(w, "healer"), healer, builtOf(w, "armory"), armory)
		}
	}
}

func TestBuildPreviewTageszeit(t *testing.T) {
	for _, phase := range []string{"", "day", "dusk", "night"} {
		o := previewOpts()
		o.Phase, o.Progress = phase, 0.5
		w := mustPreview(t, o)
		want := phase
		if phase == "" {
			want = "day"
		}
		if w.Cycle.Phase != want || w.Cycle.Progress != 0.5 {
			t.Errorf("Phase %q: Zyklus %+v", phase, w.Cycle)
		}
	}
}

func TestBuildPreviewGebaeudeAuswahl(t *testing.T) {
	o := previewOpts()
	all := mustPreview(t, o)
	var off *sim.Site
	for _, s := range all.Sites {
		if s.Kind == "wall" && s.State == "built" {
			off = s
			break
		}
	}
	if off == nil {
		t.Fatal("kein gebauter Mauerplatz im Vergleichsstand")
	}
	o.Buildings = map[string]bool{PreviewSiteKey(off.Kind, off.X): false, "gibt@es-nicht": false, PreviewSiteKey("farm", 123456): true}
	part := mustPreview(t, o)
	if len(part.Sites) != len(all.Sites)-1 {
		t.Fatalf("Plätze %d, erwartet %d (einer abgewählt, unbekannte Schlüssel ignoriert)", len(part.Sites), len(all.Sites)-1)
	}
	for _, s := range part.Sites {
		if PreviewSiteKey(s.Kind, s.X) == PreviewSiteKey(off.Kind, off.X) {
			t.Error("abgewählter Platz ist noch da")
		}
	}
	rest := map[string]int{}
	for _, s := range all.Sites {
		rest[PreviewSiteKey(s.Kind, s.X)] = s.Level
	}
	for _, s := range part.Sites {
		if rest[PreviewSiteKey(s.Kind, s.X)] != s.Level {
			t.Errorf("%s bei %v: Stufe %d statt %d (Abwahl darf die Stufen der übrigen nicht ändern)", s.Kind, s.X, s.Level, rest[PreviewSiteKey(s.Kind, s.X)])
		}
	}
}

func TestBuildPreviewIstDeterministischOhneGlobalenZustand(t *testing.T) {
	a := previewOpts()
	a.HubLevel, a.Phase, a.Progress = 3, "dusk", 0.25
	b := previewOpts()
	b.Seed, b.HubLevel, b.Phase = "anderer", 5, "night"
	first := jsonOf(t, mustPreview(t, a))
	_ = mustPreview(t, b)
	if second := jsonOf(t, mustPreview(t, a)); first != second {
		t.Error("gleiche Optionen liefern nach einem anderen Aufruf eine andere Welt")
	}
	if jsonOf(t, mustPreview(t, a)) == jsonOf(t, mustPreview(t, b)) {
		t.Error("verschiedene Optionen liefern dieselbe Welt")
	}
}

func TestBuildPreviewNullwertVerhaeltSichWieStandard(t *testing.T) {
	zero := mustPreview(t, previewOpts())
	explicit := previewOpts()
	explicit.HubLevel, explicit.Phase, explicit.Buildings = hubLevels(t), "day", map[string]bool{}
	if jsonOf(t, zero) != jsonOf(t, mustPreview(t, explicit)) {
		t.Error("Nullwerte (Hub 0, Phase leer, keine Auswahl) entsprechen nicht Hub 5, day, leere Auswahl")
	}
}

func TestBuildPreviewSites(t *testing.T) {
	base, err := BuildPreview(PreviewOptions{Seed: "k3c", Source: PreviewSource{Biome: "forest"}})
	if err != nil {
		t.Fatal(err)
	}
	var key string
	for _, s := range base.Sites {
		if s.Kind == "wall" && s.State == "built" {
			key = PreviewSiteKey(s.Kind, s.X)
			break
		}
	}
	w, err := BuildPreview(PreviewOptions{Seed: "k3c", Source: PreviewSource{Biome: "forest"}, Sites: map[string]sim.SiteLook{key: {Unbuilt: true}, "nix@1": {Level: 3}}})
	if err != nil {
		t.Fatal(err)
	}
	for _, s := range w.Sites {
		if PreviewSiteKey(s.Kind, s.X) == key && s.State == "built" {
			t.Error("Platz blieb gebaut")
		}
	}
	if _, err := BuildPreview(PreviewOptions{Seed: "k3c", Source: PreviewSource{Biome: "forest"}, Sites: map[string]sim.SiteLook{key: {Level: 99}}}); err == nil {
		t.Error("ungültige Stufe wurde angenommen")
	}
}
