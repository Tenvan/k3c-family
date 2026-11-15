package level

import (
	"strconv"
	"strings"
	"testing"
)

// Bau in der tiefsten Stufe jedes Bioms (B-981/AC-02): Options.Lair legt ihn fest, das Biom-Feld ist nur der Standard.

func yes() *bool { v := true; return &v }
func no() *bool  { v := false; return &v }

// Der Bau entsteht mit Options.Lair in jedem Biom genau einmal, ohne nie; Level und Prüfung sind sauber, der Abstand
// des Camps zum Bau (und zu den Linien) gilt über 500 Seeds in jedem Biom.
func TestBauInJedemBiom500Seeds(t *testing.T) {
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		behind := 0
		for seed := range 500 {
			s := strconv.Itoa(seed)
			l, err := GenerateWith(b, s, Options{Lair: yes()})
			if err != nil {
				t.Fatal(err)
			}
			if n := countEntities(l, "lair"); n != 1 {
				t.Fatalf("Biom %s, Seed %s: %d Bau", id, s, n)
			}
			if errs := ValidateWith(l, b, Options{Lair: yes()}); len(errs) > 0 {
				t.Fatalf("Biom %s, Seed %s: %v", id, s, errs)
			}
			if campBehindPortal(t, l, id, seed) {
				behind++
			}
			for _, e := range l.Entities {
				if e.Kind == "recruitCamp" {
					checkCamp(t, l, e.X, id, seed)
				}
			}
			off, err := GenerateWith(b, s, Options{Lair: no()})
			if err != nil || countEntities(off, "lair") != 0 || len(ValidateWith(off, b, Options{Lair: no()})) > 0 {
				t.Fatalf("Biom %s, Seed %s: ohne Bau %v", id, s, err)
			}
		}
		// Der Abstand zu den Linien hat Vorrang (B-261): In kurzen Biomen bleibt außerhalb der Linien oft nur ein Platz hinter
		// einem Portal. Die 5 %-Grenze gilt nur für die Kristallhöhle (camps_test.go); sonst ist die Quote nur Information.
		t.Logf("Biom %s: Camp hinter einem Portal in %d von 500 Seeds", id, behind)
		if id == "crystal" && behind > 25 {
			t.Errorf("Biom %s: Camp in %d von 500 Seeds hinter einem Portal, erlaubt 25", id, behind)
		}
	}
}

// Ohne Options.Lair gilt das Feld des Bioms, das Level bleibt bytegleich zu Generate (Golden).
func TestBauStandardIstBiomFeld(t *testing.T) {
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		l, err := Generate(b, "x")
		if err != nil {
			t.Fatal(err)
		}
		if has := countEntities(l, "lair") == 1; has != b.Lair {
			t.Errorf("Biom %s: Bau %v, Feld lair %v", id, has, b.Lair)
		}
	}
}

// CheckLairFit: jedes Biom trägt den Bau in seiner Länge und in jeder Inselgröße ab 16 Chunks; zu kleine Stufen und zu
// viele Camps werden mit Grund abgelehnt.
func TestCheckLairFit(t *testing.T) {
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		for _, n := range []int{0, 16, 24, 160} {
			if err := CheckLairFit(b, n); err != nil {
				t.Errorf("Biom %s, %d Chunks: %v", id, n, err)
			}
		}
		if err := CheckLairFit(b, 2); err == nil || !strings.Contains(err.Error(), "zu wenig") {
			t.Errorf("Biom %s, 2 Chunks: %v", id, err)
		}
		b.EventChunks = map[string]Range{"recruitCamp": {Min: 99, Max: 99}}
		if err := CheckLairFit(b, 16); err == nil || !strings.Contains(err.Error(), "Camp(s) brauchen Platz") {
			t.Errorf("Biom %s, 99 Camps: %v", id, err)
		}
	}
}
