package props

import (
	"slices"
	"testing"

	"k3c/engine/sim"
)

// Fälle aus src/scenes/render/buildingSprites.test.ts, hubGrowth.test.ts, backWallRules.test.ts, damageRules.test.ts
// und scaffoldView.test.ts.

func TestSiteLookBaustellenOhneGrafik(t *testing.T) {
	for _, state := range []string{"unpaid", "waitingMaterial", "waitingWorker"} {
		if l := siteLook("workshop", state, 3); l.ok() {
			t.Errorf("%s: %v", state, l)
		}
	}
	if l := siteLook("wall", "built", 0); l != (look{bg + "mauer", "stand"}) {
		t.Errorf("Mauer ohne Stufe: %v", l)
	}
	if l := siteLook("stairsUp", "built", 4); l != (look{bg + "treppe-hoch", "stand"}) {
		t.Errorf("Treppe hoch: %v", l)
	}
	if l := siteLook("castle", "built", 1); l.ok() {
		t.Errorf("unbekannte Art: %v", l)
	}
}

func TestSiteLookMauerUndTurmJeStufe(t *testing.T) {
	for _, c := range []struct{ kind, id string }{{"wall", "mauer"}, {"tower", "turm"}} {
		for s := 1; s <= 5; s++ {
			want := look{bg + c.id, stageAnim(s)}
			if l := siteLook(c.kind, "built", s); l != want {
				t.Errorf("%s %d: %v, erwartet %v", c.kind, s, l, want)
			}
		}
		if l := siteLook(c.kind, "built", 9); l.anim != "stand" {
			t.Errorf("%s 9: Grundform erwartet, %v", c.kind, l)
		}
	}
}

func TestSiteLookHubGebaeudeUndSchiff(t *testing.T) {
	for kind, id := range hubBuildings {
		if got := siteLook(kind, "built", 6); got != (look{id, "stand.s5"}) {
			t.Errorf("%s 6: höchste Stufe erwartet, %v", kind, got)
		}
		if got := siteLook(kind, "built", 0); got.anim != "stand" {
			t.Errorf("%s 0: %v", kind, got)
		}
		want := len(buildings[kind].Levels) == 0
		if isHubBuilding(kind) != want {
			t.Errorf("%s: Hub-Gebäude %v, erwartet %v (B-569)", kind, isHubBuilding(kind), want)
		}
	}
	if siteStage("workshop", 0, 3) != 3 || siteStage("storage", 2, 5) != 2 || siteStage("wall", 0, 4) != 1 {
		t.Error("Stufe der Grafik: Werkstatt je Hub-Stufe, Lager und Mauer je Platz")
	}
	for n, anim := range map[int]string{1: "stand", 2: "stand.s2", 3: "stand.s3", 4: "stand.s3"} {
		if l := siteLook("ship", "built", n); l != (look{bg + "schiff", anim}) {
			t.Errorf("Schiff %d: %v", n, l)
		}
	}
}

func TestHubLookWachstumsreihe(t *testing.T) {
	seen := map[hubLook]bool{}
	for lv := 1; lv <= 5; lv++ {
		for step := 1; step <= 3; step++ {
			seen[hubLookOf(lv, step, "")] = true
		}
	}
	if len(seen) != 15 {
		t.Errorf("15 Kombinationen, %d verschiedene Looks", len(seen))
	}
	cases := []struct {
		lv, step int
		state    string
		want     hubLook
	}{
		{1, 2, "", hubLook{main: look{bg + "lagerfeuer", "stand.s2"}}},
		{0, 9, "", hubLook{main: look{bg + "lagerfeuer", "stand.s3"}}},
		{3, 1, "", hubLook{main: look{bg + "burg", "stand.s3"}}},
		{3, 3, "", hubLook{main: look{bg + "burg", "stand.s3"}, overlay: look{bg + "burg-ausbau", "stand.s3"}}},
		{3, 3, "waitingWorker", hubLook{main: look{bg + "burg", "stand.s3"}, scaffold: true}},
		{7, 0, "", hubLook{main: look{bg + "burg", "stand.s5"}}},
	}
	for _, c := range cases {
		if got := hubLookOf(c.lv, c.step, c.state); got != c.want {
			t.Errorf("Hub %d/%d %q: %+v, erwartet %+v", c.lv, c.step, c.state, got, c.want)
		}
	}
}

