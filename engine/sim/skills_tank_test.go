package sim

import "testing"

// Tank-Linie: Wirkung, Reichweite und Abklingzeit laut monarch.json (S1.2a, B-119).

func TestTankTaunt(t *testing.T) {
	w := quietWorld(t)
	p0 := tankPlayer(t, w)
	p1 := AddPlayer(w)
	near, far := spawnEnemy(w, "skeleton", p0.X+10), spawnEnemy(w, "skeleton", p0.X-11)
	stepSkills(w, p0, PlayerCommand{Skill: 1}, dt)
	if near.TauntID != p0.ID || near.TauntFor != 4 || far.TauntFor != 0 || p0.Cooldowns[0] != 15 {
		t.Fatalf("Taunt: 10u %v/%v, 11u %v, CD %v", near.TauntID, near.TauntFor, far.TauntFor, p0.Cooldowns)
	}
	// In Reichweite beider Monarchen: ohne Taunt das nähere Ziel (Spieler 1), mit Taunt der Wirkende.
	near.X, p1.X = p0.X+1.2, p0.X+1
	dir := sign(w.HubX - near.X)
	if c := chooseTarget(w, near, dir, nil); c == nil || c.id != p0.ID {
		t.Fatalf("verspotteter Gegner zielt auf %+v, erwartet Spieler %d", c, p0.ID)
	}
	for range 121 { // 4 s
		stunned(near, dt)
	}
	if c := chooseTarget(w, near, dir, nil); c == nil || c.id != p1.ID || near.TauntID != 0 {
		t.Fatalf("nach 4 s noch verspottet: Ziel %+v, TauntID %d", c, near.TauntID)
	}
}

func TestTankShieldBash(t *testing.T) {
	w := quietWorld(t)
	p := tankPlayer(t, w)
	p.X = w.HubX + 20
	far := spawnEnemy(w, "skeleton", p.X+2.5)
	stepSkills(w, p, PlayerCommand{Skill: 2}, dt)
	if far.Stun != 0 || p.Cooldowns != nil {
		t.Fatalf("Gegner in 2,5u betäubt oder Abklingzeit ohne Treffer: Stun %v, CD %v", far.Stun, p.Cooldowns)
	}
	e := spawnEnemy(w, "skeleton", p.X+2)
	stepSkills(w, p, PlayerCommand{Skill: 2}, dt)
	if e.Stun != 2 || far.Stun != 0 || p.Cooldowns[1] != 10 {
		t.Fatalf("Shield Bash in 2u: Stun %v/%v, CD %v", e.Stun, far.Stun, p.Cooldowns)
	}
	p.X = w.HubX + 40 // aus der Reichweite: der Gegner läuft nur
	x := e.X
	for range 59 {
		stepEnemies(w, dt)
	}
	if e.X != x || e.Stun <= 0 {
		t.Fatalf("betäubter Gegner bewegt sich: %v → %v, Stun %v", x, e.X, e.Stun)
	}
	for range 5 {
		stepEnemies(w, dt)
	}
	if e.X == x || e.Stun != 0 {
		t.Fatalf("nach 2 s weiter regungslos: X %v, Stun %v", e.X, e.Stun)
	}
}

func TestTankIronWall(t *testing.T) {
	w := quietWorld(t)
	p := tankPlayer(t, w)
	stepSkills(w, p, PlayerCommand{Skill: 3}, dt)
	if p.Shield != 500 || p.ShieldFor != 10 || p.Cooldowns[2] != 30 {
		t.Fatalf("Iron Wall: Schild %v für %v s, CD %v", p.Shield, p.ShieldFor, p.Cooldowns)
	}
	applyDamage(w, p.ID, 555) // 550 nach Verteidigung: 500 absorbiert, 50 gehen durch
	if p.Shield != 0 || p.HP != 50 {
		t.Fatalf("Schild %v, HP %v", p.Shield, p.HP)
	}
	w2 := quietWorld(t)
	q := tankPlayer(t, w2)
	stepSkills(w2, q, PlayerCommand{Skill: 3}, dt)
	stepIdle(w2, q, 301) // 10 s
	if q.Shield != 0 || q.ShieldFor != 0 {
		t.Fatalf("Schild nach 10 s: %v / %v", q.Shield, q.ShieldFor)
	}
}

func TestTankLastStand(t *testing.T) {
	w := quietWorld(t)
	p := tankPlayer(t, w)
	stepSkills(w, p, PlayerCommand{Skill: 4}, dt)
	if p.LastStandFor != 10 || p.Cooldowns[3] != 120 {
		t.Fatalf("Last Stand: %v s, CD %v", p.LastStandFor, p.Cooldowns)
	}
	applyDamage(w, p.ID, 1000)
	applyDamage(w, p.ID, 1000)
	if p.HP != 1 || !isAlive(p) {
		t.Fatalf("tödliche Treffer während Last Stand: HP %v, Respawn %v", p.HP, p.RespawnIn)
	}
	stepIdle(w, p, 301) // 10 s
	applyDamage(w, p.ID, 1000)
	if isAlive(p) || p.LastStandFor != 0 {
		t.Fatalf("nach Ablauf überlebt: HP %v, Last Stand %v", p.HP, p.LastStandFor)
	}
}
