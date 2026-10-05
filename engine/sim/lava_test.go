package sim

import (
	"math"
	"strconv"
	"testing"
)

// W2.2 (B-115, Beschluss Q28): Lava im Eisenstollen schadet Spielern, Truppen und Bauern, nicht Gegnern.

// lavaWorld ist ein Eisenstollen ohne Truppen und Camps mit der Mitte des ersten Lava-Streifens.
func lavaWorld(t *testing.T) (*World, float64) {
	t.Helper()
	for seed := range 50 {
		w := newWorld(3, "lava-"+strconv.Itoa(seed), Options{})
		w.Troops, w.Camps, w.CycleSpeed = []*Troop{}, []*Camp{}, 1e-6
		for _, c := range w.Level.Chunks {
			if c.Kind == "lava" {
				return w, c.StartUnits + w.Level.ChunkWidthUnits/2
			}
		}
	}
	t.Fatal("kein Lava-Chunk in 50 Seeds")
	return nil, 0
}

func TestLavaSchadetFiguren(t *testing.T) {
	w, x := lavaWorld(t)
	dps := w.Biome.Lava.DamagePerSecond
	p0, p1 := AddPlayer(w), AddPlayer(w)
	p0.X, p1.X = x, x+w.Biome.Lava.WidthUnits // p1 steht neben dem Streifen
	peasant := spawnVagrant(w, x, x)
	peasant.Kind, peasant.HP, peasant.MaxHP, peasant.TargetX = "peasant", 1000, 1000, x
	archer := spawnVagrant(w, x, x)
	makeArcher(w, archer)
	archer.HP, archer.AnchorX = 1000, x
	hp0, hp1 := p0.HP, p1.HP
	for range 60 { // 2 s: zwei Lava-Treffer
		p0.X, p1.X, peasant.X, archer.X = x, x+w.Biome.Lava.WidthUnits, x, x
		Step(w, []PlayerCommand{{}, {}}, dt)
	}
	want := 2 * dps
	// Am Spieler wirkt die Verteidigung wie bei jedem Treffer (damagePlayer), mindestens 1 je Treffer.
	if mine := 2 * math.Max(1, dps-defenseOf(w, p0)); math.Abs((hp0-p0.HP)-mine) > 0.01 || p1.HP != hp1 {
		t.Errorf("Spieler auf Lava -%v (erwartet %v), daneben -%v", hp0-p0.HP, mine, hp1-p1.HP)
	}
	if math.Abs((1000-peasant.HP)-want) > 0.01 || math.Abs((1000-archer.HP)-want) > 0.01 {
		t.Errorf("Bauer -%v, Schütze -%v, erwartet je %v", 1000-peasant.HP, 1000-archer.HP, want)
	}
}

// Gegner nehmen keinen Lava-Schaden (Startwert, BR1 bestätigt).
func TestLavaNichtFuerGegner(t *testing.T) {
	w, x := lavaWorld(t)
	enemy := spawnEnemy(w, "goblin", x)
	enemy.Stun = math.Inf(1)
	hp := enemy.HP
	run(w, 1)
	if enemy.HP != hp {
		t.Errorf("Gegner auf Lava -%v", hp-enemy.HP)
	}
}

// Lava liegt nie im Hub, am Eingang oder am Rand (500 Seeds).
func TestLavaNichtImHubOderAmRand(t *testing.T) {
	b := biomeForDepth(3)
	for seed := range 500 {
		w := newWorld(3, strconv.Itoa(seed), Options{})
		for _, c := range w.Level.Chunks {
			if c.Kind != "lava" {
				continue
			}
			mid := c.StartUnits + w.Level.ChunkWidthUnits/2
			if math.Abs(mid-w.HubX) < b.HubWidthUnits/2 || c.Index == 0 || c.Index == len(w.Level.Chunks)-1 {
				t.Fatalf("Seed %d: Lava-Chunk %d bei %v (Hub %v)", seed, c.Index, mid, w.HubX)
			}
		}
	}
}
