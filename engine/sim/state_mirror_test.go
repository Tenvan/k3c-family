package sim

import (
	"encoding/json"
	"strings"
	"testing"
)

// Zustands-Spiegel (K4.1a, B-383/AC-01): Boss-Phase, Warnkreis, Event mit Restzeit, Inselwechsel im JSON der Welt.

func mirrorJSON(t *testing.T, v any) string {
	t.Helper()
	b, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	return string(b)
}

// mirrorSteps tickt die Insel `seconds` lang (Spiegel setzt StepIsland) und hält die Spieler am Leben.
func mirrorSteps(isl *Island, seconds float64) {
	for tm := 0.0; tm < seconds-1e-9; tm += dt {
		protect(isl.Stages[0])
		StepIsland(isl, nil, dt)
	}
}

func TestMirrorEndbossPhaseUndWarnkreis(t *testing.T) {
	isl, w, x := endIsland(t, 2)
	w.Players[0].X, w.Players[1].X = x+4, x+6
	if s := mirrorJSON(t, w.Enemies); s != "[]" {
		t.Fatal("Felder ohne Boss")
	}
	mirrorSteps(isl, dt)
	boss := warden(w)
	if boss == nil || boss.Phase != 1 || boss.Warn != nil {
		t.Fatalf("Phase 1 (Beschwörung): Phase %v, Warn %v", boss, boss.Warn)
	}
	w.Enemies = []*Enemy{boss}
	boss.HP = boss.MaxHP * 0.59
	mirrorSteps(isl, dt)
	if boss.Phase != 2 || boss.Warn == nil || boss.Warn.R != 5 || boss.Warn.X != unitX(boss.X) || boss.Warn.In != boss.AoeIn {
		t.Fatalf("Phase 2: Phase %d, Warn %+v", boss.Phase, boss.Warn)
	}
	before := boss.Warn.In
	mirrorSteps(isl, 1)
	if boss.Warn.In >= before && before > 1 {
		t.Errorf("Warn.in sinkt nicht: %v -> %v", before, boss.Warn.In)
	}
	if s := mirrorJSON(t, boss); !strings.Contains(s, `"phase":2`) || !strings.Contains(s, `"warn":{"x":`) {
		t.Errorf("JSON: %s", s)
	}
}

func TestMirrorEvent(t *testing.T) {
	isl := moonIsland(t, 7)
	if s := mirrorJSON(t, isl.Stages[0]); strings.Contains(s, `"events":[{"id"`) || strings.Contains(s, `"nightEvents"`) {
		t.Fatal("Event vor der Nacht")
	}
	runIsland(isl, nil, 2)
	w := isl.Stages[0]
	if len(w.NightEvents) != 1 || w.NightEvents[0].ID != "fullMoon" || w.NightEvents[0].SecondsLeft != w.Cycle.SecondsLeft {
		t.Fatalf("Vollmond: %+v, Zyklus %+v", w.NightEvents, w.Cycle)
	}
	left := w.NightEvents[0].SecondsLeft
	runIsland(isl, nil, 3)
	if w.NightEvents[0].SecondsLeft >= left || !strings.Contains(mirrorJSON(t, w), `"nightEvents":[{"id":"fullMoon","secondsLeft":`) {
		t.Errorf("Restzeit sinkt nicht: %v -> %v", left, w.NightEvents[0].SecondsLeft)
	}
	if cave := isl.Stages[1]; len(cave.NightEvents) != 1 {
		t.Errorf("Höhle: %+v", cave.NightEvents)
	}
	both := moonIsland(t, 91)
	runIsland(both, nil, 2)
	if got := both.Stages[0].NightEvents; len(got) != 2 {
		t.Errorf("Nacht 91: %+v", got)
	}
	day := moonIsland(t, 8)
	runIsland(day, nil, 2)
	if len(day.Stages[0].NightEvents) != 0 {
		t.Error("Event in Nacht 8")
	}
}

func TestMirrorInselwechsel(t *testing.T) {
	withTestIslands(t)
	isl, _ := gateIsland(t, 2)
	isl.EndbossDefeated = false
	stepSeconds(isl, 1)
	if s := mirrorJSON(t, isl.Stages[1]); strings.Contains(s, "islandSwitch") {
		t.Fatal("Wechsel vor dem Endboss")
	}
	isl.EndbossDefeated = true
	stepSeconds(isl, 1)
	for i, w := range isl.Stages {
		if w.IslandSwitch == nil || !w.IslandSwitch.Open || w.IslandSwitch.Ready {
			t.Fatalf("Stufe %d: %+v", i, w.IslandSwitch)
		}
	}
	if p := isl.Stages[1].IslandSwitch.Progress; p <= 0 || p >= 1 {
		t.Errorf("Fortschritt %v", p)
	}
	stepSeconds(isl, 3)
	if sw := isl.Stages[1].IslandSwitch; !sw.Ready || sw.Progress != 1 {
		t.Errorf("bereit: %+v", sw)
	}
	if s := mirrorJSON(t, isl.Stages[0]); !strings.Contains(s, `"islandSwitch":{"open":true,"progress":1,"ready":true}`) {
		t.Errorf("JSON: %s", s)
	}
}
