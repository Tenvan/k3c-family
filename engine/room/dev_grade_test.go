package room

import "testing"

// B-080/AC-02, K4.2: Die Dev-Aktion grade wechselt den Grad des Raums; er gilt ab der nächsten Welle, nicht für die laufende.
func TestDevGradeAbNaechsterWelle(t *testing.T) {
	r, p, _ := devRoom(t)
	if err := r.Dev("a", p, DevAction{Action: "wave", Slot: slot(0)}); err != nil {
		t.Fatal(err)
	}
	n1 := len(r.isl.Stages[0].SpawnQueue)
	if err := r.Dev("a", p, DevAction{Action: "grade", Grade: "ultra"}); err != nil {
		t.Fatal(err)
	}
	if r.isl.Options.Grade != "ultra" || len(r.isl.Stages[0].SpawnQueue) != n1 {
		t.Fatalf("Grad %q, Warteschlange %d statt %d", r.isl.Options.Grade, len(r.isl.Stages[0].SpawnQueue), n1)
	}
	if err := r.Dev("a", p, DevAction{Action: "wave", Slot: slot(0)}); err != nil {
		t.Fatal(err)
	}
	if n2 := len(r.isl.Stages[0].SpawnQueue); n2 <= n1 {
		t.Fatalf("neue Welle nicht größer: %d nach %d", n2, n1)
	}
}

func TestDevGradeUngueltigOderVerboten(t *testing.T) {
	r, p, _ := devRoom(t)
	if err := r.Dev("a", p, DevAction{Action: "grade", Grade: "gibtsnicht"}); err != ErrBadRequest {
		t.Fatalf("unbekannter Grad: %v", err)
	}
	r.m.Dev = false
	if err := r.Dev("a", p, DevAction{Action: "grade", Grade: "hard"}); err != ErrForbidden || r.isl.Options.Grade == "hard" {
		t.Fatalf("ohne Dev-Mode: %v, Grad %q", err, r.isl.Options.Grade)
	}
}
