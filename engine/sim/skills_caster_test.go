package sim

import (
	"math"
	"testing"
)

// Zauberer-Linie: Wirkung, Fläche, Reichweite und Abklingzeit laut monarch.json (S1.2b, B-119).

// mageSkills: die sieben Skills für alle vier aktiven Zauber (Slots: fireball, iceWall, lightningStorm, meteor).
var mageSkills = []string{"fireball", "iceWall", "arcanePower", "spellEcho", "lightningStorm", "frostArmor", "meteor"}

// casterPlayer: ein Zauberer abseits der Burg, der alle vier aktiven Zauber gelernt hat.
func casterPlayer(t *testing.T, w *World) *Player {
	t.Helper()
	p := AddPlayer(w)
	addPoolPoints(w, 7)
	mustLearn(t, w, p, mageSkills...)
	p.X = w.HubX + 40
	return p
}

// toughEnemy überlebt auch Meteor (700 HP).
func toughEnemy(w *World, x float64) *Enemy { return spawnScaled(w, "skeleton", x, 10, 1) }

func lost(e *Enemy) float64 { return e.MaxHP - e.HP }

func TestCasterFireballUndMeteor(t *testing.T) {
	for _, c := range []struct {
		slot           int
		radius, damage float64
	}{{1, 3, 30}, {4, 15, 200}} {
		w := quietWorld(t)
		p := casterPlayer(t, w)
		target := toughEnemy(w, p.X+5)
		in, out := toughEnemy(w, target.X+c.radius), toughEnemy(w, target.X+c.radius+1)
		stepSkills(w, p, PlayerCommand{Skill: c.slot}, dt)
		if lost(target) != c.damage || lost(in) != c.damage || lost(out) != 0 {
			t.Fatalf("Slot %d: Ziel %v, %vu %v, %vu %v", c.slot, lost(target), c.radius, lost(in), c.radius+1, lost(out))
		}
	}
}

func TestCasterIceWall(t *testing.T) {
	w := quietWorld(t)
	p := casterPlayer(t, w)
	target := toughEnemy(w, p.X+3)
	in, out := toughEnemy(w, target.X+3), toughEnemy(w, target.X+4)
	stepSkills(w, p, PlayerCommand{Skill: 2}, dt)
	if target.Slow != 0.5 || in.SlowFor != 5 || out.SlowFor != 0 {
		t.Fatalf("Ice Wall: Ziel %v, 3u %v s, 4u %v s", target.Slow, in.SlowFor, out.SlowFor)
	}
	p.X = w.HubX + 80 // Gegner laufen nur
	for i := range 160 {
		x0, x1 := in.X, out.X
		stepEnemies(w, dt)
		ratio := (in.X - x0) / (out.X - x1)
		if i < 145 && math.Abs(ratio-0.5) > 1e-9 || i > 151 && math.Abs(ratio-1) > 1e-9 {
			t.Fatalf("Tick %d: Geschwindigkeit verlangsamt/normal = %v", i, ratio)
		}
	}
	if in.Slow != 0 || in.SlowFor != 0 {
		t.Fatalf("Verlangsamung nach 5 s: %v / %v", in.Slow, in.SlowFor)
	}
}

func TestCasterLightningStorm(t *testing.T) {
	w := quietWorld(t)
	p := casterPlayer(t, w)
	target := toughEnemy(w, p.X+3)
	in, out := toughEnemy(w, target.X+5), toughEnemy(w, target.X+6)
	stepSkills(w, p, PlayerCommand{Skill: 3}, dt)
	if len(w.Storms) != 1 || w.Storms[0].Owner != p.ID {
		t.Fatalf("Sturm: %+v", w.Storms)
	}
	for range 75 { // 2,5 s
		stepStorms(w, dt)
	}
	if math.Abs(lost(in)-25) > 1e-6 || lost(out) != 0 {
		t.Fatalf("nach 2,5 s: 5u %v, 6u %v", lost(in), lost(out))
	}
	for range 125 { // weit über 5 s hinaus
		stepStorms(w, dt)
	}
	if math.Abs(lost(target)-50) > 1e-6 || math.Abs(lost(in)-50) > 1e-6 || lost(out) != 0 || len(w.Storms) != 0 {
		t.Fatalf("nach 5 s: Ziel %v, 5u %v, 6u %v, Stürme %d", lost(target), lost(in), lost(out), len(w.Storms))
	}
}

// TestCasterAbklingzeitUndOhneZiel: je Zauber (c) kein Gegner in Range: keine Wirkung, keine Abklingzeit;
// (b) zweiter Einsatz in der Abklingzeit ohne Wirkung, nach cooldown wieder möglich.
func TestCasterAbklingzeitUndOhneZiel(t *testing.T) {
	for slot := 1; slot <= 4; slot++ {
		w := quietWorld(t)
		p := casterPlayer(t, w)
		s, _ := skillByID(p.Slots[slot-1])
		far := toughEnemy(w, p.X+s.Effect.Range+0.5)
		effect := func() float64 { return lost(far) + far.SlowFor + float64(len(w.Storms)) }
		stepSkills(w, p, PlayerCommand{Skill: slot}, dt)
		if effect() != 0 || p.Cooldowns != nil {
			t.Fatalf("%s ohne Ziel: Wirkung %v, CD %v", s.ID, effect(), p.Cooldowns)
		}
		far.X = p.X + 1
		stepSkills(w, p, PlayerCommand{Skill: slot}, dt)
		first := effect()
		if first == 0 || p.Cooldowns[slot-1] != s.Cooldown {
			t.Fatalf("%s: Wirkung %v, CD %v (erwartet %v)", s.ID, first, p.Cooldowns, s.Cooldown)
		}
		far.SlowFor, w.Storms = 0, nil
		before := effect()
		stepSkills(w, p, PlayerCommand{Skill: slot}, dt)
		if effect() != before {
			t.Fatalf("%s: zweiter Einsatz in der Abklingzeit wirkt", s.ID)
		}
		stepIdle(w, p, int(math.Ceil(s.Cooldown/dt)))
		stepSkills(w, p, PlayerCommand{Skill: slot}, dt)
		if p.Cooldowns[slot-1] != s.Cooldown {
			t.Fatalf("%s: nach %v s kein neuer Einsatz, CD %v", s.ID, s.Cooldown, p.Cooldowns)
		}
	}
}

func TestCasterGatingMeteor(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	addPoolPoints(w, 7)
	mustLearn(t, w, p, mageSkills[:5]...) // fünf Skills der Linie, Tier 4 verlangt sechs
	if err := LearnSkill(w, p, "meteor"); err == nil {
		t.Fatal("Meteor ohne sechs Skills der Linie gelernt")
	}
	p.Slots = append(p.Slots, "meteor") // erzwungen im Slot, aber nicht gelernt
	e := toughEnemy(w, p.X+1)
	stepSkills(w, p, PlayerCommand{Skill: 4}, dt)
	if lost(e) != 0 || p.Cooldowns != nil {
		t.Fatalf("Meteor ohne Lernen: Schaden %v, CD %v", lost(e), p.Cooldowns)
	}
}
