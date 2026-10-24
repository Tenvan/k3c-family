package room

import (
	"bytes"
	"errors"
	"strings"
	"testing"
	"time"
)

// KB2.2 (B-344, KB2/AC-02): Nach „Komplett verloren“ (Island.Over) schreibt der Raum keinen Spielstand mehr; der Stand
// von vor dem Game Over bleibt bei Verlassen, Diagnose-Sicherung, Aufräumen und Herunterfahren Byte für Byte gleich.

func TestGameOverLaesstSpielstandUnveraendert(t *testing.T) {
	f, s, buf := logged(t)
	x := &peer{}
	r := need(f.m.Create("xbox", x, "verloren", true, 0, []int{0}, Options{Defeat: "lost"}))(t)
	ticks(r, 30)
	r.Leave("xbox", x)
	waitSaved(r)
	prev := need(s.Load("verloren"))(t)
	writes := s.n

	h := &peer{}
	need(f.m.Join("handy", h, r.Code, []int{0}))(t)
	need(f.m.Join("xbox", x, r.Code, []int{0}))(t)
	ticks(r, 30)
	r.mu.Lock()
	r.isl.Over = true
	r.mu.Unlock()
	ticks(r, 30)
	r.Leave("handy", h) // Verlassen bei verbliebenem Gerät
	waitSaved(r)
	if _, err := r.SaveNow(); !errors.Is(err, ErrGameOver) {
		t.Errorf("Diagnose-Sicherung nach Game Over: %v, erwartet game_over", err)
	}
	r.Drop("xbox", x) // Abbruch des letzten Geräts
	waitSaved(r)
	f.wait(EmptyFor + time.Minute) // Aufräumen des leeren Raums (saveNow)
	if !r.Closed() {
		t.Fatal("leerer Raum nach Game Over nicht aufgeräumt")
	}
	if s.n != writes {
		t.Errorf("%d Speicherungen nach Game Over", s.n-writes)
	}
	if now := need(s.Load("verloren"))(t); !bytes.Equal(now, prev) {
		t.Fatal("Spielstand nach Game Over überschrieben")
	}
	if !strings.Contains(buf.String(), "🚫 Spielstand nicht gespeichert") {
		t.Fatalf("Log ohne 🚫:\n%s", buf.String())
	}
}

// Herunterfahren nach Game Over schreibt nicht; ohne vorherigen Stand entsteht keine Datei.
func TestGameOverOhneStandBeimHerunterfahren(t *testing.T) {
	f, s, _ := logged(t)
	r := need(f.m.Create("xbox", &peer{}, "neu", true, 0, []int{0}, Options{Defeat: "lost"}))(t)
	ticks(r, 10)
	r.mu.Lock()
	r.isl.Over = true
	r.mu.Unlock()
	f.m.Close()
	if s.n != 0 {
		t.Errorf("%d Speicherungen nach Game Over", s.n)
	}
	if _, err := s.Load("neu"); err == nil {
		t.Fatal("Spielstand-Datei nach Game Over ohne vorherigen Stand angelegt")
	}
}
