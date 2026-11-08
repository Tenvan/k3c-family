package balance

import (
	"reflect"
	"testing"
)

// Parallele Läufe liefern dasselbe wie nacheinander, in der Reihenfolge der Szenarien (B-634, BS1.2).
func TestParallelWieNacheinander(t *testing.T) {
	m := Matrix{Seeds: []string{DevSeed, "1", "2"}, Players: []int{2}, Bots: []string{"saver"}, Depths: []int{0}, Days: 1}
	sc, err := m.Scenarios()
	if err != nil {
		t.Fatal(err)
	}
	old := workers
	t.Cleanup(func() { workers = old })
	workers = 1
	serial := runAll(sc, 0, nil)
	workers = 3
	calls := 0
	parallel := runAll(sc, 0, func(done, total int) { calls++ })
	if calls != len(sc) {
		t.Errorf("%d Fortschrittsmeldungen, erwartet %d", calls, len(sc))
	}
	for i := range sc {
		a, b := serial[i], parallel[i]
		if a.Scenario != b.Scenario || !a.Valid || !b.Valid || a.Replay.EndHash != b.Replay.EndHash || !reflect.DeepEqual(a.Metrics, b.Metrics) {
			t.Errorf("Lauf %d (%s): seriell und parallel verschieden", i, sc[i].Seed)
		}
	}
}
