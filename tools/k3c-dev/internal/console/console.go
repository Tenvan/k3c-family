// Package console hält die Konsolenausgabe von k3c-dev je Quelle (B-046): Prüfläufe `check:<ziel>`, das eigene Log
// und ab B-067 die Dienste. Nur im Speicher; der Puffer stirbt mit dem Programm.
package console

import (
	"sort"
	"sync"
)

const (
	// DefaultCapacity ist die Zahl der Zeilen je Quelle.
	DefaultCapacity = 2000
	// notifyQueue puffert Benachrichtigungen; ist er voll, gehen Benachrichtigungen verloren, nie Zeilen.
	notifyQueue = 256
)

// Line ist eine Zeile einer Quelle. Seq zählt je Quelle fortlaufend, auch über Reset hinweg; damit sortiert die
// Oberfläche Zeilen ein, die während des Ladens kamen.
type Line struct {
	Source string `json:"source"`
	Stream string `json:"stream"` // stdout, stderr oder log
	Text   string `json:"text"`
	Seq    int64  `json:"seq"`
}

type ring struct {
	lines []Line
	seq   int64
}

// Store hält einen Ringpuffer je Quelle. Alle Methoden sind nebenläufig sicher und blockieren nie auf den Empfänger.
type Store struct {
	mu       sync.Mutex
	capacity int
	rings    map[string]*ring
	notify   chan Line
	done     chan struct{}
	finished chan struct{}
}

// New legt einen Store an; onLine (optional) bekommt jede neue Zeile in einer eigenen Goroutine.
func New(capacity int, onLine func(Line)) *Store {
	if capacity <= 0 {
		capacity = DefaultCapacity
	}
	s := &Store{capacity: capacity, rings: map[string]*ring{}}
	if onLine != nil {
		s.notify, s.done, s.finished = make(chan Line, notifyQueue), make(chan struct{}), make(chan struct{})
		go s.dispatch(onLine)
	}
	return s
}

func (s *Store) dispatch(onLine func(Line)) {
	defer close(s.finished)
	for {
		select {
		case l := <-s.notify:
			onLine(l)
		case <-s.done:
			return
		}
	}
}

// Close beendet die Benachrichtigungen; danach wird onLine nicht mehr gerufen.
func (s *Store) Close() {
	if s.done == nil {
		return
	}
	close(s.done)
	<-s.finished
}

// Add hängt eine Zeile an; die älteste fällt heraus, wenn der Puffer voll ist.
func (s *Store) Add(source, stream, text string) {
	s.mu.Lock()
	r := s.ringFor(source)
	r.seq++
	l := Line{Source: source, Stream: stream, Text: text, Seq: r.seq}
	if len(r.lines) == s.capacity {
		copy(r.lines, r.lines[1:])
		r.lines = r.lines[:len(r.lines)-1]
	}
	r.lines = append(r.lines, l)
	s.mu.Unlock()
	if s.notify != nil {
		select {
		case s.notify <- l:
		default:
		}
	}
}

func (s *Store) ringFor(source string) *ring {
	r, ok := s.rings[source]
	if !ok {
		r = &ring{}
		s.rings[source] = r
	}
	return r
}

// Reset leert eine Quelle (neuer Lauf) und legt sie an, falls es sie noch nicht gibt.
func (s *Store) Reset(source string) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.ringFor(source).lines = nil
}

// Tail liefert die letzten n Zeilen einer Quelle (n <= 0: alle) und ob es die Quelle gibt.
func (s *Store) Tail(source string, n int) ([]Line, bool) {
	s.mu.Lock()
	defer s.mu.Unlock()
	r, ok := s.rings[source]
	if !ok {
		return nil, false
	}
	lines := r.lines
	if n > 0 && len(lines) > n {
		lines = lines[len(lines)-n:]
	}
	return append([]Line(nil), lines...), true
}

// Sources liefert alle Quellen, sortiert.
func (s *Store) Sources() []string {
	s.mu.Lock()
	defer s.mu.Unlock()
	names := make([]string, 0, len(s.rings))
	for name := range s.rings {
		names = append(names, name)
	}
	sort.Strings(names)
	return names
}
