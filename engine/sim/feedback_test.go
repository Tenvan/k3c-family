package sim

import "testing"

// Rückmeldung ohne Ziel: Schlag ins Leere und Skill ohne Ziel (S9.1a, B-321).

func playerStrikes(w *World, p *Player) (n int, hit any) {
	for _, ev := range w.Events {
		if ev["type"] == "strike" && ev["from"] == p.ID {
			n++
			hit = ev["hit"]
		}
	}
	return
}

func TestSchlagOhneZielMeldetStrikeHitFalse(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	if n, hit := stepStrikes(w, p); n != 1 || hit != false {
		t.Fatalf("ins Leere: %d strike, hit %v", n, hit)
	}
	for range 10 { // Abklingzeit läuft: kein zweites Ereignis
		if n, _ := stepStrikes(w, p); n != 0 {
			t.Fatal("strike während der Abklingzeit")
		}
	}
	w2 := quietWorld(t)
	p2 := AddPlayer(w2)
	spawnEnemy(w2, "goblin", p2.X+1)
	if n, hit := stepStrikes(w2, p2); n != 1 || hit != true {
		t.Fatalf("Treffer: %d strike, hit %v", n, hit)
	}
}

func stepStrikes(w *World, p *Player) (int, any) {
	w.Events = nil
	stepAttack(w, p, strikeCmd, dt)
	return playerStrikes(w, p)
}

func TestSkillOhneZielMeldetCastFailed(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	failed := func(slot int) []Event {
		w.Events = nil
		castSkill(w, p, slot, true)
		return eventsOfType(w, "castFailed")
	}
	if len(failed(0)) != 0 {
		t.Fatal("leerer Slot meldet castFailed")
	}
	p.Slots = []string{"fireball"}
	if len(failed(0)) != 0 {
		t.Fatal("nicht gelernter Skill meldet castFailed")
	}
	p.Skills = []string{"fireball"}
	ev := failed(0)
	if len(ev) != 1 || ev[0]["from"] != p.ID || ev[0]["slot"] != 0 || ev[0]["x"] != unitX(p.X) || p.Cooldowns != nil && p.Cooldowns[0] != 0 {
		t.Fatalf("ohne Ziel: %v, Cooldowns %v", ev, p.Cooldowns)
	}
	p.Cooldowns = []float64{5, 0, 0, 0}
	if len(failed(0)) != 0 {
		t.Fatal("laufende Abklingzeit meldet castFailed")
	}
}

// Gehaltene Skill-Taste ohne Ziel: ein castFailed je Tastendruck, nicht je Tick (B-378).
func TestGehalteneSkillTasteMeldetCastFailedEinmal(t *testing.T) {
	w := quietWorld(t)
	p, q := AddPlayer(w), AddPlayer(w)
	for _, m := range []*Player{p, q} {
		m.Slots, m.Skills = []string{"fireball"}, []string{"fireball"}
	}
	count := func(m *Player, skill, ticks int) (n int) {
		for range ticks {
			w.Events = nil
			stepSkills(w, m, PlayerCommand{Skill: skill}, dt)
			for _, ev := range eventsOfType(w, "castFailed") {
				if ev["from"] == m.ID {
					n++
				}
			}
		}
		return
	}
	if n := count(p, 1, 30); n != 1 {
		t.Fatalf("30 Ticks gehalten: %d castFailed, erwartet 1", n)
	}
	count(p, 0, 1) // loslassen
	if n := count(p, 1, 30); n != 1 {
		t.Fatalf("neuer Druck: %d castFailed, erwartet 1", n)
	}
	if n := count(q, 1, 30); n != 1 {
		t.Fatalf("Spieler 2 unabhängig: %d castFailed, erwartet 1", n)
	}
	count(p, 0, 1)
	e := toughEnemy(w, p.X+5) // Ziel erscheint beim Halten: wirkt, kein castFailed
	if n := count(p, 1, 1); n != 0 || p.Cooldowns == nil || p.Cooldowns[0] == 0 || lost(e) == 0 {
		t.Fatalf("mit Ziel: %d castFailed, CD %v, Schaden %v", n, p.Cooldowns, lost(e))
	}
}

// B-524/AC-01: Der Tod vergisst die gehaltene Skill-Taste. Halten ohne Ziel (ein castFailed), sterben, wiederbeleben,
// neu drücken → ein zweites castFailed; Spieler 2 hält durchgehend und meldet kein weiteres.
func TestSkillTasteNachTodMeldetCastFailedNeu(t *testing.T) {
	w := quietWorld(t)
	p, q := AddPlayer(w), AddPlayer(w)
	for _, m := range []*Player{p, q} {
		m.Slots, m.Skills = []string{"fireball"}, []string{"fireball"}
	}
	step := func(skillP int) (np, nq int) {
		w.Events = nil
		stepPlayers(w, []PlayerCommand{{Skill: skillP}, {Skill: 1}}, dt)
		for _, ev := range eventsOfType(w, "castFailed") {
			switch ev["from"] {
			case p.ID:
				np++
			case q.ID:
				nq++
			}
		}
		return
	}
	if np, nq := step(1); np != 1 || nq != 1 {
		t.Fatalf("erster Druck: %d/%d castFailed, erwartet je 1", np, nq)
	}
	p.HP, p.RespawnIn = 0, 3*dt // stirbt mit gehaltener Taste
	for !isAlive(p) {
		if np, nq := step(0); np != 0 || nq != 0 {
			t.Fatalf("während des Todes: %d/%d castFailed", np, nq)
		}
	}
	if np, nq := step(1); np != 1 || nq != 0 {
		t.Fatalf("neuer Druck nach dem Respawn: %d/%d castFailed, erwartet 1/0", np, nq)
	}
}

// B-378/AC-02: In der Budget-Aufstellung halten alle Spieler 1 min lang Skill 1 ohne Ziel; es gibt höchstens je Spieler
// ein castFailed, das Budget bleibt unberührt.
func TestGehalteneSkillTastenImEreignisBudget(t *testing.T) {
	isl, cmds := benchIsland(t)
	players := 0
	for _, w := range isl.Stages {
		for _, p := range w.Players {
			p.Slots, p.Skills = []string{"fireball"}, []string{"fireball"}
			players++
		}
	}
	held := make([]PlayerCommand, len(cmds))
	for i := range held {
		held[i].Skill = 1
	}
	var events, state []int
	failed := 0
	for range 1800 {
		StepIsland(isl, held, dt)
		tickBytes(t, isl, &events, &state)
		for _, w := range isl.Stages {
			failed += len(eventsOfType(w, "castFailed"))
		}
	}
	if mean, _ := meanP99(events); mean > eventBudgetBytes {
		t.Fatalf("Ereignis-Budget überschritten: Mittel %.1f Byte je Tick", mean)
	}
	if failed > players {
		t.Fatalf("%d castFailed bei %d gehaltenen Tasten, erwartet höchstens je Spieler eines", failed, players)
	}
}

func eventsOfType(w *World, typ string) (out []Event) {
	for _, ev := range w.Events {
		if ev["type"] == typ {
			out = append(out, ev)
		}
	}
	return
}
