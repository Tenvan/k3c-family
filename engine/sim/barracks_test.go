package sim

import "testing"

// W3.1 (B-116/AC-01, Sprint W3 AC-01): Kämpfer-Limit je Hub (Q29), Basis plus Zuschlag je gebauter Kaserne; geprüft
// nur beim Waffe-Holen.

func TestKaserneHebtLimitUmDatenwert(t *testing.T) {
	w := quietWorld(t)
	l := buildings["barracks"].TroopLimit
	if got := troopLimit(w); got != l.Base {
		t.Fatalf("Limit ohne Kaserne %d, laut Daten %d", got, l.Base)
	}
	barracks := buildSite(t, w, "barracks")
	if got := troopLimit(w); got != l.Base+l.PerBuilding {
		t.Errorf("Limit mit Kaserne %d, laut Daten %d", got, l.Base+l.PerBuilding)
	}
	destroySite(w, barracks)
	if got := troopLimit(w); got != l.Base {
		t.Errorf("Limit nach Zerstörung %d, erwartet %d", got, l.Base)
	}
}

// limitWorld: ein Bauer, eine Werkstatt mit einem Bogen, das Limit voll mit Bogenschützen; dazu ein Landstreicher,
// der nicht zählt; zwei Spieler.
func limitWorld(t *testing.T) (*World, *Site) {
	t.Helper()
	w := dayWorld(t)
	AddPlayer(w)
	AddPlayer(w)
	for range buildings["barracks"].TroopLimit.Base {
		makeArcher(w, spawnVagrant(w, w.HubX, w.HubX))
	}
	spawnVagrant(w, w.HubX+30, w.HubX+30)
	workshop := buildSite(t, w, "workshop")
	workshop.Bows = 1
	return w, workshop
}

func TestLimitVollBogenBleibtImRegal(t *testing.T) {
	w, workshop := limitWorld(t)
	peasant := w.Troops[0]
	run(w, 20)
	if peasant.Kind != "peasant" || workshop.Bows != 1 || fighters(w) != troopLimit(w) {
		t.Fatalf("Limit voll: Bauer ist %s, Bögen im Regal %d, Kämpfer %d/%d", peasant.Kind, workshop.Bows,
			fighters(w), troopLimit(w))
	}
	buildSite(t, w, "barracks")
	if runUntil(w, 30, func() bool { return peasant.Kind == "archer" }) < 0 || workshop.Bows != 0 {
		t.Errorf("mit Kaserne: Bauer ist %s, Bögen im Regal %d", peasant.Kind, workshop.Bows)
	}
}

// Eine zerstörte Kaserne senkt das Limit, bestehende Kämpfer bleiben.
func TestKaserneZerstoertKaempferBleiben(t *testing.T) {
	w, _ := limitWorld(t)
	barracks := buildSite(t, w, "barracks")
	peasant := w.Troops[0]
	runUntil(w, 30, func() bool { return peasant.Kind == "archer" })
	before := fighters(w)
	destroySite(w, barracks)
	run(w, 1)
	if fighters(w) != before || before != buildings["barracks"].TroopLimit.Base+1 {
		t.Errorf("Kämpfer %d nach Zerstörung, vorher %d", fighters(w), before)
	}
	other := spawnVagrant(w, w.HubX, w.HubX)
	other.Kind = "peasant"
	if bowToFetch(w, other) != nil {
		t.Error("über dem Limit gibt es trotzdem einen Abhol-Auftrag")
	}
}
