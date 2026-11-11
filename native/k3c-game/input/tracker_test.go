package input

import (
	"math"
	"slices"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

type btn = ebiten.StandardGamepadButton

// padStep ist ein Update eines Gamepads mit den erwarteten Aktionen.
type padStep struct {
	bs   []btn
	x, y float64
	want []view.Action
}

func at(want []view.Action, bs ...btn) padStep { return padStep{bs: bs, want: want} }

func stick(x, y float64, want ...view.Action) padStep { return padStep{x: x, y: y, want: want} }

// times wiederholt ein Update n-mal.
func times(n int, s padStep) []padStep {
	out := make([]padStep, n)
	for i := range out {
		out[i] = s
	}
	return out
}

func seq(parts ...[]padStep) []padStep { return slices.Concat(parts...) }

func one(s padStep) []padStep { return []padStep{s} }

var (
	none  []view.Action
	home  = []view.Action{view.ActionHome}
	menu  = []view.Action{view.ActionMenu}
	viewA = []view.Action{view.ActionView}
	back  = []view.Action{view.ActionBack}
	up    = []view.Action{view.ActionUp}
	down  = []view.Action{view.ActionDown}
	downS = []view.Action{view.ActionDown, view.ActionSkillMenu} // Steuerkreuz ↓ gedrückt: auch das Skill-Menü
	conf  = []view.Action{view.ActionConfirm}
)

// runPad spielt die Updates nach einem leeren ersten Update ab und prüft die Aktionen je Update.
func runPad(t *testing.T, name string, steps []padStep) {
	t.Helper()
	var tr tracker
	idle := padState(0)
	h := tr.padHeld(&idle)
	tr.step(&h, true, nil)
	for i, s := range steps {
		p := padState(0, s.bs...)
		p.X, p.Y = s.x, s.y
		tr.padCommand(&p)
		h := tr.padHeld(&p)
		if got := tr.step(&h, true, nil); !slices.Equal(got, s.want) {
			t.Errorf("%s, Update %d: %v, erwartet %v", name, i+1, got, s.want)
		}
	}
}

// AC-03: View + Menu gemeinsam liefert nur ActionHome, nie zusätzlich Menu oder View; Menu kurz und View allein
// lösen beim Loslassen aus; B ist ActionBack (Zurück in Menüs, im Spiel wertet game sie nicht aus).
func TestPadMenuViewHome(t *testing.T) {
	cases := []struct {
		name  string
		steps []padStep
	}{
		{"View, dann Menu", seq(one(at(none, padView)), one(at(home, padView, padMenu)), one(at(none, padView)), one(at(none)))},
		{"Menu, dann View", seq(one(at(none, padMenu)), one(at(home, padMenu, padView)), one(at(none)))},
		{"beide im selben Update", seq(one(at(home, padMenu, padView)), times(3, at(none, padMenu, padView)), one(at(none)))},
		{"nach der Kombi Menu neu drücken", seq(one(at(home, padMenu, padView)), one(at(none, padView)), one(at(home, padMenu, padView)), one(at(none)))},
		{"Menu kurz", seq(times(3, at(none, padMenu)), one(at(menu)))},
		{"Menu genau 600 ms", seq(times(menuShort, at(none, padMenu)), one(at(menu)))},
		{"Menu zu lang", seq(times(menuShort+1, at(none, padMenu)), one(at(none)))},
		{"View allein", seq(times(30, at(none, padView)), one(at(viewA)))},
		{"B zurück", seq(one(at(back, padB)), times(20, at(none, padB)), one(at(none)))},
	}
	for _, c := range cases {
		runPad(t, c.name, c.steps)
	}
}

// Gehaltene Richtungen wiederholen nach repeatDelay alle repeatEvery Updates; andere Aktionen nie (auch nicht das
// Skill-Menü auf derselben Taste).
func TestRepeat(t *testing.T) {
	steps := seq(one(at(downS, padDown)), times(repeatDelay-1, at(none, padDown)), one(at(down, padDown)),
		times(repeatEvery-1, at(none, padDown)), one(at(down, padDown)), one(at(none)), one(at(downS, padDown)))
	runPad(t, "Steuerkreuz runter gehalten", steps)
	runPad(t, "A gehalten", seq(one(at(conf, padA)), times(40, at(none, padA))))
}

// Stick und Steuerkreuz sind im Menü gleichwertig; der Stick schaltet mit Hysterese (an über 0,5, aus unter 0,3).
func TestStickNav(t *testing.T) {
	cases := []struct {
		name  string
		steps []padStep
	}{
		{"Stick hoch mit Hysterese", seq(one(stick(0, -0.45)), one(stick(0, -0.6, view.ActionUp)), one(stick(0, -0.35)),
			one(stick(0, -0.55)), one(stick(0, -0.25)), one(stick(0, -0.6, view.ActionUp)))},
		{"Stick runter, links, rechts", seq(one(stick(0, 0.7, view.ActionDown)), one(stick(0, 0)),
			one(stick(-0.7, 0, view.ActionLeft)), one(stick(0, 0)), one(stick(0.7, 0, view.ActionRight)))},
		{"Steuerkreuz und Stick zugleich: eine Flanke", seq(one(at(up, padUp)),
			one(padStep{bs: []btn{padUp}, y: -1}), one(stick(0, -1)), one(stick(0, 0)))},
	}
	for _, c := range cases {
		runPad(t, c.name, c.steps)
	}
}

// Totzone des Lauf-Sticks mit Hysterese: an ab 0,25 (wie der Browser), aus erst unter 0,15; dazwischen der Rohwert.
func TestStickMoveHysteresis(t *testing.T) {
	cases := []struct {
		x       float64
		moving  bool
		wantPos bool
	}{
		{0.2, false, false},
		{0.3, true, true},
		{0.2, true, true},
		{0.1, false, false},
		{0.2, false, false},
		{-0.3, true, true},
		{-0.16, true, true},
		{-0.14, false, false},
		{math.NaN(), false, false},
	}
	var tr tracker
	if m := tr.stickMove(0.25); m != 0.25 {
		t.Errorf("Schwelle 0,25: %v", m)
	}
	tr.stickMove(0)
	for i, c := range cases {
		m := tr.stickMove(c.x)
		if tr.moving != c.moving || (m != 0) != c.wantPos || (m != 0 && math.Signbit(m) != math.Signbit(c.x)) {
			t.Errorf("Schritt %d (x=%v): Move %v, moving %v", i, c.x, m, tr.moving)
		}
	}
	if m := tr.stickMove(1); m != 1 {
		t.Errorf("voller Ausschlag: %v", m)
	}
	// Rohwert wie im Browser (kein Umskalieren ab der Schwelle)
	for _, x := range []float64{0.25, 0.5, -0.8} {
		if m := tr.stickMove(x); m != x {
			t.Errorf("x=%v: Move %v, erwartet den Rohwert", x, m)
		}
	}
	if m := tr.stickMove(1.2); m != 1 {
		t.Errorf("über 1: %v", m)
	}
}

// Ein Gerät, das mit gehaltenen Tasten kommt, löst nichts aus, bis die Tasten einmal losgelassen sind.
func TestFirstUpdateIgnoresHeld(t *testing.T) {
	var tr tracker
	steps := []struct {
		bs   []btn
		want []view.Action
	}{
		{[]btn{padA, padMenu, padView}, none},
		{[]btn{padA, padMenu, padView}, none},
		{[]btn{padA}, none},
		{nil, none},
		{[]btn{padA}, conf},
	}
	for i, s := range steps {
		p := padState(0, s.bs...)
		h := tr.padHeld(&p)
		if got := tr.step(&h, true, nil); !slices.Equal(got, s.want) {
			t.Errorf("Update %d: %v, erwartet %v", i+1, got, s.want)
		}
	}
}

// HomeChord: beide gehalten und mindestens eine gerade gedrückt.
func TestHomeChord(t *testing.T) {
	cases := []struct{ heldView, heldMenu, justView, justMenu, want bool }{
		{true, true, true, false, true},
		{true, true, false, true, true},
		{true, true, true, true, true},
		{true, true, false, false, false},
		{true, false, true, false, false},
		{false, true, false, true, false},
	}
	for i, c := range cases {
		if got := HomeChord(c.heldView, c.heldMenu, c.justView, c.justMenu); got != c.want {
			t.Errorf("Fall %d: %v", i, got)
		}
	}
}
