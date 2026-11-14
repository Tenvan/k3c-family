package actors

import (
	"slices"
	"testing"
)

// Fälle aus src/scenes/render/mountGait.test.ts; im Stand steht das Rig in der Gangart stand (statt Schritt-Bild 0).
func TestMountGait(t *testing.T) {
	cases := []struct {
		vx, ref float64
		gait    string
		moving  bool
	}{
		{0, 5, gaitStand, false},
		{1, 5, gaitWalk, true},
		{-3, 5, gaitTrot, true},
		{5, 5, gaitGallop, true},
		{2, 0, gaitGallop, true}, // ohne Referenz Galopp
		{0.05, 5, gaitStand, false},
	}
	for _, c := range cases {
		if g, m := MountGait(c.vx, c.ref); g != c.gait || m != c.moving {
			t.Errorf("MountGait(%v, %v) = %s %v, erwartet %s %v", c.vx, c.ref, g, m, c.gait, c.moving)
		}
	}
}

// Fehlt dem Reittier eine Gangart (Elefant ohne Galopp), gilt die nächst langsamere (mountedAnim.ts: Rückfall).
func TestFallbackGait(t *testing.T) {
	elefant := func(g string) bool { return slices.Contains([]string{gaitStand, gaitWalk, gaitTrot}, g) }
	if g := FallbackGait(gaitGallop, elefant); g != gaitTrot {
		t.Errorf("Elefant Galopp → %s", g)
	}
	if g := FallbackGait(gaitWalk, elefant); g != gaitWalk {
		t.Errorf("Elefant Schritt → %s", g)
	}
	if g := FallbackGait("flug", elefant); g != "flug" {
		t.Errorf("unbekannte Gangart → %s", g)
	}
}

// Fälle aus src/scenes/render/monarchAnim.test.ts, beritten wie mountView.ts: Tod und Respawn zu Fuß (das Rig hat
// death und respawn), Treffer und Münze-Geben zeigt der Reiter nicht.
func TestMonarchWant(t *testing.T) {
	all := func(string) bool { return true }
	alive := MonarchFrame{Known: true}
	dead := MonarchFrame{Known: true, RespawnIn: 3}
	cases := []struct {
		name      string
		prev, cur MonarchFrame
		want      Want
	}{
		{"steht", alive, alive, Want{Anim: gaitStand, Gait: gaitStand}},
		{"erstes Bild", MonarchFrame{}, alive, Want{Anim: gaitStand, Gait: gaitStand}},
		{"läuft", alive, MonarchFrame{Known: true, VX: 3}, Want{Anim: gaitTrot, Gait: gaitTrot}},
		{"rennt nach links", alive, MonarchFrame{Known: true, VX: -5}, Want{Anim: gaitGallop, Gait: gaitGallop}},
		{"stirbt", alive, dead, Want{Anim: animDeath, Once: true, Hold: true}},
		{"bleibt tot", dead, dead, Want{Anim: animDeath, Once: true, Hold: true}},
		{"Respawn", dead, alive, Want{Anim: animRespawn, Once: true, Restart: true}},
	}
	for _, c := range cases {
		if got := MonarchWant(c.prev, c.cur, 5, all); got != c.want {
			t.Errorf("%s: %+v, erwartet %+v", c.name, got, c.want)
		}
	}
}

// Next: dieselbe Animation läuft weiter, ein neuer Schlag beginnt neu, eine laufende Einmal-Animation geht einer
// Schleife vor, der Tod geht allem vor und hält.
func TestNext(t *testing.T) {
	run := Pose{Anim: animRun, Start: 1}
	if p, ch := Next(run, Want{Anim: animRun}, 5); ch || p != run {
		t.Errorf("Lauf beginnt neu: %+v", p)
	}
	hit, ch := Next(run, Want{Anim: animAttack, Once: true, Restart: true}, 5)
	if !ch || hit.Start != 5 || hit.Anim != animAttack {
		t.Fatalf("Schlag: %+v", hit)
	}
	hit.Dur = 1
	if p, ch := Next(hit, Want{Anim: animIdle}, 5.5); ch || p != hit {
		t.Errorf("Schleife unterbricht den Schlag: %+v", p)
	}
	if p, ch := Next(hit, Want{Anim: animIdle}, 6.1); !ch || p.Anim != animIdle {
		t.Errorf("nach dem Schlag keine Ruhe: %+v", p)
	}
	if p, ch := Next(hit, Want{Anim: animAttack, Once: true, Restart: true}, 5.5); !ch || p.Start != 5.5 {
		t.Errorf("neuer Schlag beginnt nicht neu: %+v", p)
	}
}

