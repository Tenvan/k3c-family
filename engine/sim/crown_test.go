package sim

import (
	"math"
	"testing"
)

// Monarch zahlt Treffer mit Gold, die Krone fällt und wird geraubt (B-930, B-931, docs/rules/kampf.md §§ 5 und 6).

// crownWorld: ruhige Welt, zwei Monarchen weit weg von der Burg (P0 links, P1 rechts), Portal links bei x−60.
func crownWorld(t *testing.T) (w *World, p0, p1 *Player) {
	t.Helper()
	w = quietWorld(t)
	p0, p1 = AddPlayer(w), AddPlayer(w)
	p0.X, p1.X = w.HubX+30, w.HubX+34
	w.Portals = []float64{w.HubX + 90}
	return w, p0, p1
}

// AC-01 (B-930): Ein Treffer kostet hitCoins Münzen, die als Loot-Münze zu Boden fallen; der Getroffene sammelt sie nicht sofort ein.
func TestKroneTrefferKostetMuenzeUndSieFaelltZuBoden(t *testing.T) {
	w, p0, p1 := crownWorld(t)
	probe(p0, p1)
	w.Coins = nil
	applyDamage(w, p0.ID, 40)
	if p0.Gold != probeGold-monarch.HitCoins || p1.Gold != probeGold || p0.Crown != nil {
		t.Fatalf("Gold %d/%d, Krone %v", p0.Gold, p1.Gold, p0.Crown)
	}
	if len(w.Coins) != monarch.HitCoins || !w.Coins[0].Loot || w.Coins[0].BlockedPlayerID == nil || *w.Coins[0].BlockedPlayerID != p0.ID {
		t.Fatalf("Münzen am Boden: %+v", w.Coins)
	}
	steps(w, nil, 0.5)
	if p0.Gold != probeGold-monarch.HitCoins {
		t.Fatalf("der Getroffene sammelt die Münze sofort wieder ein: %d", p0.Gold)
	}
	p1.X = w.Coins[0].X // der Mitspieler darf sie aufsammeln
	steps(w, nil, 0.1)
	if p1.Gold != probeGold+1 {
		t.Fatalf("Mitspieler sammelt die Münze nicht auf: %d", p1.Gold)
	}
}

// AC-02 (B-930) und AC-01 (B-931): Ohne Gold fällt die Krone in hohem Bogen nach hinten (weg vom Gegner), deterministisch.
func TestKroneFaelltOhneGoldNachHinten(t *testing.T) {
	land := func(enemyDX float64) (*World, *Player) {
		w, p0, _ := crownWorld(t)
		p0.Gold = 0
		spawnEnemy(w, "goblin", p0.X+enemyDX)
		applyDamage(w, p0.ID, 5)
		return w, p0
	}
	for _, c := range []struct{ dx, dir float64 }{{1, -1}, {-1, 1}} {
		w, p0 := land(c.dx)
		c0 := p0.Crown
		if c0 == nil || c0.State != CrownFlying || c0.ToX != p0.X+c.dir*monarch.Crown.DistanceUnits {
			t.Fatalf("Gegner bei %+v: Krone %+v", c, c0)
		}
		w.Enemies = nil
		peak := 0.0
		for range int(monarch.Crown.FlightSeconds/dt) + 2 {
			Step(w, nil, dt)
			peak = math.Max(peak, p0.Crown.Z)
		}
		if p0.Crown.State != CrownGround || p0.Crown.Z != 0 || math.Abs(p0.Crown.X-c0.ToX) > 1e-9 || peak < monarch.Crown.HeightUnits*0.9 {
			t.Fatalf("Landung: %+v, Scheitel %v", p0.Crown, peak)
		}
	}
	a, pa := land(1)
	b, pb := land(1)
	steps(a, nil, 1)
	steps(b, nil, 1)
	if *pa.Crown != *pb.Crown {
		t.Fatalf("nicht deterministisch: %+v gegen %+v", pa.Crown, pb.Crown)
	}
}

