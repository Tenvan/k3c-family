package local

import (
	"io"
	"log/slog"
	"slices"
	"testing"
	"time"

	"k3c/engine/room"
	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// Aufnahme-Modus: CueSaved liegt im Frame des Takts, in dem der Raum das Speichern beginnt, unabhängig davon, wie
// lange das Schreiben dauert; zwei Läufe melden es im selben Takt.
func TestRecordingSavedCueFollowsTick(t *testing.T) {
	run := func() []uint64 {
		s := Open(Config{Dir: t.TempDir(), Now: fixedNow, Recording: true})
		if err := s.New(view.NewGame{Name: "aufnahme", Devices: twoDevices[:1]}); err != nil {
			t.Fatal(err)
		}
		var at []uint64
		for range int(room.SaveEvery/time.Second)*room.TickHz + 5 {
			if f := s.Step(); hasCue(f, view.CueSaved) {
				at = append(at, f.Tick)
			}
		}
		return at
	}
	a, b := run(), run()
	if len(a) == 0 || !slices.Equal(a, b) {
		t.Fatalf("CueSaved in Takt %v bzw. %v", a, b)
	}
	if want := uint64(int(room.SaveEvery/time.Second) * room.TickHz); !slices.Contains(a, want) {
		t.Errorf("Autosave nach SaveEvery nicht im Takt %d: %v", want, a)
	}
}

func outcomeSession() *Session {
	return &Session{p: newPeer(), log: slog.New(slog.NewTextHandler(io.Discard, nil)), name: "x"}
}

// Ausgang: victory bzw. gameOver unter den Ereignissen, sonst verloren, sobald die Spielzeit einer Stufe trotz Takt
// steht (Insel ruht nach gameOver in einer Stufe ohne lokalen Spieler). Der erste Ausgang bleibt.
func TestJudgeOutcome(t *testing.T) {
	s := outcomeSession()
	w := &sim.World{Time: 1}
	s.p.worlds[0] = w
	s.judge()
	w.Time = 1.5
	s.judge()
	if s.ended != view.OutcomeNone {
		t.Fatalf("laufende Insel: %v", s.ended)
	}
	s.judge()
	if s.ended != view.OutcomeLost {
		t.Errorf("stehende Insel: %v", s.ended)
	}
	s = outcomeSession()
	s.p.worlds[1] = &sim.World{}
	s.p.events = []view.Event{{Data: sim.Event{"type": "victory"}}, {Data: sim.Event{"type": "gameOver"}}}
	s.judge()
	if s.ended != view.OutcomeWon {
		t.Errorf("Sieg: %v", s.ended)
	}
	s.p.events = nil
	s.judge()
	if s.ended != view.OutcomeWon {
		t.Errorf("Ausgang bleibt: %v", s.ended)
	}
	s.p.worlds[1] = &sim.World{} // neue Welt derselben Stufe (Inselwechsel): kein Stillstand
	if s.p.stalled() {
		t.Error("neue Welt gilt als stehend")
	}
}

// Seats liefert einen eigenen Puffer: Frame.Seats (mit Lost von game) bleibt unberührt.
func TestSeatsOwnBuffer(t *testing.T) {
	s := testSession(t, t.TempDir())
	if len(s.Seats()) != 0 || s.Settings() != view.DefaultSettings() {
		t.Fatalf("ohne Spiel: %v %+v", s.Seats(), s.Settings())
	}
	if err := s.New(view.NewGame{Name: "plaetze", Devices: twoDevices}); err != nil {
		t.Fatal(err)
	}
	f := s.Step()
	f.Seats[0].Lost = true
	if got := s.Seats(); len(got) != 2 || got[1].Device != twoDevices[1] || !f.Seats[0].Lost {
		t.Errorf("Seats %+v, Frame %+v", got, f.Seats)
	}
}
