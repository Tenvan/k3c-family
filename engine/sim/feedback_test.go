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
		castSkill(w, p, slot)
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

func eventsOfType(w *World, typ string) (out []Event) {
	for _, ev := range w.Events {
		if ev["type"] == typ {
			out = append(out, ev)
		}
	}
	return
}
