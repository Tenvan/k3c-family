package room

import (
	"testing"
	"time"
)

func (s *memStore) Delete(name string) error {
	delete(s.data, name)
	s.deleted = append(s.deleted, name)
	return nil
}

// B-086: Ein leerer Testraum (Präfix test-) löscht beim Aufräumen seinen Spielstand, andere Räume behalten ihn.
func TestTestRaumLoeschtSpielstandBeimAufraeumen(t *testing.T) {
	f := newFixture()
	x, y := &peer{}, &peer{}
	test := need(f.m.Create("xbox", x, "test-ab12", true, 0, []int{0}, Options{}))(t)
	real := need(f.m.Create("handy", y, "familie", true, 0, []int{0}, Options{}))(t)
	test.Leave("xbox", x)
	real.Leave("handy", y)
	f.wait(9 * time.Minute)
	if len(f.store.deleted) != 0 {
		t.Fatalf("vor 10 min gelöscht: %v", f.store.deleted)
	}
	f.wait(2 * time.Minute)
	if len(f.m.Rooms()) != 0 {
		t.Fatalf("Räume nicht aufgeräumt: %d", len(f.m.Rooms()))
	}
	if len(f.store.deleted) != 1 || f.store.deleted[0] != "test-ab12" {
		t.Fatalf("gelöscht: %v, erwartet nur test-ab12", f.store.deleted)
	}
	if _, ok := f.store.data["familie"]; !ok {
		t.Fatal("der echte Spielstand muss bleiben")
	}
}
