package balance

import (
	"fmt"
	"strings"
)

// Seed-Modus (B-634, BS1.1): Bis zur Abnahme der neuen Werte (Projekt BAS) rechnet ein Lauf ohne Seed-Angabe nur den
// ersten Seed der Datei (`k3c-dev`), danach wieder alle. Ausdrücklich übergebene Seeds (--seeds N, --seed-list) gehen
// vor dem Modus.
const (
	SeedModeDev  = "dev"     // nur der erste Seed der Datei
	SeedModeAll  = "all"     // alle Seeds der Datei (Standard, wenn das Feld fehlt)
	seedModePick = "auswahl" // ausdrücklich übergebene Seeds (WithSeeds); nur im Speicher, nie in der Datei
)

// DevSeed ist der erste Seed der Datei während BAS; ein Test prüft, dass er vorn steht.
const DevSeed = "k3c-dev"

func validSeedMode(m string) bool { return m == "" || m == SeedModeDev || m == SeedModeAll }

// seedList: die ersten n Seeds der Datei; n = 0 folgt dem Seed-Modus (dev: nur der erste, sonst alle).
func (t *Targets) seedList(n int) []string {
	if n == 0 && t.SeedMode == SeedModeDev {
		n = 1
	}
	if n > 0 && n < len(t.Seeds) {
		return t.Seeds[:n]
	}
	return t.Seeds
}

// WithSeeds liefert eine Kopie, die genau diese Seeds rechnet (--seed-list, sim_test seedList); der Modus gilt dann
// nicht mehr. Leere oder doppelte Seeds sind ein Fehler.
func (t *Targets) WithSeeds(seeds []string) (*Targets, error) {
	out := *t
	out.Seeds, out.SeedMode = seeds, SeedModeAll
	if err := out.validateSeeds(); err != nil {
		return nil, err
	}
	out.SeedMode = seedModePick
	return &out, nil
}

// allSeeds liefert eine Kopie ohne Seed-Modus dev: ohne Angabe rechnet sie alle Seeds (Sensitivität, Kurven).
func (t *Targets) allSeeds() *Targets {
	if t.SeedMode != SeedModeDev {
		return t
	}
	out := *t
	out.SeedMode = SeedModeAll
	return &out
}

func (t *Targets) validateSeeds() error {
	if !validSeedMode(t.SeedMode) {
		return fmt.Errorf("seedMode %q: %s oder %s", t.SeedMode, SeedModeDev, SeedModeAll)
	}
	if len(t.Seeds) == 0 {
		return fmt.Errorf("seeds: mindestens ein Seed nötig")
	}
	seen := map[string]bool{}
	for _, s := range t.Seeds {
		if s == "" || seen[s] {
			return fmt.Errorf("seeds: %q ist leer oder doppelt", s)
		}
		seen[s] = true
	}
	return nil
}

// seedModeOf nennt den Modus eines Laufs über seeds: dev, wenn der Modus dev gilt und genau der erste Seed lief, all
// bei allen Seeds der Datei, sonst auswahl.
func (t *Targets) seedModeOf(seeds []string) string {
	switch {
	case t.SeedMode == seedModePick:
		return seedModePick
	case t.SeedMode == SeedModeDev && len(seeds) == 1 && seeds[0] == t.Seeds[0]:
		return SeedModeDev
	case len(seeds) == len(t.Seeds):
		return SeedModeAll
	}
	return seedModePick
}

// seedText ist die Kopfzeile des Berichts zu Seeds und Modus, z. B. „1 Seed, Seed-Modus dev (k3c-dev)“.
func (s Summary) seedText() string {
	word := "Seeds"
	if s.Seeds == 1 {
		word = "Seed"
	}
	out := fmt.Sprintf("%d %s", s.Seeds, word)
	if s.SeedMode == "" {
		return out
	}
	out += ", Seed-Modus " + s.SeedMode
	if len(s.SeedNames) > 0 {
		out += " (" + strings.Join(s.SeedNames, ", ") + ")"
	}
	return out
}
