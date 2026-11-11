package fx

import (
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

func fxOf(d sim.Event, spots map[string]float64) (effect, bool) {
	px := func(i int) (float64, bool) { return 5, i == 0 }
	return effectFor("de", d, strOf(d, "type"), px, spots)
}

// effectFor (effects.test.ts, GR5.1 AC-01/AC-03): je Feedback-Ereignis genau ein Effekt am Ort des Ereignisses.
func TestEffektJeEreignis(t *testing.T) {
	cases := []struct {
		d    sim.Event
		kind kind
		x    float64
	}{
		{sim.Event{"type": "hit", "x": 3.0, "target": "enemy", "id": 1, "damage": 2.0}, kindHit, 3},
		{sim.Event{"type": "kill", "kind": "gnoll", "x": 4.0, "gold": 1}, kindKill, 4},
		{sim.Event{"type": "coinPickup", "player": 0, "x": 2.0}, kindCoin, 2},
		{sim.Event{"type": "coinGive", "player": 0, "x": 2.0, "to": "site"}, kindCoinGive, 2},
		{sim.Event{"type": "buildProgress", "site": 1, "kind": "wall", "x": 7.0, "percent": 25}, kindBuild, 7},
		{sim.Event{"type": "playerDown", "player": 0}, kindDown, 5},
		{sim.Event{"type": "playerDown", "player": 3, "x": 4.0}, kindDown, 4},
		{sim.Event{"type": "built", "kind": "tower", "x": 9.0}, kindBuilt, 9},
		{sim.Event{"type": "strike", "from": 4, "x": 10.0, "hit": false}, kindSwing, 10},
	}
	for _, c := range cases {
		e, ok := fxOf(c.d, map[string]float64{})
		if !ok || e.kind != c.kind || e.x != c.x {
			t.Errorf("%v: %+v %v", c.d, e, ok)
		}
	}
	for _, d := range []sim.Event{{"type": "dusk"}, {"type": "zukunft", "x": 1.0}, {"type": "playerDown", "player": 3},
		{"type": "strike", "from": 7, "x": 3.0}} {
		if _, ok := fxOf(d, map[string]float64{}); ok {
			t.Errorf("%v: kein Effekt", d)
		}
	}
}

func TestEffektBauOrtUndRueckmeldung(t *testing.T) {
	spots := map[string]float64{}
	if _, ok := fxOf(sim.Event{"type": "built", "kind": "wall"}, spots); ok {
		t.Error("built ohne Ort und ohne Baufortschritt: nichts")
	}
	fxOf(sim.Event{"type": "buildProgress", "kind": "wall", "x": 7.0}, spots)
	if e, ok := fxOf(sim.Event{"type": "built", "kind": "wall"}, spots); !ok || e.x != 7 {
		t.Errorf("built am Ort des letzten Baufortschritts: %+v", e)
	}
	if e, _ := fxOf(sim.Event{"type": "strike", "from": 9, "x": 12.0, "hit": true}, spots); e.by != 9 {
		t.Errorf("Ausholen gehört dem Spieler from: %d", e.by)
	}
	a, _ := fxOf(sim.Event{"type": "castFailed", "from": 1, "x": 10.0}, spots)
	b, _ := effectFor("en", sim.Event{"x": 40.0}, "castFailed", nil, spots)
	if a.text != "kein Ziel" || b.text != "no target" || a.x == b.x {
		t.Errorf("„kein Ziel“ am eigenen Ort: %+v %+v", a, b)
	}
}

func TestSchadenszahl(t *testing.T) {
	e, ok := damageFor(sim.Event{"type": "hit", "x": 3.0, "target": "enemy", "damage": 2.4})
	if !ok || e.text != "3" || e.target || e.kind != kindDamage {
		t.Errorf("Gegner: aufgerundete Zahl: %+v", e)
	}
	if e, _ := damageFor(sim.Event{"x": 1.0, "target": "player", "damage": 5.0}); e.text != "5" || !e.target {
		t.Errorf("Monarch getroffen: rot, %+v", e)
	}
	if _, ok := damageFor(sim.Event{"x": 1.0, "damage": 0.0}); ok {
		t.Error("ohne Schaden keine Zahl")
	}
}

// Blitzgrenze (effectRules.test.ts, GR5.2 AC-05): höchstens 3 Blitze je Sekunde.
func TestBlitzgrenze(t *testing.T) {
	last, ran := -1.0, 0
	for t0 := 0.0; t0 < 1; t0 += 1.0 / 60 {
		if flashAllowed(t0, last) {
			last, ran = t0, ran+1
		}
	}
	if ran > maxFlashesPerSecond {
		t.Errorf("%d Blitze in einer Sekunde", ran)
	}
	if flashAllowed(flashMinGap-0.01, 0) || !flashAllowed(flashMinGap, 0) {
		t.Error("Sperrfenster")
	}
}

func TestSpawnGrenzen(t *testing.T) {
	s := &stageFx{spots: map[string]float64{}, lastFlash: -1}
	for i := range 20 {
		s.spawn(effect{kind: kindKill, look: config[kindKill].look}, float64(i))
	}
	if s.pieces > maxLivePieces || len(s.effects) != maxLivePieces/7 {
		t.Errorf("höchstens %d Teile: %d in %d Effekten", maxLivePieces, s.pieces, len(s.effects))
	}
	s = &stageFx{spots: map[string]float64{}, lastFlash: -1}
	no := effect{kind: kindNoTarget, x: 10, text: "kein Ziel", look: config[kindNoTarget].look}
	s.spawn(no, 0)
	no.x = 10.5
	s.spawn(no, 0.1)
	if len(s.effects) != 1 {
		t.Error("derselbe Text an derselben Stelle nur einmal")
	}
	s.spawn(effect{kind: kindHit, look: config[kindHit].look}, 0)
	s.spawn(effect{kind: kindHit, look: config[kindHit].look}, 0.1)
	if len(s.effects) != 2 {
		t.Error("Blitz im Sperrfenster verworfen")
	}
	s.expire(0.5)
	if len(s.effects) != 1 || s.effects[0].kind != kindNoTarget {
		t.Errorf("abgelaufene fallen weg: %+v", s.effects)
	}
}

func TestSkillLooks(t *testing.T) {
	if l, ok := skillLook("fireball"); !ok || l.color != 0xdf7126 || l.motion != motionBurst {
		t.Errorf("eigener Look: %+v", l)
	}
	if _, ok := skillLook("unbekannt"); ok {
		t.Error("unbekannter Skill: kein Look")
	}
	e, ok := castFor(sim.Event{"type": "skillCast", "from": 1, "skill": "meteor", "x": 20.0, "radius": 3.0})
	if !ok || e.look.radius != 3*unitPx || e.x != 20 {
		t.Errorf("Flächenzauber mit dem Radius des Skills: %+v", e)
	}
}

func TestPartikel(t *testing.T) {
	if dx, dy := particleOffset(motionRise, 0, 3, 10); dx != -8 || dy != -20 {
		t.Errorf("rise: %v %v", dx, dy)
	}
	if dx, dy := particleOffset(motionWave, 1, 4, 10); dx != 12 || dy != 30 {
		t.Errorf("wave: %v %v", dx, dy)
	}
	ring, end := pieceAt(config[kindKill].look, 0, 0), pieceAt(config[kindKill].look, 0, 1)
	if ring.alpha != 0.8 || end.alpha != 0 || end.radius != 34*1.8 {
		t.Errorf("Ring wächst und blendet aus: %+v %+v", ring, end)
	}
	if p := pieceAt(config[kindKill].look, 1, 2); p.radius != 2.5 {
		t.Errorf("Fortschritt begrenzt: %+v", p)
	}
}

// Zwei Läufe mit denselben Ereignissen liefern dieselben Effekte (kein Zufall, Zeit nur aus Frame.Clock).
func TestListenDeterministisch(t *testing.T) {
	run := func() []effect {
		isl, err := sim.CreateIsland("wald", []int{0}, 1, sim.CreateOptions{})
		if err != nil {
			t.Fatal(err)
		}
		w := isl.Stages[0]
		sim.AddPlayer(w)
		d := New(nil).(*drawer)
		f := &view.Frame{Stages: []view.Stage{{Index: 0, World: w}}, Seats: []view.Seat{{Player: 0}}}
		for tick := range 90 {
			sim.Step(w, []sim.PlayerCommand{{MoveX: 1, Attack: true}}, 1.0/30)
			f.Clock, f.Events = float64(tick)/30, f.Events[:0]
			for _, e := range w.Events {
				f.Events = append(f.Events, view.Event{Stage: 0, Data: e})
			}
			f.Events = append(f.Events, view.Event{Stage: 0, Data: sim.Event{"type": "hit", "x": 3.0, "damage": float64(tick)}})
			d.Listen(f)
		}
		return append([]effect(nil), d.stage(0).effects...)
	}
	a, b := run(), run()
	if len(a) == 0 || len(a) != len(b) {
		t.Fatalf("Effekte: %d und %d", len(a), len(b))
	}
	for i := range a {
		if a[i] != b[i] {
			t.Errorf("Effekt %d verschieden", i)
		}
	}
}
