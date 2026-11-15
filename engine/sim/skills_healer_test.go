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
	weak, weaker, far := troopAt(w, p.X+6, 3), troopAt(w, p.X-5, 2), troopAt(w, p.X+7, 1) // far: schwächer, aber außer Reichweite
	stepSkills(w, p, PlayerCommand{Skill: 1}, dt)
	if weaker.HP != 10 || weak.HP != 3 || far.HP != 1 || p.Cooldowns[0] != 10 {
		t.Fatalf("Heal: schwächste Truppe %v, andere %v, außer Reichweite %v, CD %v", weaker.HP, weak.HP, far.HP, p.Cooldowns)
	}
	stepIdle(w, p, 301) // 10 s
	weak.HP = 8
	stepSkills(w, p, PlayerCommand{Skill: 1}, dt)
	if weak.HP != weak.MaxHP {
		t.Fatalf("Heal über MaxHP: %v", weak.HP)
	}
}

func TestHealerGroupHeal(t *testing.T) {
	w := quietWorld(t)
	p := healerPlayer(t, w)
	in, out := troopAt(w, p.X-10, 2), troopAt(w, p.X+11, 2)
	stepSkills(w, p, PlayerCommand{Skill: 2}, dt)
	if in.HP != 10 || out.HP != 2 || p.Cooldowns[1] != 30 {
		t.Fatalf("Group Heal: Truppen %v/%v, CD %v", in.HP, out.HP, p.Cooldowns)
	}
}

// Divine Shield fängt Treffer ohne Goldverlust ab, bis die Schild-Punkte aufgebraucht sind (kampf.md § 5).
func TestHealerDivineShield(t *testing.T) {
	w := quietWorld(t)
	p := healerPlayer(t, w)
	probe(p)
	stepSkills(w, p, PlayerCommand{Skill: 3}, dt)
	if p.Shield != 100 || p.ShieldFor != 10 || p.Cooldowns[2] != 20 {
		t.Fatalf("Divine Shield: %v für %v s, CD %v", p.Shield, p.ShieldFor, p.Cooldowns)
	}
	applyDamage(w, p.ID, 60)
	applyDamage(w, p.ID, 60) // zweiter Treffer: Schild fast leer, auch der Rest wird abgefangen
	if p.Shield != 0 || p.Gold != probeGold {
		t.Fatalf("Schild %v, Gold %v", p.Shield, p.Gold)
	}
	applyDamage(w, p.ID, 15) // Schild leer: eine Münze
	if p.Gold != probeGold-monarch.HitCoins {
		t.Fatalf("Gold %v nach Treffer ohne Schild", p.Gold)
	}
}

// Resurrection bringt gefallene Kronen im Radius sofort zu ihrem Monarchen zurück (Umdeutung, kampf.md § 5).
func TestHealerResurrection(t *testing.T) {
	w := quietWorld(t)
	p := healerPlayer(t, w)
	q, r := AddPlayer(w), AddPlayer(w)
	stepSkills(w, p, PlayerCommand{Skill: 4}, dt) // (c) keine Krone im Radius
	if p.Cooldowns != nil && p.Cooldowns[3] != 0 {
		t.Fatalf("Resurrection ohne gefallene Krone: CD %v", p.Cooldowns)
	}
	q.X, r.X = p.X+15, p.X+16
	fell(q)
	fell(r)
	stepSkills(w, p, PlayerCommand{Skill: 4}, dt)
	if q.Crown != nil || r.Crown == nil || p.Cooldowns[3] != 180 {
		t.Fatalf("Resurrection: Krone Q %v, R (außer Reichweite) %v, CD %v", q.Crown, r.Crown, p.Cooldowns)
	}
}

// TestHealerAbklingzeit (b): zweiter Einsatz in der Abklingzeit ohne Wirkung, nach cooldown wieder möglich.
func TestHealerAbklingzeit(t *testing.T) {
	for slot := 1; slot <= 4; slot++ {
		w := quietWorld(t)
		p := healerPlayer(t, w)
		q := AddPlayer(w)
		q.X = p.X + 1
		tr := troopAt(w, p.X+1, 1)
		s, _ := defaultValues().skillByID(p.Slots[slot-1])
		reset := func() { p.Shield, p.ShieldFor, tr.HP = 0, 0, 1; fell(q) }
		effect := func() float64 { return p.Shield + tr.HP + float64(b2i(q.Crown == nil)) } // ohne Wirkung: 1
		reset()
		stepSkills(w, p, PlayerCommand{Skill: slot}, dt)
		if effect() == 1 || p.Cooldowns[slot-1] != s.Cooldown {
			t.Fatalf("%s: Wirkung %v, CD %v", s.ID, effect(), p.Cooldowns)
		}
		reset()
		stepSkills(w, p, PlayerCommand{Skill: slot}, dt)
		if effect() != 1 {
			t.Fatalf("%s: zweiter Einsatz in der Abklingzeit wirkt (%v)", s.ID, effect())
		}
		stepIdle(w, p, int(math.Ceil(s.Cooldown/dt)))
		stepSkills(w, p, PlayerCommand{Skill: slot}, dt)
		if effect() == 1 || p.Cooldowns[slot-1] != s.Cooldown {
			t.Fatalf("%s: nach %v s kein neuer Einsatz, CD %v", s.ID, s.Cooldown, p.Cooldowns)
		}
	}
}

// TestHealerHeiltZauberer (d): Zauberer (Spieler 0) und Heiler (Spieler 1) im selben Kampf über Step. Mit Passiven:
// Fireball 30 × (1 + Arcane Power 0,2 + Blessings 0,1) = 39; die Heiler-Linie heilt Truppen, nicht den Zauberer.
func TestHealerHeiltZauberer(t *testing.T) {
	w := quietWorld(t)
	mage, healer := AddPlayer(w), AddPlayer(w)
	addPoolPoints(w, 7)
	mustLearn(t, w, mage, mageSkills...)
	mustLearn(t, w, healer, healerSkills...)
	mage.X = w.HubX + 40
	healer.X = mage.X + 2
	e := toughEnemy(w, mage.X+6)
	tr := troopAt(w, mage.X, 2)
	Step(w, []PlayerCommand{{Skill: 1}, {Skill: 1}}, dt)
	if !near(lost(e), 39) || tr.HP != tr.MaxHP {
		t.Fatalf("Fireball %v, Truppe %v (Heal des Heilers)", lost(e), tr.HP)
	}
	if mage.Cooldowns[0] != 8*cooldownMult || healer.Cooldowns[0] != 10 || mage.Cooldowns[1] != 0 {
		t.Fatalf("Abklingzeiten Zauberer %v, Heiler %v", mage.Cooldowns, healer.Cooldowns)
	}
}
