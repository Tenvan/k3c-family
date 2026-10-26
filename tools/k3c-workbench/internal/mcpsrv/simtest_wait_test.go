package mcpsrv

import (
	"context"
	"strings"
	"testing"
	"time"
)

// B-644/AC-01: sim_test start mit wait wartet bis zum Ende und liefert den Endstatus; mit progressToken kommen Lebenszeichen.
func TestSimTestWartetBisZumEnde(t *testing.T) {
	release := make(chan struct{})
	base := fakeBalance(t, 1)
	s, _ := fakeServer(t, func(spec runSpec) runResult {
		if strings.Contains(strings.Join(spec.args, " "), "--report-dir") {
			<-release
		}
		return base(spec)
	})
	s.checkWait, s.progressEvery = time.Millisecond, 5*time.Millisecond
	got := make(chan string, 8)
	go func() {
		if msg := <-got; !strings.Contains(msg, "läuft seit") {
			t.Errorf("Lebenszeichen: %q", msg)
		}
		close(release)
	}()
	out, err := s.simTest(progressCtx(got), simTestIn{Action: "start", Seeds: 3, Wait: true})
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(out, "fertig") || !strings.Contains(out, "Bericht: ") {
		t.Errorf("Endstatus fehlt:\n%s", out)
	}
}

// B-644/AC-01: Ohne progressToken endet das Warten nach checkWait mit der ID; wait {id} wartet weiter.
func TestSimTestWartenOhneFortschrittNenntId(t *testing.T) {
	release := make(chan struct{})
	base := fakeBalance(t, 1)
	s, _ := fakeServer(t, func(spec runSpec) runResult {
		if strings.Contains(strings.Join(spec.args, " "), "--report-dir") {
			<-release
		}
		return base(spec)
	})
	s.checkWait, s.progressEvery = 20*time.Millisecond, time.Hour
	out, err := s.simTest(context.Background(), simTestIn{Action: "start", Seeds: 3, Wait: true})
	if err != nil || !strings.Contains(out, "läuft noch") || !strings.Contains(out, `action: wait, id: "run-1"`) {
		t.Fatalf("%v\n%s", err, out)
	}
	close(release)
	s.checkWait = 10 * time.Second
	out, err = s.simTest(context.Background(), simTestIn{Action: "wait", ID: "run-1"})
	if err != nil || !strings.Contains(out, "fertig") {
		t.Fatalf("%v\n%s", err, out)
	}
	if _, err := s.simTest(context.Background(), simTestIn{Action: "wait", ID: "run-99"}); err == nil {
		t.Error("unbekannte ID muss ein Fehler sein")
	}
}
