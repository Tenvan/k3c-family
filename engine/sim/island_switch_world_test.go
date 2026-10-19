package sim

import "testing"

// Inselwechsel in einer Welt mit zwei Inseln (WZ18.3, B-488/AC-03, docs/rules/welten.md § 6): mit 2 Spielern zur
// nächsten Insel der Welt-Datei, Spielstand mit ihrer Insel-ID; die letzte Insel gilt je Welt.

// nordmeerSwitch: beide Spieler am Wechselpunkt von Insel 1 der Test-Welt, bis der Wechsel bereit ist; dann die
// nächste Insel nach NextDef.
func nordmeerSwitch(t *testing.T) (cur, next *Island) {
	t.Helper()
	w := nordmeer(t)
	cur = nordmeerIsland(t, w, 2)
	if cur.isLast() || cur.WorldID() != "nordmeer" || cur.IslandID() != "island1" {
		t.Fatalf("Insel 1: letzte %v, Welt %q, Insel %q", cur.isLast(), cur.WorldID(), cur.IslandID())
	}
	ev := stepSeconds(cur, hub.Travel.Seconds+0.5)
	if !cur.SwitchReady || count(ev, "islandSwitch") != 1 || count(ev, "victory") != 0 {
		t.Fatalf("bereit %v, %d× islandSwitch, %d× victory", cur.SwitchReady, count(ev, "islandSwitch"), count(ev, "victory"))
	}
	def, ok := cur.NextDef()
	if !ok || def.ID != "eisinsel" {
		t.Fatalf("nächste Insel %q, ok %v", def.ID, ok)
	}
	next, err := NextIsland(cur, def)
	if err != nil {
		t.Fatal(err)
	}
	return cur, next
}

func TestInselwechselInWeltMitZweiSpielern(t *testing.T) {
	_, next := nordmeerSwitch(t)
	if first := next.Stages[0]; len(first.Players) != 2 || first.Players[0].Index != 0 || first.Players[1].Index != 1 {
		t.Fatalf("Spieler auf Insel 2: %d", len(first.Players))
	}
	if s := next.ToSave("2026-10-19T20:00:00Z"); s.World != "nordmeer" || s.IslandID != "eisinsel" || next.Number != 1 {
		t.Fatalf("Spielstand: Welt %q, Insel %q, Nummer %d", s.World, s.IslandID, next.Number)
	}
}

func TestInselwechselLetzteInselDerWelt(t *testing.T) {
	_, next := nordmeerSwitch(t)
	if _, ok := next.NextDef(); ok || !next.isLast() {
		t.Fatal("Insel 2 ist die letzte der Welt")
	}
	next.EndbossDefeated = true // letzte Insel der Welt: kein Wechselpunkt, Sieg
	ev := stepSeconds(next, 3)
	if count(ev, "islandGateOpen") != 0 || next.SwitchReady || count(ev, "victory") != 1 {
		t.Errorf("letzte Insel: %d× islandGateOpen, bereit %v, %d× victory", count(ev, "islandGateOpen"), next.SwitchReady, count(ev, "victory"))
	}
}
