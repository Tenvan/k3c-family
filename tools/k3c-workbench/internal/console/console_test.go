package console

import (
	"fmt"
	"strings"
	"testing"
	"time"
)

func TestRingHaeltDieNeuestenZeilen(t *testing.T) {
	s := New(3, nil)
	for i := 1; i <= 5; i++ {
		s.Add("a", "stdout", fmt.Sprint(i))
	}
	lines, ok := s.Tail("a", 0)
	if !ok || len(lines) != 3 || lines[0].Text != "3" || lines[2].Seq != 5 {
		t.Errorf("Ring: %+v", lines)
	}
	if tail, _ := s.Tail("a", 2); len(tail) != 2 || tail[0].Text != "4" {
		t.Errorf("Tail: %+v", tail)
	}
	if _, ok := s.Tail("fehlt", 1); ok {
		t.Error("unbekannte Quelle gefunden")
	}
}

func TestResetBehaeltDieFolge(t *testing.T) {
	s := New(10, nil)
	s.Add("a", "stdout", "alt")
	s.Reset("a")
	s.Reset("neu")
	s.Add("a", "stdout", "neu")
	lines, _ := s.Tail("a", 0)
	if len(lines) != 1 || lines[0].Seq != 2 {
		t.Errorf("nach Reset: %+v", lines)
	}
	if got := strings.Join(s.Sources(), ","); got != "a,neu" {
		t.Errorf("Quellen: %s", got)
	}
}

func TestBenachrichtigungBlockiertNie(t *testing.T) {
	block := make(chan struct{})
	s := New(10, func(Line) { <-block })
	done := make(chan struct{})
	go func() {
		for range notifyQueue * 4 {
			s.Add("a", "stdout", "x")
		}
		close(done)
	}()
	select {
	case <-done:
	case <-time.After(5 * time.Second):
		t.Fatal("Add blockiert auf einen langsamen Empfänger")
	}
	close(block)
	s.Close()
}

func TestLineWriter(t *testing.T) {
	var got []string
	w := NewLineWriter(func(line string) { got = append(got, line) })
	long := strings.Repeat("x", 100_000)
	for _, part := range []string{"ers", "te\r\nzwei", "te\n" + long + "\n", "kaputt \xff", ""} {
		if _, err := w.Write([]byte(part)); err != nil {
			t.Fatal(err)
		}
	}
	w.Flush()
	if len(got) != 4 || got[0] != "erste" || got[1] != "zweite" || got[2] != long || got[3] != "kaputt �" {
		t.Errorf("Zeilen: %d, erste %q, zweite %q, letzte %q", len(got), got[0], got[1], got[len(got)-1])
	}
}
