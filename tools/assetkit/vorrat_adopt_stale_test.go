package assets

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// B-677: Ein veralteter Stand lässt bei der Übernahme weder Datei noch Credit-Zeile noch Index-Vermerk zurück
// (Prüfung unter mu vor dem ersten Schreiben).
func TestVorratAdoptStaleHinterlaesstNichts(t *testing.T) {
	root, client, _ := vorratRepo(t)
	if _, err := VorratDownload(context.Background(), root, suggestion(t, root, "v1"), client, day, nil); err != nil {
		t.Fatal(err)
	}
	stand, err := Stamp(root)
	if err != nil {
		t.Fatal(err)
	}
	reg, _ := Load(root)
	reg.Comment = "ein anderer Schreiber"
	if err := reg.Save(root); err != nil {
		t.Fatal(err)
	}
	if _, err := vorratAdopt(root, "v1", stand, day); !errors.Is(err, ErrStale) {
		t.Fatalf("stand veraltet: %v", err)
	}
	if _, err := os.Stat(filepath.Join(root, "public", "audio", "kandidaten")); !os.IsNotExist(err) {
		t.Errorf("datei unter public/ geblieben: %v", err)
	}
	credits, _ := os.ReadFile(filepath.Join(root, "public", "audio", "README.md"))
	if strings.Contains(string(credits), "kandidaten") {
		t.Errorf("credit-zeile geblieben:\n%s", credits)
	}
	if v, _ := LoadVorrat(root); v.find("v1").Public != "" {
		t.Error("index vermerkt eine Übernahme")
	}
	now, _ := Stamp(root)
	if rel, err := vorratAdopt(root, "v1", now, day); err != nil || rel == "" {
		t.Errorf("auf aktuellem stand: %q %v", rel, err)
	}
}
