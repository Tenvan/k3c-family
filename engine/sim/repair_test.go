package sim

import (
	"math"
	"testing"
)

// W1.3 (B-112/AC-04, AC-05, AC-06, Sprint W1 AC-03, AC-04): Zerstörung setzt die Stufe zurück, Reparatur ohne Gold und
// Material zwischen den Wellen, Spielstand mit Hub- und Platz-Stufen.

// Mauer Stufe 2 zerstört: unpaid, Stufe 1, MaxHP der Stufe 1, gleiches X; Hub-Stufe und äußere Linie bleiben.
func TestZerstoerungSetztStufeZurueck(t *testing.T) {
	w := dayWorld(t)
	w.HubLevel = 3
	inner, outer := lineSite(t, w, "wall", -1, 1), lineSite(t, w, "wall", -1, 2)
	built(inner)
	built(outer)
	inner.Level = 2
	inner.MaxHP = buildings["wall"].Levels[1].HP
	inner.HP, inner.Upgrade, inner.UpgradePaid = inner.MaxHP, "waitingMaterial", 10
	x := inner.X
	applyDamage(w, inner.ID, inner.HP)
	if inner.State != "unpaid" || levelOf(w, inner) != 1 || inner.MaxHP != buildings["wall"].Levels[0].HP || inner.X != x {
		t.Fatalf("nach Zerstörung: %+v", inner)
	}
	if inner.Upgrade != "" || inner.UpgradePaid != 0 {
		t.Errorf("laufender Ausbau nicht verloren: %+v", inner)
	}
	if w.HubLevel != 3 || outer.State != "built" {
		t.Errorf("Hub-Stufe %d, äußere Mauer %s", w.HubLevel, outer.State)
	}
}

// damagedTower ist ein gebauter Turm mit halben HP in einer Welt mit zwei Spielern.
func damagedTower(t *testing.T) (*World, *Site) {
	t.Helper()
	w := dayWorld(t)
	AddPlayer(w)
	AddPlayer(w)
	tower := lineSite(t, w, "tower", 1, 1)
	built(tower)
	tower.HP = tower.MaxHP / 2
	w.Troops[0].X = tower.X
	return w, tower
}

// Reparatur zwischen den Wellen: HP voll nach dem fehlenden Anteil der Bauzeit, Vorrat und Gold unverändert.
func TestReparaturOhneGoldUndMaterial(t *testing.T) {
	w, tower := damagedTower(t)
	*w.Stock = Stock{Wood: 9, Stone: 4}
	stock, gold := *w.Stock, []int{w.Players[0].Gold, w.Players[1].Gold}
	took := runUntil(w, 3*buildings["tower"].BuildSeconds, func() bool { return tower.HP >= tower.MaxHP })
	if half := buildings["tower"].BuildSeconds / 2; took < half-dt || took > half+0.5 {
		t.Errorf("Reparatur dauerte %.2f s, erwartet die halbe Bauzeit %v s", took, half)
	}
	if *w.Stock != stock || w.Players[0].Gold != gold[0] || w.Players[1].Gold != gold[1] {
		t.Errorf("Reparatur kostet: Vorrat %+v → %+v, Gold %v → %d/%d", stock, *w.Stock, gold, w.Players[0].Gold, w.Players[1].Gold)
	}
}

// Bei Gefahr (Gegner in der Welt, Nacht) keine Reparatur.
func TestReparaturNichtBeiGefahr(t *testing.T) {
	w, tower := damagedTower(t)
	e := spawnEnemy(w, "goblin", w.WidthUnits-1)
	e.Stun = math.Inf(1)
	before := tower.HP
	run(w, buildings["tower"].BuildSeconds)
	if tower.HP != before {
		t.Fatalf("mit Gegnern repariert: %v → %v", before, tower.HP)
	}
	w.Enemies = []*Enemy{}
	w.CycleSpeed = 1
	w.Time = (globalDayNight.DayMinutes + globalDayNight.TwilightMinutes) * 60
	run(w, 1)
	if w.Cycle.Phase != "night" || tower.HP != before {
		t.Fatalf("nachts repariert (Phase %s): %v → %v", w.Cycle.Phase, before, tower.HP)
	}
}

// Hub-Stufe, Mauer-Stufe (auch auf einer äußeren Linie) und laufender Ausbau überleben Speichern und Laden.
func TestSpielstandHubUndPlatzStufen(t *testing.T) {
	isl := mustIsland(t, "stufen", []int{0, 1})
	a := isl.Stages[0]
	a.HubLevel = 3
	outer := lineSite(t, a, "wall", 1, 2)
	built(outer)
	outer.Level, outer.MaxHP, outer.HP = 3, buildings["wall"].Levels[2].HP, 700
	outer.Upgrade, outer.UpgradePaid = "waitingMaterial", 20
	a.hubSite.UpgradePaid = 30
	loaded := loadIsland(t, saveJSON(t, isl.ToSave("2026-10-05T12:00:00Z")))
	la := loaded.Stages[0]
	lo := lineSite(t, la, "wall", 1, 2)
	if la.HubLevel != 3 || loaded.Stages[1].HubLevel != 1 {
		t.Errorf("Hub-Stufen nach dem Laden %d/%d", la.HubLevel, loaded.Stages[1].HubLevel)
	}
	if lo.Level != 3 || lo.MaxHP != outer.MaxHP || lo.HP != 700 || lo.Upgrade != "waitingMaterial" || lo.UpgradePaid != 20 {
		t.Errorf("äußere Mauer nach dem Laden: %+v", lo)
	}
	if la.hubSite.UpgradePaid != 30 {
		t.Errorf("Hub-Ausbau nach dem Laden: %+v", la.hubSite)
	}
}
