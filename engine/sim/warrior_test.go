package sim

import (
	"math"
	"testing"
)

// W4.3a (B-014, B-122; Sprint W4 AC-04, AC-06): Schwert an der Werkstatt, Krieger hinter der äußersten gebauten
// Sperre (Q46), Nahkampf mit den Werten aus data/troops.json.

// warriorPostOf lässt den Krieger zum Posten laufen und gibt seine Position zurück.
func warriorPostOf(w *World, tr *Troop) float64 {
	run(w, 15)
	return tr.X
}

// firstHit lässt einen Skelett-Gegner bei x loslaufen und gibt den ersten Schaden zurück, den er nimmt (0 = keiner).
func firstHit(w *World, x float64) float64 {
	e := spawnEnemy(w, "skeleton", x)
	for tm := 0.0; tm < 20; tm += dt {
		Step(w, nil, dt)
		if e.HP < e.MaxHP {
			return e.MaxHP - e.HP
		}
	}
	return 0
}

func TestWarriorNimmtSchwertUndKaempftHinterDerMauer(t *testing.T) {
	w := dayWorld(t)
	AddPlayer(w)
	AddPlayer(w)
	workshop := buildSite(t, w, "workshop")
	workshop.Swords = 1
	wall := sturdy(lineSite(t, w, "wall", -1, 1))
	peasant := w.Troops[0]
	if runUntil(w, 30, func() bool { return peasant.Kind == "warrior" }) < 0 || workshop.Swords != 0 {
		t.Fatalf("Bauer ist %s, Schwerter im Regal %d", peasant.Kind, workshop.Swords)
	}
	if peasant.MaxHP != troops["warrior"].HP {
		t.Errorf("Krieger-HP %v, laut Daten %v", peasant.MaxHP, troops["warrior"].HP)
	}
	want := wall.X + troops["warrior"].PostUnits
	if x := warriorPostOf(w, peasant); math.Abs(x-want) > arrive {
		t.Fatalf("Krieger bei %v, Posten hinter der Mauer %v erwartet", x, want)
	}
	if got := firstHit(w, wall.X-8); got != troops["warrior"].Damage {
		t.Errorf("Gegner an der Mauer nimmt %v Schaden, laut Daten %v", got, troops["warrior"].Damage)
	}
}

func TestWarriorStehtZwischenMauerUndTor(t *testing.T) {
	w := dayWorld(t)
	sturdy(lineSite(t, w, "wall", -1, 1))
	gate := sturdy(lineSite(t, w, "gate", -1, 1))
	tr := w.Troops[0]
	makeFighter(w, tr, "warrior")
	want := gate.X + troops["warrior"].PostUnits
	if x := warriorPostOf(w, tr); math.Abs(x-want) > arrive {
		t.Fatalf("Krieger bei %v, Posten hinter dem Tor %v erwartet", x, want)
	}
	if got := firstHit(w, gate.X-8); got != troops["warrior"].Damage {
		t.Errorf("Gegner am Tor nimmt %v Schaden, laut Daten %v", got, troops["warrior"].Damage)
	}
}

// Mit gebauter Linie 2 steht keiner an Linie 1 (Q54), zwei Krieger verteilen sich auf beide Seiten.
func TestWarriorAeussersteLinieUndBeideSeiten(t *testing.T) {
	w := dayWorld(t)
	inner := sturdy(lineSite(t, w, "wall", -1, 1))
	outer := sturdy(lineSite(t, w, "wall", -1, 2))
	right := sturdy(lineSite(t, w, "wall", 1, 1))
	a, b := w.Troops[0], spawnVagrant(w, w.HubX, w.HubX)
	makeFighter(w, a, "warrior")
	makeFighter(w, b, "warrior")
	run(w, 30)
	left, other := a, b
	if a.X > w.HubX {
		left, other = b, a
	}
	if math.Abs(left.X-(outer.X+troops["warrior"].PostUnits)) > arrive || math.Abs(left.X-inner.X) < 2 {
		t.Errorf("linker Krieger bei %v, erwartet an Linie 2 (%v), nicht an Linie 1 (%v)", left.X, outer.X, inner.X)
	}
	if math.Abs(other.X-(right.X-troops["warrior"].PostUnits)) > arrive {
		t.Errorf("rechter Krieger bei %v, erwartet hinter der rechten Mauer %v", other.X, right.X)
	}
}

// (b) Ein bezahltes Schwert liegt erst nach craftSeconds.sword im Regal.
func TestWarriorSchwertNachHerstellungszeit(t *testing.T) {
	w := quietWorld(t)
	s := buildSite(t, w, "workshop")
	*w.Stock = Stock{Wood: 1000}
	s.SwordPaidGold = troops["warrior"].Cost.Gold
	secs := 0.0
	for ; s.Swords == 0 && secs < 60; secs += dt {
		w.Time += dt
		stepSites(w)
	}
	if want := buildings["workshop"].CraftSeconds["sword"]; !aboutDt(secs-dt, want) {
		t.Errorf("Schwert nach %.2f s, erwartet %.2f s", secs-dt, want)
	}
	if w.Stock.Wood != 1000-troops["warrior"].Cost.Wood {
		t.Errorf("Holz %d, erwartet %d verbraucht", w.Stock.Wood, troops["warrior"].Cost.Wood)
	}
}

// (e) Zwei Spieler zahlen gleichzeitig: einer am Bogen (Werkstatt), einer am Schwert-Zahlziel.
func TestWarriorZweiSpielerZahlenBogenUndSchwert(t *testing.T) {
	w := quietWorld(t)
	p1, p2 := AddPlayer(w), AddPlayer(w)
	s := buildSite(t, w, "workshop")
	spot, ok := swordSpot(s)
	if !ok {
		t.Fatal("Werkstatt ohne Schwert-Zahlziel")
	}
	g1, g2 := p1.Gold, p2.Gold
	for tm := 0.0; tm < 15; tm += dt {
		p1.X, p2.X = s.X, spot
		Step(w, []PlayerCommand{{Pay: true}, {Pay: true}}, dt)
	}
	bow, sword := troops["archer"].Cost.Gold, troops["warrior"].Cost.Gold
	if s.BowPaidGold != bow || s.SwordPaidGold != sword || g1-p1.Gold != bow || g2-p2.Gold != sword {
		t.Errorf("Bogen %d/%d (Spieler 1 zahlte %d), Schwert %d/%d (Spieler 2 zahlte %d)",
			s.BowPaidGold, bow, g1-p1.Gold, s.SwordPaidGold, sword, g2-p2.Gold)
	}
}
