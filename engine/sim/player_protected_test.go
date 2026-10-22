package sim

import "testing"

// BD3.2, B-214/AC-02 (Teil Simulation): Ein geschützter Monarch (Protected, vom Raum gesetzt) wird von Gegnern nicht
// als Ziel gewählt, verliert keine HP und erzeugt kein hit; ohne Schutz trifft ihn derselbe Gegner.

// protectedRun stellt einen Gegner der Art kind neben Spieler 1 und rechnet 10 s; Ergebnis: HP-Verlust und Zahl der
// hit-Ereignisse an Spieler 1.
func protectedRun(t *testing.T, kind string, dx float64, protected bool) (lost float64, hits int) {
	t.Helper()
	w := quietWorld(t)
	p0, p1 := AddPlayer(w), AddPlayer(w)
	p0.X, p1.X = w.HubX+40, w.HubX-40
	p1.Protected = protected
	hp := p1.HP
	spawnEnemy(w, kind, p1.X+dx)
	for _, ev := range eventsOf(w, "hit", 30*10, []PlayerCommand{{}, {}}) {
		if ev["target"] == "player" && ev["id"] == p1.Index {
			hits++
		}
	}
	return hp - p1.HP, hits
}

func TestProtectedOhneTreffer(t *testing.T) {
	for _, c := range []struct {
		kind string
		dx   float64
	}{{"goblin", -0.5}, {"goblinArcher", -6}} {
		if lost, hits := protectedRun(t, c.kind, c.dx, false); hits == 0 {
			t.Fatalf("%s ohne Schutz: HP-Verlust %v, hits %d – Testaufbau trifft nicht", c.kind, lost, hits)
		}
		if lost, hits := protectedRun(t, c.kind, c.dx, true); lost != 0 || hits != 0 {
			t.Errorf("%s mit Schutz: HP-Verlust %v, hits %d", c.kind, lost, hits)
		}
	}
}

func TestProtectedKeinZiel(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	p.X = w.HubX - 40
	e := spawnEnemy(w, "goblin", p.X-0.5)
	if got := targetOf(w, e); got != p.ID {
		t.Fatalf("ohne Schutz: Ziel %d, erwartet Spieler %d", got, p.ID)
	}
	p.Protected = true
	if got := targetOf(w, e); got == p.ID {
		t.Fatal("geschützter Monarch ist Ziel")
	}
	for _, c := range aoeTargets(w, p.X, 2) {
		if c.player == p {
			t.Fatal("geschützter Monarch im Flächenschaden")
		}
	}
}
