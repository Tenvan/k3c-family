package sim

import (
	"math"
	"testing"
)

// Vollmond-Belohnung je bespielter Stufe (KB1.1, B-379/AC-01) und Blutmond-Faktor auf Boss-Flächen (B-380/AC-01).

// fullMoonReward: Münzen, die der Tagesanbruch nach Vollmond-Nacht 7 in jeder Stufe der Insel streut.
func fullMoonReward(isl *Island) []int {
	got := make([]int, len(isl.Stages))
	for i, w := range isl.Stages {
		w.Cycle = CycleInfo{Phase: "day", Day: 8}
		before := len(w.Coins)
		nightEventsEnd(w)
		got[i] = len(w.Coins) - before
	}
	return got
}

func TestVollmondBelohnungInDerBespieltenStufe(t *testing.T) {
	reward := fullMoon().RewardGold
	isl := mustIsland(t, "mond", []int{0, 1})
	AddIslandPlayer(isl, 1)
	AddIslandPlayer(isl, 1)
	if got := fullMoonReward(isl); got[0] != 0 || got[1] != reward {
		t.Fatalf("beide in Stufe 1: Münzen je Stufe %v, erwartet [0 %d]", got, reward)
	}
	isl.Stages[1].castleFellNight = 7 // Burg der Stufe 1 fiel, Stufe 0 hielt
	if got := fullMoonReward(isl); got[0] != 0 || got[1] != 0 {
		t.Fatalf("Burg der Stufe 1 gefallen: keine Belohnung erwartet, %v", got)
	}
}

func TestVollmondBelohnungJeStufeMitSpielern(t *testing.T) {
	reward := fullMoon().RewardGold
	isl := mustIsland(t, "mond", []int{0, 1, 2})
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 2)
	if got := fullMoonReward(isl); got[0] != reward || got[1] != 0 || got[2] != reward {
		t.Fatalf("Spieler in Stufe 0 und 2: Münzen je Stufe %v, erwartet [%d 0 %d]", got, reward, reward)
	}
	isl.Stages[2].Players[0].Free = true // getrennter Monarch spielt nicht
	if got := fullMoonReward(isl); got[2] != 0 {
		t.Fatalf("nur ein getrennter Monarch in Stufe 2: keine Belohnung erwartet, %d", got[2])
	}
}

// bloodMoonDeep: Insel mit allen fünf Stufen, ein Spieler oben, alle Stufen in Nacht n.
func bloodMoonDeep(t *testing.T, n int) *Island {
	t.Helper()
	isl := deepIsland(t, 1, 0)
	setCycle(isl, n, 0.5)
	return isl
}

func TestBlutmondVerstaerktBossFlaechenschlag(t *testing.T) {
	for _, c := range []struct {
		night  int
		factor float64
	}{{12, 1}, {13, bloodMoon().DamageFactor}} {
		w := bloodMoonDeep(t, c.night).Stages[1]
		x := w.HubX + 20
		boss := spawnBoss(w, miniBoss(1), x)
		tr := &Troop{ID: w.newID(), Kind: "warrior", X: x, HP: 1000, MaxHP: 1000}
		w.Troops = append(w.Troops, tr)
		boss.AoeIn = 0
		bossAbility(w, boss)
		if want := 1000 - boss.Damage*c.factor; math.Abs(tr.HP-want) > 1e-9 {
			t.Errorf("Nacht %d: Flächenschlag HP %.2f, erwartet %.2f", c.night, tr.HP, want)
		}
	}
}

func TestBlutmondVerstaerktSplitter(t *testing.T) {
	for _, c := range []struct {
		night  int
		factor float64
	}{{12, 1}, {13, bloodMoon().DamageFactor}} {
		w := bloodMoonDeep(t, c.night).Stages[4]
		x := w.HubX + 30
		boss := spawnBoss(w, miniBoss(4), x)
		target, near := warriorAt(w, x-6.5), warriorAt(w, x-8)
		bossAbility(w, boss)
		for range 30 {
			stepProjectiles(w, dt)
		}
		want := 1000 - boss.Damage*c.factor
		if math.Abs(target.HP-want) > 1e-9 || math.Abs(near.HP-want) > 1e-9 {
			t.Errorf("Nacht %d: Ziel %.2f, Splash %.2f, erwartet je %.2f", c.night, target.HP, near.HP, want)
		}
	}
}

func TestBlutmondLaesstBodenflaecheOhneFaktor(t *testing.T) {
	w := bloodMoonDeep(t, 13).Stages[3]
	if moonDamage(w, 10) == 10 {
		t.Fatal("Nacht 13 muss Blutmond sein")
	}
	h := &Hazard{X: w.HubX + 30, Radius: 2, DamagePerSecond: 7, SecondsLeft: 5, Cause: "test"}
	w.Hazards = append(w.Hazards, h)
	tr := warriorAt(w, h.X)
	stepHazards(w, dt)
	if tr.HP != 1000-7 {
		t.Fatalf("Bodenfläche im Blutmond: HP %.2f, erwartet %d", tr.HP, 1000-7)
	}
}
