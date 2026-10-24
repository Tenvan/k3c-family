package sim

import (
	"slices"
	"testing"
)

// Dieb-Linie: Backstab und Smoke Bomb laut monarch.json (B-577/AC-02).

// thiefPlayer: ein Spieler mit beiden aktiven Dieb-Skills (Slots: backstab, smokeBomb), rechts der Burg.
func thiefPlayer(t *testing.T, w *World) *Player {
	t.Helper()
	p := AddPlayer(w)
	addPoolPoints(w, 2)
	mustLearn(t, w, p, "backstab", "smokeBomb")
	p.X = w.HubX + 20
	return p
}

func TestThiefBackstabVornUndHinten(t *testing.T) {
	for _, c := range []struct {
		name    string
		dx      float64
		fleeing bool
		want    float64
	}{
		{"vorn (Gegner blickt zur Burg)", 1, false, 40},
		{"hinten (Monarch zwischen Gegner und Portal)", -1, false, 80},
		{"fliehend blickt zum Portal", 1, true, 80},
	} {
		w := quietWorld(t)
		p := thiefPlayer(t, w)
		en := toughEnemy(w, p.X+c.dx)
		en.HomeX, en.Fleeing = w.HubX+60, c.fleeing
		stepSkills(w, p, PlayerCommand{Skill: 1}, dt)
		if lost(en) != c.want || p.Cooldowns[0] != 12 {
			t.Errorf("%s: Schaden %v (erwartet %v), CD %v", c.name, lost(en), c.want, p.Cooldowns)
		}
	}
}

func TestThiefBackstabOhneZielKeineAbklingzeit(t *testing.T) {
	w := quietWorld(t)
	p := thiefPlayer(t, w)
	far := toughEnemy(w, p.X+2.5)
	stepSkills(w, p, PlayerCommand{Skill: 1}, dt)
	if lost(far) != 0 || p.Cooldowns != nil {
		t.Fatalf("Backstab ohne Gegner in 2u: Schaden %v, CD %v", lost(far), p.Cooldowns)
	}
}

// Unsichtbar: kein Ziel (auch nicht verspottet), kein Schaden, nach 5 s wieder sichtbar; Abklingzeit 25 s.
func TestThiefSmokeBombUnsichtbar(t *testing.T) {
	w := quietWorld(t)
	p := thiefPlayer(t, w)
	other := AddPlayer(w)
	other.X = p.X + 1.4
	en := spawnEnemy(w, "skeleton", p.X+1)
	dir := sign(w.HubX - en.X)
	en.TauntID, en.TauntFor = p.ID, 10
	stepSkills(w, p, PlayerCommand{Skill: 2}, dt)
	if p.StealthFor != 5 || p.Cooldowns[1] != 25 {
		t.Fatalf("Smoke Bomb: StealthFor %v, CD %v", p.StealthFor, p.Cooldowns)
	}
	if c := chooseTarget(w, en, dir, nil); c == nil || c.id != other.ID {
		t.Fatalf("Gegner zielt auf %+v, erwartet den sichtbaren Spieler %d", c, other.ID)
	}
	hp := p.HP
	applyDamageBy(w, p.ID, 50, "skeleton")
	if p.HP != hp {
		t.Fatalf("Unsichtbarer nimmt Schaden: %v → %v", hp, p.HP)
	}
	stepIdle(w, p, 151) // 151 Ticks > 5 s
	if p.StealthFor != 0 {
		t.Fatalf("nach 5 s noch unsichtbar: %v", p.StealthFor)
	}
	if c := chooseTarget(w, en, dir, nil); c == nil || c.id != p.ID {
		t.Fatalf("nach 5 s: Ziel %+v, erwartet den Dieb %d (näher, verspottet)", c, p.ID)
	}
}

// Ein eigener Schlag oder Backstab beendet die Unsichtbarkeit vorzeitig (Annahme B-577).
func TestThiefSmokeBombEndetBeiAngriff(t *testing.T) {
	w := quietWorld(t)
	p := thiefPlayer(t, w)
	en := toughEnemy(w, p.X+1)
	stepSkills(w, p, PlayerCommand{Skill: 2}, dt)
	stepAttack(w, p, PlayerCommand{Attack: true}, dt)
	if p.StealthFor != 0 || lost(en) == 0 {
		t.Fatalf("Schlag aus der Unsichtbarkeit: StealthFor %v, Schaden %v", p.StealthFor, lost(en))
	}
	p.Cooldowns[1] = 0
	stepSkills(w, p, PlayerCommand{Skill: 2}, dt)
	stepSkills(w, p, PlayerCommand{Skill: 1}, dt)
	if p.StealthFor != 0 || p.Cooldowns[0] != 12 {
		t.Fatalf("Backstab aus der Unsichtbarkeit: StealthFor %v, CD %v", p.StealthFor, p.Cooldowns)
	}
}

// Zwei Diebe im selben Tick: Wirkung und Abklingzeit je Spieler getrennt.
func TestThiefZweiSpielerImSelbenTick(t *testing.T) {
	w := quietWorld(t)
	a, b := AddPlayer(w), AddPlayer(w)
	addPoolPoints(w, 2)
	mustLearn(t, w, a, "backstab", "smokeBomb")
	mustLearn(t, w, b, "backstab", "smokeBomb")
	a.X, b.X = w.HubX+40, w.HubX+80
	nearB := toughEnemy(w, b.X+1)
	Step(w, []PlayerCommand{{Skill: 2}, {Skill: 1}}, dt)
	if a.StealthFor <= 0 || b.StealthFor != 0 || lost(nearB) != 40 {
		t.Fatalf("Stealth A %v, B %v, Backstab-Schaden %v", a.StealthFor, b.StealthFor, lost(nearB))
	}
	if !slices.Equal(a.Cooldowns, []float64{0, 25, 0, 0}) || !slices.Equal(b.Cooldowns, []float64{12, 0, 0, 0}) {
		t.Fatalf("Abklingzeiten nicht je Spieler: A %v, B %v", a.Cooldowns, b.Cooldowns)
	}
}

// Das Preset Dieb lernt beide Skills (B-577/AC-01).
func TestThiefPresetLerntBeideSkills(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	p.Skills, p.Slots = nil, nil
	addPoolPoints(w, 2)
	if err := ApplyPreset(w, p, "thief"); err != nil {
		t.Fatal(err)
	}
	if !slices.Equal(p.Skills, []string{"backstab", "smokeBomb"}) || !slices.Equal(p.Slots, []string{"backstab", "smokeBomb"}) {
		t.Fatalf("Preset Dieb: Skills %v, Slots %v", p.Skills, p.Slots)
	}
}
