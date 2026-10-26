package room

import (
	"bytes"
	"log/slog"
	"strings"
	"testing"

	"k3c/engine/sim"
)

// B-178/AC-01: dev ohne Dev-Mode ist verboten (vor jeder Feldprüfung) und steht als Warnung im Log.
func TestDevOhneDevModeVerboten(t *testing.T) {
	f := newFixture()
	var out bytes.Buffer
	f.m.Log = slog.New(slog.NewTextHandler(&out, nil))
	p := &peer{}
	r := need(f.m.Create("geraet-1234567890", p, "dev", true, 0, []int{0}, Options{}))(t)
	for _, a := range []DevAction{{Action: "gold"}, {Action: "material"}, {Action: "timescale"}, {Action: "gibtsnicht"}} {
		if err := r.Dev("geraet-1234567890", p, a); err != ErrForbidden {
			t.Fatalf("%s: %v, erwartet forbidden", a.Action, err)
		}
	}
	log := out.String()
	if n := strings.Count(log, "Dev-Aktion abgelehnt"); n != 4 {
		t.Fatalf("%d Warnungen, erwartet 4:\n%s", n, log)
	}
	if !strings.Contains(log, "level=WARN") || !strings.Contains(log, "device=geraet-1") || !strings.Contains(log, "aktion=gold") {
		t.Fatalf("Warnung unvollständig:\n%s", log)
	}
}

// Mit Dev-Mode: fremde Verbindung und (noch) unbekannte Aktionen ergeben bad_request.
func TestDevMitDevModePrueftVerbindung(t *testing.T) {
	f := newFixture()
	f.m.Dev = true
	p := &peer{}
	r := need(f.m.Create("a", p, "dev", true, 0, []int{0}, Options{}))(t)
	if err := r.Dev("a", &peer{}, DevAction{Action: "gold"}); err != ErrBadRequest {
		t.Fatalf("fremde Verbindung: %v", err)
	}
	if err := r.Dev("b", p, DevAction{Action: "gold"}); err != ErrBadRequest {
		t.Fatalf("fremdes Gerät: %v", err)
	}
	if err := r.Dev("a", p, DevAction{Action: "gibtsnicht"}); err != ErrBadRequest {
		t.Fatalf("unbekannte Aktion: %v", err)
	}
}

// devRoom ist ein Dev-Raum mit zwei Slots eines Geräts und Log-Puffer.
func devRoom(t *testing.T) (*Room, *peer, *bytes.Buffer) {
	t.Helper()
	f := newFixture()
	f.m.Dev = true
	var out bytes.Buffer
	f.m.Log = slog.New(slog.NewTextHandler(&out, nil))
	p := &peer{}
	return need(f.m.Create("a", p, "dev", true, 0, []int{0, 1}, Options{}))(t), p, &out
}

// islPlayer ist der Insel-Spieler mit Index idx.
func islPlayer(t *testing.T, r *Room, idx int) *sim.Player {
	t.Helper()
	for _, w := range r.isl.Stages {
		for _, p := range w.Players {
			if p.Index == idx {
				return p
			}
		}
	}
	t.Fatalf("kein Spieler %d", idx)
	return nil
}

func slot(n int) *int { return &n }

// B-178/AC-02: gold fällt beim Monarchen des Slots (nicht beim anderen) und wird beim nächsten Tick aufgehoben.
func TestDevGold(t *testing.T) {
	r, p, _ := devRoom(t)
	m0, m1 := islPlayer(t, r, r.devices["a"].slots[0]), islPlayer(t, r, r.devices["a"].slots[1])
	m0.Gold, m1.Gold = 0, 0
	w := r.isl.Stages[r.isl.StageOf(m0.Index)]
	at := func(x float64) (n int) {
		for _, c := range w.Coins {
			if c.X == x {
				n++
			}
		}
		return n
	}
	before0, before1 := at(m0.X), at(m1.X)
	if err := r.Dev("a", p, DevAction{Action: "gold", Slot: slot(0), Amount: 20}); err != nil {
		t.Fatal(err)
	}
	if at(m0.X)-before0 != 20 || at(m1.X) != before1 {
		t.Fatalf("Münzen bei Monarch 0: +%d, bei Monarch 1: +%d", at(m0.X)-before0, at(m1.X)-before1)
	}
	r.Tick()
	if m0.Gold < 20 || m1.Gold != 0 {
		t.Fatalf("Gold Monarch 0 %d, Monarch 1 %d", m0.Gold, m1.Gold)
	}
}

