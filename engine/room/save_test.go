package room

import (
	"bytes"
	"errors"
	"log/slog"
	"strings"
	"testing"
	"time"

	"k3c/engine/sim"
	"k3c/engine/store"
)

// Speichern beim Verlassen und Trennen (S2.2, B-147): jedes Gerät, das geht oder abbricht, löst eine Speicherung aus;
// ein Schreibfehler lässt den vorigen Stand unverändert.

// countStore zählt die Speicherungen und gibt sie an einen echten Ordner weiter; mit fail scheitert Store.
type countStore struct {
	*store.Saves
	n    int
	fail error
}

func (s *countStore) Store(name string, data []byte) (string, error) {
	s.n++
	if s.fail != nil {
		return "", s.fail
	}
	return s.Saves.Store(name, data)
}

// logged ist eine Fixture über einem echten Ordner, deren Log (ab Debug) in buf landet.
func logged(t *testing.T) (*fixture, *countStore, *bytes.Buffer) {
	t.Helper()
	f := newFixture()
	s := &countStore{Saves: &store.Saves{Dir: t.TempDir()}}
	buf := &bytes.Buffer{}
	f.m.Store = s
	f.m.Log = slog.New(slog.NewTextHandler(buf, &slog.HandlerOptions{Level: slog.LevelDebug}))
	return f, s, buf
}

// tickToNight beschleunigt den Zyklus aller Stufen und tickt, bis Nacht ist.
func tickToNight(t *testing.T, r *Room) {
	t.Helper()
	for _, w := range r.isl.Stages {
		w.CycleSpeed = 1000
	}
	for i := 0; r.isl.Stages[0].Cycle.Phase != "night"; i++ {
		if i > 300 {
			t.Fatalf("keine Nacht nach 10 s, Phase %q", r.isl.Stages[0].Cycle.Phase)
		}
		_ = r.Tick()
	}
}

// (a) AC-03/AC-04: Gerät 2 verlässt → eine Speicherung, Gerät 1 bricht ab (das letzte) → noch eine; der Stand lädt und
// nennt Zeitpunkt, Tag, Phase und Tiefen.
func TestSpeichernBeiVerlassenUndAbbruch(t *testing.T) {
	f, s, buf := logged(t)
	x, h := &peer{}, &peer{}
	r := need(f.m.Create("xbox", x, "nacht", true, 0, []int{0}, Options{}))(t)
	need(f.m.Join("handy", h, r.Code, []int{0}))(t)
	tickToNight(t, r)
	before := s.n
	r.Leave("handy", h)
	waitSaved(r)
	if s.n != before+1 {
		t.Fatalf("Verlassen bei verbliebenem Gerät: %d Speicherungen", s.n-before)
	}
	r.Drop("xbox", x)
	waitSaved(r)
	if s.n != before+2 {
		t.Fatalf("Abbruch des letzten Geräts: %d Speicherungen", s.n-before)
	}
	saved := need(sim.ParseIslandSave(need(s.Load("nacht"))(t)))(t)
	if saved.Phase != "night" || saved.Day < 1 || saved.SavedAt == "" || len(saved.Players) != 2 || saved.Players[1].Depth != 0 {
		t.Fatalf("Stand: Phase %q, Tag %d, savedAt %q, Spieler %+v", saved.Phase, saved.Day, saved.SavedAt, saved.Players)
	}
	if _, err := sim.FromIslandSave(saved, 1); err != nil {
		t.Fatalf("Stand lädt nicht: %v", err)
	}
	for _, line := range strings.Split(buf.String(), "\n") {
		if strings.Contains(line, "Spielstand gespeichert") {
			t.Log(line) // Speicherdauer (ms) für das Ergebnis der Session
		}
	}
}

// (b) AC-03: Scheitert das Schreiben, bleibt die Datei Byte für Byte gleich, das Log nennt den Fehler, und der leere
// Raum fährt nach EmptyFor trotzdem herunter.
func TestSchreibfehlerLaesstStandUnveraendert(t *testing.T) {
	f, s, buf := logged(t)
	x := &peer{}
	r := need(f.m.Create("xbox", x, "heil", true, 0, []int{0}, Options{}))(t)
	r.Leave("xbox", x)
	f.wait(EmptyFor + time.Minute)
	prev := need(s.Load("heil"))(t)

	s.fail = errors.New("platte voll")
	y := &peer{}
	r = need(f.m.Create("xbox", y, "heil", false, 0, []int{0}, Options{}))(t)
	ticks(r, 30)
	r.Leave("xbox", y)
	waitSaved(r)
	if now := need(s.Load("heil"))(t); !bytes.Equal(now, prev) {
		t.Fatal("die Datei hat sich trotz Schreibfehler geändert")
	}
	if !strings.Contains(buf.String(), "Spielstand nicht gespeichert") || !strings.Contains(buf.String(), "platte voll") {
		t.Fatalf("Log ohne Fehler:\n%s", buf.String())
	}
	f.wait(EmptyFor + time.Minute)
	if len(f.m.Rooms()) != 0 || !r.Closed() {
		t.Fatalf("Raum nach Schreibfehler nicht aufgeräumt: %d Räume", len(f.m.Rooms()))
	}
}
