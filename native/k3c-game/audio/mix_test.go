package audio

import (
	"math"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// stageWorld baut eine Welt mit Monarchen an den Orten xs (Index = Position) zur Tageszeit phase.
func stageWorld(phase string, xs ...float64) *sim.World {
	w := &sim.World{Cycle: sim.CycleInfo{Phase: phase}}
	for i, x := range xs {
		w.Players = append(w.Players, &sim.Player{Index: i, X: x})
	}
	return w
}

// gameFrame ist ein Frame mit einer Stufe und n lokalen Spielern darin.
func gameFrame(w *sim.World, n int, events ...sim.Event) *view.Frame {
	f := &view.Frame{Settings: view.DefaultSettings(), Stages: []view.Stage{{Index: 0, World: w}}}
	for i := range n {
		f.Seats = append(f.Seats, view.Seat{Player: i, Monarch: i})
	}
	for _, e := range events {
		f.Events = append(f.Events, view.Event{Data: e})
	}
	return f
}

func effect(m *Mix, slot string) (float64, int) {
	n, g := 0, 0.0
	for _, r := range m.Effects {
		if r.Slot == slot {
			n++
			g = r.Gain
		}
	}
	return g, n
}

// Beispiel B-801: Monarch zahlt eine Münze → Münz-Klang, voll im Bild, nachts leiser.
func TestPlanCoinGive(t *testing.T) {
	p := newPlanner(realRegister(t))
	m := p.plan(gameFrame(stageWorld("day", 10), 1, sim.Event{"type": "coinGive", "x": 10.0}))
	if g, n := effect(m, "sfx:coinGive"); n != 1 || g != 1 {
		t.Errorf("Tag: %v ×%d", g, n)
	}
	m = p.plan(gameFrame(stageWorld("night", 10), 1, sim.Event{"type": "coinGive", "x": 10.0}))
	if g, _ := effect(m, "sfx:coinGive"); g != nightGainVal {
		t.Errorf("Nacht: %v", g)
	}
}

// Drosselung: viele gleiche Klänge in einem Takt klingen einmal, mit dem lautesten Pegel; ungleiche bleiben getrennt.
func TestPlanThrottlesSameSlotPerTick(t *testing.T) {
	p := newPlanner(realRegister(t))
	far := 10 + spanPerSeat // außerhalb des Bilds: gedämpft
	m := p.plan(gameFrame(stageWorld("day", 10), 1,
		sim.Event{"type": "hit", "x": far}, sim.Event{"type": "hit", "x": 10.0}, sim.Event{"type": "hit", "x": far},
		sim.Event{"type": "kill", "x": far}, sim.Event{"type": "unbekannt", "x": 10.0}))
	if g, n := effect(m, "sfx:hit"); n != 1 || g != 1 {
		t.Errorf("hit: %v ×%d", g, n)
	}
	if g, n := effect(m, "sfx:kill"); n != 1 || g <= 0 || g >= edgeGain {
		t.Errorf("kill gedämpft: %v ×%d", g, n)
	}
	if len(m.Effects) != 2 {
		t.Errorf("Aufträge %+v", m.Effects)
	}
	if m = p.plan(gameFrame(stageWorld("day", 10), 1)); len(m.Effects) != 0 {
		t.Error("nächster Takt ohne Ereignis: still")
	}
}

// Zwei Spieler: Klänge beider Ausschnitte, nichts doppelt; das nähere Ohr bestimmt den Pegel.
func TestPlanTwoPlayers(t *testing.T) {
	p := newPlanner(realRegister(t))
	w := stageWorld("day", 0, 500)
	m := p.plan(gameFrame(w, 2, sim.Event{"type": "coinPickup", "x": 500.0}, sim.Event{"type": "arrow", "x": 0.0}))
	for _, slot := range []string{"sfx:coin", "sfx:arrow"} {
		if g, n := effect(m, slot); n != 1 || g != 1 {
			t.Errorf("%s: %v ×%d", slot, g, n)
		}
	}
	solo := newPlanner(realRegister(t)).plan(gameFrame(w, 1, sim.Event{"type": "coinPickup", "x": 500.0}))
	if g, _ := effect(solo, "sfx:coin"); g != 0 {
		t.Errorf("allein weit weg: %v", g)
	}
}

// Warnung: global (ungedämpft) und senkt die Musik; skillCast nach Linie.
func TestPlanWarnAndSkill(t *testing.T) {
	p := newPlanner(realRegister(t))
	w := stageWorld("day", 0)
	for range crossfadeTicks {
		p.plan(gameFrame(w, 1))
	}
	if g := p.mix.Music[0].Gain; g != 1 {
		t.Fatalf("Tagesmusik %v", g)
	}
	p.plan(gameFrame(w, 1, sim.Event{"type": "dusk"}, sim.Event{"type": "skillCast", "skill": "fireball", "x": 0.0}))
	for range int(math.Ceil(duckAttack * tps)) {
		p.plan(gameFrame(w, 1))
	}
	if g := p.mix.Music[0].Gain; math.Abs(g-duckLevel) > 1e-9 {
		t.Errorf("Ducking: %v", g)
	}
	m := p.plan(gameFrame(w, 1, sim.Event{"type": "wave", "x": 900.0}, sim.Event{"type": "skillCast", "skill": "heal", "x": 0.0}))
	if g, n := effect(m, "sfx:warn"); n != 1 || g != 1 {
		t.Errorf("Warnung: %v ×%d", g, n)
	}
	if _, n := effect(m, "sfx:skill:healer"); n != 1 {
		t.Error("Skill-Klang der Linie fehlt")
	}
}

func TestPlanHubAndMenuCues(t *testing.T) {
	p := newPlanner(realRegister(t))
	w := stageWorld("day", 0)
	w.HubLevel = 1
	if _, n := effect(p.plan(gameFrame(w, 1)), slotHub); n != 0 {
		t.Error("erstes Sehen klingt nicht")
	}
	w.HubLevel = 2
	if _, n := effect(p.plan(gameFrame(w, 1)), slotHub); n != 1 {
		t.Error("Ausbau klingt")
	}
	f := &view.Frame{Settings: view.DefaultSettings(), Cues: []view.Cue{{Kind: view.CueMove}, {Kind: view.CueConfirm}, {Kind: view.CuePause}}}
	m := p.plan(f)
	if len(m.Effects) != 2 || m.Effects[0].Slot != slotUISelect || m.Effects[1].Slot != slotUIConfirm {
		t.Errorf("Menü-Klänge %+v", m.Effects)
	}
}

// Beispiel B-801: Nacht beginnt → Musik wechselt mit Überblenden; Menü ohne Spiel = Lobby; Burg fällt = Niederlage.
func TestPlanMusicFollowsState(t *testing.T) {
	p := newPlanner(realRegister(t))
	menu := &view.Frame{Settings: view.DefaultSettings()}
	p.plan(menu)
	if p.state != stateLobby || p.mix.Music[0].Slot != "music:lobby" {
		t.Fatalf("Menü: %d %+v", p.state, p.mix.Music)
	}
	w := stageWorld("day", 0)
	p.plan(gameFrame(w, 1))
	w.Cycle.Phase = "night"
	m := p.plan(gameFrame(w, 1, sim.Event{"type": "night"}))
	if m.Music[0].Slot != "music:nacht" || m.Music[1].Slot != "music:tag" {
		t.Errorf("Nacht blendet über: %+v", m.Music)
	}
	w.Enemies = []*sim.Enemy{{}}
	if p.plan(gameFrame(w, 1)); p.state != stateBattle {
		t.Errorf("Gegner: %d", p.state)
	}
	p.plan(gameFrame(w, 1, sim.Event{"type": "castleFallen"}))
	w.Enemies = nil
	if p.plan(gameFrame(w, 1)); p.state != stateDefeat {
		t.Errorf("Niederlage bleibt: %d", p.state)
	}
	if p.plan(menu); p.state != stateLobby {
		t.Errorf("zurück im Menü: %d", p.state)
	}
}

// Hufschlag je reitendem Monarch; in der Pause und für Stehende keiner; Lücke im Register = Stille.
func TestPlanRideLoopsAndPause(t *testing.T) {
	p := newPlanner(realRegister(t))
	w := stageWorld("night", 0, 3, 5)
	w.Players[0].VX = 2
	w.Players[1].VX = 0.1
	w.Players[2].VX, w.Players[2].RespawnIn = 2, 3
	m := p.plan(gameFrame(w, 1))
	if len(m.Loops) != 1 || m.Loops[0].Key != 0 || m.Loops[0].Gain != nightGainVal*rideGain {
		t.Errorf("Schleifen %+v", m.Loops)
	}
	f := gameFrame(w, 1)
	f.Paused = true
	if m = p.plan(f); len(m.Loops) != 0 {
		t.Error("Pause: kein Hufschlag")
	}
	empty := newPlanner(&Register{})
	if m = empty.plan(gameFrame(w, 1)); m.Music[0].Slot != "" {
		t.Errorf("ohne Register still: %+v", m.Music)
	}
}

// Der Listen-Pfad legt im Takt ohne neuen Klang nichts an (Puffer werden wiederverwendet).
func TestPlanNoAllocsPerFrame(t *testing.T) {
	p := newPlanner(realRegister(t))
	w := stageWorld("day", 0, 4)
	w.Players[1].VX = 2
	f := gameFrame(w, 2)
	p.plan(f)
	if n := testing.AllocsPerRun(100, func() { p.plan(f) }); n != 0 {
		t.Errorf("%v Allokationen je Frame", n)
	}
}

// Mit Zellen im Frame gilt der sichtbare Bereich der Kamera, nicht die feste Breite um den Monarchen.
func TestEarsFollowCellCamera(t *testing.T) {
	p := newPlanner(realRegister(t))
	f := gameFrame(stageWorld("day", 10), 1, sim.Event{"type": "hit", "x": 100.0})
	f.Cells = []view.Cell{{Player: 0, Camera: view.Camera{X: 100, Scale: 10, Half: 200}}}
	if g, _ := effect(p.plan(f), "sfx:hit"); g != 1 {
		t.Errorf("im Bild der Kamera: %v", g)
	}
	f = gameFrame(stageWorld("day", 100), 1, sim.Event{"type": "hit", "x": 100.0})
	f.Cells = []view.Cell{{Player: 0, Camera: view.Camera{X: 10, Scale: 10, Half: 200}}}
	if g, _ := effect(p.plan(f), "sfx:hit"); g >= edgeGain {
		t.Errorf("außerhalb der Kamera, am Monarchen: %v", g)
	}
}
