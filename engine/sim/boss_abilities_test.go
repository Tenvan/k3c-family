package sim

import (
	"fmt"
	"testing"
)

// Minibosse von Eisenstollen und Kristallhöhle (K2.1b, B-130/AC-01): Auslöser, Flammenspur, Splitter-Wurf,
// Belohnung, ein Miniboss je Stufe der Insel 1, Determinismus.

// deepIsland: Insel mit allen fünf Stufen der Insel 1 und `players` Spielern in der Stufe `stage`; ohne Truppen und
// Gegner, damit nur der Boss kämpft.
func deepIsland(t *testing.T, players, stage int) *Island {
	t.Helper()
	isl, err := CreateIsland("boss", []int{0, 1, 2, 3, 4}, 1)
	if err != nil {
		t.Fatal(err)
	}
	for range players {
		AddIslandPlayer(isl, stage)
	}
	for _, w := range isl.Stages {
		w.Troops, w.Enemies, w.SpawnQueue = []*Troop{}, []*Enemy{}, []QueuedSpawn{}
	}
	return isl
}

// (a) Lava-Golem und Splitter-Titan kommen mit Welle 3 ihrer Stufe über das erste Portal, nicht mit Welle 2.
func TestBossMinibossEisenstollenUndKristallhoehle(t *testing.T) {
	isl := deepIsland(t, 1, 0)
	for stage, id := range map[int]string{3: "lavaGolem", 4: "shardTitan"} {
		w := isl.Stages[stage]
		waveUntil(w, 2)
		if len(bossesIn(w)) != 0 {
			t.Fatalf("%s schon in Welle 2", id)
		}
		startWave(w)
		b := bossesIn(w)
		if len(b) != 1 || b[0].Kind != id || b[0].X != w.Portals[0] {
			t.Fatalf("Welle 3: Bosse %v, erwartet %s am ersten Portal %.1f", b, id, w.Portals[0])
		}
	}
}

// (b) Flammenspur: Die Fläche trifft beide Spieler darin je Sekunde, eine Truppe außerhalb nicht; ein stehender Boss
// stapelt keine Flächen; nach Ablauf ist die Fläche weg und schadet nicht mehr.
func TestBossFlammenspur(t *testing.T) {
	isl := deepIsland(t, 2, 3)
	w := isl.Stages[3]
	x := w.HubX + 20
	boss := spawnBoss(w, miniBoss(3), x)
	a := miniBoss(3).Ability
	boss.AoeIn = 0
	bossAbility(w, boss)
	boss.AoeIn = 0
	bossAbility(w, boss)
	if len(w.Hazards) != 1 || w.Hazards[0].X != x || w.Hazards[0].Radius != a.Radius {
		t.Fatalf("Flächen %v, erwartet eine bei %.1f", w.Hazards, x)
	}
	w.Enemies = []*Enemy{} // nur die Fläche wirkt
	ps := isl.Players()
	ps[0].X, ps[1].X = x-1, x+1
	far := warriorAt(w, x+a.Radius+0.5)
	run(w, 0.5)
	for _, p := range ps {
		if p.HP >= p.MaxHP {
			t.Fatalf("Spieler %d in der Fläche ohne Schaden", p.Index)
		}
	}
	if far.HP != 1000 {
		t.Fatal("Truppe außerhalb der Fläche getroffen")
	}
	hp := ps[0].HP
	run(w, 1)
	if ps[0].HP >= hp {
		t.Fatal("kein Schaden in der nächsten Sekunde")
	}
	run(w, a.DurationSeconds)
	if len(w.Hazards) != 0 {
		t.Fatalf("Fläche nach %.0f s nicht ausgelaufen", a.DurationSeconds)
	}
	hp0, hp1 := ps[0].HP, ps[1].HP
	run(w, 2)
	if ps[0].HP != hp0 || ps[1].HP != hp1 {
		t.Fatal("Schaden nach Ablauf der Fläche")
	}
}

// (b) Der gehende Golem hinterlässt eine Spur aus mehreren Flächen.
func TestBossFlammenspurBeimGehen(t *testing.T) {
	isl := deepIsland(t, 1, 0)
	w := isl.Stages[3]
	spawnBoss(w, miniBoss(3), w.Portals[0])
	run(w, 5)
	if len(w.Hazards) < 2 {
		t.Fatalf("%d Flächen nach 5 s Gehen, erwartet eine Spur", len(w.Hazards))
	}
}

