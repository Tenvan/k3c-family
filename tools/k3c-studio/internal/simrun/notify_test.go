package simrun

import (
	"context"
	"sync"
	"testing"
	"time"
)

// B-662/AC-04: Änderungen an Läufen melden sich gedrosselt (höchstens etwa 2 je Sekunde je Lauf), die letzte kommt
// immer an; ein fertiger Lauf meldet geänderte Berichte.
func TestLaufMeldetAenderungenGedrosselt(t *testing.T) {
	root := t.TempDir()
	g := New(nil)
	var mu sync.Mutex
	var runs []Info
	reports := make(chan string, 4)
	g.SetNotify(Notify{
		Run: func(r string, info Info) {
			mu.Lock()
			defer mu.Unlock()
			if r == root {
				runs = append(runs, info)
			}
		},
		Reports: func(r string) { reports <- r },
	})
	release := make(chan struct{})
	counted := make(chan int, 1)
	run, err := g.Start(root, "Repo-Wurzel", "x", nil, func(_ context.Context, run *Run) {
		// eine Sekunde lang alle paar Millisekunden Fortschritt
		n := 0
		for end := time.Now().Add(time.Second); time.Now().Before(end); time.Sleep(5 * time.Millisecond) {
			n++
			g.SetProgress(run, n, 0)
		}
		counted <- n
		<-release
		g.Finish(run, Done, "Pass", nil, ReportDir(run))
	}, true)
	if err != nil {
		t.Fatal(err)
	}
	updates := <-counted
	time.Sleep(time.Second) // die letzte Meldung kommt höchstens 0,5 s nach der letzten Änderung
	mu.Lock()
	n := len(runs)
	last := runs[len(runs)-1]
	mu.Unlock()
	if n < 2 || n > 5 {
		t.Errorf("%d Meldungen für %d Änderungen in etwa 1 s, erwartet etwa 2 je Sekunde", n, updates)
	}
	if last.Done != updates || last.State != Running {
		t.Errorf("letzte Meldung %+v, erwartet Fortschritt %d", last, updates)
	}
	close(release)
	select {
	case r := <-reports:
		if r != root {
			t.Errorf("Berichte von %q", r)
		}
	case <-time.After(5 * time.Second):
		t.Fatal("keine Meldung „Berichte geändert“")
	}
	deadline := time.Now().Add(5 * time.Second)
	for time.Now().Before(deadline) {
		mu.Lock()
		state := runs[len(runs)-1].State
		mu.Unlock()
		if state == Done {
			return
		}
		time.Sleep(20 * time.Millisecond)
	}
	t.Errorf("Ende von %s nicht gemeldet", run.ID())
}
