package room

import (
	"encoding/json"
	"testing"

	"k3c/engine/sim"
)

// BD3.1, B-214: Pause setzt den Zustand des eigenen Geräts; zweites pause und resume ohne Pause ändern nichts, eine
// fremde Verbindung ist bad_request.
func TestPauseSetztGeraet(t *testing.T) {
	f := newFixture()
	p := &peer{}
	r := need(f.m.Create("xbox", p, "pause", true, 0, []int{0}, Options{}))(t)
	for _, step := range []struct{ on, want bool }{{false, false}, {true, true}, {true, true}, {false, false}} {
		if err := r.Pause("xbox", p, step.on); err != nil {
			t.Fatal(err)
		}
		if got := r.devices["xbox"].paused; got != step.want {
			t.Fatalf("pause(%v): paused %v, erwartet %v", step.on, got, step.want)
		}
	}
	if err := r.Pause("xbox", &peer{}, true); err != ErrBadRequest {
		t.Fatalf("fremde Verbindung: %v", err)
	}
}

// worldJSON ist der Zustand aller Stufen des Raums.
func worldJSON(t *testing.T, r *Room) string {
	t.Helper()
	raw, err := json.Marshal(r.isl.Stages)
	if err != nil {
		t.Fatal(err)
	}
	return string(raw)
}

// BD3.3, B-214/AC-01: Im Couch-Raum (ein Gerät, 2 Spieler) steht der Zustand während der Pause, auch mit Eingaben;
// nach resume läuft er weiter.
func TestPauseCouchRaumSteht(t *testing.T) {
	f := newFixture()
	x := &peer{}
	r := need(f.m.Create("xbox", x, "couch", true, 0, []int{0, 1}, Options{}))(t)
	ticks(r, 5)
	need(0, r.Pause("xbox", x, true))(t)
	if err := r.Input("xbox", x, map[int]sim.PlayerCommand{0: {MoveX: 1}, 1: {MoveX: -1}}); err != nil {
		t.Fatal(err)
	}
	before := worldJSON(t, r)
	ticks(r, 90)
	if worldJSON(t, r) != before {
		t.Fatal("Zustand hat sich während der Couch-Pause geändert")
	}
	for _, p := range r.isl.Players() {
		if p.Protected {
			t.Fatal("Couch-Pause schützt Monarchen (unnötig, der Raum steht)")
		}
	}
	need(0, r.Pause("xbox", x, false))(t)
	if err := r.Input("xbox", x, map[int]sim.PlayerCommand{0: {MoveX: 1}}); err != nil {
		t.Fatal(err)
	}
	ticks(r, 30)
	if worldJSON(t, r) == before {
		t.Fatal("Zustand steht nach resume weiter")
	}
}

// online: Gerät A (xbox) und B (handy) mit je einem Monarchen; A läuft nach rechts, B nach rechts.
func onlineRoom(t *testing.T) (r *Room, a, b *peer, ia, ib int) {
	t.Helper()
	f := newFixture()
	a, b = &peer{}, &peer{}
	r = need(f.m.Create("xbox", a, "online", true, 0, []int{0}, Options{}))(t)
	need(f.m.Join("handy", b, r.Code, []int{0}))(t)
	ia, ib = r.devices["xbox"].slots[0], r.devices["handy"].slots[0]
	return r, a, b, ia, ib
}

// BD3.3, B-214/AC-02: Online steht der Monarch des pausierten Geräts still und ist 1800 Ticks (60 s) geschützt,
// danach verwundbar (steht weiter); das andere Gerät läuft unverändert, ungeschützt. Ein zweites pause verlängert nicht.
func TestPauseOnlineSchutz60s(t *testing.T) {
	r, a, b, ia, ib := onlineRoom(t)
	players := r.isl.Players()
	need(0, r.Pause("xbox", a, true))(t)
	_ = r.Input("xbox", a, map[int]sim.PlayerCommand{0: {MoveX: 1}})
	_ = r.Input("handy", b, map[int]sim.PlayerCommand{0: {MoveX: 1}})
	ax, bx := players[ia].X, players[ib].X
	ticks(r, 100)
	need(0, r.Pause("xbox", a, true))(t) // zweites pause: Frist läuft weiter
	if players[ia].X != ax || players[ib].X <= bx {
		t.Fatalf("A x %v → %v (soll stehen), B x %v → %v (soll laufen)", ax, players[ia].X, bx, players[ib].X)
	}
	ticks(r, protectTicks-101)
	if !players[ia].Protected || players[ib].Protected {
		t.Fatalf("Tick %d: A geschützt %v, B geschützt %v", protectTicks-1, players[ia].Protected, players[ib].Protected)
	}
	ticks(r, 1) // Tick 1800: letzter geschützter Tick
	if !players[ia].Protected {
		t.Fatal("Schutz endet vor 1800 Ticks")
	}
	ticks(r, 1)
	if players[ia].Protected || players[ia].X != ax {
		t.Fatalf("nach 60 s: geschützt %v, x %v → %v (soll verwundbar stehen)", players[ia].Protected, ax, players[ia].X)
	}
	_ = r.Input("xbox", a, map[int]sim.PlayerCommand{0: {MoveX: 1}}) // während der Pause: verfällt mit resume
	need(0, r.Pause("xbox", a, false))(t)
	ticks(r, 5)
	if players[ia].X != ax {
		t.Fatal("A läuft nach resume mit der Eingabe aus der Pause")
	}
	_ = r.Input("xbox", a, map[int]sim.PlayerCommand{0: {MoveX: 1}})
	ticks(r, 5)
	if players[ia].X == ax {
		t.Fatal("A läuft nach resume nicht")
	}
}

// BD3.3, B-214: Bricht die Verbindung des pausierten Geräts ab, endet die Pause (Frist laut § 3).
func TestPauseEndetBeiTrennung(t *testing.T) {
	r, a, _, ia, _ := onlineRoom(t)
	need(0, r.Pause("xbox", a, true))(t)
	ticks(r, 10)
	r.Drop("xbox", a)
	ticks(r, 1)
	if r.devices["xbox"].paused || r.isl.Players()[ia].Protected {
		t.Fatal("Pause oder Schutz besteht nach der Trennung")
	}
}

// BD3.3, B-214: Die Pause gilt für alle Spieler des Geräts (Split-Screen): Beide Monarchen der Xbox sind geschützt,
// der des Handys nicht; geht das Handy, ist es ein Couch-Raum und der Raum steht.
func TestPauseZweiSpielerAmGeraet(t *testing.T) {
	f := newFixture()
	x, h := &peer{}, &peer{}
	r := need(f.m.Create("xbox", x, "split", true, 0, []int{0, 1}, Options{}))(t)
	need(f.m.Join("handy", h, r.Code, []int{0}))(t)
	need(0, r.Pause("xbox", x, true))(t)
	ticks(r, 1)
	players := r.isl.Players()
	for slot, idx := range r.devices["xbox"].slots {
		if !players[idx].Protected {
			t.Fatalf("Xbox-Slot %d ungeschützt", slot)
		}
	}
	if players[r.devices["handy"].slots[0]].Protected {
		t.Fatal("Handy geschützt")
	}
	r.Leave("handy", h)
	before := worldJSON(t, r)
	ticks(r, 30)
	if worldJSON(t, r) != before {
		t.Fatal("Couch-Raum nach dem Gehen des Handys läuft weiter")
	}
}
