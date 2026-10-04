package room

import (
	"testing"
	"time"

	"k3c/engine/sim"
)

// waitSaved wartet, bis der Raum keinen Spielstand mehr im Hintergrund schreibt (B-274: warten statt schlafen).
func waitSaved(r *Room) {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.flush()
}

// blockStore hält jedes Store an, bis release geschlossen ist; entered meldet den Beginn.
type blockStore struct {
	*memStore
	entered chan struct{}
	release chan struct{}
}

func (s *blockStore) Store(name string, data []byte) (string, error) {
	s.entered <- struct{}{}
	<-s.release
	return s.memStore.Store(name, data)
}

// N1/AC-03 (B-276/AC-03): Ein Stufenwechsel speichert, ohne dass Tick auf die Datei wartet; nach dem Flush liegt der
// Stand vor. SaveNow schreibt synchron.
func TestStufenwechselSpeichertImHintergrund(t *testing.T) {
	f := newFixture()
	bs := &blockStore{memStore: f.store, entered: make(chan struct{}, 8), release: make(chan struct{})}
	f.m.Store = bs
	r := need(f.m.Create("xbox", &peer{}, "hinten", true, 0, []int{0}, Options{}))(t)
	moveToExit(t, r, 0)
	done := make(chan struct{})
	go func() { ticks(r, 70); close(done) }()
	select {
	case <-done:
	case <-time.After(10 * time.Second):
		t.Fatal("Tick wartet auf die Datei")
	}
	if r.isl.StageOf(0) != 1 || f.store.saves != 0 {
		t.Fatalf("Stufe %d, %d Speicherungen vor dem Schreiben", r.isl.StageOf(0), f.store.saves)
	}
	<-bs.entered
	close(bs.release)
	waitSaved(r)
	if f.store.saves != 1 || need(sim.ParseIslandSave(f.store.data["hinten"]))(t).Players[0].Depth != 1 {
		t.Fatalf("nach dem Flush: %d Speicherungen", f.store.saves)
	}
	if _, err := r.SaveNow(); err != nil || f.store.saves != 2 {
		t.Fatalf("SaveNow: %v, %d Speicherungen", err, f.store.saves)
	}
}

// Herunterfahren wartet auf eine laufende Hintergrund-Speicherung und speichert danach selbst.
func TestHerunterfahrenWartetAufSpeicherung(t *testing.T) {
	f := newFixture()
	bs := &blockStore{memStore: f.store, entered: make(chan struct{}, 8), release: make(chan struct{})}
	f.m.Store = bs
	x, h := &peer{}, &peer{}
	r := need(f.m.Create("xbox", x, "warten", true, 0, []int{0}, Options{}))(t)
	need(f.m.Join("handy", h, r.Code, []int{0}))(t)
	r.Leave("handy", h) // Speicherung im Hintergrund, hängt im Store
	<-bs.entered
	closed := make(chan struct{})
	go func() { f.m.Close(); close(closed) }()
	select {
	case <-closed:
		t.Fatal("Close wartet nicht auf die Hintergrund-Speicherung")
	default:
	}
	close(bs.release)
	<-closed
	if f.store.saves != 2 {
		t.Fatalf("%d Speicherungen, erwartet Hintergrund + Herunterfahren", f.store.saves)
	}
}