// B-178/AC-02, AC-03: ungültige Felder ergeben bad_request.
func TestDevUngueltig(t *testing.T) {
	r, p, _ := devRoom(t)
	for name, a := range map[string]DevAction{
		"fremder Slot":        {Action: "gold", Slot: slot(3), Amount: 10},
		"amount 0":            {Action: "gold", Slot: slot(0), Amount: 0},
		"amount 1001":         {Action: "gold", Slot: slot(0), Amount: 1001},
		"ohne slot":           {Action: "gold", Amount: 10},
		"unbekannter Stoff":   {Action: "material", Slot: slot(0), Amount: 10, Resource: "gold"},
		"material amount 0":   {Action: "material", Slot: slot(0), Resource: "wood"},
		"material ohne slot":  {Action: "material", Amount: 10, Resource: "wood"},
		"material fremd Slot": {Action: "material", Slot: slot(2), Amount: 10, Resource: "wood"},
	} {
		if err := r.Dev("a", p, a); err != ErrBadRequest {
			t.Errorf("%s: %v, erwartet bad_request", name, err)
		}
	}
}

// B-178/AC-03: material erhöht den Insel-Vorrat bis zum Lager-Maximum, der Rest wird ohne Fehler verworfen.
func TestDevMaterial(t *testing.T) {
	r, p, _ := devRoom(t)
	wood := r.isl.Stock.Wood
	if err := r.Dev("a", p, DevAction{Action: "material", Slot: slot(1), Amount: 100, Resource: "wood"}); err != nil {
		t.Fatal(err)
	}
	if r.isl.Stock.Wood != wood+100 {
		t.Fatalf("Holz %d, erwartet %d", r.isl.Stock.Wood, wood+100)
	}
	for range 3 {
		if err := r.Dev("a", p, DevAction{Action: "material", Slot: slot(0), Amount: devMaxAmount, Resource: "stone"}); err != nil {
			t.Fatal(err)
		}
	}
	if limit := 300 * len(r.isl.Stages); r.isl.Stock.Stone != limit { // Lager-Maximum ohne gebautes Lager
		t.Fatalf("Stein %d, erwartet Maximum %d", r.isl.Stock.Stone, limit)
	}
}

// B-178/AC-06: je gelungener Dev-Aktion genau eine Info-Zeile mit Gerät, Aktion, Werten und Raum.
func TestDevLog(t *testing.T) {
	r, p, out := devRoom(t)
	_ = r.Dev("a", p, DevAction{Action: "gold", Slot: slot(1), Amount: 10})
	_ = r.Dev("a", p, DevAction{Action: "material", Slot: slot(0), Amount: 20, Resource: "iron"})
	_ = r.Dev("a", p, DevAction{Action: "gold", Slot: slot(3), Amount: 10}) // abgelehnt, keine Zeile
	var lines []string
	for _, l := range strings.Split(out.String(), "\n") {
		if strings.Contains(l, `msg="🐛 Dev-Aktion"`) {
			lines = append(lines, l)
		}
	}
	if len(lines) != 2 {
		t.Fatalf("%d Zeilen, erwartet 2:\n%s", len(lines), out.String())
	}
	for i, want := range [][]string{
		{"level=INFO", "device=a", "aktion=gold", "slot=1", "amount=10", "room=" + r.Code},
		{"aktion=material", "slot=0", "amount=20", "resource=iron", "genommen=20", "room=" + r.Code},
	} {
		for _, w := range want {
			if !strings.Contains(lines[i], w) {
				t.Errorf("Zeile %d ohne %q: %s", i, w, lines[i])
			}
		}
	}
}

