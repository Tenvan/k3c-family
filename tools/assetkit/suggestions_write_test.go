package assets

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"testing"
)

// B-600/AC-02 (WZ29.7): Übernahme aus dem Chat nur auf dem Stand des Registers; veralteter Stand schreibt nichts.
func TestVorratAdoptIfStand(t *testing.T) {
	root, client, _ := vorratRepo(t)
	if _, err := VorratDownload(context.Background(), root, suggestion(t, root, "v1"), client, day, nil); err != nil {
		t.Fatal(err)
	}
	if _, _, err := VorratAdoptIf(root, "v1", "alt", day); !errors.Is(err, ErrStale) {
		t.Fatalf("veralteter Stand: %v", err)
	}
	if reg, _ := Load(root); len(reg.Find("sfx:coin").Choices) != 0 {
		t.Fatal("veralteter Stand hat zugeordnet")
	}
	stand, _ := Stamp(root)
	rel, next, err := VorratAdoptIf(root, "v1", stand, day)
	if err != nil || next == stand {
		t.Fatalf("Übernahme: %q %q %v", rel, next, err)
	}
	if reg, _ := Load(root); len(reg.Find("sfx:coin").Choices) != 1 || reg.Find("sfx:coin").Choices[0].File != rel {
		t.Fatalf("Zuordnung fehlt: %+v", reg.Find("sfx:coin").Choices)
	}
	// noch einmal: keine doppelte Zuordnung, der Stand bleibt
	if again, same, err := VorratAdoptIf(root, "v1", next, day); err != nil || again != rel || same != next {
		t.Fatalf("zweite Übernahme: %q %q %v", again, same, err)
	}
}

// B-600/AC-02: Verwerfen setzt den Status in der Liste und schreibt atomar; fremder Slot oder unbekannte ID scheitern.
func TestDropSuggestion(t *testing.T) {
	root, _, _ := vorratRepo(t)
	if err := DropSuggestion(root, "boss", "v1"); err == nil {
		t.Error("fremder Slot soll scheitern")
	}
	if err := DropSuggestion(root, "", "fehlt"); err == nil {
		t.Error("unbekannte ID soll scheitern")
	}
	if err := DropSuggestion(root, "sfx:coin", "v1"); err != nil {
		t.Fatal(err)
	}
	if s := suggestion(t, root, "v1"); s.Status != VorratDropped {
		t.Errorf("Status %q", s.Status)
	}
	if left, _ := filepath.Glob(filepath.Join(root, "data", ".assets-*")); len(left) != 0 {
		t.Errorf("Zwischendateien bleiben: %v", left)
	}
}

// Die echte Liste übersteht Laden und Speichern unverändert (Verwerfen ändert nur den Status).
func TestVorschlagslisteRundlauf(t *testing.T) {
	src := filepath.Join(repoRoot(t), SuggestionsPath)
	want, err := os.ReadFile(src)
	if err != nil {
		t.Fatal(err)
	}
	root := t.TempDir()
	writeFile(t, filepath.Join(root, SuggestionsPath), string(want))
	list, err := LoadSuggestions(root)
	if err != nil {
		t.Fatal(err)
	}
	if err := list.save(root); err != nil {
		t.Fatal(err)
	}
	if got, _ := os.ReadFile(filepath.Join(root, SuggestionsPath)); string(got) != string(want) {
		t.Error("Speichern ändert die Form der Vorschlagsliste")
	}
}
