package sim

import (
	"math"
	"slices"
	"testing"

	"k3c/engine/rng"
)

// Gegner und Pools der tiefen Stufen Eisenstollen (Tiefe 3) und Kristallhöhle (Tiefe 4) (K1.3, gegner.md §§ 1–3).

var deepPools = map[int][]string{
	3: {"lavaSlime", "ironBeetle", "fireSpirit"},
	4: {"crystalSpider", "shardling", "crystalGuardian"},
}

// (a) Beide Biome laden gültige Pools (2 Standard, 1 Elite, keine Nacht-Gegner); ohne Elite oder mit unbekanntem
// Gegner scheitert das Laden.
func TestPoolTiefeStufenLaden(t *testing.T) {
	for depth, want := range deepPools {
		b := biomeForDepth(depth)
		tiers := map[string]int{}
		for _, k := range b.Enemies.Portal {
			tiers[enemyData[k].Tier]++
		}
		if err := checkPool(b, enemyData); err != nil || !slices.Equal(b.Enemies.Portal, want) ||
			len(b.Enemies.Night) != 0 || tiers["standard"] != 2 || tiers["elite"] != 1 {
			t.Fatalf("Tiefe %d: %v, Pool %v/%v, Stufen %v", depth, err, b.Enemies.Portal, b.Enemies.Night, tiers)
		}
		noElite, unknown := b, b
		noElite.Enemies.Portal = want[:2]
		unknown.Enemies.Night = []string{"dragon"}
		if checkPool(noElite, enemyData) == nil || checkPool(unknown, enemyData) == nil {
			t.Fatalf("Tiefe %d: Pool ohne Elite oder mit unbekanntem Gegner geladen", depth)
		}
	}
}

// (b) Welle 6 zieht nur Gegner aus dem Pool, darunter mindestens einen Elite.
func TestPoolWellePlantAusDemPool(t *testing.T) {
	for depth, pool := range deepPools {
		plan := planWave(biomeForDepth(depth), 6, rng.New("k1-pool"), []float64{10, 50, 90}, false, 1)
		elites := 0
		for _, s := range plan {
			if !slices.Contains(pool, s.kind) {
				t.Fatalf("Tiefe %d: %s nicht im Pool", depth, s.kind)
			}
			if enemyData[s.kind].Tier == "elite" {
				elites++
			}
		}
		if elites < 1 || len(plan) < 11 {
			t.Fatalf("Tiefe %d: %d Gegner, %d Elite", depth, len(plan), elites)
		}
	}
}

// (c) Skalierung nach der Tabelle für Insel 1 (`depthScaling`, Grad-Faktor 1): HP, Schaden und Tempo je Tiefe.
func TestPoolSkalierungTiefe(t *testing.T) {
	s := waves.DepthScaling
	for depth, pool := range deepPools {
		w := newWorld(depth, "k1-skalierung", Options{})
		f := float64(depth)
		for _, kind := range pool {
			d, e := enemyData[kind], spawnEnemy(w, kind, w.HubX-40)
			hp, dmg := math.Round(d.HP*math.Pow(s.HP, f)), math.Round(d.Damage*math.Pow(s.Damage, f))
			if e.HP != hp || e.MaxHP != hp || e.Damage != dmg || math.Abs(e.Speed-d.Speed*math.Pow(s.Speed, f)) > 1e-9 {
				t.Fatalf("%s in Tiefe %d: HP %v (%v), Schaden %v (%v), Tempo %v", kind, depth, e.HP, hp, e.Damage, dmg, e.Speed)
			}
		}
	}
	// Stichprobe mit festen Zahlen: Lavaschleim in Tiefe 3 = 120 × 1,5³ HP, 25 × 1,3³ Schaden.
	if e := spawnEnemy(newWorld(3, "k1-skalierung", Options{}), "lavaSlime", 0); e.HP != 405 || e.Damage != 55 {
		t.Fatalf("Lavaschleim Tiefe 3: HP %v, Schaden %v", e.HP, e.Damage)
	}
}

// (d) Ab Tiefe 3 drei Portale.
func TestPoolDreiPortale(t *testing.T) {
	for depth := range deepPools {
		if b, w := biomeForDepth(depth), newWorld(depth, "k1-portale", Options{}); b.Portals.Count != 3 || len(w.Portals) != 3 {
			t.Fatalf("Tiefe %d: %d Portale im Biom, %d in der Welt", depth, b.Portals.Count, len(w.Portals))
		}
	}
}

// (e) Splitterwicht: ein Schwarm-Platz erzeugt 4 Gegner am selben Portal.
func TestGegnerSplitterwichtSchwarm(t *testing.T) {
	b := biomeForDepth(4)
	b.Enemies.Portal = []string{"shardling"}
	plan := planWave(b, 1, rng.New("splitter"), []float64{10, 50, 90}, false, 1)
	if enemyData["shardling"].SwarmSize != 4 || len(plan) < 20 || len(plan)%4 != 0 {
		t.Fatalf("%d Splitterwichte", len(plan))
	}
	for i, s := range plan {
		if want := []float64{10, 50, 90}[(i/4)%3]; s.kind != "shardling" || s.x != want {
			t.Fatalf("Gegner %d: %s am Portal %v, erwartet %v", i, s.kind, s.x, want)
		}
	}
}

// (e) Kristallwächter: Flächenschlag trifft beide Spieler im Radius 3, nicht den außerhalb stehenden Krieger.
func TestGegnerKristallwaechterFlaechenschlag(t *testing.T) {
	w := quietWorld(t)
	p1, p2 := AddPlayer(w), AddPlayer(w)
	x := w.HubX - 30
	spawnEnemy(w, "crystalGuardian", x)
	p1.X, p2.X = x+2, x+4.5
	out := warriorAt(w, x+6)
	stepEnemies(w, dt)
	if a := enemyData["crystalGuardian"].Aoe; a.Radius != 3 || a.IntervalSeconds != 4 ||
		p1.HP >= p1.MaxHP || p2.HP >= p2.MaxHP || out.HP != out.MaxHP {
		t.Fatalf("Spieler %v/%v, %v/%v, Krieger %v", p1.HP, p1.MaxHP, p2.HP, p2.MaxHP, out.HP)
	}
}

// (e) Feuergeist: hält 8 Units Abstand zum Krieger und schießt.
func TestGegnerFeuergeistKiting(t *testing.T) {
	w := quietWorld(t)
	x := w.HubX - 30
	e := spawnEnemy(w, "fireSpirit", x-10)
	e.X = x
	warrior := warriorAt(w, x+5)
	stepFor(w, 3)
	if dist := warrior.X - e.X; dist < 8 || dist > 8.2 || len(w.Projectiles) < 2 {
		t.Fatalf("Abstand %v, %d Geschosse", dist, len(w.Projectiles))
	}
}
