package sim

import (
	"testing"
)

// Passive Skills mit 2 Spielern (S1.2c, B-119/AC-03): erwartete Werte aus monarch.json (vorläufige Startwerte).

// passiveWorld: ruhige Welt mit zwei Spielern weit weg von der Burg, je 3 Pool-Punkte.
func passiveWorld(t *testing.T) (*World, *Player, *Player) {
	t.Helper()
	w := quietWorld(t)
	p0, p1 := AddPlayer(w), AddPlayer(w)
	addPoolPoints(w, 3)
	p0.X, p1.X = w.HubX+40, w.HubX+40
	return w, p0, p1
}

func stepTicks(w *World, ticks int) {
	for range ticks {
		Step(w, nil, dt)
	}
}

func effectOf(id string) skillEffect { return monarch.Skills[id].Effect }

// (a) Armor Aura von Spieler 0 senkt den Schaden an Truppen im Radius 8, nicht außerhalb (Monarchen haben keine Verteidigung mehr).
func TestPassivesArmorAura(t *testing.T) {
	w, p0, _ := passiveWorld(t)
	mustLearn(t, w, p0, "taunt", "shieldBash", "armorAura")
	bonus, aura := effectOf("armorAura").Defense, effectOf("armorAura").Aura
	in, out := troopAt(w, p0.X+aura, 100), troopAt(w, p0.X-aura-0.5, 100)
	applyDamage(w, in.ID, 17)
	applyDamage(w, out.ID, 17)
	if in.HP != 100-(17-bonus) || out.HP != 100-17 {
		t.Fatalf("Schaden in der Aura: HP %v, außerhalb %v", in.HP, out.HP)
	}
}

// (b) Healing Aura: Spieler 1 (Heiler) heilt Truppen im Radius 8 mit 2 HP/s, Monarchen nicht (kampf.md § 5).
func TestPassivesHealingAura(t *testing.T) {
	w, p0, p1 := passiveWorld(t)
	mustLearn(t, w, p1, "heal", "groupHeal", "healingAura")
	p0.X = p1.X + 8
	in, out := troopAt(w, p1.X-8, 2), troopAt(w, p1.X+9, 2)
	stepTicks(w, 1) // Truppen laufen danach los: nur der erste Tick zählt für sie
	if in.HP != 2+float64(2*dt) || out.HP != 2 {
		t.Fatalf("Healing Aura: Truppe 8u %v, 9u %v", in.HP, out.HP)
	}
}

// (c) Thick Skin und Regeneration: Werte des Besitzers, Spieler 1 bleibt unverändert.
func TestPassivesTankBesitzer(t *testing.T) {
	w, p0, p1 := passiveWorld(t)
	addPoolPoints(w, 2)
	mustLearn(t, w, p0, "taunt", "shieldBash", "thickSkin", "regeneration")
	probe(p0, p1)
	every := effectOf("thickSkin").FreeHitEvery
	for range every {
		applyDamage(w, p0.ID, 5)
		applyDamage(w, p1.ID, 5)
	}
	if p0.Gold != probeGold-(every-1)*monarch.HitCoins || p1.Gold != probeGold-every*monarch.HitCoins {
		t.Fatalf("Thick Skin: P0 Gold %v (jeder %d. Treffer gratis), P1 %v", p0.Gold, every, p1.Gold)
	}
	if w.Cycle.Phase != "day" {
		t.Fatalf("Phase %q, erwartet Tag", w.Cycle.Phase)
	}
	p0.Gold, p1.Gold, w.Coins = 10, 10, nil
	stepTicks(w, int(5/dt)+2) // 5 s × 0,2 Münzen/s
	if p0.Gold != 11 || p1.Gold != 10 {
		t.Fatalf("Regeneration (Gold-Rinnsal): P0 %v, P1 %v", p0.Gold, p1.Gold)
	}
}

// (c) Arcane Power (+20 % Zauberschaden) und Elemental Mastery (−15 % Abklingzeit).
func TestPassivesZauberer(t *testing.T) {
	w, p0, p1 := passiveWorld(t)
	mustLearn(t, w, p0, "fireball", "iceWall", "arcanePower")
	mustLearn(t, w, p1, "fireball", "iceWall", "elementalMastery")
	e := toughEnemy(w, p0.X+5)
	stepSkills(w, p0, PlayerCommand{Skill: 1}, dt)
	if !near(lost(e), 36) || p0.Cooldowns[0] != 8 {
		t.Fatalf("Arcane Power: Schaden %v, CD %v", lost(e), p0.Cooldowns)
	}
	stepSkills(w, p1, PlayerCommand{Skill: 1}, dt)
	if !near(lost(e), 66) || !near(p1.Cooldowns[0], 6.8) {
		t.Fatalf("Elemental Mastery: Schaden %v, CD %v", lost(e), p1.Cooldowns)
	}
}

// (c) Blessings: Tempo und Schlagschaden +10 % für Monarchen im Radius 8.
func TestPassivesBlessings(t *testing.T) {
	w, p0, p1 := passiveWorld(t)
	mustLearn(t, w, p1, "heal", "groupHeal", "blessings")
	p0.X = p1.X - 8
	e := toughEnemy(w, p0.X-1)
	stepAttack(w, p0, PlayerCommand{Attack: true}, dt)
	if !near(speedOf(w, p0), 5.5) || !near(lost(e), 11) {
		t.Fatalf("Blessings in 8u: Tempo %v, Schlag %v", speedOf(w, p0), lost(e))
	}
	p0.X = p1.X - 9
	if speedOf(w, p0) != monarch.Base.Speed || damageMultOf(w, p0, true) != 1 {
		t.Fatalf("Blessings in 9u: Tempo %v, Faktor %v", speedOf(w, p0), damageMultOf(w, p0, true))
	}
}

