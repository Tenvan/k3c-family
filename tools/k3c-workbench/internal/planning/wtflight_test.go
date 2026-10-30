package planning

import (
	"sync"
	"sync/atomic"
	"testing"
	"time"
)

// Viele gleichzeitige Aufrufe teilen sich wenige Läufe (hier: ein laufender plus höchstens ein Folgelauf).
func TestFlightBuendeltGleichzeitigeAufrufe(t *testing.T) {
	f := &flight{bases: map[string]Data{}}
	var runs atomic.Int32
	release := make(chan struct{})
	check := func(map[string]Data) ([]WorktreeDiff, error) {
		runs.Add(1)
		<-release
		return []WorktreeDiff{{Branch: "b"}}, nil
	}
	var wg sync.WaitGroup
	for i := 0; i < 50; i++ {
		arrived := time.Now()
		wg.Add(1)
		go func() {
			defer wg.Done()
			if d, err := f.run(arrived, check); err != nil || len(d) != 1 {
				t.Errorf("Ergebnis %v, %v", d, err)
			}
		}()
	}
	time.Sleep(50 * time.Millisecond)
	close(release)
	wg.Wait()
	if n := runs.Load(); n > 2 {
		t.Fatalf("%d Läufe für 50 gleichzeitige Aufrufe, erwartet höchstens 2", n)
	}
}

// Wer nach dem Ende eines Laufs ankommt, bekommt einen frischen Lauf (kein veraltetes Ergebnis).
func TestFlightFrischerLaufNachAnkunft(t *testing.T) {
	f := &flight{bases: map[string]Data{}}
	var runs int
	check := func(map[string]Data) ([]WorktreeDiff, error) { runs++; return nil, nil }
	for i := 0; i < 3; i++ {
		time.Sleep(2 * time.Millisecond)
		if _, err := f.run(time.Now(), check); err != nil {
			t.Fatal(err)
		}
	}
	if runs != 3 {
		t.Fatalf("%d Läufe, erwartet 3", runs)
	}
}

// Der Cache der Merge-Bases bleibt zwischen den Läufen und ist begrenzt.
func TestFlightBaseCache(t *testing.T) {
	f := &flight{bases: map[string]Data{}}
	check := func(b map[string]Data) ([]WorktreeDiff, error) { b["x"] = Data{}; return nil, nil }
	time.Sleep(2 * time.Millisecond)
	_, _ = f.run(time.Now(), check)
	if _, ok := f.bases["x"]; !ok {
		t.Fatal("Stand nach dem Lauf nicht im Cache")
	}
	for i := 0; i <= maxBases; i++ {
		f.bases[string(rune('a'+i))] = Data{}
	}
	time.Sleep(2 * time.Millisecond)
	_, _ = f.run(time.Now(), func(b map[string]Data) ([]WorktreeDiff, error) {
		if len(b) != 0 {
			t.Errorf("Cache nicht begrenzt: %d Einträge", len(b))
		}
		return nil, nil
	})
}
