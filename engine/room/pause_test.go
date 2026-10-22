package room

import "testing"

// BD3.1, B-214: Pause setzt den Zustand des eigenen Geräts; zweites pause und resume ohne Pause ändern nichts, eine
// fremde Verbindung ist bad_request.
func TestPauseSetztGeraet(t *testing.T) {
	f := newFixture()
	p := &peer{}
	r := need(f.m.Create("xbox", p, "pause", true, 0, []int{0}, Options{}))(t)
	for _, step := range []struct{ on, want bool }{{false, false}, {true, true}, {true, true}, {false, false}} {
		if err := r.Pause("xbox", p, step.on); err != nil {
			t.Fatal(err)
		}
		if got := r.devices["xbox"].paused; got != step.want {
			t.Fatalf("pause(%v): paused %v, erwartet %v", step.on, got, step.want)
		}
	}
	if err := r.Pause("xbox", &peer{}, true); err != ErrBadRequest {
		t.Fatalf("fremde Verbindung: %v", err)
	}
}