// Ohne Krone kann der Monarch nur laufen und eine Krone aufheben: kein Schlag, kein Skill, kein Zahlen, keine Münzen.
func TestKroneOhneKroneNurLaufen(t *testing.T) {
	w, p0, _ := crownWorld(t)
	p0.Gold = 5
	fell(p0)
	p0.Crown.X = p0.X + 100
	x := p0.X
	e := spawnEnemy(w, "goblin", p0.X+1)
	cmd := PlayerCommand{MoveX: 1, Attack: true, Pay: true, Skill: 1}
	w.Coins = append(w.Coins, &Coin{ID: w.newID(), X: p0.X})
	hp := e.HP
	steps(w, []PlayerCommand{cmd, {}}, 1)
	if p0.X <= x || e.HP != hp || p0.Paying || p0.Gold != 5 {
		t.Fatalf("x %v → %v, Gegner-HP %v → %v, zahlt %v, Gold %d", x, p0.X, hp, e.HP, p0.Paying, p0.Gold)
	}
	if targetOf(w, e) == p0.ID {
		t.Fatal("ein Monarch ohne Krone ist Ziel eines Gegners")
	}
}

// Der Besitzer hebt seine Krone auf, wenn er hineinläuft.
func TestKroneBesitzerHebtAuf(t *testing.T) {
	w, p0, _ := crownWorld(t)
	p0.Gold = 0
	dropCrown(w, p0, "goblin")
	steps(w, nil, monarch.Crown.FlightSeconds+0.2)
	p0.X = p0.Crown.X
	steps(w, nil, 0.1)
	if p0.Crown != nil {
		t.Fatalf("Krone nicht aufgehoben: %+v", p0.Crown)
	}
}

// AC-02 (B-931): Ein Gegner hebt die Krone auf, trägt sie ins Portal: Kronenraub, Game Over in jedem Grad.
func TestKroneRaubIstGameOver(t *testing.T) {
	w, p0, _ := crownWorld(t)
	fell(p0)
	e := spawnEnemy(w, "goblin", p0.X+0.5)
	p0.X -= 20 // der Besitzer steht nicht an der Krone
	e.HomeX = w.Portals[0]
	var over bool
	for range 30 * 60 {
		Step(w, nil, dt)
		if p0.Crown != nil && p0.Crown.State == CrownEnemy && !e.Fleeing {
			t.Fatal("Träger flieht nicht")
		}
		for _, ev := range w.Events {
			over = over || ev["type"] == "gameOver"
		}
		if over {
			break
		}
	}
	if !over {
		t.Fatalf("kein Game Over nach Kronenraub; Krone %+v, Gegner bei %v", p0.Crown, e.X)
	}
}

func TestKroneRaubStuftMitInselOver(t *testing.T) {
	isl := mustIsland(t, "krone-raub", []int{0})
	AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	p := w.Players[0]
	fell(p)
	e := spawnEnemy(w, "goblin", p.X)
	e.Crown, e.Fleeing, e.HomeX = p.Crown, true, e.X
	p.Crown.State, p.Crown.Carrier = CrownEnemy, e.ID
	leaveThroughPortal(w)
	if !isl.Over {
		t.Fatal("Island.Over nach Kronenraub nicht gesetzt")
	}
}

// Ohne Portale in der Stufe gibt es keinen Raub: Gegner heben die Krone nicht auf.
func TestKroneKeinRaubOhnePortale(t *testing.T) {
	w, p0, _ := crownWorld(t)
	w.Portals = nil
	fell(p0)
	e := spawnEnemy(w, "goblin", p0.X)
	takeCrownOrLoot(w, e)
	if p0.Crown.State != CrownGround || e.Crown != nil || e.Fleeing {
		t.Fatalf("Raub ohne Portal: %+v", p0.Crown)
	}
}

