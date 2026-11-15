package hud

import (
	"math"
	"testing"

	"k3c/native/k3c-game/view"
)

var _ view.VoyageDrawer = (*hud)(nil)

// B-977/AC-04: Die Lage des Schiffs ist eine reine Funktion der Zeit: gleiche Zeit, gleiche Lage; Start am Liegeplatz
// der verlassenen Insel, nach der Fahrzeit am Ziel (dort schaukelt es weiter), dazwischen über der geraden Strecke und
// nie rückwärts.
func TestVoyagePosIsPureFunctionOfTime(t *testing.T) {
	const ax, ay, bx, by, arc, bob, bobHz, dur = 100.0, 400.0, 900.0, 490.0, 28.0, 2.5, 0.7, 3.0
	pos := func(t float64) (float64, float64) { return voyagePos(ax, ay, bx, by, arc, bob, bobHz, t, dur) }
	for _, at := range []float64{0, 0.4, 1.5, 2.9, 3, 7.25} {
		x1, y1 := pos(at)
		x2, y2 := pos(at)
		if x1 != x2 || y1 != y2 {
			t.Errorf("t=%v: (%v, %v) und (%v, %v)", at, x1, y1, x2, y2)
		}
	}
	if x, y := pos(0); x != ax || y != ay {
		t.Errorf("Start: (%v, %v), will (%v, %v)", x, y, ax, ay)
	}
	for _, at := range []float64{dur, dur + 1.3, dur + 60} { // wartet am Ziel: x fest, y schaukelt um den Liegeplatz
		if x, y := pos(at); x != bx || math.Abs(y-by) > bob+1e-9 {
			t.Errorf("t=%v am Ziel: (%v, %v), will x %v und y %v ± %v", at, x, y, bx, by, bob)
		}
	}
	if _, y1 := pos(dur + 0.2); func() bool { _, y2 := pos(dur + 0.5); return y1 == y2 }() {
		t.Error("am Ziel steht das Schiff still (kein Schaukeln)")
	}
	if _, y := pos(dur / 2); y > (ay+by)/2-arc+bob {
		t.Errorf("Mitte der Strecke: y %v liegt nicht auf dem Bogen", y)
	}
	prev := ax
	for i := 1; i <= 90; i++ {
		x, _ := pos(float64(i) / 30)
		if x < prev {
			t.Fatalf("Takt %d: x %v nach %v (rückwärts)", i, x, prev)
		}
		prev = x
	}
}

// B-977/AC-04: Die Rechnung der Lage legt nichts an; die vordere Welle bewegt sich mit der Zeit.
func TestVoyagePosAllocatesNothing(t *testing.T) {
	var sink float64
	if n := testing.AllocsPerRun(100, func() {
		x, y := voyagePos(100, 400, 900, 490, 28, 2.5, 0.7, 1.25, 3)
		sink = x + y + voyageShift(14, 0.22, 1.25) + float64(voyageLane(0, 1)) + voyageEase(1.25, 3)
	}); n != 0 || sink == 0 {
		t.Errorf("Lage des Schiffs: %v Allokationen", n)
	}
	if voyageShift(14, 0.22, 0.5) == voyageShift(14, 0.22, 1.5) {
		t.Error("vordere Welle bewegt sich nicht")
	}
}

// GS2.4: Das Schiff bleibt die ganze Fahrt in einer Zeichen-Reihe, der vorderen von Start und Ziel: Es springt
// unterwegs nicht vor oder hinter eine Insel, gleich in welche Richtung die Fahrt geht.
func TestVoyageLaneStaysInFront(t *testing.T) {
	for _, c := range [][3]int{{0, 0, 0}, {0, 1, 1}, {1, 0, 1}, {2, 0, 2}, {1, 2, 2}} {
		if got := voyageLane(c[0], c[1]); got != c[2] {
			t.Errorf("Reihe von %d nach %d: %d, will %d", c[0], c[1], got, c[2])
		}
	}
}

