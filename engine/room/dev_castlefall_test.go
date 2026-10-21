package room

import (
	"strings"
	"testing"
)

// WZ14.5 (B-468/AC-05): Dev-Befehl castleFall löst den Stufenverlust in der Stufe des Monarchen aus, über Gerät und /dm.
func TestDevBurgfall(t *testing.T) {
	r, p, out := devRoom(t)
	m0 := islPlayer(t, r, r.devices["a"].slots[0])
	w := r.isl.Stages[r.isl.StageOf(m0.Index)]
	for _, pl := range w.Players {
		pl.Gold = 10
	}
	if err := r.Dev("a", p, DevAction{Action: "castleFall", Slot: slot(0)}); err != nil {
		t.Fatal(err)
	}
	for _, pl := range w.Players {
		if pl.Gold != 5 {
			t.Errorf("Gold von Monarch %d nach Burgfall %d, erwartet 5", pl.Index, pl.Gold)
		}
	}
	for _, tr := range w.Troops {
		if tr.Kind != "vagrant" {
			t.Errorf("Truppe %s überlebt den Burgfall", tr.Kind)
		}
	}
	if err := r.Dev("a", p, DevAction{Action: "castleFall"}); err != ErrBadRequest {
		t.Errorf("ohne slot: %v, erwartet bad_request", err)
	}
	if err := r.DevPage(DevAction{Action: "castleFall", Slot: slot(m0.Index)}); err != nil {
		t.Errorf("über /dm: %v", err)
	}
	if want := "aktion=castleFall"; !strings.Contains(out.String(), want) {
		t.Errorf("Log ohne %s:\n%s", want, out.String())
	}
}
