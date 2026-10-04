package store

import (
	"bytes"
	"fmt"
	"os"
	"path/filepath"
	"testing"
	"time"
)

// B-142/AC-01, AC-02: Bei Obergrenze + 20 Berichten löscht Store die ältesten (20 plus Platz für den neuen), saves/
// daneben bleibt Byte für Byte gleich.
func TestReportsRotierenAeltesteZuerst(t *testing.T) {
	root := t.TempDir()
	dir := filepath.Join(root, "reports")
	saves := filepath.Join(root, "saves")
	for _, d := range []string{dir, filepath.Join(saves, "backups", "autosave")} {
		if err := os.MkdirAll(d, 0o755); err != nil {
			t.Fatal(err)
		}
	}
	saveFiles := map[string][]byte{
		filepath.Join(saves, "autosave.json"):                       []byte(`{"version":3}`),
		filepath.Join(saves, "backups", "autosave", "gamepad-1.json"): []byte(`{"alt":true}`),
	}
	for path, data := range saveFiles {
		if err := os.WriteFile(path, data, 0o644); err != nil {
			t.Fatal(err)
		}
	}
	start := time.Date(2026, 1, 1, 0, 0, 0, 0, time.UTC)
	name := func(i int) string {
		return "gamepad-" + start.Add(time.Duration(i)*time.Minute).Format("2006-01-02T15-04-05-000Z") + ".json"
	}
	for i := range MaxReports + 20 {
		if err := os.WriteFile(filepath.Join(dir, name(i)), fmt.Appendf(nil, `{"i":%d}`, i), 0o644); err != nil {
			t.Fatal(err)
		}
	}

	r := &Reports{Dir: dir, Now: func() time.Time { return start.Add(1000 * time.Minute) }}
	if _, err := r.Store([]byte(`{"neu":true}`), "1.2.3.4"); err != nil {
		t.Fatal(err)
	}

	if n := r.Count(); n != MaxReports {
		t.Errorf("Berichte: %d, erwartet %d", n, MaxReports)
	}
	for i := range MaxReports + 20 {
		_, err := os.Stat(filepath.Join(dir, name(i)))
		if gone := os.IsNotExist(err); gone != (i <= 20) {
			t.Errorf("Bericht %d gelöscht = %v", i, gone)
		}
	}
	for path, want := range saveFiles {
		if got, err := os.ReadFile(path); err != nil || !bytes.Equal(got, want) {
			t.Errorf("%s verändert: %q, %v", path, got, err)
		}
	}
}

// S2.3 (B-150): StoreSession legt session-<zeit>.json an, überschreibt keinen Report derselben Millisekunde und lehnt
// Nicht-Objekte ab.
func TestStoreSessionLegtNeueDateiAn(t *testing.T) {
	dir := t.TempDir()
	r := &Reports{Dir: dir, Now: func() time.Time { return time.Date(2026, 10, 4, 20, 0, 0, 0, time.UTC) }}
	a, err := r.StoreSession([]byte(`{"schema":1}`))
	if err != nil {
		t.Fatal(err)
	}
	b, err := r.StoreSession([]byte(`{"schema":1,"b":true}`))
	if err != nil {
		t.Fatal(err)
	}
	if filepath.Base(a) != "session-2026-10-04T20-00-00-000Z.json" || a == b {
		t.Fatalf("Pfade %s, %s", a, b)
	}
	if got, _ := os.ReadFile(a); string(got) != `{"schema":1}` {
		t.Fatalf("erster Report überschrieben: %s", got)
	}
	if _, err := r.StoreSession([]byte(`[1]`)); err != ErrInvalid {
		t.Fatalf("Liste angenommen: %v", err)
	}
}
