package assets

import (
	"strings"
	"testing"
)

// Die echte Vorschlagsliste: je Lücke und Skill-Linie mindestens drei gültige Kandidaten (B-601/AC-01, WZ29.5).
func TestEchteVorschlagsliste(t *testing.T) {
	root := repoRoot(t)
	reg, err := Load(root)
	if err != nil {
		t.Fatal(err)
	}
	s, err := LoadSuggestions(root)
	if err != nil {
		t.Fatal(err)
	}
	targets := SuggestionTargets(reg)
	if len(targets) == 0 {
		t.Fatal("keine Ziele: weder Lücken noch Skill-Linien im Register")
	}
	for _, e := range CheckSuggestions(s.List, targets) {
		t.Error(e)
	}
}

func TestVorschlagPruefung(t *testing.T) {
	ok := Suggestion{ID: "a", Slot: "boss", Kind: KindGraphic, Title: "Boss", Source: "OpenGameArt", Page: "https://opengameart.org/x",
		FileURL: "https://opengameart.org/x.png", License: "CC-BY 4.0", FileName: "x.png", Size: 100, Priority: 1, Status: "offen"}
	targets := map[string]string{"boss": KindGraphic}
	list := []Suggestion{ok, ok, ok}
	for i := range list {
		list[i].ID = string(rune('a' + i))
		list[i].FileURL += strings.Repeat("?", i)
	}
	if errs := CheckSuggestions(list, targets); len(errs) != 0 {
		t.Fatalf("gültige Liste abgelehnt: %v", errs)
	}
	bad := map[string]func(*Suggestion){
		"NC":       func(s *Suggestion) { s.License = "CC-BY-NC 4.0" },
		"ND":       func(s *Suggestion) { s.License = "CC BY-ND 3.0" },
		"Sampling": func(s *Suggestion) { s.License = "Sampling+ 1.0" },
		"Größe":    func(s *Suggestion) { s.Size = MaxSuggestionBytes + 1 },
		"http":     func(s *Suggestion) { s.FileURL = "http://example.org/x.png" },
		"Quelle":   func(s *Suggestion) { s.Source = "irgendwo" },
	}
	for name, f := range bad {
		s := ok
		f(&s)
		if checkSuggestion(s) == nil {
			t.Errorf("%s: nicht abgelehnt", name)
		}
	}
	if errs := CheckSuggestions(list[:2], targets); len(errs) != 1 {
		t.Errorf("zwei Kandidaten: %v", errs)
	}
	if errs := CheckSuggestions(append(list, list[0]), targets); len(errs) != 1 {
		t.Errorf("doppelte ID: %v", errs)
	}
}

func TestVorschlagZiele(t *testing.T) {
	reg := &Register{Slots: []Slot{
		{ID: "boss", Kind: KindGraphic},
		{ID: "sfx:skill:mage", Kind: KindSound, Choices: []Choice{{File: "a.ogg"}}},
		{ID: "castle", Kind: KindGraphic, Choices: []Choice{{File: "b.png"}}},
	}}
	got := SuggestionTargets(reg)
	want := map[string]string{"boss": KindGraphic, "sfx:skill:mage": KindSound, "skill-effekt:mage": KindGraphic}
	if len(got) != len(want) {
		t.Fatalf("Ziele %v, erwartet %v", got, want)
	}
	for k, v := range want {
		if got[k] != v {
			t.Errorf("%s: %q, erwartet %q", k, got[k], v)
		}
	}
}
