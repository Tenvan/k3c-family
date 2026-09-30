package store

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"testing"
	"time"
)

// clock liefert bei jedem Aufruf eine Sekunde später.
func clock() func() time.Time {
	t := time.Date(2026, 9, 30, 12, 0, 0, 0, time.UTC)
	return func() time.Time {
		t = t.Add(time.Second)
		return t
	}
}

func save(n int) []byte { return []byte(fmt.Sprintf(`{"version":1,"campaignId":"a","n":%d}`, n)) }

// B-028/AC-01: nach 7 Speichervorgängen liegen genau die 5 zuletzt überschriebenen Stände vor (2 bis 6).
func TestRotationBehaeltFuenf(t *testing.T) {
	s := &Saves{Dir: t.TempDir(), Now: clock()}
	for i := 1; i <= 7; i++ {
		if _, err := s.Store("autosave", save(i)); err != nil {
			t.Fatal(err)
		}
	}
	list, err := s.Backups("autosave")
	if err != nil || len(list) != BackupKeep {
		t.Fatalf("Backups = %v, %v", list, err)
	}
	for i, b := range list { // neueste zuerst: 6, 5, 4, 3, 2
		data, _ := os.ReadFile(filepath.Join(s.backupDir("autosave"), b.Name))
		if want := string(save(6 - i)); string(data) != want || b.SavedAt.IsZero() || b.Size == 0 {
			t.Errorf("Sicherung %d = %s (%+v), erwartet %s", i, data, b, want)
		}
	}
	if cur, _ := s.Load("autosave"); string(cur) != string(save(7)) {
		t.Errorf("aktuell = %s", cur)
	}
	if n := s.Count(); n != 1 {
		t.Errorf("Count = %d, Sicherungen zählen nicht mit", n)
	}
}

// Slots mit gemeinsamem Präfix teilen keine Sicherungen.
func TestRotationJeSlot(t *testing.T) {
	s := &Saves{Dir: t.TempDir(), Now: clock()}
	for i := 1; i <= 3; i++ {
		_, _ = s.Store("auto", save(i))
		_, _ = s.Store("auto-save", save(i))
	}
	a, _ := s.Backups("auto")
	b, _ := s.Backups("auto-save")
	if len(a) != 2 || len(b) != 2 {
		t.Errorf("auto %d, auto-save %d Sicherungen", len(a), len(b))
	}
}

// B-028/AC-02: eine Sicherung wird der aktuelle Stand, der bisherige wird selbst gesichert.
func TestRestore(t *testing.T) {
	s := &Saves{Dir: t.TempDir(), Now: clock()}
	for i := 1; i <= 3; i++ {
		_, _ = s.Store("autosave", save(i))
	}
	list, _ := s.Backups("autosave")
	oldest := list[len(list)-1] // Stand 1
	if err := s.Restore("autosave", oldest.Name); err != nil {
		t.Fatal(err)
	}
	if cur, _ := s.Load("autosave"); string(cur) != string(save(1)) {
		t.Errorf("nach Wiederherstellen = %s", cur)
	}
	after, _ := s.Backups("autosave")
	newest, _ := os.ReadFile(filepath.Join(s.backupDir("autosave"), after[0].Name))
	if string(newest) != string(save(3)) {
		t.Errorf("bisheriger Stand nicht gesichert: %s", newest)
	}
	for _, bad := range []string{"fehlt.json", "../autosave.json", `..\autosave.json`, ""} {
		if err := s.Restore("autosave", bad); !errors.Is(err, ErrNotFound) {
			t.Errorf("Restore(%q) = %v", bad, err)
		}
	}
	if err := s.Restore("../x", oldest.Name); !errors.Is(err, ErrSlot) {
		t.Errorf("Slot: %v", err)
	}
}