// (d) Guardian: Truppen im Radius 5 um den Besitzer nehmen 20 % weniger Schaden.
func TestPassivesGuardian(t *testing.T) {
	w, p0, _ := passiveWorld(t)
	mustLearn(t, w, p0, "taunt", "shieldBash", "guardian")
	in, out := troopAt(w, p0.X+5, 10), troopAt(w, p0.X-6, 10)
	applyDamage(w, in.ID, 5)
	applyDamage(w, out.ID, 5)
	if in.HP != 6 || out.HP != 5 {
		t.Fatalf("Guardian: 5u %v, 6u %v", in.HP, out.HP)
	}
}

// (d) Frost Armor: Ein Gegner, der den Besitzer im Nahkampf trifft, läuft 2 s mit Faktor 0,7; Spieler 1 ohne Passiv nicht.
func TestPassivesFrostArmor(t *testing.T) {
	w, p0, p1 := passiveWorld(t)
	mustLearn(t, w, p0, "fireball", "iceWall", "frostArmor")
	e0, e1 := toughEnemy(w, p0.X+1), toughEnemy(w, p1.X-1)
	attack(w, e0, &target{id: p0.ID, x: p0.X, kind: "player", player: p0})
	attack(w, e1, &target{id: p1.ID, x: p1.X, kind: "player", player: p1})
	if e0.Slow != 0.7 || e0.SlowFor != 2 || e1.SlowFor != 0 {
		t.Fatalf("Frost Armor: %v für %v s, ohne Passiv %v s", e0.Slow, e0.SlowFor, e1.SlowFor)
	}
	e0.Slow, e0.SlowFor = 0.5, 4 // stärkere Ice Wall bleibt
	attack(w, e0, &target{id: p0.ID, x: p0.X, kind: "player", player: p0})
	if e0.Slow != 0.5 || e0.SlowFor != 4 {
		t.Fatalf("Ice Wall überschrieben: %v für %v s", e0.Slow, e0.SlowFor)
	}
}

// (d) Holy Ground: Truppen im Burgradius werden geheilt, egal wo der Heiler steht; Monarchen haben keine HP.
func TestPassivesHolyGround(t *testing.T) {
	w, _, p1 := passiveWorld(t)
	mustLearn(t, w, p1, "heal", "groupHeal", "holyGround")
	tr := troopAt(w, w.Castle.X, 2)
	stepTicks(w, 1)
	if tr.HP != 2+float64(1*dt) {
		t.Fatalf("Holy Ground: Truppe %v", tr.HP)
	}
}

// echoRun: Zauberer mit gelerntem Passiv wirkt 50-mal Fireball (Abklingzeit zurückgesetzt); Ergebnis: Schaden am Ziel.
func echoRun(t *testing.T, passive string) (float64, *World) {
	t.Helper()
	w, p0, _ := passiveWorld(t)
	mustLearn(t, w, p0, "fireball", "iceWall", passive)
	e := spawnScaled(w, "skeleton", p0.X+5, 1000, 1)
	for range 50 {
		p0.Cooldowns = nil
		stepSkills(w, p0, PlayerCommand{Skill: 1}, dt)
	}
	return lost(e), w
}

// (e) Spell Echo: gleicher Seed gleiches Ergebnis, Echos treten auf; ohne Spell Echo bleibt w.rng unberührt.
func TestPassivesSpellEcho(t *testing.T) {
	a, _ := echoRun(t, "spellEcho")
	b, _ := echoRun(t, "spellEcho")
	if a != b || a <= 50*30 || a >= 100*30 {
		t.Fatalf("Spell Echo: Lauf 1 %v, Lauf 2 %v (50 Zauber à 30)", a, b)
	}
	w, p0, _ := passiveWorld(t)
	mustLearn(t, w, p0, "fireball", "iceWall", "frostArmor")
	toughEnemy(w, p0.X+5)
	before := *w.rng
	stepSkills(w, p0, PlayerCommand{Skill: 1}, dt)
	if *w.rng != before || p0.Cooldowns[0] == 0 {
		t.Fatal("ohne Spell Echo Zufallswert verbraucht oder kein Zauber")
	}
}

// (f) Ohne gelernte Passive liefern die Hilfsfunktionen die Basiswerte.
func TestPassivesOhneBasiswerte(t *testing.T) {
	w, p0, p1 := passiveWorld(t)
	mustLearn(t, w, p0, "taunt", "shieldBash")
	for _, p := range []*Player{p0, p1} {
		if speedOf(w, p) != monarch.Base.Speed || damageMultOf(w, p, true) != 1 || cooldownMultOf(w, p) != 1 {
			t.Fatalf("Spieler %d weicht von der Basis ab", p.Index)
		}
	}
}

// Alle zwölf Passive stehen mit Wirkung und als vorläufig markiert in monarch.json.
func TestPassivesDaten(t *testing.T) {
	n := 0
	for _, s := range defaultValues().skills.list {
		if s.Kind != "passive" || s.Line == "thief" || s.Line == "purse" { // Geldsack: purse_test.go
			continue
		}
		n++
		if !s.Provisional || s.Effect.Type == "" {
			t.Errorf("%s: provisional %v, effect %q", s.ID, s.Provisional, s.Effect.Type)
		}
	}
	if n != 12 {
		t.Fatalf("%d Passive statt 12", n)
	}
}
