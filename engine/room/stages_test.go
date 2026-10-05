package room

import (
	"fmt"
	"slices"
	"testing"

	"k3c/engine/level"
)

// stagePeer schreibt Level und Zustand mit ihrer Stufe (Index in Island.Stages) mit: "level 1", "state 0" …
type stagePeer struct {
	peer
	msgs []string
}

func (p *stagePeer) Level(stage, depth int, l level.Layout) {
	p.peer.Level(stage, depth, l)
	p.msgs = append(p.msgs, fmt.Sprintf("level %d", stage))
}

func (p *stagePeer) State(tick, stage int, s any) {
	p.peer.State(tick, stage, s)
	p.msgs = append(p.msgs, fmt.Sprintf("state %d", stage))
}

// since sind die Nachrichten ab Index from.
func (p *stagePeer) since(from int) []string { return slices.Clone(p.msgs[from:]) }

// zweiStufen: Xbox mit Slots 0 und 1 (Monarchen 0 und 1), Handy mit Slot 0 (Monarch 2); Monarch 0 wechselt in Stufe 1.
func zweiStufen(t *testing.T) (*fixture, *Room, *stagePeer, *stagePeer) {
	t.Helper()
	f := newFixture()
	x, h := &stagePeer{}, &stagePeer{}
	r := need(f.m.Create("xbox", x, "stufen", true, 0, []int{0, 1}, Options{}))(t)
	need(f.m.Join("handy", h, r.Code, []int{0}))(t)
	moveToExit(t, r, 0)
	ticks(r, 70)
	if r.isl.StageOf(0) != 1 || r.isl.StageOf(1) != 0 || r.isl.StageOf(2) != 0 {
		t.Fatalf("Stufen: %v", r.depths())
	}
	return f, r, x, h
}

// beginsWithLevel: Für Stufe s kommt kein Zustand vor ihrem Level, und das Level steht vor dem ersten Zustand.
func beginsWithLevel(msgs []string, s int) bool {
	lv, st := slices.Index(msgs, fmt.Sprintf("level %d", s)), slices.Index(msgs, fmt.Sprintf("state %d", s))
	return lv >= 0 && st > lv
}

// (a) B-176/AC-02: Ein Gerät mit zwei Spielern in zwei Stufen bekommt Level und Zustand beider Stufen, je Tick aufsteigend;
// ein Gerät mit einem Spieler nur seine Stufe. seats nennt die Stufe je Slot.
func TestStufenZweiSpielerEinGeraet(t *testing.T) {
	_, r, x, h := zweiStufen(t)
	if !beginsWithLevel(x.msgs, 1) || slices.Contains(h.msgs, "level 1") || slices.Contains(h.msgs, "state 1") {
		t.Fatalf("Xbox %v, Handy %v", x.msgs, h.msgs)
	}
	xi, hi := len(x.msgs), len(h.msgs)
	ticks(r, 2)
	if got := x.since(xi); !slices.Equal(got, []string{"state 0", "state 1", "state 0", "state 1"}) {
		t.Fatalf("Xbox je Tick: %v", got)
	}
	if got := h.since(hi); !slices.Equal(got, []string{"state 0", "state 0"}) {
		t.Fatalf("Handy je Tick: %v", got)
	}
	if fmt.Sprint(x.you) != "[{0 0 1 1} {1 1 0 0}]" {
		t.Fatalf("seats: %v", x.you)
	}
}

// (b) B-176/AC-03: Verlässt der einzige Spieler eines Geräts seine Stufe, endet der alte Strom; der neue beginnt mit
// Level und Zustand. Verlässt einer von zwei Spielern eine Stufe, die das Gerät weiter hat, kommt kein neues Level.
func TestStufenWechselBeendetAltenStrom(t *testing.T) {
	_, r, x, h := zweiStufen(t)
	hi, xi := len(h.msgs), len(x.msgs)
	moveToExit(t, r, 2)
	ticks(r, 70)
	if r.isl.StageOf(2) != 1 {
		t.Fatalf("Handy-Monarch in Stufe %d", r.isl.StageOf(2))
	}
	got := h.since(hi)
	lv := slices.Index(got, "level 1")
	if !beginsWithLevel(got, 1) || slices.Contains(got[lv:], "state 0") {
		t.Fatalf("Handy: %v", got)
	}
	moveToExit(t, r, 1) // der zweite Xbox-Spieler folgt in Stufe 1, Stufe 0 hat kein Xbox-Slot mehr
	ticks(r, 70)
	got = x.since(xi)
	last := slices.Index(got, "state 1")
	for i, m := range got {
		if m == "state 0" {
			last = i
		}
	}
	if slices.Contains(got, "level 1") || slices.Contains(got, "level 0") || slices.Contains(got[last+1:], "state 0") {
		t.Fatalf("Xbox: %v", got)
	}
	if n := len(got) - last - 1; n < 10 || slices.ContainsFunc(got[last+1:], func(m string) bool { return m != "state 1" }) {
		t.Fatalf("Xbox nach dem Wechsel: %v", got[last+1:])
	}
}

// (c) B-176/AC-03: Wiederverbinden liefert Level und Zustand aller Stufen des Geräts, aufsteigend.
func TestStufenWiederverbinden(t *testing.T) {
	f, r, x, _ := zweiStufen(t)
	r.Drop("xbox", x)
	x2 := &stagePeer{}
	need(f.m.Join("xbox", x2, r.Code, []int{0, 1}))(t)
	if !slices.Equal(x2.msgs, []string{"level 0", "state 0", "level 1", "state 1"}) {
		t.Fatalf("nach Wiederverbinden: %v", x2.msgs)
	}
	ticks(r, 1)
	if got := x2.since(4); !slices.Equal(got, []string{"state 0", "state 1"}) {
		t.Fatalf("danach: %v", got)
	}
}
