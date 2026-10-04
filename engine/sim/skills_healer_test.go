package sim

import (
	"math"
	"testing"
)

// Heiler-Linie: Wirkung, Fläche, Reichweite und Abklingzeit laut monarch.json (S1.2b, B-119).

// healerSkills: die sieben Skills für alle vier aktiven Heiler-Skills (Slots: heal, groupHeal, divineShield,
// resurrection).
var healerSkills = []string{"heal", "groupHeal", "healingAura", "blessings", "divineShield", "holyGround", "resurrection"}

func healerPlayer(t *testing.T, w *World) *Player {
	t.Helper()
	p := AddPlayer(w)
	addPoolPoints(w, 7)
	mustLearn(t, w, p, healerSkills...)
	return p
}

func troopAt(w *World, x, hp float64) *Troop {
	tr := &Troop{ID: w.newID(), Kind: "archer", X: x, AnchorX: x, HP: hp, MaxHP: 10}
	w.Troops = append(w.Troops, tr)
	return tr
}

func TestHealerHeal(t *testing.T) {
	w := quietWorld(t)
	p := healerPlayer(t, w)
	q, r := AddPlayer(w), AddPlayer(w)
	q.X, r.X = p.X+6, p.X+7 // r ist schwächer, aber außer Reichweite
	p.HP, q.HP, r.HP = 90, 30, 10
	stepSkills(w, p, PlayerCommand{Skill: 1}, dt)
	if q.HP != 80 || p.HP != 90 || r.HP != 10 || p.Cooldowns[0] != 10 {
		t.Fatalf("Heal: Wirkender %v, 6u %v, 7u %v, CD %v", p.HP, q.HP, r.HP, p.Cooldowns)
	}
	stepIdle(w, p, 301) // 10 s
	q.HP = 70
	stepSkills(w, p, PlayerCommand{Skill: 1}, dt)
	if q.HP != q.MaxHP {
		t.Fatalf("Heal über MaxHP: %v", q.HP)
	}
}

func TestHealerGroupHeal(t *testing.T) {
	w := quietWorld(t)
	p := healerPlayer(t, w)
	q, r := AddPlayer(w), AddPlayer(w)
	q.X, r.X = p.X+10, p.X-11
	in, out := troopAt(w, p.X-10, 2), troopAt(w, p.X+11, 2)
	p.HP, q.HP, r.HP = 90, 50, 50
	stepSkills(w, p, PlayerCommand{Skill: 2}, dt)
	if p.HP != 100 || q.HP != 80 || r.HP != 50 || in.HP != 10 || out.HP != 2 || p.Cooldowns[1] != 30 {
		t.Fatalf("Group Heal: %v/%v/%v, Truppen %v/%v, CD %v", p.HP, q.HP, r.HP, in.HP, out.HP, p.Cooldowns)
	}
}

func TestHealerDivineShield(t *testing.T) {
	w := quietWorld(t)
	p := healerPlayer(t, w)
	stepSkills(w, p, PlayerCommand{Skill: 3}, dt)
	if p.Shield != 100 || p.ShieldFor != 10 || p.Cooldowns[2] != 20 {
		t.Fatalf("Divine Shield: %v für %v s, CD %v", p.Shield, p.ShieldFor, p.Cooldowns)
	}
	applyDamage(w, p.ID, 105) // 100 nach Verteidigung: ganz absorbiert
	applyDamage(w, p.ID, 15)  // Schild leer: 10 gehen durch
	if p.Shield != 0 || p.HP != 90 {
		t.Fatalf("Schild %v, HP %v", p.Shield, p.HP)
	}
}

func TestHealerResurrection(t *testing.T) {
	w := quietWorld(t)
	p := healerPlayer(t, w)
	stepSkills(w, p, PlayerCommand{Skill: 4}, dt) // (c) keine Truppe im Radius
	if p.Cooldowns != nil {
		t.Fatalf("Resurrection ohne Truppe: CD %v", p.Cooldowns)
	}
	weak, strong, far := troopAt(w, p.X+15, 2), troopAt(w, p.X-15, 8), troopAt(w, p.X+16, 2)
	stepSkills(w, p, PlayerCommand{Skill: 4}, dt)
	if weak.HP != 5 || strong.HP != 8 || far.HP != 2 || p.Cooldowns[3] != 180 {
		t.Fatalf("Resurrection: %v/%v/%v, CD %v", weak.HP, strong.HP, far.HP, p.Cooldowns)
	}
}

// TestHealerAbklingzeit (b): zweiter Einsatz in der Abklingzeit ohne Wirkung, nach cooldown wieder möglich.
func TestHealerAbklingzeit(t *testing.T) {
	for slot := 1; slot <= 4; slot++ {
		w := quietWorld(t)
		p := healerPlayer(t, w)
		tr := troopAt(w, p.X+1, 1)
		s, _ := skillByID(p.Slots[slot-1])
		reset := func() { p.HP, p.Shield, p.ShieldFor, tr.HP = 10, 0, 0, 1 }
		effect := func() float64 { return p.HP + p.Shield + tr.HP }
		reset()
		stepSkills(w, p, PlayerCommand{Skill: slot}, dt)
		if effect() == 11 ||p.Cooldowns[slot-1] != s.Cooldown {
			t.Fatalf("%s: Wirkung %v, CD %v", s.ID, effect(), p.Cooldowns)
		}
		reset()
		stepSkills(w, p, PlayerCommand{Skill: slot}, dt)
		if effect() != 11 {
			t.Fatalf("%s: zweiter Einsatz in der Abklingzeit wirkt (%v)", s.ID, effect())
		}
		stepIdle(w, p, int(math.Ceil(s.Cooldown/dt)))
		stepSkills(w, p, PlayerCommand{Skill: slot}, dt)
		if effect() == 11 || p.Cooldowns[slot-1] != s.Cooldown {
			t.Fatalf("%s: nach %v s kein neuer Einsatz, CD %v", s.ID, s.Cooldown, p.Cooldowns)
		}
	}
}

// TestHealerHeiltZauberer (d): Zauberer (Spieler 0) und Heiler (Spieler 1) im selben Kampf über Step.
func TestHealerHeiltZauberer(t *testing.T) {
	w := quietWorld(t)
	mage, healer := AddPlayer(w), AddPlayer(w)
	addPoolPoints(w, 7)
	mustLearn(t, w, mage, mageSkills...)
	mustLearn(t, w, healer, healerSkills...)
	mage.X = w.HubX + 40
	healer.X = mage.X + 2
	e := toughEnemy(w, mage.X+6)
	mage.HP = 40
	Step(w, []PlayerCommand{{Skill: 1}, {Skill: 1}}, dt)
	if lost(e) != 30 || mage.HP != 90 || healer.HP != 100 {
		t.Fatalf("Fireball %v, Zauberer %v, Heiler %v", lost(e), mage.HP, healer.HP)
	}
	if mage.Cooldowns[0] != 8 || healer.Cooldowns[0] != 10 || mage.Cooldowns[1] != 0 {
		t.Fatalf("Abklingzeiten Zauberer %v, Heiler %v", mage.Cooldowns, healer.Cooldowns)
	}
}