// voyageOf ist die Fahrt von Grünhain nach Theben über die Karte der Aufnahme.
func voyageOf() *view.Voyage {
	return &view.Voyage{From: "gruenhain", To: "theben", Done: []string{"gruenhain"}, Open: []string{"kyoto", "theben", "avalon"},
		Duration: 3}
}

// Die Karte der Fahrt: Start und Ziel an ihrem Platz, Start abgeschlossen und ohne Schiff am Liegeplatz, Ziel gewählt,
// die übrigen Ziele bleiben Ziele, kein Cursor. Ein zweiter Aufruf mit derselben Fahrt baut nicht neu.
func TestVoyageViewBuildsMap(t *testing.T) {
	var y voyageView
	v := voyageOf()
	if !y.use("de", shotRoute, v) {
		t.Fatal("Karte der Fahrt nicht gebaut")
	}
	if y.from != (seaSpot{0, 0, 1}) || y.to != (seaSpot{1, 1, 3}) {
		t.Errorf("Plätze: Start %+v, Ziel %+v", y.from, y.to)
	}
	checkVoyageNodes(t, &y)
	if y.sea.hint != tr("de", "sea.voyage") || y.sea.title == "" {
		t.Errorf("Texte: %q, %q", y.sea.title, y.sea.hint)
	}
	if n := testing.AllocsPerRun(50, func() { y.use("de", shotRoute, v) }); n != 0 {
		t.Errorf("unveränderte Fahrt: %v Allokationen", n)
	}
	if y.use("en", shotRoute, v); y.sea.hint != tr("en", "sea.voyage") {
		t.Errorf("Sprachwechsel baut nicht neu: %q", y.sea.hint)
	}
}

// B-977/R5: Ohne Route, mit unbekannter Insel, ohne abgeschlossenen Start oder ohne Grafik ist die Fahrt nicht zu
// zeichnen (game zeigt den Ladebildschirm).
func TestVoyageNotReady(t *testing.T) {
	cases := map[string]struct {
		route []view.RouteIsland
		edit  func(*view.Voyage)
	}{
		"ohne Route":           {nil, func(*view.Voyage) {}},
		"Ziel unbekannt":       {shotRoute, func(v *view.Voyage) { v.To = "atlantis" }},
		"Start unbekannt":      {shotRoute, func(v *view.Voyage) { v.From, v.Done = "atlantis", []string{"atlantis"} }},
		"Start nicht abgelegt": {shotRoute, func(v *view.Voyage) { v.Done = nil }},
		"Start gleich Ziel":    {shotRoute, func(v *view.Voyage) { v.To = v.From }},
		"ohne Ziel":            {shotRoute, func(v *view.Voyage) { v.To = "" }},
	}
	for name, c := range cases {
		var y voyageView
		v := voyageOf()
		c.edit(v)
		if y.use("de", c.route, v) {
			t.Errorf("%s: Karte gebaut", name)
		}
	}
	h := New(nil).(*hud)
	f := &view.Frame{Route: shotRoute}
	if h.VoyageReady(f, voyageOf()) || h.DrawVoyage(nil, f, voyageOf()) {
		t.Error("ohne Grafik: Fahrt bereit oder gezeichnet")
	}
}

// checkVoyageNodes prüft die Inseln der Karte der Fahrt von Grünhain nach Theben.
func checkVoyageNodes(t *testing.T, y *voyageView) {
	t.Helper()
	from, to := y.node(y.from), y.node(y.to)
	if from.kind != seaDone || from.current || !to.chosen || to.kind != seaTarget {
		t.Errorf("Start %+v, Ziel %+v", *from, *to)
	}
	for _, r := range y.sea.rows {
		for _, n := range r.nodes {
			if n.cursor || n.current {
				t.Errorf("%s: Cursor %v, Schiff %v", n.id, n.cursor, n.current)
			}
			if want := n.id == "kyoto" || n.id == "theben" || n.id == "avalon"; (n.kind == seaTarget) != want {
				t.Errorf("%s: Zustand %v", n.id, n.kind)
			}
		}
	}
}
