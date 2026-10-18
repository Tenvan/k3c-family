package sim

import (
	"slices"
	"testing"
)

// Zwei Linien gleichzeitig: Tank und Zauberer zweier Spieler in einer Welt (SK1.2, B-007).

func TestSkillsTankUndZaubererImSelbenTick(t *testing.T) {
	w := quietWorld(t)
	tank, mage := AddPlayer(w), AddPlayer(w)
	addPoolPoints(w, 1)
	mustLearn(t, w, tank, "taunt")
	mustLearn(t, w, mage, "fireball")
	tank.X, mage.X = w.HubX+40, w.HubX+80 // weit genug auseinander: jede Wirkung trifft nur den eigenen Gegner
	nearTank, nearMage := toughEnemy(w, tank.X+5), toughEnemy(w, mage.X+5)

	Step(w, []PlayerCommand{{Skill: 1}, {Skill: 1}}, dt)

	if nearTank.TauntID != tank.ID || nearTank.TauntFor <= 0 || lost(nearTank) != 0 {
		t.Fatalf("Taunt: Gegner beim Tank %v/%v, verlorene HP %v", nearTank.TauntID, nearTank.TauntFor, lost(nearTank))
	}
	if lost(nearMage) <= 0 || nearMage.TauntID != 0 {
		t.Fatalf("Fireball: Gegner beim Zauberer verlor %v HP, TauntID %v", lost(nearMage), nearMage.TauntID)
	}
	if !slices.Equal(tank.Cooldowns, []float64{15, 0, 0, 0}) || !slices.Equal(mage.Cooldowns, []float64{8, 0, 0, 0}) {
		t.Fatalf("Abklingzeiten nicht je Spieler: Tank %v, Zauberer %v", tank.Cooldowns, mage.Cooldowns)
	}
	if !slices.Equal(tank.Slots, []string{"taunt"}) || !slices.Equal(mage.Slots, []string{"fireball"}) {
		t.Fatalf("Slots: Tank %v, Zauberer %v", tank.Slots, mage.Slots)
	}
}

func TestSkillsFremdeLinieOhnePunkteAbgelehnt(t *testing.T) {
	w := quietWorld(t)
	tank, mage := AddPlayer(w), AddPlayer(w)
	addPoolPoints(w, 1)
	mustLearn(t, w, tank, "taunt")
	mustLearn(t, w, mage, "fireball")
	free := AvailablePoints(w, mage)
	for _, id := range []string{"taunt", "iceWall"} { // Pool leer, danach (nach Punkt) Tier fehlt
		if err := LearnSkill(w, mage, id); err == nil {
			t.Fatalf("%s ohne freien Punkt gelernt", id)
		}
	}
	addPoolPoints(w, 5)
	if err := LearnSkill(w, mage, "lightningStorm"); err == nil {
		t.Fatal("Tier 3 des Zauberers mit 1 Skill der Linie gelernt")
	}
	if want := []string{"fireball"}; !slices.Equal(mage.Skills, want) || AvailablePoints(w, mage) != free+5 {
		t.Fatalf("Zustand nach Ablehnung: Skills %v, frei %d", mage.Skills, AvailablePoints(w, mage))
	}
	if !slices.Equal(tank.Skills, []string{"taunt"}) {
		t.Fatalf("Tank verändert: %v", tank.Skills)
	}
}
