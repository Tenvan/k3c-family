package sim

import (
	"encoding/json"
	"math"
	"testing"

	"k3c/data"
)

// Standard-Reittier (S1.3, B-152/AC-02, docs/rules/monarch.md § 7): erwartete Werte aus monarch.json › mount.

// (a) Jeder Monarch hat beim Beitritt das Reittier aus den Daten: Welt mit 2 Spielern und Insel mit 2 Stufen.
func TestMountBeimBeitritt(t *testing.T) {
	w := quietWorld(t)
	isl := mustIsland(t, "reittier-a", []int{0, 1})
	players := []*Player{AddPlayer(w), AddPlayer(w), AddIslandPlayer(isl, 0), AddIslandPlayer(isl, 1)}
	for _, p := range players {
		if MountOf(p) != monarch.Mount {
			t.Errorf("Spieler %d: Reittier %+v, erwartet %+v", p.Index, MountOf(p), monarch.Mount)
		}
	}
	if monarch.Mount.ID == "" || monarch.Mount.SpeedFactor <= 0 || monarch.Mount.SprintFactor <= 0 {
		t.Fatalf("monarch.json › mount unvollständig: %+v", monarch.Mount)
	}
}

// mountSpeeds: zwei Spieler laufen 5 s nach rechts, Spieler 1 sprintet; Ergebnis VX beider.
func mountSpeeds(t *testing.T) (walk, sprint float64) {
	t.Helper()
	w := quietWorld(t)
	p0, p1 := AddPlayer(w), AddPlayer(w)
	cmds := []PlayerCommand{{MoveX: 1}, {MoveX: 1, Sprint: true}}
	for range 150 {
		Step(w, cmds, dt)
	}
	return p0.VX, p1.VX
}

func wantSpeeds() (walk, sprint float64) {
	m := monarch.Mount
	walk = monarch.Base.Speed * m.SpeedFactor
	return walk, walk * monarch.SprintMultiplier * m.SprintFactor
}

// (b) Tempo = Basis × Faktor, Sprint = Basis × Faktor × sprintMultiplier × Sprintfaktor.
func TestMountGeschwindigkeit(t *testing.T) {
	walk, sprint := mountSpeeds(t)
	ww, ws := wantSpeeds()
	if math.Abs(walk-ww) > 1e-6 || math.Abs(sprint-ws) > 1e-6 {
		t.Fatalf("Tempo %v (erwartet %v), Sprint %v (erwartet %v)", walk, ww, sprint, ws)
	}
}

// (c) Ein anderer Faktor in einer Kopie der Daten ändert beide Geschwindigkeiten ohne Codeänderung.
func TestMountFaktorAusDaten(t *testing.T) {
	saved := monarch
	t.Cleanup(func() { monarch = saved })
	baseWalk, baseSprint := mountSpeeds(t)
	monarch.Mount.SpeedFactor, monarch.Mount.SprintFactor = 1.5, 1.2
	walk, sprint := mountSpeeds(t)
	ww, ws := wantSpeeds()
	if math.Abs(walk-ww) > 1e-6 || math.Abs(sprint-ws) > 1e-6 || walk == baseWalk || sprint == baseSprint {
		t.Fatalf("Faktor 1,5/1,2: Tempo %v (erwartet %v), Sprint %v (erwartet %v)", walk, ww, sprint, ws)
	}
}

// (d) Zweimal derselbe Seed und dieselben Eingaben ergeben denselben Weltzustand.
func TestMountDeterministisch(t *testing.T) {
	run := func() string {
		w := quietWorld(t)
		AddPlayer(w)
		AddPlayer(w)
		cmds := []PlayerCommand{{MoveX: 1, Sprint: true}, {MoveX: -1}}
		for range 300 {
			Step(w, cmds, dt)
		}
		b, err := json.Marshal(w)
		if err != nil {
			t.Fatal(err)
		}
		return string(b)
	}
	if a, b := run(), run(); a != b {
		t.Fatal("gleicher Seed, gleiche Eingaben: Weltzustand weicht ab")
	}
}

// (e) Datenprüfung: mount.sprite ist ein Tier aus sprites.json › mounts (ein unbekannter Schlüssel lässt diesen Test
// scheitern, nicht das Spiel).
func TestMountSpriteBekannt(t *testing.T) {
	raw, err := data.Files.ReadFile("sprites.json")
	if err != nil {
		t.Fatal(err)
	}
	var sprites struct{ Mounts map[string]json.RawMessage }
	if err := json.Unmarshal(raw, &sprites); err != nil {
		t.Fatal(err)
	}
	if _, ok := sprites.Mounts[monarch.Mount.Sprite]; !ok {
		t.Fatalf("monarch.json › mount.sprite %q fehlt in sprites.json › mounts", monarch.Mount.Sprite)
	}
}

// (f) Ein Stufenwechsel ändert das Reittier nicht.
func TestMountNachStufenwechsel(t *testing.T) {
	isl := mustIsland(t, "reittier-f", []int{0, 1})
	a := AddIslandPlayer(isl, 0)
	before := MountOf(a)
	standFor(isl, 2.5, map[*Player]float64{a: exitX(t, isl.Stages[0])})
	if isl.StageOf(0) != 1 {
		t.Fatalf("Spieler 0 muss in Stufe 1 sein, ist in %d", isl.StageOf(0))
	}
	if got := MountOf(isl.Stages[1].Players[0]); got != before || got != monarch.Mount {
		t.Fatalf("Reittier nach Wechsel %+v, vorher %+v", got, before)
	}
}
