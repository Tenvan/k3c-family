package room

import (
	"errors"
	"testing"
)

// BD5.1, B-506/AC-01: Ein Spielstand, der in einem Raum offen ist (auch ohne verbundenes Gerät), wird nicht gelöscht;
// sonst läuft das Löschen unter der Sperre des Managers.
func TestDeleteSaveLehntOffenenStandAb(t *testing.T) {
	f := newFixture()
	x := &peer{}
	need(f.m.Create("xbox", x, "familie", true, 0, []int{0}, Options{}))(t)
	called := 0
	del := func() error { called++; return nil }
	if err := f.m.DeleteSave("familie", del); !errors.Is(err, ErrSaveInUse) || called != 0 {
		t.Fatalf("offen: %v, gelöscht %d", err, called)
	}
	ok(t, f.m.DeleteSave("anderer", del))
	if called != 1 {
		t.Fatalf("nicht offen: gelöscht %d", called)
	}
	if err := f.m.DeleteSave("../x", del); !errors.Is(err, ErrBadRequest) || called != 1 {
		t.Fatalf("ungültiger Name: %v", err)
	}
}