// Dev-Aktionen für Lager, Baukosten, Ausbau und Tageszeit (nur mit Dev-Mode, Felder werden geprüft).
func TestDevLagerKostenAusbau(t *testing.T) {
	r, p, _ := devRoom(t)
	yes, no := true, false
	if err := r.Dev("a", p, DevAction{Action: "cheapBuild"}); err != ErrBadRequest {
		t.Fatalf("cheapBuild ohne on: %v", err)
	}
	if err := r.Dev("a", p, DevAction{Action: "buildAll", Level: 6}); err != ErrBadRequest {
		t.Fatalf("buildAll Stufe 6: %v", err)
	}
	ok(t, r.Dev("a", p, DevAction{Action: "maxStock"}))
	ok(t, r.Dev("a", p, DevAction{Action: "maxStock"}))
	ok(t, r.Dev("a", p, DevAction{Action: "cheapBuild", On: &yes}))
	ok(t, r.Dev("a", p, DevAction{Action: "buildAll", Level: 2}))
	ok(t, r.Dev("a", p, DevAction{Action: "fillStock"}))
	if g := islPlayer(t, r, 0).Gold; g != 30+200 {
		t.Fatalf("Gold %d nach fillStock, erwartet Maximum 230", g)
	}
	s := r.Summary()
	if s.DevStockPlus != 200 || s.DevPurseBonus != 200 || !s.DevCheap {
		t.Fatalf("Summary %+v", s)
	}
	if max := sim.EconomyOf(r.isl.Stages[0]).StockMax; max < 200 || r.isl.Stock.Wood != max || r.isl.Stages[0].HubLevel != 2 {
		t.Fatalf("Holz %d, Hub-Stufe %d", r.isl.Stock.Wood, r.isl.Stages[0].HubLevel)
	}
	ok(t, r.Dev("a", p, DevAction{Action: "cheapBuild", On: &no}))
	if r.Summary().DevCheap {
		t.Fatal("cheapBuild bleibt an")
	}
}

// Nächste Tageszeit und Zeitraffer-Auto: Der Zeitraffer endet im ersten Tick nach dem Tageszeitwechsel, einmalig.
func TestDevZeitrafferEndetMitTageszeit(t *testing.T) {
	r, p, _ := devRoom(t)
	yes := true
	ok(t, r.Dev("a", p, DevAction{Action: "timescale", Factor: 4}))
	ok(t, r.Dev("a", p, DevAction{Action: "timescaleAuto", On: &yes}))
	r.Tick()
	if factorOf(r) != 4 || !r.Summary().DevAutoReset {
		t.Fatal("Zeitraffer endet ohne Tageszeitwechsel")
	}
	ok(t, r.Dev("a", p, DevAction{Action: "nextPhase"}))
	r.Tick()
	if got := factorOf(r); got != 1 || r.Summary().DevAutoReset {
		t.Fatalf("Faktor %d, Auto %v nach Tageszeitwechsel", got, r.Summary().DevAutoReset)
	}
	ok(t, r.Dev("a", p, DevAction{Action: "timescale", Factor: 2}))
	ok(t, r.Dev("a", p, DevAction{Action: "nextPhase"}))
	r.Tick()
	if factorOf(r) != 2 {
		t.Fatal("Zeitraffer ohne Auto beendet")
	}
}

// Review TR4.2: Ein Beobachter löst keine Dev-Aktion aus, auch keine raumweite (Grad, Phase, Zeitraffer).
func TestDevBeobachterAbgelehnt(t *testing.T) {
	r, _, _ := devRoom(t)
	o := &peer{}
	if _, err := r.m.Observe("o", o, r.Code); err != nil {
		t.Fatal(err)
	}
	if err := r.Dev("o", o, DevAction{Action: "grade", Grade: "ultra"}); err != ErrBadRequest {
		t.Fatalf("Beobachter dev grade: %v", err)
	}
	if r.isl.Options.Grade == "ultra" {
		t.Fatal("Grad durch Beobachter geändert")
	}
}