// Stirbt der Träger, fällt die Krone an seiner Stelle zu Boden.
func TestKroneTraegerStirbt(t *testing.T) {
	w, p0, _ := crownWorld(t)
	fell(p0)
	e := spawnEnemy(w, "goblin", p0.X)
	e.HomeX = w.Portals[0]
	takeCrownOrLoot(w, e)
	if p0.Crown.State != CrownEnemy || !e.Fleeing {
		t.Fatalf("Gegner hebt nicht auf: %+v", p0.Crown)
	}
	e.X, e.HP = p0.X+7, 0
	Step(w, nil, dt)
	if p0.Crown.State != CrownGround || p0.Crown.X != p0.X+7 {
		t.Fatalf("Krone nach dem Tod des Trägers: %+v", p0.Crown)
	}
}

// AC-04 (B-931), 2 Spieler: Der Mitspieler hebt die fremde Krone auf, trägt sie zum Besitzer zurück und setzt sie ihm auf.
func TestKroneMitspielerBringtZurueck(t *testing.T) {
	w, p0, p1 := crownWorld(t)
	fell(p0)
	p0.Crown.X, p0.X = p0.X-10, p0.X-10
	p1.X = p0.Crown.X + 3
	p1.Crown = nil
	cmd := []PlayerCommand{{}, {MoveX: -1}}
	for range 30 * 5 {
		Step(w, cmd, dt)
		if p0.Crown == nil {
			break
		}
	}
	if p0.Crown != nil || p1.Crown != nil {
		t.Fatalf("Krone nicht zurückgebracht: P0 %+v, P1 %+v, P1 bei %v", p0.Crown, p1.Crown, p1.X)
	}
}

// Eine Krone je Monarch: zwei gefallene Kronen sind zwei Objekte, ein Träger nimmt nur eine mit.
func TestKroneEineJeMonarch(t *testing.T) {
	w, p0, p1 := crownWorld(t)
	p2 := AddPlayer(w)
	x := p0.X
	p0.X, p1.X, p2.X = x-30, x+30, x
	fell(p0)
	fell(p1)
	p0.Crown.X, p1.Crown.X = x, x
	stepCrowns(w, dt)
	if p0.Crown.State != CrownPlayer || p0.Crown.Carrier != p2.ID || p1.Crown.State != CrownGround {
		t.Fatalf("P2 trägt eine Krone, die zweite bleibt liegen: P0 %+v, P1 %+v", p0.Crown, p1.Crown)
	}
}

// Ein Dieb (stealsGold) raubt Loot-Münzen vom Boden und flieht damit; ohne Portale lässt er sie liegen.
func TestKroneDiebRaubtTreffermuenze(t *testing.T) {
	w, p0, _ := crownWorld(t)
	probe(p0)
	w.Coins = []*Coin{{ID: w.newID(), X: p0.X + 20, Loot: true}, {ID: w.newID(), X: p0.X + 20}}
	thieves := kindsWith(t, "stealsGold")
	e := spawnEnemy(w, thieves[0], p0.X+20)
	e.HomeX = w.Portals[0]
	takeCrownOrLoot(w, e)
	if e.CarriedGold != 1 || !e.Fleeing || len(w.Coins) != 1 || w.Coins[0].Loot {
		t.Fatalf("Dieb: trägt %d, flieht %v, Münzen %+v", e.CarriedGold, e.Fleeing, w.Coins)
	}
}

// AC-03 (B-931): Burgfall beendet das Spiel nicht mehr, in keinem Niederlage-Modus.
func TestBurgfallBeendetDasSpielNicht(t *testing.T) {
	for _, mode := range defeatNames {
		isl := mustIsland(t, "burgfall-"+mode, []int{0})
		AddIslandPlayer(isl, 0)
		if err := SetOptions(isl, IslandOptions{Grade: "ultra", Goal: "endboss", Defeat: mode}, false); err != nil {
			t.Fatal(err)
		}
		w := isl.Stages[0]
		w.Castle.HP = 0
		Step(w, nil, dt)
		if isl.Over || len(eventsOfType(w, "gameOver")) != 0 {
			t.Fatalf("Modus %s: Burgfall beendet das Spiel", mode)
		}
	}
	if err := validateOptions(IslandOptions{Grade: "normal", Goal: "endboss", Defeat: "lost"}); err == nil {
		t.Fatal("der Modus lost (Komplett verloren) darf nicht mehr gelten")
	}
}
