package mcpsrv

import (
	"context"
	"errors"
	"sync"
	"testing"
	"time"
)

// Jeder Lauf meldet Beginn und Ende, auch bei gescheitertem Start und Zeitlimit; Checks() kennt danach alle Ziele.
func TestCheckRunMeldetBeginnUndEnde(t *testing.T) {
	results := []runResult{ // in der Reihenfolge der Läufe unten
		{exit: 1, dur: 12 * time.Millisecond},
		{err: errors.New("task fehlt")},
		{timedOut: true, exit: -1},
	}
	n := 0
	s, _ := fakeServer(t, func(runSpec) runResult { n++; return results[n-1] })
	var mu sync.Mutex
	var got []CheckState
	s.cfg.OnCheck = func(st CheckState) {
		mu.Lock()
		got = append(got, st)
		mu.Unlock()
	}
	for _, target := range []string{"task:test", "task:lint", "task:check"} {
		_, _ = s.checkRun(context.Background(), checkIn{Target: target})
	}
	if len(got) != 6 {
		t.Fatalf("%d Meldungen, erwartet 6: %+v", len(got), got)
	}
	for i := 0; i < 6; i += 2 {
		if !got[i].Running || got[i+1].Running || got[i].Name != got[i+1].Name {
			t.Errorf("Paar %d: %+v / %+v", i/2, got[i], got[i+1])
		}
	}
	if got[1].Exit != 1 || got[3].Error == "" || !got[5].TimedOut {
		t.Errorf("Ergebnisse: %+v", got)
	}
	checks := s.Checks()
	if len(checks) != 3 || checks[0].Name != "task:check" || checks[0].Running {
		t.Errorf("Checks() = %+v", checks)
	}
}

// Ein Wiederholungslauf meldet in Checks() seinen eigenen Beginn, nicht den des vorigen Laufs.
func TestChecksZeigtBeginnDesLaufendenLaufs(t *testing.T) {
	release := make(chan struct{})
	n := 0
	s, started := fakeServer(t, func(runSpec) runResult {
		n++
		if n == 2 {
			<-release
		}
		return runResult{at: time.Now().Add(-time.Hour)}
	})
	_, _ = s.checkRun(context.Background(), checkIn{Target: "task:test"})
	done := make(chan struct{})
	go func() { _, _ = s.checkRun(context.Background(), checkIn{Target: "task:test"}); close(done) }()
	for started.Load() < 2 {
		time.Sleep(time.Millisecond)
	}
	checks := s.Checks()
	close(release)
	<-done
	if len(checks) != 1 || !checks[0].Running || time.Since(checks[0].At) > time.Minute {
		t.Errorf("Checks() während des zweiten Laufs = %+v", checks)
	}
}
