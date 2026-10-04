package room

import (
	"encoding/json"
	"strings"
	"testing"

	"k3c/engine/sim"
)

// scaledRoom ist ein Dev-Raum (devRoom) mit festen, verschiedenen Eingaben für beide Slots.
func scaledRoom(t *testing.T) (*Room, *peer) {
	t.Helper()
	r, p, _ := devRoom(t)
	ok(t, r.Input("a", p, map[int]sim.PlayerCommand{0: {MoveX: 1, Sprint: true}, 1: {MoveX: -1, Pay: true}}))
	return r, p
}

// state ist der Zustand der Insel: Welt je Stufe und Vorrat.
func state(t *testing.T, r *Room) string {
	t.Helper()
	r.mu.Lock()
	defer r.mu.Unlock()
	return string(need(json.Marshal(r.isl.Stages))(t)) + string(need(json.Marshal(r.isl.Stock))(t))
}

func factorOf(r *Room) int {
	r.mu.Lock()
	defer r.mu.Unlock()
	return r.scale()
}

// B-178/AC-04: ein Tick mit Faktor 4 (2) ergibt denselben Zustand wie 4 (2) normale Ticks bei gleichem Seed und
// gleicher Eingabe; Faktor 1 ist ein normaler Tick. Mehrere Ticks hintereinander, damit Tage und Wellen mitlaufen.
func TestDevTimescaleGleichNormalenTicks(t *testing.T) {
	for _, factor := range []int{1, 2, 4} {
		fast, p := scaledRoom(t)
		ok(t, fast.Dev("a", p, DevAction{Action: "timescale", Factor: factor}))
		slow, _ := scaledRoom(t)
		for range 30 {
			fast.Tick()
			ticks(slow, factor)
		}
		if state(t, fast) != state(t, slow) {
			t.Fatalf("Faktor %d: Zustand weicht von %d normalen Ticks ab", factor, 30*factor)
		}
		if fast.tick != 30 || slow.tick != 30*factor {
			t.Fatalf("Faktor %d: Tick-Zähler %d und %d", factor, fast.tick, slow.tick)
		}
	}
}

// Faktoren außerhalb 1, 2, 4, 8 ergeben bad_request, der Faktor bleibt.
func TestDevTimescaleUngueltig(t *testing.T) {
	r, p, _ := devRoom(t)
	ok(t, r.Dev("a", p, DevAction{Action: "timescale", Factor: MaxTimescale}))
	for _, f := range []int{3, 0, -1, 9, 16} {
		if err := r.Dev("a", p, DevAction{Action: "timescale", Factor: f}); err != ErrBadRequest {
			t.Fatalf("Faktor %d: %v", f, err)
		}
	}
	if got := factorOf(r); got != MaxTimescale {
		t.Fatalf("Faktor %d, erwartet %d", got, MaxTimescale)
	}
}

// Verlässt das letzte Gerät den Raum (pausiert), endet der Zeitraffer, auch nach Verbindungsabbruch; Log nennt ihn.
func TestDevTimescaleEndetBeiPause(t *testing.T) {
	for _, leave := range []bool{true, false} {
		r, p, out := devRoom(t)
		ok(t, r.Dev("a", p, DevAction{Action: "timescale", Factor: 4}))
		if leave {
			r.Leave("a", p)
		} else {
			r.Drop("a", p)
		}
		if got := factorOf(r); got != 1 {
			t.Fatalf("leave=%v: Faktor %d nach Pause", leave, got)
		}
		if !strings.Contains(out.String(), "Zeitraffer beendet") {
			t.Fatalf("leave=%v: Log ohne Ende des Zeitraffers:\n%s", leave, out)
		}
	}
}

// Zwei Spieler laufen im Zeitraffer beide weiter, jeder nach seiner Eingabe; der Faktor steht im Log.
func TestDevTimescaleZweiSpielerUndLog(t *testing.T) {
	r, p, out := devRoom(t)
	ok(t, r.Input("a", p, map[int]sim.PlayerCommand{0: {MoveX: 1}, 1: {MoveX: -1}}))
	ok(t, r.Dev("a", p, DevAction{Action: "timescale", Factor: 4}))
	ps := r.isl.Players()
	x0, x1 := ps[0].X, ps[1].X
	r.Tick()
	if ps[0].X <= x0 || ps[1].X >= x1 {
		t.Fatalf("Spieler laufen nicht: %v→%v, %v→%v", x0, ps[0].X, x1, ps[1].X)
	}
	log := out.String()
	if !strings.Contains(log, "Dev-Aktion") || !strings.Contains(log, "aktion=timescale") || !strings.Contains(log, "faktor=4") {
		t.Fatalf("Log ohne Faktor:\n%s", log)
	}
}

// B-231: In der Dev-Pause tickt der Raum ohne Schritt (Zustand bleibt, Tick-Zähler läuft); danach rechnet er weiter.
// pause ohne paused ist bad_request.
func TestDevPauseHaeltRaumAn(t *testing.T) {
	r, p := scaledRoom(t)
	if err := r.Dev("a", p, DevAction{Action: "pause"}); err != ErrBadRequest {
		t.Fatalf("ohne paused: %v", err)
	}
	on, off := true, false
	ok(t, r.Dev("a", p, DevAction{Action: "pause", Paused: &on}))
	before := state(t, r)
	ticks(r, 10)
	if state(t, r) != before || r.tick != 10 {
		t.Fatalf("angehalten: Zustand geändert oder Tick %d", r.tick)
	}
	ok(t, r.Dev("a", p, DevAction{Action: "pause", Paused: &off}))
	ticks(r, 1)
	if state(t, r) == before {
		t.Fatal("nach der Pause: Zustand unverändert")
	}
}
