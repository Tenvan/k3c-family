package assets

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

var day = time.Date(2026, 10, 14, 9, 0, 0, 0, time.UTC)

// B-298/AC-04: Treffer bleiben gespeichert (auch nach Neuladen), ohne Doppel; Verwerfen entfernt sie.
func TestTrefferAblage(t *testing.T) {
	root := repo(t)
	hits := []Hit{
		{Title: "Coin", Link: "https://freesound.org/s/1/", Source: "freesound", License: "CC0 1.0"},
		{Title: "Coin itch", Link: "https://x.itch.io/coin", Source: "Agent"},
	}
	if n, err := AddHits(root, "sfx:coin", hits, day); err != nil || n != 2 {
		t.Fatalf("AddHits: %d %v", n, err)
	}
	if n, _ := AddHits(root, "sfx:coin", hits[:1], day); n != 0 {
		t.Errorf("Doppel angelegt: %d", n)
	}
	got, err := HitsOf(root, "sfx:coin")
	if err != nil || len(got) != 2 || got[0].LicenseStatus != LicenseKnown || got[1].LicenseStatus != LicenseCheck || got[0].Created != "2026-10-14" || got[0].ID == "" {
		t.Fatalf("gespeichert: %+v %v", got, err)
	}
	dropHit(t, root, got[0].ID)
}

func dropHit(t *testing.T, root, id string) {
	t.Helper()
	if err := DropHit(root, "sfx:coin", id); err != nil {
		t.Fatal(err)
	}
	if got, _ := HitsOf(root, "sfx:coin"); len(got) != 1 || got[0].Title != "Coin itch" {
		t.Errorf("nach Verwerfen: %+v", got)
	}
	if err := DropHit(root, "sfx:coin", "nichts"); err == nil {
		t.Error("unbekannter Treffer ohne Fehler")
	}
	if _, err := AddHits(root, "gibts-nicht", []Hit{{Title: "a", Link: "https://a"}}, day); err == nil || !strings.Contains(err.Error(), "unbekannter Asset-Slot") {
		t.Errorf("unbekannter Slot: %v", err)
	}
}

func TestTrefferKaputteDateiBleibt(t *testing.T) {
	root := repo(t)
	path := filepath.Join(root, ResearchPath)
	writeFile(t, path, "[")
	if _, err := AddHits(root, "farm", []Hit{{Title: "a", Link: "https://a"}}, day); err == nil {
		t.Error("kaputte Datei ohne Fehler")
	}
	if b, _ := os.ReadFile(path); string(b) != "[" {
		t.Errorf("überschrieben: %q", b)
	}
}

// Q70: übernehmbar, NC markiert, ND und Quellen ohne Weitergabe ausgeschlossen, Unklares prüfen.
func TestClassifyLicense(t *testing.T) {
	cases := []struct{ lic, link, status, flag string }{
		{"CC0 1.0", "https://opengameart.org/x", LicenseKnown, ""},
		{"CC BY-SA 4.0", "https://commons.wikimedia.org/x", LicenseKnown, "SA"},
		{"CC BY-NC 3.0", "https://freesound.org/x", LicenseKnown, "NC"},
		{"OGA-BY 3.0", "", LicenseKnown, ""},
		{"CC BY-NC-ND 4.0", "", LicenseExcluded, "ND"},
		{"CC0 1.0", "https://pixabay.com/sound-effects/x", LicenseExcluded, ""},
		{"", "https://x.itch.io/coin", LicenseCheck, ""},
		{"Attribution required", "", LicenseCheck, ""},
	}
	for _, c := range cases {
		if st, fl := ClassifyLicense(c.lic, c.link); st != c.status || fl != c.flag {
			t.Errorf("%q %q: %s %s, erwartet %s %s", c.lic, c.link, st, fl, c.status, c.flag)
		}
	}
}
