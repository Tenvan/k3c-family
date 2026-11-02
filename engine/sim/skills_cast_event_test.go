package sim

import "testing"

// B-674/AC-01: Aktive Skills melden skillCast mit Spieler, Skill, Wirkort und Radius; Flächenzauber am Ziel, ohne Ziel
// kein Ereignis (nur castFailed). Zwei Spieler melden getrennt.
func TestSkillCastWirkortUndRadius(t *testing.T) {
	w := quietWorld(t)
	p := casterPlayer(t, w)
	target := toughEnemy(w, p.X+8)
	stepSkills(w, p, PlayerCommand{Skill: 1}, dt) // fireball: range 10, radius 3
	evs := eventsOfType(w, "skillCast")
	if len(evs) != 1 {
		t.Fatalf("fireball: %d skillCast, erwartet 1", len(evs))
	}
	ev := evs[0]
	if ev["from"] != p.ID || ev["skill"] != "fireball" || ev["x"] != unitX(target.X) || ev["radius"] != 3.0 {
		t.Fatalf("fireball: %v, Ziel bei %v", ev, unitX(target.X))
	}
}

func TestSkillCastOhneZielKeinEreignis(t *testing.T) {
	w := quietWorld(t)
	p := casterPlayer(t, w)
	toughEnemy(w, p.X+16) // außerhalb von meteor (range 15)
	stepSkills(w, p, PlayerCommand{Skill: 4}, dt)
	if n := len(eventsOfType(w, "skillCast")); n != 0 {
		t.Fatalf("meteor ohne Ziel: %d skillCast", n)
	}
	if n := len(eventsOfType(w, "castFailed")); n != 1 {
		t.Fatalf("meteor ohne Ziel: %d castFailed, erwartet 1", n)
	}
}

// Ziel stirbt im selben Takt: Das Ereignis trägt den Ort, an dem gewirkt wurde; zweiter Spieler meldet getrennt.
func TestSkillCastZielStirbtZweiSpieler(t *testing.T) {
	w := quietWorld(t)
	p := casterPlayer(t, w)
	q := AddPlayer(w) // Startverteilung von Spieler 2 kann Zauber schon enthalten
	addPoolPoints(w, 7)
	for _, id := range mageSkills {
		if CanLearn(w, q, id) == nil {
			mustLearn(t, w, q, id)
		}
	}
	q.X = p.X - 30
	if q.Slots[1] != "iceWall" {
		t.Fatalf("Slots Spieler 2: %v", q.Slots)
	}
	weak := spawnScaled(w, "skeleton", p.X+4, 0.01, 1)
	toughEnemy(w, q.X-6)
	stepSkills(w, p, PlayerCommand{Skill: 1}, dt)
	stepSkills(w, q, PlayerCommand{Skill: 2}, dt) // iceWall: range 8, radius 3
	evs := eventsOfType(w, "skillCast")
	if len(evs) != 2 || weak.HP > 0 {
		t.Fatalf("%d skillCast, Ziel HP %v", len(evs), weak.HP)
	}
	if evs[0]["from"] != p.ID || evs[0]["x"] != unitX(p.X+4) {
		t.Fatalf("Spieler 1: %v", evs[0])
	}
	if evs[1]["from"] != q.ID || evs[1]["skill"] != "iceWall" || evs[1]["x"] != unitX(q.X-6) {
		t.Fatalf("Spieler 2: %v", evs[1])
	}
}

// Skills ohne Fläche melden am Wirkenden mit Radius 0 (B-674: wie heute am Monarchen).
func TestSkillCastOhneFlaecheAmMonarchen(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	addPoolPoints(w, 2)
	mustLearn(t, w, p, "taunt")
	p.X = w.HubX + 40
	toughEnemy(w, p.X+2)
	stepSkills(w, p, PlayerCommand{Skill: 1}, dt)
	evs := eventsOfType(w, "skillCast")
	if len(evs) != 1 || evs[0]["x"] != unitX(p.X) || evs[0]["radius"] != 0.0 {
		t.Fatalf("taunt: %v", evs)
	}
}
