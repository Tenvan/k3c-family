package assets

import (
	"image"
	"image/png"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// repo legt ein Mini-Repo an: data/assets.json mit einem Grafik- und einem Sound-Slot, zwei Bilder, ein Sound.
func repo(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	writePNG(t, filepath.Join(root, "public", "grafik", "town", "well.png"), 64, 48)
	writePNG(t, filepath.Join(root, "public", "grafik", "town", "wagon.png"), 32, 32)
	writeFile(t, filepath.Join(root, "public", "audio", "coin.ogg"), "ogg")
	writeFile(t, filepath.Join(root, RegisterPath), `{
  "_comment": "Test",
  "slots": [
    { "id": "farm", "art": "grafik", "kategorie": "gebaeude", "beschreibung": "Farm", "wahl": [] },
    { "id": "sfx:coin", "art": "sound", "kategorie": "sfx", "beschreibung": "Münze", "bus": "sfx", "wahl": [] }
  ]
}
`)
	return root
}

func writeFile(t *testing.T, path, text string) {
	t.Helper()
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(path, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
}

func writePNG(t *testing.T, path string, w, h int) {
	t.Helper()
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		t.Fatal(err)
	}
	f, err := os.Create(path)
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = f.Close() }()
	if err := png.Encode(f, image.NewNRGBA(image.Rect(0, 0, w, h))); err != nil {
		t.Fatal(err)
	}
}

// B-298/AC-02: mehrere Zuordnungen stehen in gesetzter Reihenfolge im Register und kommen so wieder heraus.
func TestReihenfolgeImRegister(t *testing.T) {
	root := repo(t)
	want := []Choice{{File: "grafik/town/wagon.png"}, {File: "grafik/town/well.png"}}
	if _, err := SetChoices(root, "farm", want); err != nil {
		t.Fatal(err)
	}
	reg, err := Load(root)
	if err != nil {
		t.Fatal(err)
	}
	got := reg.Find("farm").Choices
	if len(got) != 2 || got[0].File != want[0].File || got[1].File != want[1].File {
		t.Errorf("Reihenfolge: %+v", got)
	}
	if st := Status(root, *reg.Find("farm")); st != StatusAssigned {
		t.Errorf("Status %q", st)
	}
}

// B-298/AC-03: ein Ausschnitt wird gespeichert und gelesen; außerhalb des Bildes wird er abgelehnt.
func TestAusschnitt(t *testing.T) {
	root := repo(t)
	cut := &Rect{X: 0, Y: 0, W: 64, H: 48}
	if _, err := SetChoices(root, "farm", []Choice{{File: "grafik/town/well.png", Cut: cut}}); err != nil {
		t.Fatal(err)
	}
	reg, _ := Load(root)
	if got := reg.Find("farm").Choices[0].Cut; got == nil || *got != *cut {
		t.Errorf("Ausschnitt %+v", got)
	}
	_, err := SetChoices(root, "farm", []Choice{{File: "grafik/town/well.png", Cut: &Rect{X: 40, Y: 0, W: 32, H: 48}}})
	if err == nil || !strings.Contains(err.Error(), "außerhalb") {
		t.Errorf("Ausschnitt außerhalb: %v", err)
	}
}

func TestUngueltigeZuordnungen(t *testing.T) {
	root := repo(t)
	cases := map[string][]Choice{
		"fehlt":         {{File: "grafik/town/gibts-nicht.png"}},
		"unter public/": {{File: "../data/assets.json"}},
		"Art grafik":    {{File: "audio/coin.ogg"}},
	}
	for want, choices := range cases {
		if _, err := SetChoices(root, "farm", choices); err == nil || !strings.Contains(err.Error(), want) {
			t.Errorf("%s: %v", want, err)
		}
	}
	if _, err := SetChoices(root, "nichts", nil); err == nil || !strings.Contains(err.Error(), "unbekannter Asset-Slot") {
		t.Errorf("unbekannt: %v", err)
	}
	if _, err := SetChoices(root, "sfx:coin", []Choice{{File: "audio/coin.ogg", Sprite: "coin"}}); err != nil {
		t.Errorf("Sound: %v", err)
	}
}

// Ein kaputtes Register wird nie überschrieben.
func TestKaputtesRegisterBleibt(t *testing.T) {
	root := repo(t)
	path := filepath.Join(root, RegisterPath)
	writeFile(t, path, "{ kaputt")
	if _, err := SetChoices(root, "farm", nil); err == nil || !strings.Contains(err.Error(), RegisterPath) {
		t.Errorf("Fehler ohne Pfad: %v", err)
	}
	if b, _ := os.ReadFile(path); string(b) != "{ kaputt" {
		t.Errorf("Register überschrieben: %q", b)
	}
}

func TestStatus(t *testing.T) {
	root := repo(t)
	reg, _ := Load(root)
	if st := Status(root, *reg.Find("farm")); st != StatusGap {
		t.Errorf("ohne Zuordnung: %q", st)
	}
	s := Slot{ID: "x", Kind: KindGraphic, Choices: []Choice{{File: "grafik/town/weg.png"}}}
	if st := Status(root, s); st != StatusMissing {
		t.Errorf("fehlende Datei: %q", st)
	}
}

// Speichern behält das Format: zwei Leerzeichen, Umlaute im Klartext, Zeilenende \n.
func TestSpeichernFormat(t *testing.T) {
	root := repo(t)
	if _, err := SetChoices(root, "farm", []Choice{{File: "grafik/town/well.png"}}); err != nil {
		t.Fatal(err)
	}
	b, _ := os.ReadFile(filepath.Join(root, RegisterPath))
	text := string(b)
	if !strings.Contains(text, "\n  \"slots\"") || !strings.Contains(text, "Münze") || !strings.HasSuffix(text, "}\n") {
		t.Errorf("Format:\n%s", text)
	}
}