// (c) Splitter-Wurf: Der Einschlag trifft das Ziel und eine nahe Truppe, eine ferne nicht; jeder Angriff des
// Splitter-Titans ist ein Wurf mit Splash.
func TestBossSplitterWurf(t *testing.T) {
	isl := deepIsland(t, 1, 0)
	w := isl.Stages[4]
	x := w.HubX + 30
	boss := spawnBoss(w, miniBoss(4), x)
	a := miniBoss(4).Ability
	target, near, far := warriorAt(w, x-6.5), warriorAt(w, x-6.5-1.5), warriorAt(w, x-6.5-a.Radius-1)
	bossAbility(w, boss)
	if len(w.Projectiles) != 1 || w.Projectiles[0].Splash != a.Radius || w.Projectiles[0].TargetID != target.ID {
		t.Fatalf("Geschosse %+v, erwartet ein Splitter auf %d", w.Projectiles, target.ID)
	}
	if boss.Cooldown != a.IntervalSeconds || boss.AoeIn != a.IntervalSeconds {
		t.Fatalf("Abklingzeit %.1f/%.1f, erwartet %.1f", boss.Cooldown, boss.AoeIn, a.IntervalSeconds)
	}
	for range 30 {
		stepProjectiles(w, dt)
	}
	if target.HP != 1000-boss.Damage || near.HP != 1000-boss.Damage {
		t.Fatalf("Ziel %.0f, nahe Truppe %.0f, erwartet je %.0f", target.HP, near.HP, 1000-boss.Damage)
	}
	if far.HP != 1000 {
		t.Fatal("ferne Truppe getroffen")
	}
	for range 200 {
		Step(w, nil, dt)
		for _, p := range w.Projectiles {
			if p.Team == "enemy" && p.Splash == 0 {
				t.Fatal("Splitter-Titan greift ohne Splash an")
			}
		}
	}
}

// (d) Belohnung: 100 Gold als Münzen und 50 Eisen bzw. Kristall im Vorrat der Insel.
func TestBossBelohnungEisenUndKristall(t *testing.T) {
	isl := deepIsland(t, 2, 0)
	for stage, stock := range map[int]*int{3: &isl.Stock.Iron, 4: &isl.Stock.Crystal} {
		w := isl.Stages[stage]
		boss := spawnBoss(w, miniBoss(stage), w.HubX+15)
		coins, before := len(w.Coins), *stock
		boss.HP = 0
		removeDeadEnemies(w)
		if got := len(w.Coins) - coins; got != 100 {
			t.Fatalf("Stufe %d: %d Münzen, erwartet 100", stage, got)
		}
		if got := *stock - before; got != 50 {
			t.Fatalf("Stufe %d: %d %s, erwartet 50", stage, got, w.Biome.PrimaryResource)
		}
	}
}

// (e) Jede Stufe der Insel 1 hat genau einen Miniboss.
func TestBossJedeStufeEinMiniboss(t *testing.T) {
	want := []string{"goblinLeader", "trollKing", "ratQueen", "lavaGolem", "shardTitan"} // Wald bis Kristallhöhle
	for depth, id := range want {
		n := 0
		for _, b := range bosses {
			if b.Kind == "mini" && b.Depth == depth {
				n++
			}
		}
		if n != 1 || miniBoss(depth).ID != id {
			t.Fatalf("Tiefe %d: %d Minibosse, erwartet genau %s", depth, n, id)
		}
	}
}

// (f) Gleicher Seed, gleicher Verlauf: zwei Spieler, beide Bosse kämpfen, Flächen und Splitter wirken.
func TestBossFlaechenDeterministisch(t *testing.T) {
	trace := func() string {
		isl := deepIsland(t, 1, 3)
		AddIslandPlayer(isl, 4)
		for _, s := range []int{3, 4} {
			w := isl.Stages[s]
			spawnBoss(w, miniBoss(s), w.HubX+15)
		}
		out, hazards, shards := "", 0, 0
		for range 1200 {
			StepIsland(isl, nil, dt)
			hazards += len(isl.Stages[3].Hazards)
			shards += len(isl.Stages[4].Projectiles)
		}
		if hazards == 0 || shards == 0 {
			t.Fatalf("Flächen %d, Splitter %d: Bosse wirken nicht", hazards, shards)
		}
		for _, s := range []int{3, 4} {
			w := isl.Stages[s]
			for _, e := range w.Enemies {
				out += fmt.Sprintf(" %s:%.3f:%.0f", e.Kind, e.X, e.HP)
			}
			for _, h := range w.Hazards {
				out += fmt.Sprintf(" h%.3f:%.3f", h.X, h.SecondsLeft)
			}
			for _, p := range w.Players {
				out += fmt.Sprintf(" p%d:%.3f:%.0f", p.Index, p.X, p.HP)
			}
		}
		return out
	}
	if a, b := trace(), trace(); a != b {
		t.Fatalf("Verlauf weicht ab:\n%s\n%s", a, b)
	}
}
