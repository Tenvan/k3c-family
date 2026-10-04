package sim

import "testing"

// Ursache im Ereignis playerDown (B-182, W0 › AC-07).

// downCause lässt Gegner der Art kind bei Spieler 1 (1 HP, ohne Gold) angreifen und liefert sein playerDown.
func downCause(t *testing.T, kind string, dx float64) Event {
	t.Helper()
	w := quietWorld(t)
	p0, p1 := AddPlayer(w), AddPlayer(w)
	p0.X, p1.X = w.HubX+40, w.HubX-40
	p1.HP, p1.Gold = 1, 0
	spawnEnemy(w, kind, p1.X+dx)
	downs := eventsOf(w, "playerDown", 30*10, []PlayerCommand{{}, {}})
	if len(downs) != 1 {
		t.Fatalf("%s: erwartet genau ein playerDown (Spieler 1), erhalten %v", kind, downs)
	}
	return downs[0]
}

func TestCauseNahkampfNenntGegnerart(t *testing.T) {
	if ev := downCause(t, "goblin", -0.5); ev["player"] != 1 || ev["cause"] != "goblin" {
		t.Fatalf("Nahkampf: %v", ev)
	}
}

func TestCauseGeschossNenntGegnerart(t *testing.T) {
	if ev := downCause(t, "goblinArcher", -6); ev["player"] != 1 || ev["cause"] != "goblinArcher" {
		t.Fatalf("Geschoss: %v", ev)
	}
}

func TestCauseOhneGegnerFesterWert(t *testing.T) {
	w := quietWorld(t)
	AddPlayer(w)
	p1 := AddPlayer(w)
	applyDamage(w, p1.ID, 1e6)
	if ev := w.Events[len(w.Events)-1]; ev["type"] != "playerDown" || ev["player"] != 1 || ev["cause"] != causeOther {
		t.Fatalf("ohne Gegner: %v", ev)
	}
}
