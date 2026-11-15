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
	probe(p)
	applyDamage(w, p.ID, 300) // der Schild fängt den Treffer ohne Goldverlust ab
	if p.Shield != 200 || p.Gold != probeGold || p.Crown != nil {
		t.Fatalf("Schild %v, Gold %v", p.Shield, p.Gold)
	}
	applyDamage(w, p.ID, 300) // der Rest-Schild nimmt auch den Überschuss
	applyDamage(w, p.ID, 300) // Schild leer: der Treffer kostet eine Münze
	if p.Shield != 0 || p.Gold != probeGold-monarch.HitCoins {
		t.Fatalf("Schild %v, Gold %v", p.Shield, p.Gold)
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
	p.Gold = monarch.HitCoins // die letzte Münze
	applyDamage(w, p.ID, 1000)
	if p.Crown != nil || p.Gold != monarch.HitCoins || p.LastStandFor != 0 {
		t.Fatalf("Last Stand: Krone %v, Gold %d, Last Stand %v", p.Crown, p.Gold, p.LastStandFor)
	}
	applyDamage(w, p.ID, 1000) // ohne Last Stand kostet der Treffer die letzte Münze
	applyDamage(w, p.ID, 1000) // ohne Gold fällt die Krone
	if p.Crown == nil {
		t.Fatalf("nach Ablauf bleibt die Krone: Gold %d", p.Gold)
	}
}

// TestTankSchildNurGroesser (B-220/AC-01): Divine Shield (100) ersetzt einen laufenden Schild nur, wenn er größer ist;
// die Abklingzeit startet trotzdem.
func TestTankSchildNurGroesser(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	divine := monarch.Skills["divineShield"].Effect
	p.Shield, p.ShieldFor = 400, 5
	if !castShield(w, p, divine) || p.Shield != 400 || p.ShieldFor != 5 {
		t.Fatalf("laufender 400er-Schild: %v für %v s", p.Shield, p.ShieldFor)
	}
	p.Shield = 50
	if !castShield(w, p, divine) || p.Shield != 100 || p.ShieldFor != 10 {
		t.Fatalf("Rest 50: %v für %v s", p.Shield, p.ShieldFor)
	}
}
