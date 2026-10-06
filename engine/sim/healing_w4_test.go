package sim

import (
	"math"
	"testing"
)

// W4.3b (B-122/AC-03, Sprint W4 AC-05): Regressionstest des Heilplatzes aus W3.2 für Kämpfer. Ein verwundeter Krieger
// und ein Spieler in Reichweite werden geheilt, ein Krieger außerhalb nicht, ohne gebauten Heilplatz niemand.

// healW4World: Heilplatz, je ein verwundeter Krieger in und außerhalb der Reichweite, ein verwundeter Spieler am Platz.
func healW4World(t *testing.T) (w *World, healer *Site, near, far *Troop, p *Player) {
	t.Helper()
	w = dayWorld(t)
	healer, _ = hubSiteOf(t, w, "healer")
	r := buildings["healer"].Heal.RadiusUnits
	near, far = fighterAt(w, "warrior", healer.X+r/2), fighterAt(w, "warrior", healer.X+r+4)
	p = AddPlayer(w)
	p.X = healer.X
	near.HP, far.HP, p.HP = 10, 10, 10
	return w, healer, near, far, p
}

func TestHealW4KriegerUndSpielerInReichweite(t *testing.T) {
	w, healer, near, far, p := healW4World(t)
	built(healer)
	stepHealing(w, 1) // eine Sekunde Heilung, ohne dass die Krieger zum Posten laufen
	want := 10 + buildings["healer"].Heal.HPPerSecond
	if math.Abs(near.HP-want) > 1e-9 || math.Abs(p.HP-want) > 1e-9 {
		t.Errorf("in Reichweite: Krieger %v, Spieler %v, laut Daten %v", near.HP, p.HP, want)
	}
	if far.HP != 10 {
		t.Errorf("Krieger außerhalb der Reichweite geheilt: %v", far.HP)
	}
}

func TestHealW4OhneHeilplatzKeineHeilung(t *testing.T) {
	w, _, near, far, p := healW4World(t)
	stepHealing(w, 1)
	run(w, 1)
	if near.HP != 10 || far.HP != 10 || p.HP != 10 {
		t.Errorf("ohne Heilplatz geheilt: Krieger %v, %v, Spieler %v", near.HP, far.HP, p.HP)
	}
}
