package sim

import (
	"math"
	"testing"
)

// KS7 (B-941, B-942): Auslöser und Banner vor dem äußersten Tor, Schilde erst ab Hub-Stufe, Gestell mit Stapel; mit 2 Spielern.

func TestAusloeserUndBannerLiegenAusserhalbDesTors(t *testing.T) {
	for _, mode := range []string{CombatGroups, CombatGeneral} {
		w, left, right := combatIsland(t, mode)
		kind := map[string]string{CombatGroups: "trigger", CombatGeneral: "banner"}[mode]
		out := w.values().Economy.Combat.GateOutUnits
		if out <= 0 {
			t.Fatalf("gateOutUnits %v", out)
		}
		if r := spotFor(w, kind, 1); math.Abs(r.X-(right+out)) > 1e-9 {
			t.Errorf("%s rechts: X %v, erwartet %v (Tor %v)", mode, r.X, right+out, right)
		}
		if l := spotFor(w, kind, -1); math.Abs(l.X-(left-out)) > 1e-9 {
			t.Errorf("%s links: X %v, erwartet %v (Tor %v)", mode, l.X, left-out, left)
		}
	}
}

func TestZahlstelleWeichtBauplaetzenAus(t *testing.T) {
	w, _, right := combatIsland(t, CombatGroups)
	c := w.values().Economy.Combat
	blocker := emptySite(w, "tower", right+c.GateOutUnits+0.5)
	w.Sites = append(w.Sites, blocker)
	got := spotFor(w, "trigger", 1).X
	if want := blocker.X + c.SpotClearUnits; math.Abs(got-want) > 1e-9 {
		t.Fatalf("Auslöser bei %v, erwartet hinter dem Bauplatz bei %v", got, want)
	}
	for _, s := range w.Sites {
		if s != blocker && s.Kind != "wall" && math.Abs(s.X-got) < c.SpotClearUnits {
			t.Errorf("Auslöser liegt auf Platz %s bei %v", s.Kind, s.X)
		}
	}
	if left := spotFor(w, "trigger", -1).X; math.Abs(left-(w.HubX-10-c.GateOutUnits)) > 1e-9 {
		t.Errorf("linke Seite unberührt, X %v", left)
	}
}

func TestSchildErstAbHubStufeMitGestell(t *testing.T) {
	w, _, _ := combatIsland(t, CombatGroups)
	min := w.values().Economy.Combat.ShieldMinHubLevel
	if min < 2 {
		t.Fatalf("shieldMinHubLevel %d", min)
	}
	w.HubLevel = min - 1
	if spotFor(w, "shield", 0).Kind != "" {
		t.Error("unter der Stufe darf es keine Schild-Zahlstelle geben")
	}
	if _, _, vis := ShieldRack(w); vis {
		t.Error("Gestell darf unter der Stufe nicht sichtbar sein")
	}
	w.HubLevel = min
	if spotFor(w, "shield", 0).Kind != "shield" {
		t.Error("ab der Stufe gibt es die Schild-Zahlstelle")
	}
	x, n, vis := ShieldRack(w)
	if !vis || n != 0 || x != spotFor(w, "shield", 0).X {
		t.Errorf("Gestell leer: x %v n %d sichtbar %v", x, n, vis)
	}
}

func TestGestellStapelBegrenztUndZahltNichtUeberMax(t *testing.T) {
	w, _, _ := combatIsland(t, CombatGroups)
	spawnSoldiers(w, 8)
	max := w.values().Economy.Combat.ShieldStackMax
	w.combat.shields = max + 2
	if _, n, _ := ShieldRack(w); n != max {
		t.Errorf("Stapel %d, erwartet höchstens %d", n, max)
	}
	for i := range w.Players { // beide Spieler: bei vollem Gestell nimmt die Zahlstelle nichts
		w.Players[i].Gold = 40
		if s := spotFor(w, "shield", 0); combatPayable(w, w.Players[i], s) {
			t.Errorf("Spieler %d: Schild zahlbar bei vollem Gestell", i)
		}
	}
	w.combat.shields = 2
	if _, n, _ := ShieldRack(w); n != 2 {
		t.Errorf("Stapel %d, erwartet 2", n)
	}
	if !combatPayable(w, w.Players[1], spotFor(w, "shield", 0)) {
		t.Error("Spieler 2 kann bei freiem Platz zahlen")
	}
}

// Review KS7: Ein Tor nahe dem Levelrand legt das Zahlziel nicht hinter den Rand.
func TestZahlstelleBleibtImLevel(t *testing.T) {
	w, _, right := combatIsland(t, CombatGroups)
	w.WidthUnits = right + 1
	if x := spotFor(w, "trigger", 1).X; x > w.WidthUnits || x < 0 {
		t.Fatalf("Auslöser bei %v hinter dem Rand %v", x, w.WidthUnits)
	}
}
