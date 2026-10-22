package store

import (
	"errors"
	"os"
	"path/filepath"
	"testing"
	"time"
)

// BD5.1, B-506/B-533: Discard nimmt den Stand aus der Liste, legt ihn als Sicherung ab und lässt ältere Sicherungen stehen.
func TestSavesDiscardBehaeltSicherungen(t *testing.T) {
	now := time.Date(2026, 10, 22, 12, 0, 0, 0, time.UTC)
	s := &Saves{Dir: t.TempDir(), Now: func() time.Time { now = now.Add(time.Second); return now }}
	for _, day := range []string{"1", "2"} {
		if _, err := s.Store("nord", []byte(`{"version":8,"campaignId":"nord","day":`+day+`}`)); err != nil {
			t.Fatal(err)
		}
	}
	if err := s.Discard("nord"); err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(filepath.Join(s.Dir, "nord.json")); !errors.Is(err, os.ErrNotExist) {
		t.Fatalf("Stand liegt noch: %v", err)
	}
	if len(s.List()) != 0 {
		t.Fatalf("Liste: %v", s.List())
	}
	list, err := s.Backups("nord")
	if err != nil || len(list) != 2 {
		t.Fatalf("Sicherungen: %v %v", list, err)
	}
	// Der gelöschte Stand ist die neueste Sicherung und lässt sich wiederherstellen.
	if err := s.Restore("nord", list[0].Name); err != nil {
		t.Fatal(err)
	}
	if data, _ := s.Load("nord"); string(data) != `{"version":8,"campaignId":"nord","day":2}` {
		t.Fatalf("wiederhergestellt: %s", data)
	}
	if err := s.Discard("fehlt"); !errors.Is(err, ErrNotFound) {
		t.Fatalf("fehlender Stand: %v", err)
	}
	if err := s.Discard("../x"); !errors.Is(err, ErrSlot) {
		t.Fatalf("ungültiger Name: %v", err)
	}
}

// BD5.1, B-506: List nennt die Welt des Stands (Version 8), ältere Stände ohne Feld bleiben ohne world.
func TestSavesListNenntWelt(t *testing.T) {
	s := &Saves{Dir: t.TempDir()}
	for name, data := range map[string]string{
		"nord": `{"version":8,"campaignId":"nord","savedAt":"2026-10-22T10:00:00Z","world":"nordmeer"}`,
		"alt":  `{"version":4,"campaignId":"alt","savedAt":"2026-10-01T10:00:00Z"}`,
	} {
		if _, err := s.Store(name, []byte(data)); err != nil {
			t.Fatal(err)
		}
	}
	got := s.List()
	if len(got) != 2 || got[0].World != "" || got[1].World != "nordmeer" {
		t.Fatalf("Liste: %+v", got)
	}
}
