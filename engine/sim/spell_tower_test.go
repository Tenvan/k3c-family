package sim

import "testing"

// W3.2 (B-116/AC-01, Sprint W3 AC-01): Der Zaubertum (Turm-Stufe spell.fromLevel) schießt selbst und trifft alle
// Gegner im Radius um das Ziel (Q31); die Schützen steigen ab und bleiben Kämpfer. Werte aus tower.spell.

// spellWorld: Turm der Linie 1 links auf Stufe level, Gegner fest an Abständen zum Turm (nach außen), zwei Spieler.
func spellWorld(t *testing.T, level int, offsets ...float64) (*World, *Site, []*Enemy) {
	t.Helper()
	w := dayWorld(t)
	AddPlayer(w)
	AddPlayer(w)
	w.HubLevel = level
	tower := lineSite(t, w, "tower", -1, 1)
	built(tower)
	tower.Level = level
	enemies := []*Enemy{}
	for _, dx := range offsets {
		e := spawnEnemy(w, "skeleton", tower.X-dx)
		e.HP, e.MaxHP = 1000, 1000
		enemies = append(enemies, e)
	}
	return w, tower, enemies
}

func TestZaubertumFlaechenschadenImTakt(t *testing.T) {
	sp := buildings["tower"].Spell
	near := sp.RangeUnits / 2
	// Ziel, im Radius, außerhalb des Radius (in Reichweite), außerhalb der Reichweite.
	w, _, es := spellWorld(t, sp.FromLevel, near, near+sp.RadiusUnits-0.5, near+sp.RadiusUnits+0.5, sp.RangeUnits+sp.RadiusUnits+1)
	stepSpellTowers(w, dt)
	for i, want := range []float64{1000 - sp.Damage, 1000 - sp.Damage, 1000, 1000} {
		if es[i].HP != want {
			t.Errorf("Gegner %d nach dem ersten Schuss HP %v, erwartet %v", i, es[i].HP, want)
		}
	}
	for tm := dt; tm < sp.IntervalSeconds-dt/2; tm += dt {
		stepSpellTowers(w, dt)
	}
	if es[0].HP != 1000-sp.Damage {
		t.Errorf("vor Ablauf des Takts erneut getroffen: HP %v", es[0].HP)
	}
	stepSpellTowers(w, dt)
	if es[0].HP != 1000-2*sp.Damage {
		t.Errorf("nach %v s kein zweiter Schuss: HP %v", sp.IntervalSeconds, es[0].HP)
	}
}

func TestTurmUnterZaubertumStufeSchiesstNicht(t *testing.T) {
	sp := buildings["tower"].Spell
	w, _, es := spellWorld(t, sp.FromLevel-1, 2)
	for tm := 0.0; tm < 2*sp.IntervalSeconds; tm += dt {
		stepSpellTowers(w, dt)
	}
	if es[0].HP != 1000 {
		t.Errorf("Turm-Stufe %d macht Flächenschaden: HP %v", sp.FromLevel-1, es[0].HP)
	}
}

// Ausbau auf die Zaubertum-Stufe: der Schütze steigt ab, bleibt Kämpfer, der Turm nimmt keinen mehr auf.
func TestZaubertumSchuetzenSteigenAb(t *testing.T) {
	sp := buildings["tower"].Spell
	w, tower, _ := spellWorld(t, sp.FromLevel-1)
	w.HubLevel = sp.FromLevel
	archer := spawnVagrant(w, w.HubX, tower.X)
	makeArcher(w, archer)
	archer.AnchorX, archer.TowerID = tower.X, intPtr(tower.ID)
	players := w.Players
	if took := upgradeAt(t, w, players, tower); took < 0 {
		t.Fatal("Ausbau auf die Zaubertum-Stufe nicht fertig")
	}
	run(w, 1)
	if archer.TowerID != nil || archer.Kind != "archer" || fighters(w) != 1 {
		t.Errorf("Schütze: Turm %v, Art %s, Kämpfer %d", archer.TowerID, archer.Kind, fighters(w))
	}
	if freeTower(w, archer) == tower {
		t.Error("Zaubertum bietet weiter Plätze für Schützen")
	}
	e := spawnEnemy(w, "skeleton", tower.X-2)
	e.HP, e.MaxHP = 1000, 1000
	Step(w, nil, dt) // der Schuss läuft im Takt der Sim, sofort, ohne Flugzeit
	if e.HP != 1000-sp.Damage {
		t.Errorf("Zaubertum schießt im Spiel nicht: Gegner-HP %v", e.HP)
	}
}
