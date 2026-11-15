package hud

import (
	"slices"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// Geschichtszeile (B-974, GS1.3): Wendepunkte reihen die Zeilen der Engine in jeder Spielerhälfte ein, Ablauf nach
// Frame.Clock, keine Eingabe, keine Allokation in Ruhe. Welten mit Geschichte gibt es noch nicht (B-976): Testwelt.

// Zeilen der Testwelt: Inseln a (Etappe 1) und b (Etappe 2, letzte Insel der Welt).
const (
	lineStart1 = "Oru kamae KRONE no sen."
	lineStart2 = "Shiro nu BRENNT."
	lineArrA   = "Oru no REIS, sen SCHIFF nu."
	lineBossA  = "Nu kamae MASKE, shiro TIEF."
	lineDoneA  = "MASKE nu, KRONE oru sen."
	lineEtap1  = "Sen no MEER, oru KRONE kamae."
	lineArrB   = "Sen oru HAFEN, nu SEE."
	lineDoneB  = "Nu sen RUHE, oru BERG."
	lineEnd    = "Shiro no FRIEDE, kamae KRONE sen."
)

func storyWorld() *sim.WorldDef {
	return &sim.WorldDef{ID: "test",
		Story: sim.WorldStory{Start: []string{lineStart1, lineStart2}, Etappen: map[string][]string{"1": {lineEtap1}},
			End: []string{lineEnd}},
		Islands: []sim.IslandDef{
			{ID: "a", Etappe: 1, Story: sim.IslandStory{Arrival: []string{lineArrA}, Boss: []string{lineBossA}, Done: []string{lineDoneA}}},
			{ID: "b", Etappe: 2, Story: sim.IslandStory{Arrival: []string{lineArrB}, Done: []string{lineDoneB}}},
		}}
}

// storyFrame: zwei Spieler in verschiedenen Stufen einer Insel (Spieler 1 an der Oberfläche, Spieler 2 in der Tiefe),
// Kampagne der Testwelt auf Insel a, Spiel Nummer 1.
func storyFrame(t *testing.T) *view.Frame {
	t.Helper()
	isl, err := sim.CreateIsland("wald", []int{0, 1}, 1, sim.CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	a, b := sim.AddIslandPlayer(isl, 0), sim.AddIslandPlayer(isl, 1)
	return &view.Frame{
		Stages: []view.Stage{{Index: 0, World: isl.Stages[0]}, {Index: 1, World: isl.Stages[1]}},
		Seats:  []view.Seat{{Player: 0, Device: keys1, Monarch: a.Index, Stage: 0}, {Player: 1, Device: pad, Monarch: b.Index, Stage: 1}},
		Campaign: view.Campaign{Run: 1, World: storyWorld(), Island: "a"},
	}
}

// endBossID ist die ID eines Endbosses aus den Daten.
func endBossID(t *testing.T) string {
	t.Helper()
	for id := range endBossPhases {
		return id
	}
	t.Fatal("bosses.json ohne Endboss")
	return ""
}

func event(stage int, typ string, kv ...any) view.Event {
	d := sim.Event{"type": typ}
	for i := 0; i+1 < len(kv); i += 2 {
		d[kv[i].(string)] = kv[i+1]
	}
	return view.Event{Stage: stage, Data: d}
}

// told sind die Einträge des Spielers p: der laufende, dann die wartenden.
func told(h *hud, p int) []storyEntry {
	q := &h.seats[p].story
	if q.showing.a == "" {
		return slices.Clone(q.queue)
	}
	return append([]storyEntry{q.showing}, q.queue...)
}

// AC-01: Neue Kampagne, islandSwitch, bossSpawned und bossDefeated des Endbosses reihen die Zeilen der Engine ein; zwei
// Spieler in verschiedenen Stufen sehen dieselben Zeilen im selben Frame, auch bei einem Ereignis der anderen Stufe.
func TestGeschichteWendepunkte(t *testing.T) {
	end := endBossID(t)
	cases := []struct {
		name string
		set  func(f *view.Frame)
		want []storyEntry
	}{
		{"neue Kampagne", func(f *view.Frame) { f.Campaign.Fresh = true },
			[]storyEntry{{lineStart1, lineStart2}, {lineArrA, ""}}},
		{"Ankunft", func(f *view.Frame) {
			f.Campaign.Island, f.Campaign.Completed = "b", []string{"a"}
			f.Events = []view.Event{event(0, "islandSwitch", "island", 1, "target", "b")}
		}, []storyEntry{{lineArrB, ""}}},
		{"Wechsel gescheitert", func(f *view.Frame) { f.Events = []view.Event{event(0, "islandSwitch", "target", "b")} }, nil},
		{"Endboss in der Tiefe ausgelöst", func(f *view.Frame) { f.Events = []view.Event{event(1, "bossSpawned", "boss", end)} },
			[]storyEntry{{lineBossA, ""}}},
		{"Endboss besiegt, Etappe abgeschlossen", func(f *view.Frame) { f.Events = []view.Event{event(1, "bossDefeated", "boss", end)} },
			[]storyEntry{{lineDoneA, ""}, {lineEtap1, ""}}},
		{"Endboss der letzten Insel besiegt und Sieg", func(f *view.Frame) {
			f.Campaign.Island, f.Campaign.Completed = "b", []string{"a"}
			f.Events, f.Outcome = []view.Event{event(0, "bossDefeated", "boss", end)}, view.OutcomeWon
		}, []storyEntry{{lineDoneB, ""}, {lineEnd, ""}}},
		{"Sieg ohne Endboss auf der letzten Insel", func(f *view.Frame) {
			f.Campaign.Island, f.Campaign.Completed, f.Outcome = "b", []string{"a"}, view.OutcomeWon
		}, []storyEntry{{lineDoneB, ""}, {lineEnd, ""}}},
		{"Miniboss", func(f *view.Frame) {
			f.Events = []view.Event{event(0, "bossSpawned", "boss", "kein-endboss"), event(0, "bossDefeated", "boss", "kein-endboss")}
		}, nil},
		{"Spielstand geladen", func(*view.Frame) {}, nil},
		{"Welt ohne Geschichte", func(f *view.Frame) {
			f.Campaign.World, f.Campaign.Fresh = &sim.WorldDef{ID: "leer", Islands: []sim.IslandDef{{ID: "a"}}}, true
			f.Events = []view.Event{event(0, "bossSpawned", "boss", end)}
		}, nil},
	}
	for _, c := range cases {
		f := storyFrame(t)
		c.set(f)
		h := New(nil).(*hud)
		h.Listen(f)
		if !h.seats[0].active || !h.seats[1].active || h.seats[0].stage == h.seats[1].stage {
			t.Fatalf("%s: zwei Spieler in verschiedenen Stufen erwartet", c.name)
		}
		for p := range 2 {
			if got := told(h, p); !slices.Equal(got, c.want) {
				t.Errorf("%s, Spieler %d: %q, erwartet %q", c.name, p+1, got, c.want)
			}
		}
	}
}

// AC-02: einblenden, stehen, ausblenden nach Frame.Clock; zwei Einträge laufen nacheinander; Pause und Ladeanzeige
// halten die Einblendung an; ein Wendepunkt während der Einblendung wird angehängt, nichts erscheint doppelt.
func TestGeschichteAblauf(t *testing.T) {
	f := storyFrame(t)
	f.Campaign.Fresh, f.Clock = true, 100
	h := New(nil).(*hud)
	h.Listen(f)
	f.Campaign.Fresh = false
	q := &h.seats[1].story
	at := func(clock float64, want string, alpha float64) {
		t.Helper()
		f.Clock = clock
		h.Listen(f)
		if q.showing.a != want || q.alpha() != alpha {
			t.Errorf("Uhr %v: %q Deckkraft %v, erwartet %q %v", clock, q.showing.a, q.alpha(), want, alpha)
		}
	}
	at(100, lineStart1, 0)
	at(100.25, lineStart1, 0.5)
	at(100.5, lineStart1, 1)
	f.Paused = true
	at(103, lineStart1, 1) // Pause: die Einblendung steht
	f.Paused, f.Loading = false, true
	at(104, lineStart1, 1) // Ladeanzeige: steht
	f.Loading = false
	at(108, lineStart1, 1) // 4 s nach dem Einblenden (0,5 s davor, 3,5 s in Pause und Laden zählen nicht)
	at(108.25, lineStart1, 0.5)
	f.Events = []view.Event{event(0, "bossSpawned", "boss", endBossID(t)), event(1, "bossSpawned", "boss", endBossID(t))}
	at(108.5, lineArrA, 0) // der zweite Eintrag folgt; die Boss-Zeile hängt dahinter, einmal
	f.Events = nil
	if len(q.queue) != 1 || q.queue[0].a != lineBossA {
		t.Errorf("wartend %q, erwartet einmal die Boss-Zeile", q.queue)
	}
	at(113.5, lineBossA, 0)
	at(116, lineBossA, 1)
	at(118.5, "", 0)
	if n := len(told(h, 0)); n != 0 {
		t.Errorf("Spieler 1 hat noch %d Einträge", n)
	}
}

// Anderes Spiel oder kein Spiel verwirft die Einträge; wer später dazukommt, sieht erst den nächsten Wendepunkt.
func TestGeschichteNeuesSpielUndBeitritt(t *testing.T) {
	f := storyFrame(t)
	f.Campaign.Fresh = true
	both := f.Seats
	f.Seats = both[:1]
	h := New(nil).(*hud)
	h.Listen(f)
	f.Campaign.Fresh, f.Seats = false, both
	h.Listen(f)
	if len(told(h, 0)) != 2 || len(told(h, 1)) != 0 {
		t.Errorf("Beitritt: Spieler 1 %q, Spieler 2 %q", told(h, 0), told(h, 1))
	}
	f.Campaign.Run = 2 // Spielstand geladen, ohne Frame im Menü dazwischen
	h.Listen(f)
	if len(told(h, 0)) != 0 {
		t.Errorf("anderes Spiel: %q bleibt stehen", told(h, 0))
	}
	f.Campaign.Fresh = true
	h.Listen(f)
	h.Listen(&view.Frame{})
	if len(told(h, 0)) != 0 || len(told(h, 1)) != 0 {
		t.Error("ohne Spiel bleiben Einträge stehen")
	}
}

// AC-03: Die Einblendung verbraucht keine Eingabe und legt in Ruhe nichts an (mit und ohne laufenden Eintrag).
func TestGeschichteOhneEingabeUndAllokation(t *testing.T) {
	f := storyFrame(t)
	f.Campaign.Fresh = true
	h := New(nil).(*hud)
	h.Listen(f)
	f.Campaign.Fresh = false
	if h.seats[0].story.showing.a == "" {
		t.Fatal("keine Einblendung")
	}
	in := input(acts(view.ActionLeft, view.ActionConfirm), acts(view.ActionRight, view.ActionBack))
	if st := h.Update(in, f, &session{w: f.Stages[0].World}); st.Consumed[0] || st.Consumed[1] {
		t.Errorf("Einblendung verbraucht Eingabe: %+v", st.Consumed)
	}
	if h.seats[0].menu.open || h.seats[1].menu.open || h.seats[0].sea.open {
		t.Error("Einblendung öffnet ein Overlay")
	}
	step := func() {
		f.Clock += 1.0 / 30
		h.story(f)
	}
	if n := testing.AllocsPerRun(60, step); n != 0 {
		t.Errorf("laufende Einblendung legt %v Objekte je Frame an", n)
	}
	for range 3 { // beide Einträge laufen ab
		f.Clock += 60
		h.story(f)
	}
	if len(told(h, 0)) != 0 {
		t.Fatalf("Einblendung nicht abgelaufen: %q", told(h, 0))
	}
	if n := testing.AllocsPerRun(60, step); n != 0 {
		t.Errorf("Ruhezustand legt %v Objekte je Frame an", n)
	}
}

// Review: tell mischt keine Zeilen über die Grenze des ersten Eintrags, auch wenn er (gegen die Zeilenregel) mehr als
// zwei Zeilen hätte; EndHolding meldet game, solange ein Eintrag läuft oder wartet (Zeilen zum Sieg vor dem Hauptmenü).
func TestGeschichteEintraegeUndSpielende(t *testing.T) {
	f := storyFrame(t)
	h := New(nil).(*hud)
	h.Listen(f)
	if h.EndHolding() {
		t.Error("EndHolding ohne Eintrag")
	}
	cases := []struct {
		first int
		want  []storyEntry
	}{
		{0, []storyEntry{{"1", "2"}, {"3", "4"}, {"5", ""}}},
		{1, []storyEntry{{"1", ""}, {"2", "3"}, {"4", "5"}}},
		{2, []storyEntry{{"1", "2"}, {"3", "4"}, {"5", ""}}},
		{3, []storyEntry{{"1", "2"}, {"3", ""}, {"4", "5"}}},
		{9, []storyEntry{{"1", "2"}, {"3", "4"}, {"5", ""}}},
	}
	for _, c := range cases {
		h.seats[0].story.reset()
		h.tell([]string{"1", "2", "3", "4", "5"}, c.first)
		if got := told(h, 0); !slices.Equal(got, c.want) {
			t.Errorf("first %d: %q, erwartet %q", c.first, got, c.want)
		}
	}
	var _ view.EndHolder = h
	for n := 0; h.EndHolding(); n++ {
		if n > 10 {
			t.Fatal("EndHolding endet nicht")
		}
		f.Clock += storySeconds
		h.Listen(f)
	}
	if len(told(h, 0)) != 0 {
		t.Errorf("EndHolding false, aber %q steht noch an", told(h, 0))
	}
}
