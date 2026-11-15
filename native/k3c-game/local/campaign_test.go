package local

import (
	"slices"
	"testing"

	"k3c/engine/room"
	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// B-974: Frame.Campaign nennt Welt und Insel des laufenden Spiels; Fresh steht genau im ersten Frame einer neuen
// Kampagne, nie nach dem Laden eines Spielstands (welten.md § 6.2: Laden ist kein Wendepunkt).
func TestCampaignFreshOnlyOnNewGame(t *testing.T) {
	s := testSession(t, t.TempDir())
	start := sim.StandardWorld().Islands[0].ID
	check := func(when string, c view.Campaign, run uint64, fresh bool) {
		t.Helper()
		want := view.Campaign{Run: run, Fresh: fresh}
		if run > 0 {
			want.World, want.Island = c.World, start
		}
		if c.Run != want.Run || c.Fresh != want.Fresh || c.Island != want.Island || (c.World == nil) != (run == 0) || len(c.Completed) != 0 {
			t.Errorf("%s: %+v", when, c)
		}
	}
	check("ohne Spiel", s.Step().Campaign, 0, false)
	if err := s.New(view.NewGame{Devices: twoDevices}); err != nil {
		t.Fatal(err)
	}
	check("erster Frame des neuen Spiels", s.Step().Campaign, 1, true)
	check("zweiter Frame", s.Step().Campaign, 1, false)
	if err := s.Save(); err != nil {
		t.Fatal(err)
	}
	if err := s.New(view.NewGame{Name: "zwei", Devices: twoDevices}); err != nil {
		t.Fatal(err)
	}
	check("zweites Spiel", s.Step().Campaign, 2, true)
	if err := s.Load("spiel-1", twoDevices); err != nil {
		t.Fatal(err)
	}
	for range 3 {
		check("geladen", s.Step().Campaign, 3, false)
	}
}

// Der Stand folgt dem Inselwechsel: Die verlassene Insel gilt als abgeschlossen, das Ziel als aktuelle. Meldet der Takt
// den Wechsel, stehen die Spieler aber weiter am Wechselpunkt (der Raum hat nicht getauscht), bleibt alles.
func TestCampaignFollowsIslandSwitch(t *testing.T) {
	ev := []view.Event{{Data: sim.Event{"type": "night"}}, {Data: sim.Event{"type": "islandSwitch", "target": "b"}}}
	gate := &sim.World{IslandSwitch: &sim.SwitchState{Open: true, Ready: true}}
	p := &peer{events: ev, seats: []room.Seat{{Slot: 0, Stage: 1}}, worlds: map[int]*sim.World{0: gate, 1: {}}}
	c := campaign{island: "a"}
	c.follow(p) // Stufe 0 trägt noch die alte Insel, die Spieler stehen in der neuen
	if c.island != "b" || !slices.Equal(c.completed, []string{"a"}) {
		t.Errorf("nach dem Wechsel: Insel %q, abgeschlossen %v", c.island, c.completed)
	}
	c.follow(p) // derselbe Takt noch einmal: nichts doppelt
	if c.island != "b" || len(c.completed) != 1 {
		t.Errorf("doppelt gefolgt: %q %v", c.island, c.completed)
	}
	stuck := campaign{island: "a"}
	p.worlds[1] = gate
	stuck.follow(p)
	if stuck.island != "a" || len(stuck.completed) != 0 {
		t.Errorf("Wechsel nicht vollzogen: Insel %q, abgeschlossen %v", stuck.island, stuck.completed)
	}
	if n := testing.AllocsPerRun(20, func() { stuck.follow(p); _ = stuck.frame(true) }); n != 0 {
		t.Errorf("follow und frame legen %v Objekte an", n)
	}
}
