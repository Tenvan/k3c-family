package hud

import (
	"math"
	"slices"
	"strings"
	"testing"
	"unicode/utf8"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

func TestFitBanner(t *testing.T) {
	if got := fitBanner("Nacht", bannerMaxChars); got != "Nacht" {
		t.Errorf("kurz bleibt: %q", got)
	}
	long := fitBanner("Die Nacht bricht an und die Gegner kommen aus allen Portalen gleichzeitig über die Mauer", bannerMaxChars)
	if utf8.RuneCountInString(long) > bannerMaxChars || !strings.HasSuffix(long, "…") {
		t.Errorf("lang wird mit … gekürzt: %q", long)
	}
	if got := fitBanner("Welle 2\nbeginnt", bannerMaxChars); got != "Welle 2 beginnt" {
		t.Errorf("einzeilig: %q", got)
	}
}

func TestEnqueueBanner(t *testing.T) {
	if got := enqueueBanner([]string{"Nacht"}, "Welle 1", "Welle 2"); !slices.Equal(got, []string{"Nacht", "Welle 2"}) {
		t.Errorf("hinten angehängt: %v", got)
	}
	if got := enqueueBanner(nil, "Nacht", "Nacht"); len(got) != 0 {
		t.Errorf("laufende Meldung nicht doppelt: %v", got)
	}
	if got := enqueueBanner([]string{"Nacht"}, "", "Nacht"); len(got) != 1 {
		t.Errorf("wartende Meldung nicht doppelt: %v", got)
	}
	if got := enqueueBanner([]string{"Nacht"}, "", "   "); len(got) != 1 {
		t.Errorf("leere Meldung nicht eingereiht: %v", got)
	}
}

// Viele Meldungen gleichzeitig: Warteschlange, je Meldung 2,8 s, bei mehr als zwei wartenden die Hälfte.
func TestBannerQueueTakt(t *testing.T) {
	var b bannerQueue
	for _, s := range []string{"A", "B", "C", "D"} {
		b.add(s)
	}
	b.step(0)
	if b.showing != "A" || b.until != bannerSeconds/2 {
		t.Fatalf("A mit halber Zeit (3 wartend): %q bis %v", b.showing, b.until)
	}
	b.step(1)
	if b.showing != "A" {
		t.Error("A läuft noch")
	}
	b.step(1.4)
	if b.showing != "B" || math.Abs(b.until-(1.4+bannerSeconds)) > 1e-9 {
		t.Errorf("B mit voller Zeit: %q bis %v", b.showing, b.until)
	}
	b.step(10)
	b.step(20)
	b.step(30)
	if b.showing != "" || len(b.queue) != 0 {
		t.Errorf("am Ende leer: %q %v", b.showing, b.queue)
	}
}

func TestEreignisTexte(t *testing.T) {
	ev := func(d sim.Event) view.Event { return view.Event{Data: d} }
	checkTexts(t, []textCase{
		{"Dämmerung", eventText("de", ev(sim.Event{"type": "dusk"}), ""), "Nacht naht!"},
		{"Nacht", eventText("de", ev(sim.Event{"type": "night", "day": 3}), ""), "Nacht 3"},
		{"Welle", eventText("de", ev(sim.Event{"type": "wave", "wave": 1, "count": 4}), ""), "Portal öffnet sich!\n4 Gegner kommen"},
		{"leere Welle", eventText("de", ev(sim.Event{"type": "wave", "count": 0}), ""), ""},
		{"Truhe", eventText("de", ev(sim.Event{"type": "chest", "player": 1, "gold": 5}), ""), "P2 findet 5 Gold"},
		{"gebaut", eventText("de", ev(sim.Event{"type": "built", "kind": "wall", "x": 3.5}), ""), "Mauer fertig"},
		{"zerstört englisch", eventText("en", ev(sim.Event{"type": "destroyed", "kind": "tower"}), ""), "Tower destroyed!"},
		{"geklaut", eventText("de", ev(sim.Event{"type": "goldStolen", "player": 0, "amount": 2}), ""), "P1: 2 Gold geklaut!"},
		{"gefallen", eventText("de", ev(sim.Event{"type": "playerDown", "player": 0}), ""), "P1 ist gefallen"},
		{"angekommen", eventText("de", ev(sim.Event{"type": "arrived", "name": "Höhle"}), "cave"), "Tiefe 1 (Höhle)"},
		{"geworben", eventText("de", ev(sim.Event{"type": "recruited"}), ""), ""},
		{"unbekannt", eventText("de", ev(sim.Event{"type": "zukunft"}), ""), ""},
	})
}
