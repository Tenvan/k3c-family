package actors

import (
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// newTestDrawer: Drawer ohne Grafik (lib nil); Listen rechnet die Posen trotzdem.
func newTestDrawer(t *testing.T) *drawer {
	t.Helper()
	d, ok := New(nil).(*drawer)
	if !ok || d.cat == nil {
		t.Fatal("kein Katalog")
	}
	return d
}

func frameOf(clock float64, w *sim.World) *view.Frame {
	return &view.Frame{Clock: clock, Stages: []view.Stage{{Index: 0, Level: "wald", World: w}}}
}

// Beispiel B-796: Zwei Monarchen reiten in entgegengesetzte Richtungen; jeder blickt in seine Richtung, Gangart je
// Monarch eigen.
func TestTwoMonarchsOwnGaitAndFacing(t *testing.T) {
	d := newTestDrawer(t)
	w := &sim.World{Players: []*sim.Player{
		{ID: 1, Index: 0, X: 10, VX: 5, Facing: 1},
		{ID: 2, Index: 1, X: 12, VX: -1, Facing: -1},
	}}
	d.Listen(frameOf(1, w))
	a, b := d.figs[key{0, classPlayer, 1}], d.figs[key{0, classPlayer, 2}]
	if a == nil || b == nil {
		t.Fatal("Monarch fehlt")
	}
	if a.left || !b.left {
		t.Errorf("Blickrichtung: %v %v", a.left, b.left)
	}
	if a.pose.Gait != gaitGallop || b.pose.Gait != gaitWalk {
		t.Errorf("Gangarten %q %q", a.pose.Gait, b.pose.Gait)
	}
	if a.look.Variant != "" || b.look.Variant != "spieler2" || a.shift == b.shift {
		t.Errorf("Variante oder Versatz gleich: %+v %+v", a.look, b.look)
	}
	w.Players[0].VX, w.Players[0].Facing = 0, 0 // steht: Blick bleibt
	d.Listen(frameOf(1.1, w))
	if a.left || a.pose.Gait != gaitStand {
		t.Errorf("stehender Monarch: %+v", a.pose)
	}
}

// Tod und Respawn: death hält zu Fuß, nach dem Tod respawn, danach wieder beritten.
func TestMonarchDeathAndRespawn(t *testing.T) {
	d := newTestDrawer(t)
	p := &sim.Player{ID: 1, X: 10}
	w := &sim.World{Players: []*sim.Player{p}}
	d.Listen(frameOf(1, w))
	p.RespawnIn = 5
	d.Listen(frameOf(2, w))
	fg := d.figs[key{0, classPlayer, 1}]
	if fg.pose.Anim != animDeath || !fg.pose.Hold || fg.pose.Gait != "" {
		t.Fatalf("Tod: %+v", fg.pose)
	}
	p.RespawnIn = 0
	d.Listen(frameOf(3, w))
	if fg.pose.Anim != animRespawn || fg.pose.Dur != defaultOnce {
		t.Fatalf("Respawn: %+v", fg.pose)
	}
	d.Listen(frameOf(3+defaultOnce/2, w))
	if fg.pose.Anim != animRespawn {
		t.Errorf("Respawn bricht ab: %+v", fg.pose)
	}
	d.Listen(frameOf(4, w))
	if fg.pose.Gait != gaitStand {
		t.Errorf("nach dem Respawn nicht beritten: %+v", fg.pose)
	}
}

// Smoke Bomb: Der unsichtbare Monarch wird fast durchsichtig, danach wieder deckend.
func TestMonarchStealthAlpha(t *testing.T) {
	d := newTestDrawer(t)
	p := &sim.Player{ID: 1, X: 10, StealthFor: 2}
	w := &sim.World{Players: []*sim.Player{p}}
	d.Listen(frameOf(1, w))
	fg := d.figs[key{0, classPlayer, 1}]
	if fg.alpha != stealthMax {
		t.Errorf("unsichtbar: alpha %v, erwartet %v", fg.alpha, stealthMax)
	}
	p.StealthFor = 0
	d.Listen(frameOf(1.1, w))
	if fg.alpha != 1 {
		t.Errorf("sichtbar: alpha %v, erwartet 1", fg.alpha)
	}
}

// Truppen und Gegner: Richtung aus der Bewegung, Turm hebt die Truppe, Fliehen blendet, Fledermaus fliegt,
// verschwundene Figuren fallen heraus, ein Berufswechsel legt die Figur neu an.
func TestTroopsAndEnemies(t *testing.T) {
	d := newTestDrawer(t)
	tower := 7
	w := &sim.World{
		Sites:   []*sim.Site{{ID: 7, X: 20}},
		Troops:  []*sim.Troop{{ID: 1, Kind: "archer", X: 20, TowerID: &tower}, {ID: 2, Kind: "vagrant", X: 5}},
		Enemies: []*sim.Enemy{{ID: 3, Kind: "goblin", X: 30, Fleeing: true}, {ID: 4, Kind: "bat", X: 15}},
	}
	d.Listen(frameOf(1, w))
	w.Enemies[0].X = 30.5
	d.Listen(frameOf(1.1, w))
	archer, gob, bat := d.figs[key{0, classTroop, 1}], d.figs[key{0, classEnemy, 3}], d.figs[key{0, classEnemy, 4}]
	if archer.lift != towerLift || !archer.left { // nächster Gegner: Fledermaus links
		t.Errorf("Bogenschütze auf dem Turm: lift %v, links %v", archer.lift, archer.left)
	}
	if gob.left || gob.pose.Anim != animRun || gob.alpha != fleeAlpha {
		t.Errorf("fliehender Goblin: %+v left %v alpha %v", gob.pose, gob.left, gob.alpha)
	}
	if bat.lift != 130.0/unitPx || !bat.look.Sheet {
		t.Errorf("Fledermaus: %+v", bat)
	}
	old := d.figs[key{0, classTroop, 2}]
	w.Troops[1].Kind = "peasant"
	w.Enemies = w.Enemies[:1]
	d.Listen(frameOf(1.2, w))
	if d.figs[key{0, classEnemy, 4}] != nil {
		t.Error("verschwundene Fledermaus bleibt")
	}
	if nw := d.figs[key{0, classTroop, 2}]; nw == old || nw.look.Asset != "truppen/bauer" {
		t.Errorf("Bauer nicht neu angelegt: %+v", nw)
	}
}

// Unbekannte Arten erscheinen nicht (Meldung einmal), die Pause hält die Darstellungszeit an.
func TestUnknownKindAndPause(t *testing.T) {
	d := newTestDrawer(t)
	w := &sim.World{Enemies: []*sim.Enemy{{ID: 1, Kind: "gibtsnicht"}}}
	d.Listen(frameOf(1, w))
	d.Listen(frameOf(2, w))
	if len(d.figs) != 0 || !d.warned[warnKey{"gegner", "gibtsnicht"}] {
		t.Errorf("unbekannter Gegner: %d Figuren", len(d.figs))
	}
	f := frameOf(3, w)
	f.Paused = true
	d.Listen(f)
	if d.now != 1 {
		t.Errorf("Zeit läuft in der Pause: %v", d.now)
	}
}
