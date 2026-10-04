package sim

import "testing"

// Skill-Rahmen: Slot-Befehl, Abklingzeit je Spieler, nicht gelernt = nichts, Gating (S1.2a, B-119).

// tankPlayer: ein Spieler, der die vier aktiven Tank-Skills gelernt hat (Slots: taunt, shieldBash, ironWall, lastStand).
// Füller für das Gating sind Passive ohne Einfluss auf Verteidigung (guardian wirkt nur auf Truppen; thickSkin und
// regeneration nur über Step), damit die Zahlen der aktiven Skills ohne Passiv gelten (S1.2c).
func tankPlayer(t *testing.T, w *World) *Player {
	t.Helper()
	p := AddPlayer(w)
	addPoolPoints(w, 7)
	mustLearn(t, w, p, "taunt", "shieldBash", "guardian", "thickSkin", "ironWall", "regeneration", "lastStand")
	return p
}

func stepIdle(w *World, p *Player, ticks int) {
	for range ticks {
		stepSkills(w, p, PlayerCommand{}, dt)
	}
}

func TestSkillsLeerOderNichtGelernt(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	addPoolPoints(w, 3)
	e := spawnEnemy(w, "skeleton", p.X+1)
	for slot := 1; slot <= 4; slot++ { // nichts gelernt, alle Slots leer
		stepSkills(w, p, PlayerCommand{Skill: slot}, dt)
	}
	p.Slots = []string{"taunt", "shieldBash"} // im Slot, aber nicht gelernt
	stepSkills(w, p, PlayerCommand{Skill: 1}, dt)
	stepSkills(w, p, PlayerCommand{Skill: 2}, dt)
	if p.Cooldowns != nil || e.TauntFor != 0 || e.Stun != 0 {
		t.Fatalf("Wirkung ohne gelernten Skill: CD %v, Taunt %v, Stun %v", p.Cooldowns, e.TauntFor, e.Stun)
	}
}

func TestSkillsAbklingzeitBlockiertZweitenEinsatz(t *testing.T) {
	w := quietWorld(t)
	p := tankPlayer(t, w)
	e := spawnEnemy(w, "skeleton", p.X+1)
	stepSkills(w, p, PlayerCommand{Skill: 2}, dt)
	if e.Stun != 2 || p.Cooldowns[1] != 10 {
		t.Fatalf("Shield Bash: Stun %v, CD %v", e.Stun, p.Cooldowns)
	}
	e.Stun = 0
	stepSkills(w, p, PlayerCommand{Skill: 2}, dt)
	stepIdle(w, p, 297) // mit den beiden Befehls-Ticks 299 Ticks < 10 s
	stepSkills(w, p, PlayerCommand{Skill: 2}, dt)
	if e.Stun != 0 {
		t.Fatalf("zweiter Einsatz in der Abklingzeit: Stun %v, CD %v", e.Stun, p.Cooldowns[1])
	}
	stepIdle(w, p, 2)
	stepSkills(w, p, PlayerCommand{Skill: 2}, dt)
	if e.Stun != 2 || p.Cooldowns[1] != 10 {
		t.Fatalf("nach 10 s kein Einsatz: Stun %v, CD %v", e.Stun, p.Cooldowns[1])
	}
}

func TestSkillsZweiSpielerGetrennt(t *testing.T) {
	w := quietWorld(t)
	p0, p1 := AddPlayer(w), AddPlayer(w)
	addPoolPoints(w, 7)
	for _, p := range []*Player{p0, p1} {
		mustLearn(t, w, p, "taunt", "shieldBash", "armorAura", "thickSkin", "ironWall")
	}
	Step(w, []PlayerCommand{{Skill: 3}, {}}, dt)
	if p0.Shield != 500 || p0.Cooldowns[2] != 30 || p1.Shield != 0 || p1.Cooldowns != nil {
		t.Fatalf("Spieler 0 wirkt Iron Wall: P0 %v/%v, P1 %v/%v", p0.Shield, p0.Cooldowns, p1.Shield, p1.Cooldowns)
	}
	Step(w, []PlayerCommand{{}, {Skill: 3}}, dt)
	if p1.Shield != 500 || p1.Cooldowns[2] != 30 || p0.Cooldowns[2] >= 30 {
		t.Fatalf("Spieler 1 wirkt Iron Wall: P1 %v/%v, P0 CD %v", p1.Shield, p1.Cooldowns, p0.Cooldowns)
	}
}

func TestSkillsGatingOhnePunkte(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	addPoolPoints(w, 1)
	if err := LearnSkill(w, p, "ironWall"); err == nil {
		t.Fatal("Tier 3 ohne Punkte der Linie gelernt")
	}
	addPoolPoints(w, 9)
	mustLearn(t, w, p, "taunt")
	if err := LearnSkill(w, p, "ironWall"); err == nil {
		t.Fatal("Tier 3 mit 1 Tank-Skill gelernt (verlangt 4)")
	}
	p.Slots = append(p.Slots, "ironWall") // erzwungen im Slot: ohne Lernen keine Wirkung
	stepSkills(w, p, PlayerCommand{Skill: 2}, dt)
	if p.Shield != 0 || p.Cooldowns != nil {
		t.Fatalf("Iron Wall ohne Gating: Schild %v, CD %v", p.Shield, p.Cooldowns)
	}
}