// Der Tod geht einem laufenden Schlag vor, hält und beginnt nicht neu.
func TestNextDeathHolds(t *testing.T) {
	hit := Pose{Anim: animAttack, Start: 5, Dur: 1}
	death, _ := Next(hit, Want{Anim: animDeath, Once: true, Hold: true}, 5.2)
	death.Dur = 1
	if death.Anim != animDeath || death.Running(9) {
		t.Errorf("Tod: %+v", death)
	}
	if p, ch := Next(death, Want{Anim: animDeath, Once: true, Hold: true}, 9); ch || p != death {
		t.Errorf("Tod beginnt neu: %+v", p)
	}
}

func TestAnimTimeAndSheetFrame(t *testing.T) {
	if got := AnimTime(Pose{Start: 2}, 3, 0.5); got != 1.5 {
		t.Errorf("Schleife: %v", got)
	}
	if got := AnimTime(Pose{Start: 2, Dur: 1, Hold: true}, 10, 0.5); got >= 1 || got < 0.99 {
		t.Errorf("Halten: %v", got)
	}
	if got := AnimTime(Pose{Start: 2, Dur: 1}, 2.25, 0.5); got != 0.25 {
		t.Errorf("Einmal: %v", got)
	}
	if got := AnimTime(Pose{Start: 2}, 1, 0); got != 0 {
		t.Errorf("vor dem Beginn: %v", got)
	}
	if SheetFrame(1.1, 8, 8, false) != 0 || SheetFrame(0.5, 8, 8, false) != 4 || SheetFrame(3, 8, 8, true) != 7 {
		t.Error("SheetFrame")
	}
	if SheetFrame(1, 0, 8, false) != 0 || SheetFrame(1, 8, 0, false) != 0 {
		t.Error("SheetFrame ohne Bilder oder Bildrate")
	}
}

// Fälle aus animate in worldRenderer.ts: Bewegung über MOVING_PX läuft und dreht, ein steigender Cooldown schlägt.
func TestFigureWant(t *testing.T) {
	cases := []struct {
		name string
		m    Motion
		want Want
		dir  int
	}{
		{"neu", Motion{X: 3}, Want{Anim: animIdle}, 0},
		{"steht", Motion{Known: true, PrevX: 3, X: 3}, Want{Anim: animIdle}, 0},
		{"zittert", Motion{Known: true, PrevX: 3, X: 3.01}, Want{Anim: animIdle}, 0},
		{"läuft links", Motion{Known: true, PrevX: 3, X: 2.9}, Want{Anim: animRun}, -1},
		{"schlägt", Motion{Known: true, PrevX: 3, X: 3, PrevCD: 0, CD: 1, AttackCooldown: true}, Want{Anim: animAttack, Once: true, Restart: true}, 0},
		{"Cooldown sinkt", Motion{Known: true, PrevX: 3, X: 3, PrevCD: 1, CD: 0.9, AttackCooldown: true}, Want{Anim: animIdle}, 0},
		{"wartet nur", Motion{Known: true, PrevX: 3, X: 3, PrevCD: 0, CD: 2}, Want{Anim: animIdle}, 0},
	}
	for _, c := range cases {
		if w, d := FigureWant(c.m); w != c.want || d != c.dir {
			t.Errorf("%s: %+v %d, erwartet %+v %d", c.name, w, d, c.want, c.dir)
		}
	}
}

func TestTroopWant(t *testing.T) {
	still := Motion{Known: true, PrevX: 3, X: 3, PrevCD: 0, CD: 2}
	if w, d := TroopWant("vagrant", still, false, 1); w.Anim != animIdle || d != 0 {
		t.Errorf("Landstreicher wartet: %+v %d", w, d)
	}
	if w, _ := TroopWant("warrior", still, false, 0); w.Anim != animAttack || !w.Once {
		t.Errorf("Krieger schlägt nicht: %+v", w)
	}
	if w, d := TroopWant("archer", Motion{Known: true, X: 3, PrevX: 3}, false, -1); w.Anim != animIdle || d != -1 {
		t.Errorf("Bogenschütze schaut nicht zum Gegner: %+v %d", w, d)
	}
	if w, _ := TroopWant("peasant", Motion{Known: true, X: 3, PrevX: 3}, true, 0); w.Anim != animAttack || w.Once {
		t.Errorf("Bauer arbeitet nicht in Schleife: %+v", w)
	}
	if w, d := TroopWant("peasant", Motion{Known: true, X: 3.5, PrevX: 3}, true, 0); w.Anim != animRun || d != 1 {
		t.Errorf("laufender Bauer: %+v %d", w, d)
	}
}
