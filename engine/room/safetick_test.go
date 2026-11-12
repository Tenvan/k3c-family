package room

import (
	"errors"
	"testing"
)

// B-803/AC-01: Ein Panic im Takt über SafeTick schließt den Raum wie im Server: Eine schon begonnene
// Hintergrund-Speicherung läuft zu Ende, bevor SafeTick zurückkehrt; danach schreibt der Raum nichts mehr.
func TestSafeTickPanicSchliesstRaum(t *testing.T) {
	f := newFixture()
	bs := &blockStore{memStore: f.store, entered: make(chan struct{}, 8), release: make(chan struct{})}
	f.m.Store = bs
	x := &peer{}
	r := need(f.m.Create("xbox", x, "absturz", true, 0, []int{0}, Options{}))(t)
	for range saveTicks {
		if !r.SafeTick() {
			t.Fatal("Raum vor dem Absturz geschlossen")
		}
	}
	<-bs.entered // der Autosave schreibt im Hintergrund und hängt im Store
	r.beforeStep = func() { panic("Absturz im Test") }
	done := make(chan bool)
	go func() { done <- r.SafeTick() }()
	close(bs.release)
	if <-done {
		t.Fatal("SafeTick meldet nach Panic einen lebenden Raum")
	}
	saves := f.store.saves // SafeTick wartet auf die begonnene Speicherung (crash › flush)
	if _, failures := f.m.Status(); saves != 1 || !r.Closed() || !x.has("closed") || f.m.Room(r.Code) != nil ||
		len(failures) != 1 || failures[0].Error != "Absturz im Test" {
		t.Fatalf("nach Panic: %d Speicherungen, geschlossen %v, Abstürze %+v", saves, r.Closed(), failures)
	}
	r.beforeStep = nil
	if r.SafeTick() {
		t.Error("geschlossener Raum tickt weiter")
	}
	if _, err := r.SaveNow(); !errors.Is(err, ErrClosed) {
		t.Errorf("SaveNow nach Absturz: %v", err)
	}
	r.Leave("xbox", x)
	if waitSaved(r); f.store.saves != saves {
		t.Fatalf("Speicherungen nach Absturz: %d", f.store.saves-saves)
	}
}
