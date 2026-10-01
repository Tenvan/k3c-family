package store

import (
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

func TestSavesRundlaufUndPruefung(t *testing.T) {
	s := &Saves{Dir: t.TempDir()}
	if _, err := s.Load("autosave"); !errors.Is(err, ErrNotFound) {
		t.Errorf("leer: %v", err)
	}
	if b, err := s.Store("autosave", []byte(`{"version":1,"campaignId":"ci"}`)); err != nil || b != "" {
		t.Fatalf("Store: %q, %v", b, err)
	}
	if data, err := s.Load("autosave"); err != nil || !strings.Contains(string(data), `"campaignId":"ci"`) {
		t.Errorf("Load: %s, %v", data, err)
	}
	for _, bad := range []string{`kaputt`, `null`, `[]`, `{"version":1}`, `{"campaignId":"x","version":"1"}`, `{"campaignId":1,"version":1}`} {
		if _, err := s.Store("autosave", []byte(bad)); !errors.Is(err, ErrInvalid) {
			t.Errorf("%s: %v", bad, err)
		}
	}
	for _, slot := range []string{"", "Auto", "../x", `..\x`, "a/b", strings.Repeat("a", 33)} {
		if _, err := s.Store(slot, []byte(`{"version":1,"campaignId":"ci"}`)); !errors.Is(err, ErrSlot) {
			t.Errorf("Slot %q: %v", slot, err)
		}
	}
	if _, err := s.Store("autosave", []byte(`{"version":1,"campaignId":"`+strings.Repeat("x", MaxSaveBytes)+`"}`)); !errors.Is(err, ErrTooLarge) {
		t.Errorf("zu groß: %v", err)
	}
}

// Wie server/saves.mjs: überschreibt ein Stand ein anderes Spiel, wird das alte vorher gesichert.
func TestSavesSichernBeiAnderemSpiel(t *testing.T) {
	dir := t.TempDir()
	s := &Saves{Dir: dir, Now: func() time.Time { return time.UnixMilli(1700000000000) }}
	_, _ = s.Store("autosave", []byte(`{"version":1,"campaignId":"a","savedAt":"2026-09-30T12:00:00.000Z"}`))
	if b, _ := s.Store("autosave", []byte(`{"version":1,"campaignId":"a"}`)); b != "" {
		t.Errorf("gleiches Spiel gesichert: %q", b)
	}
	_, _ = s.Store("autosave", []byte(`{"version":1,"campaignId":"a","savedAt":"../../evil"}`))
	b, err := s.Store("autosave", []byte(`{"version":1,"campaignId":"b"}`))
	if err != nil || b != "autosave-----evil.json" {
		t.Fatalf("Sicherung = %q, %v (savedAt darf kein Pfad werden)", b, err)
	}
	if _, err := os.Stat(filepath.Join(dir, b)); err != nil {
		t.Error(err)
	}
	_, _ = s.Store("slot2", []byte(`{"version":1,"campaignId":"a"}`))
	if b, _ := s.Store("slot2", []byte(`{"version":1,"campaignId":"b"}`)); b != "slot2-1700000000000.json" {
		t.Errorf("ohne savedAt: %q", b)
	}
}

// Ein unterbrochenes Schreiben (hier: Ziel ist ein Ordner) lässt keine halbe Datei und keinen Rest zurück.
func TestWriteAtomicLaesstBeiFehlerNichtsZurueck(t *testing.T) {
	dir := t.TempDir()
	target := filepath.Join(dir, "x.json")
	if err := os.Mkdir(target, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := writeAtomic(target, []byte("{}")); err == nil {
		t.Fatal("kein Fehler")
	}
	entries, _ := os.ReadDir(dir)
	if len(entries) != 1 {
		t.Errorf("Reste: %v", entries)
	}
}

func TestReports(t *testing.T) {
	dir := t.TempDir()
	r := &Reports{Dir: dir, Now: func() time.Time { return time.Date(2026, 9, 30, 12, 0, 0, 0, time.UTC) }}
	if _, err := r.Store([]byte("kaputt"), "1.2.3.4"); !errors.Is(err, ErrInvalid) {
		t.Errorf("kaputt: %v", err)
	}
	a, err := r.Store([]byte(`{"ci":true}`), "1.2.3.4")
	if err != nil || filepath.Base(a) != "gamepad-2026-09-30T12-00-00-000Z.json" {
		t.Fatalf("Store: %q, %v", a, err)
	}
	b, _ := r.Store([]byte(`{"ci":2}`), "1.2.3.4")
	if filepath.Base(b) != "gamepad-2026-09-30T12-00-00-000Z-1.json" {
		t.Errorf("zweiter in derselben ms: %q", b)
	}
	var got map[string]any
	data, _ := os.ReadFile(a)
	if json.Unmarshal(data, &got) != nil || got["ci"] != true || got["remoteAddress"] != "1.2.3.4" || got["receivedAt"] == nil {
		t.Errorf("Inhalt: %s", data)
	}
	if _, err := r.Store([]byte(`{"x":"`+strings.Repeat("x", MaxReportBytes)+`"}`), ""); !errors.Is(err, ErrTooLarge) {
		t.Errorf("zu groß: %v", err)
	}
}

// fill legt Spielstände an; das zweite Speichern erzeugt je eine Sicherung.
func fill(t *testing.T, s *Saves, slots ...string) {
	t.Helper()
	for _, slot := range slots {
		for range 2 {
			if _, err := s.Store(slot, []byte(`{"version":1,"campaignId":"ci"}`)); err != nil {
				t.Fatal(err)
			}
		}
	}
}

func TestSavesPurge(t *testing.T) {
	s := &Saves{Dir: t.TempDir()}
	fill(t, s, "test-alt", "test-neu", "familie")
	old := time.Now().Add(-48 * time.Hour)
	for _, slot := range []string{"test-alt", "familie"} {
		if err := os.Chtimes(filepath.Join(s.Dir, slot+".json"), old, old); err != nil {
			t.Fatal(err)
		}
	}
	if n, err := s.Purge("test-", time.Now().Add(-24*time.Hour)); err != nil || n != 1 {
		t.Fatalf("Purge: %d, %v", n, err)
	}
	for slot, want := range map[string]bool{"test-alt": false, "test-neu": true, "familie": true} {
		if _, err := s.Load(slot); (err == nil) != want {
			t.Errorf("%s vorhanden = %v, erwartet %v", slot, err == nil, want)
		}
	}
	if _, err := os.Stat(s.backupDir("test-alt")); !errors.Is(err, os.ErrNotExist) {
		t.Errorf("Sicherungen von test-alt bleiben: %v", err)
	}
	if _, err := s.Purge("", time.Now()); !errors.Is(err, ErrSlot) {
		t.Errorf("leeres Präfix: %v", err)
	}
}

func TestSavesDelete(t *testing.T) {
	s := &Saves{Dir: t.TempDir()}
	fill(t, s, "test-x", "familie")
	if err := s.Delete("test-x"); err != nil {
		t.Fatal(err)
	}
	if err := s.Delete("test-x"); err != nil {
		t.Errorf("zweites Löschen: %v", err)
	}
	if err := s.Delete("../x"); !errors.Is(err, ErrSlot) {
		t.Errorf("ungültiger Name: %v", err)
	}
	if _, err := s.Load("test-x"); !errors.Is(err, ErrNotFound) {
		t.Errorf("test-x noch da: %v", err)
	}
	if _, err := s.Load("familie"); err != nil {
		t.Errorf("familie weg: %v", err)
	}
}
