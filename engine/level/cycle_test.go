package level

import (
	"strings"
	"testing"
)

// W12.1 (B-470/AC-02): Die Dauer des Morgengrauens steht in data/biomes/*.json › cycle; fehlt eine Dauer des
// Tag/Nacht-Zyklus oder ist sie nicht positiv, lehnt das Laden ab.

func TestWaldHatVierPhasen(t *testing.T) {
	b, err := LoadBiome("forest")
	if err != nil {
		t.Fatal(err)
	}
	c := b.Cycle
	if c.Type != "dayNight" || c.DayMinutes <= 0 || c.TwilightMinutes <= 0 || c.NightMinutes <= 0 || c.DawnMinutes <= 0 {
		t.Fatalf("Zyklus %+v", c)
	}
}

func TestCycleCheckLehntFehlendeOderNichtPositiveDauerAb(t *testing.T) {
	ok := Cycle{Type: "dayNight", DayMinutes: 6, TwilightMinutes: 2, NightMinutes: 4, DawnMinutes: 2}
	if err := ok.Check(); err != nil {
		t.Fatal(err)
	}
	for name, mutate := range map[string]func(*Cycle){
		"dawnMinutes":     func(c *Cycle) { c.DawnMinutes = 0 },
		"nightMinutes":    func(c *Cycle) { c.NightMinutes = -1 },
		"dayMinutes":      func(c *Cycle) { c.DayMinutes = 0 },
		"twilightMinutes": func(c *Cycle) { c.TwilightMinutes = -0.5 },
	} {
		c := ok
		mutate(&c)
		if err := c.Check(); err == nil || !strings.Contains(err.Error(), name) {
			t.Errorf("%s: Fehler %v", name, err)
		}
	}
	if err := (Cycle{Type: "aggressionPool", PercentPerMinute: 1}).Check(); err != nil {
		t.Errorf("Aggressionspool ohne Phasen abgelehnt: %v", err)
	}
}
