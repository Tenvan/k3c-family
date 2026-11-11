package hud

import (
	"testing"

	"k3c/engine/level"
	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

func endBoss(hp, maxHP float64, phase int) *sim.Enemy {
	return &sim.Enemy{ID: 7, Kind: "crystalHeartWarden", Boss: true, HP: hp, MaxHP: maxHP, Phase: phase}
}

func TestBossLeiste(t *testing.T) {
	goblin := &sim.Enemy{ID: 1, Kind: "goblin", HP: 10, MaxHP: 10}
	b := bossBar("de", []*sim.Enemy{goblin, endBoss(50, 100, 2)})
	if b != (bossView{true, 7, 0.5, "Kristallherz-Wächter", "Phase 2 von 3"}) {
		t.Errorf("Endboss: %+v", b)
	}
	for _, c := range []struct{ hp, maxHP, want float64 }{{0, 100, 0}, {100, 100, 1}, {50, 0, 0}} {
		if got := bossBar("de", []*sim.Enemy{endBoss(c.hp, c.maxHP, 1)}).share; got != c.want {
			t.Errorf("Anteil %v/%v: %v", c.hp, c.maxHP, got)
		}
	}
	mini := &sim.Enemy{ID: 3, Kind: "goblinLeader", Boss: true, HP: 10, MaxHP: 10}
	if got := bossBar("de", []*sim.Enemy{mini, endBoss(1, 1, 1)}).id; got != 7 {
		t.Errorf("Endboss vor Miniboss: %d", got)
	}
	if got := bossBar("de", []*sim.Enemy{mini}); got != (bossView{true, 3, 1, "Goblin-Anführer", ""}) {
		t.Errorf("Miniboss ohne Phase: %+v", got)
	}
	if got := bossBar("de", []*sim.Enemy{{Kind: "neu", Boss: true}}).name; got != "neu" {
		t.Errorf("unbekannter Boss: %q", got)
	}
	if got := bossBar("en", []*sim.Enemy{endBoss(1, 2, 2)}); got.name != "Crystal Heart Warden" || got.phase != "Phase 2 of 3" {
		t.Errorf("englisch: %+v", got)
	}
	if bossBar("de", []*sim.Enemy{goblin}).ok || bossBar("de", nil).ok {
		t.Error("ohne Boss keine Leiste")
	}
	if bossTitle(b) != "Kristallherz-Wächter  ·  Phase 2 von 3" {
		t.Errorf("Titel: %q", bossTitle(b))
	}
}

func TestEventBanner(t *testing.T) {
	ev := func(id string, left float64) sim.ActiveEvent { return sim.ActiveEvent{ID: id, SecondsLeft: left} }
	raid := &sim.Merchant{Resource: "wood", Leaves: 3, Raid: true}
	checkTexts(t, []textCase{
		{"ein Event", eventBanner(newEventsKey("de", []sim.ActiveEvent{ev("fullMoon", 90)}, 1, nil)), "Vollmond  ·  1:30"},
		{"Zeitraffer", eventBanner(newEventsKey("de", []sim.ActiveEvent{ev("bloodMoon", 120)}, 2, nil)), "Blutmond  ·  1:00"},
		{"zwei", eventBanner(newEventsKey("de", []sim.ActiveEvent{ev("fullMoon", 61), ev("bloodMoon", 61)}, 1, nil)),
			"Vollmond  ·  1:01\nBlutmond  ·  1:01"},
		{"Überfall", eventBanner(newEventsKey("de", nil, 1, raid)), "Händler-Überfall"},
		{"unbekannt", eventBanner(newEventsKey("de", []sim.ActiveEvent{ev("neu", 5)}, 1, nil)), "neu  ·  0:05"},
		{"keins", eventBanner(newEventsKey("de", nil, 1, &sim.Merchant{Resource: "wood"})), ""},
	})
}

func radarWorld() *sim.World {
	return &sim.World{
		WidthUnits: 1000, Castle: sim.Castle{X: 500}, Portals: []float64{100, 900},
		Level:   level.Layout{Entities: []level.Entity{{Kind: "tree", X: 40}, {Kind: "exit", X: 1000}}},
		Sites:   []*sim.Site{{Kind: "stairsUp", X: 250}, {Kind: "stairsDown", X: 750}, {Kind: "wall", X: 300}},
		Enemies: []*sim.Enemy{{X: 0}, {X: 1000}, {X: -20}, {X: 1300}, {X: 500}},
		Players: []*sim.Player{{Index: 0, X: 500}, {Index: 1, X: 250, RespawnIn: 3}},
	}
}

func TestRadarMarker(t *testing.T) {
	ms := radarMarkers(nil, radarWorld(), 0)
	pos := func(kind radarKind) []float64 {
		var out []float64
		for _, m := range ms {
			if m.kind == kind {
				out = append(out, m.pos)
			}
		}
		return out
	}
	checks := map[radarKind][]float64{radarCastle: {0.5}, radarPortal: {0.1, 0.9}, radarExit: {1}, radarStairs: {0.25, 0.75},
		radarEnemy: {0, 1, 0, 1, 0.5}, radarPlayer: {0.5, 0.25}}
	for kind, want := range checks {
		if got := pos(kind); !equalF(got, want) {
			t.Errorf("Art %d: %v, erwartet %v", kind, got, want)
		}
	}
	last := ms[len(ms)-2:]
	if last[0].kind != radarPlayer || !last[0].own || last[1].own || !last[1].down || last[1].index != 1 {
		t.Errorf("Monarchen zuletzt, nur der eigene eigen, gefallen markiert: %+v", last)
	}
	if got := radarMarkers(ms, &sim.World{Castle: sim.Castle{X: 5}}, -1); got[0].pos != 0 {
		t.Error("Breite 0 rechnet nicht durch null")
	}
}

func TestRadarAusschnittUndGroesse(t *testing.T) {
	k := view.Camera{X: 500, Scale: 10, Half: 500}
	if from, to := radarView(k, 1000); from != 0.45 || to != 0.55 {
		t.Errorf("Ausschnitt: %v %v", from, to)
	}
	if from, to := radarView(view.Camera{X: 0, Scale: 10, Half: 500}, 1000); from != 0 || to != 0.05 {
		t.Errorf("am Rand begrenzt: %v %v", from, to)
	}
	if w, h := radarSize(1920, 1); w != 640 || h != 20 {
		t.Errorf("1920: %v × %v", w, h)
	}
	if w, _ := radarSize(1000, 1); w != 500 {
		t.Errorf("schmal: halbe Breite, %v", w)
	}
}

func equalF(a, b []float64) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if d := a[i] - b[i]; d > 1e-9 || d < -1e-9 {
			return false
		}
	}
	return true
}