func wallAt(id int, x float64, level int, state string) *sim.Site {
	return &sim.Site{ID: id, Kind: "wall", X: x, Level: level, State: state}
}

func TestBackWalls(t *testing.T) {
	cases := []struct {
		sites []*sim.Site
		want  []backWall
	}{
		{[]*sim.Site{wallAt(1, 480, 3, "built"), wallAt(2, 440, 1, "built"), wallAt(3, 400, 2, "unpaid")},
			[]backWall{{2, 440, 480, 1}}},
		{[]*sim.Site{wallAt(1, 520, 2, "built"), wallAt(2, 560, 4, "built"), wallAt(3, 600, 5, "built"),
			wallAt(4, 480, 1, "built"), wallAt(5, 440, 2, "built")},
			[]backWall{{5, 440, 480, 2}, {2, 520, 560, 4}, {3, 560, 600, 5}}},
		{[]*sim.Site{wallAt(1, 520, 1, "built"), wallAt(2, 560, 1, "waitingWorker"), wallAt(3, 600, 1, "built")}, nil},
		{[]*sim.Site{wallAt(1, 520, 0, "built"), wallAt(2, 560, 0, "built"), {ID: 9, Kind: "tower", X: 540, State: "built"}},
			[]backWall{{2, 520, 560, 1}}},
	}
	for i, c := range cases {
		got, _ := backWalls(nil, nil, c.sites, 500)
		if !slices.Equal(got, c.want) {
			t.Errorf("Fall %d: %v, erwartet %v", i, got, c.want)
		}
	}
	if backWallLook(6).ok() || backWallLook(2) != (look{bg + "hintergrundmauer", "stand.s2"}) {
		t.Error("Hintergrundmauer je Stufe 1–5")
	}
}

func TestDamageStageUndRuine(t *testing.T) {
	for hp, want := range map[float64]int{90: 0, 89: 1, 60: 1, 59: 2, 30: 2, 29: 3, 1: 3} {
		if got := damageStage("built", hp, 90); got != want {
			t.Errorf("HP %v: Stufe %d, erwartet %d", hp, got, want)
		}
	}
	if damageStage("unpaid", 0, 90) != 0 || damageStage("built", 0, 0) != 0 {
		t.Error("nicht gebaut oder ohne MaxHP: keine Überlagerung")
	}
	if !showRubble(true, "built", 23, 90) || showRubble(true, "built", 45, 90) || showRubble(false, "built", 23, 90) ||
		showRubble(true, "unpaid", 0, 90) {
		t.Error("Trümmer nur an einer gebauten Ruine unter 50 %")
	}
}

func TestGeruestUndBauschild(t *testing.T) {
	want := map[string]string{"wall": "1", "tower": "3", "nursery": "4", "stairsUp": "4", "stairsDown": "4", "workshop": "6",
		"storage": "6", "barracks": "6", "tavern": "6", "healer": "6", "smithy": "6", "armory": "6", "ship": "6"}
	for kind, c := range want {
		if l := scaffoldLook(kind); l.asset != bg+"geruest-"+c {
			t.Errorf("%s: %v", kind, l)
		}
	}
	for _, state := range []string{"waitingMaterial", "waitingWorker"} {
		if siteMarker(state, true) != markScaffold {
			t.Errorf("%s: Gerüst", state)
		}
	}
	if siteMarker("unpaid", true) != markSign || siteMarker("unpaid", false) != markNone || siteMarker("built", true) != markNone {
		t.Error("Bauschild nur am bezahlbaren ungebauten Platz")
	}
	if signVariant("wall") != 1 || signVariant("stairsDown") != 2 || signVariant("tavern") != 0 {
		t.Error("Varianten des Bauschilds")
	}
}

func TestStageAnim(t *testing.T) {
	for n, want := range map[int]string{-1: "stand", 0: "stand", 1: "stand", 2: "stand.s2", 5: "stand.s5", 9: "stand.s5"} {
		if got := stageAnim(n); got != want {
			t.Errorf("%d: %s", n, got)
		}
	}
}
