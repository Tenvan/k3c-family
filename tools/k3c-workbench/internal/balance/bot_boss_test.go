package balance

import (
	"slices"
	"testing"

	"k3c/engine/sim"
)

// BS11.3/AC-04: Kennzahlen und Ziele des Insel-Laufs (B-642/AC-04).
func TestMessgroessenEndboss(t *testing.T) {
	won, lost := ptr(120), (*int)(nil)
	m := &Metrics{IslandMetrics: IslandMetrics{EndbossTick: won, EndbossDay: ptr(17), Stages: []StageMetrics{{}, {ArrivalDay: ptr(5)}}}}
	if v := measures["endbossDefeated"].values(m); len(v) != 1 || v[0] != 1 {
		t.Errorf("endbossDefeated %v", v)
	}
	if v := measures["endbossDay"].values(m); len(v) != 1 || v[0] != 17 {
		t.Errorf("endbossDay %v", v)
	}
	if v := measures["caveBeforeDay6"].values(m); v[0] != 1 {
		t.Errorf("caveBeforeDay6 %v", v)
	}
	m.EndbossTick, m.EndbossDay, m.Stages[1].ArrivalDay = lost, nil, ptr(6)
	if v := measures["endbossDefeated"].values(m); v[0] != 0 {
		t.Errorf("nicht besiegt: %v", v)
	}
	if v := measures["endbossDay"].values(m); len(v) != 0 {
		t.Errorf("ohne Sieg kein Tag: %v", v)
	}
	if v := measures["caveBeforeDay6"].values(m); v[0] != 0 {
		t.Errorf("Höhle erst Tag 6: %v", v)
	}
}

// Die Ziele des Meilensteins stehen in der Datei, mit Welt standard, und die Datei lässt sich laden (Schema).
func TestZieleInselInDerDatei(t *testing.T) {
	ts, err := DefaultTargets()
	if err != nil {
		t.Fatal(err)
	}
	found := map[string]bool{}
	for _, g := range ts.Targets {
		if g.World == "standard" && g.Bot == "coop2" && g.Days == 40 {
			found[g.Measure] = true
		}
	}
	for _, name := range []string{"endbossDefeated", "endbossDay"} {
		if !found[name] {
			t.Errorf("Ziel %s mit Welt standard fehlt", name)
		}
	}
	if _, err := LoadTargets([]byte(`{"version":1,"margin":{},"seeds":["1"],"targets":[{"measure":"endbossDefeated","name":"x","kind":"share","players":2,"bot":"coop2","depth":0,"days":1,"world":"gibtsnicht","lower":1,"upper":2,"rule":"r"}]}`)); err == nil {
		t.Error("unbekannte Welt muss abgelehnt werden")
	}
}

// Bosskampf: Ein verletzter Bot zieht sich zur Burg zurück, ein Bot neben einem Gegner schlägt zu.
func TestBosskampfRueckzugUndSchlag(t *testing.T) {
	isl, err := newIsland(Scenario{Seed: "1", Players: 1, Days: 1, World: "standard"})
	if err != nil {
		t.Fatal(err)
	}
	w, p := isl.Stages[0], isl.Stages[0].Players[0]
	bot := withBossFight(passive, 4)
	p.X = w.HubX + 20
	p.HP = 0.2 * p.MaxHP
	if cmd := bot(w, p); cmd.MoveX >= 0 {
		t.Errorf("Rückzug zur Burg erwartet: %+v", cmd)
	}
	p.HP = p.MaxHP
	w.Enemies = append(w.Enemies, &sim.Enemy{ID: 99, X: p.X + 1, HP: 5, MaxHP: 5})
	defer func() { w.Enemies = slices.DeleteFunc(w.Enemies, func(e *sim.Enemy) bool { return e.ID == 99 }) }()
	if cmd := bot(w, p); !cmd.Attack {
		t.Errorf("Schlag erwartet: %+v", cmd)
	}
}
