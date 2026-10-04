package store

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

// (d) S2.2, B-147/AC-03: List nennt neue und alte Stände mit ihren Feldern, keine Sicherungen, eine kaputte Datei als
// Eintrag mit error.
func TestSavesListNenntFelderOhneSicherungen(t *testing.T) {
	s := &Saves{Dir: t.TempDir()}
	v4 := `{"version":4,"campaignId":"neu","savedAt":"2026-10-04T22:00:00Z","day":3,"phase":"night",` +
		`"players":[{"index":0,"depth":1},{"index":1,"depth":0},{"index":2,"depth":1}]}`
	for _, data := range []string{v4, v4} { // zweimal: der erste Stand wandert in die rotierenden Sicherungen
		if _, err := s.Store("neu", []byte(data)); err != nil {
			t.Fatal(err)
		}
	}
	if _, err := s.Store("alt", []byte(`{"version":3,"campaignId":"alt","savedAt":"2026-10-01T10:00:00Z","players":[]}`)); err != nil {
		t.Fatal(err)
	}
	// Sicherung eines fremden Spiels (<slot>-<savedAt>.json) und eine kaputte Datei liegen daneben.
	for name, data := range map[string]string{"neu-2026-10-01T10-00-00Z.json": v4, "kaputt.json": "{kaputt"} {
		if err := os.WriteFile(filepath.Join(s.Dir, name), []byte(data), 0o600); err != nil {
			t.Fatal(err)
		}
	}
	got, _ := json.Marshal(s.List())
	want := `[{"name":"alt","savedAt":"2026-10-01T10:00:00Z","version":3},` +
		`{"name":"kaputt","error":"ungültig"},` +
		`{"name":"neu","savedAt":"2026-10-04T22:00:00Z","version":4,"day":3,"phase":"night","depths":[0,1]}]`
	if string(got) != want {
		t.Fatalf("Liste:\n%s\nerwartet:\n%s", got, want)
	}
	if empty, _ := json.Marshal((&Saves{Dir: filepath.Join(s.Dir, "fehlt")}).List()); string(empty) != "[]" {
		t.Fatalf("ohne Ordner: %s", empty)
	}
}
