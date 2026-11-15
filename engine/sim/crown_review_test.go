package sim

import "testing"

// Review-Fixes KS3: Fortified-Aura, geschützte Nacht, Free-Monarch, Last Stand nur mit letzter Münze.

func TestFortifiedAuraFuerTruppen(t *testing.T) {
	w, p0, _ := passiveWorld(t)
	mustLearn(t, w, p0, "taunt", "shieldBash", "fortified")
	e := effectOf("fortified")
	in, out := troopAt(w, p0.X+e.Aura, 100), troopAt(w, p0.X+e.Aura+1, 100)
	applyDamage(w, in.ID, 10)
	applyDamage(w, out.ID, 10)
	if in.HP != 100-(10-e.Defense) || out.HP != 90 {
		t.Fatalf("Fortified: in der Aura %v, außerhalb %v", in.HP, out.HP)
	}
}

func TestKroneKeinRaubInGeschuetzterNacht(t *testing.T) {
	isl := islandWith(t, "krone-schutz", "easy", 1)
	w := isl.Stages[0]
	w.Cycle = CycleInfo{Phase: "night", Day: 1}
	w.Portals = []float64{w.HubX + 90}
	p := w.Players[0]
	fell(p)
	p.Crown.X, p.X = w.HubX+30, w.HubX
	e := spawnEnemy(w, "goblin", p.Crown.X)
	takeCrownOrLoot(w, e)
	if e.Crown != nil || p.Crown.State != CrownGround {
		t.Fatal("Raub in geschützter Nacht")
	}
}

func TestKroneFreierMonarchBekommtSieZurueck(t *testing.T) {
	w, p0, _ := crownWorld(t)
	fell(p0)
	p0.X -= 20
	p0.Free = true
	spawnEnemy(w, "goblin", p0.Crown.X)
	Step(w, nil, dt)
	if p0.Crown != nil {
		t.Fatalf("Krone des abwesenden Monarchen: %+v", p0.Crown)
	}
}

func TestLastStandNurMitLetzterMuenze(t *testing.T) {
	w := quietWorld(t)
	p := tankPlayer(t, w)
	p.LastStandFor, p.Gold = 10, 0
	applyDamage(w, p.ID, 5)
	if p.Crown == nil {
		t.Fatal("ohne Gold fällt die Krone auch mit Last Stand")
	}
}
