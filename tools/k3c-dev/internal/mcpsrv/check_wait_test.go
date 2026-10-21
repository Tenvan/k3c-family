package mcpsrv

import (
	"context"
	"strings"
	"testing"
	"time"
)

// progressCtx ist ein Aufruf mit progressToken: Jede Meldung landet in got.
func progressCtx(got chan<- string) context.Context {
	return context.WithValue(context.Background(), progressKey{}, progressFunc(func(msg string, _ time.Duration) {
		select {
		case got <- msg:
		default:
		}
	}))
}

// B-520/AC-01: Mit progressToken wartet check_run bis zum Ende und meldet unterwegs Fortschritt, auch über checkWait hinaus.
func TestCheckRunMitFortschrittWartetBisZumEnde(t *testing.T) {
	release := make(chan struct{})
	s, _ := fakeServer(t, func(spec runSpec) runResult {
		<-release
		spec.out("stderr", "--- FAIL: TestKaputt (0.00s)")
		return runResult{exit: 1, dur: 3 * time.Minute}
	})
	s.checkWait, s.progressEvery = time.Millisecond, 2*time.Millisecond
	got := make(chan string, 8)
	go func() {
		for i := 0; i < 2; i++ {
			if msg := <-got; !strings.Contains(msg, "task:check:go läuft seit") {
				t.Errorf("Fortschritt: %q", msg)
			}
		}
		close(release)
	}()
	text, err := s.checkRun(progressCtx(got), checkIn{Target: "task:check:go"})
	if err != nil || !strings.Contains(text, "task:check:go · exit 1") || !strings.Contains(text, "FAIL: TestKaputt") {
		t.Fatalf("Antwort: %q, %v", text, err)
	}
}

// B-520/AC-02: check_run {run} wartet wie der Start auf das Ende, statt sofort „läuft noch“ zu melden.
func TestCheckRunRunWartetAufDasEnde(t *testing.T) {
	release := make(chan struct{})
	s, _ := fakeServer(t, func(runSpec) runResult { <-release; return runResult{dur: 5 * time.Minute} })
	s.checkWait = 5 * time.Millisecond
	text, err := s.checkRun(context.Background(), checkIn{Target: "task:check"})
	m := reRunID.FindStringSubmatch(text)
	if err != nil || m == nil {
		t.Fatalf("ohne Fortschritt erwartet Lauf-ID: %q, %v", text, err)
	}
	s.progressEvery = time.Millisecond
	got := make(chan string, 8)
	go func() { <-got; close(release) }()
	text, err = s.checkRun(progressCtx(got), checkIn{Run: m[1]})
	if err != nil || !strings.HasPrefix(text, "task:check · exit 0") {
		t.Fatalf("Ergebnis über run: %q, %v", text, err)
	}
}

// B-520: Ohne progressToken bricht das Warten nach checkWait ab; der Lauf bleibt über seine ID erreichbar.
func TestCheckRunOhneFortschrittNachWartezeitLaufID(t *testing.T) {
	release := make(chan struct{})
	defer close(release)
	s, _ := fakeServer(t, func(runSpec) runResult { <-release; return runResult{} })
	s.checkWait = 5 * time.Millisecond
	text, err := s.checkRun(context.Background(), checkIn{Target: "task:test"})
	if err != nil || !strings.Contains(text, "läuft noch") || !strings.Contains(text, "weiter warten mit check_run {run:") {
		t.Fatalf("Antwort: %q, %v", text, err)
	}
}

func TestWithProgressOhneTokenOhneMelder(t *testing.T) {
	if progressOf(withProgress(context.Background(), nil)) != nil {
		t.Fatal("ohne Aufruf kein Fortschritts-Melder")
	}
}
