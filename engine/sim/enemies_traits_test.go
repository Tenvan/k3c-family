package sim

import (
	"encoding/json"
	"io/fs"
	"math"
	"slices"
	"testing"
	"testing/fstest"

	"k3c/data"
	"k3c/engine/rng"
)

// Traits und Angriffsrate der Gegner (K1.1, gegner.md § 2). Die Tests rufen stepEnemies direkt, damit Truppen und
// Spieler stehen bleiben, wo der Test sie hinstellt.

func warriorAt(w *World, x float64) *Troop {
	t := &Troop{ID: w.newID(), Kind: "warrior", X: x, HP: 1000, MaxHP: 1000}
	w.Troops = append(w.Troops, t)
	return t
}

// hitsOn zählt die hit-Ereignisse auf die ID.
func hitsOn(w *World, id int) int {
	n := 0
	for _, ev := range w.Events {
		if ev["type"] == "hit" && ev["id"] == id {
			n++
		}
	}
	return n
}

func stepFor(w *World, seconds float64) {
	for range int(math.Round(seconds / dt)) {
		stepEnemies(w, dt)
		w.Time += dt
	}
}

// (a) Flächenschlag: zwei Truppen im Radius 3 um das Ziel nehmen Schaden, eine außerhalb nicht; der zweite Schlag
// kommt erst nach 4 s, dazwischen greift der Troll normal an.
func TestGegnerFlaechenschlag(t *testing.T) {
	w := quietWorld(t)
	x := w.HubX - 30
	e := spawnEnemy(w, "caveTroll", x)
	target, near, out := warriorAt(w, x+2), warriorAt(w, x+4.5), warriorAt(w, x+6)
	var aoeAt []int
	for i := range int(4.5 / dt) {
		before := near.HP
		stepEnemies(w, dt)
		if near.HP < before {
			aoeAt = append(aoeAt, i)
		}
	}
	gap := float64(aoeAt[len(aoeAt)-1]-aoeAt[0]) * dt
	// Der Schlag fällt auf den nächsten Angriffs-Takt ab 4 s (Abklingzeit in ganzen Ticks).
	if len(aoeAt) != 2 || gap < 4-dt || gap > 4.25 || out.HP != out.MaxHP || hitsOn(w, target.ID) != 5 {
		t.Fatalf("Flächenschläge %v (Abstand %.2f s), außen %v, Ziel %d Treffer, Troll bei %v", aoeAt, gap, out.HP,
			hitsOn(w, target.ID), e.X)
	}
}

// (g) Zwei Spieler im Radius: beide nehmen Schaden.
func TestGegnerFlaechenschlagZweiSpieler(t *testing.T) {
	w := quietWorld(t)
	p1, p2 := AddPlayer(w), AddPlayer(w)
	x := w.HubX - 30
	spawnEnemy(w, "caveTroll", x)
	p1.X, p2.X = x+1.5, x+3.5
	stepEnemies(w, dt)
	if p1.HP >= p1.MaxHP || p2.HP >= p2.MaxHP {
		t.Fatalf("Spieler 1 %v/%v, Spieler 2 %v/%v", p1.HP, p1.MaxHP, p2.HP, p2.MaxHP)
	}
}

// (b) Ein Schwarm-Platz erzeugt swarmSize Gegner der Art am selben Portal.
func TestGegnerSchwarmPlatz(t *testing.T) {
	b := biomeForDepth(2)
	b.Enemies.Portal, b.Enemies.Night = []string{"ratSwarm"}, nil
	size := enemyData["ratSwarm"].SwarmSize
	plan := planWave(b, 1, rng.New("schwarm"), []float64{10, 90}, false, 1)
	if size != 4 || len(plan) < 5*size || len(plan)%size != 0 {
		t.Fatalf("Schwarmgröße %d, %d Gegner", size, len(plan))
	}
	for i, s := range plan {
		slot := i / size
		if want := []float64{10, 90}[slot%2]; s.kind != "ratSwarm" || s.x != want {
			t.Fatalf("Gegner %d: %s am Portal %v, erwartet %v", i, s.kind, s.x, want)
		}
	}
}

// (c) Phasen: alle 8 s ab Spawn für 2 s unverwundbar, Treffer ändern dann die HP nicht und melden kein hit.
func TestGegnerPhasen(t *testing.T) {
	w := quietWorld(t)
	w.Time = 5
	e := spawnEnemy(w, "mineGhost", w.HubX-30)
	for _, c := range []struct {
		age  float64
		hurt bool
	}{{1, true}, {7.9, true}, {8.1, false}, {9.9, false}, {10.1, true}, {15.9, true}, {16.5, false}, {18.1, true}} {
		w.Time, w.Events = 5+c.age, nil
		hp := e.HP
		applyDamage(w, e.ID, 1)
		if hurt := e.HP < hp; hurt != c.hurt || (hitsOn(w, e.ID) == 1) != c.hurt {
			t.Fatalf("nach %.1f s: Schaden %v, erwartet %v", c.age, hurt, c.hurt)
		}
	}
}

// (d) Kiting: Der Goblin Archer weicht einem Krieger unter 8 Units aus und schießt weiter.
func TestGegnerKiting(t *testing.T) {
	w := quietWorld(t)
	x := w.HubX - 30
	e := spawnEnemy(w, "goblinArcher", x-10)
	e.X = x
	warrior := warriorAt(w, x+5)
	stepFor(w, 3)
	dist := warrior.X - e.X
	if dist < enemyData["goblinArcher"].KiteDistance || dist > 8.2 || len(w.Projectiles) < 3 || e.X < e.HomeX {
		t.Fatalf("Abstand %v, %d Geschosse, Archer bei %v (Portal %v)", dist, len(w.Projectiles), e.X, e.HomeX)
	}
}

// (d) Kiting am Rand: Am Portal bleibt der Archer stehen und schießt.
func TestGegnerKitingAmPortal(t *testing.T) {
	w := quietWorld(t)
	x := w.HubX - 30
	e := spawnEnemy(w, "goblinArcher", x)
	warriorAt(w, x+3)
	stepFor(w, 2)
	if e.X != x || len(w.Projectiles) < 2 {
		t.Fatalf("Archer bei %v (Portal %v), %d Geschosse", e.X, x, len(w.Projectiles))
	}
}

// (d) Kiting an der Mauer: Der Archer weicht nicht durch die Mauer aus, er bleibt davor stehen und schießt.
func TestGegnerKitingAnDerMauer(t *testing.T) {
	w := quietWorld(t)
	wall := leftWall(t, w)
	e := spawnEnemy(w, "goblinArcher", wall.X-20)
	e.X = wall.X + 1
	warriorAt(w, wall.X+4)
	stepFor(w, 2)
	if math.Abs(e.X-(wall.X+body)) > 1e-9 || len(w.Projectiles) < 2 {
		t.Fatalf("Archer bei %v, Mauer bei %v, %d Geschosse", e.X, wall.X, len(w.Projectiles))
	}
}

// withEnemy legt für einen Test eine Gegnerart an.
func withEnemy(t *testing.T, kind string, d EnemyData) {
	t.Helper()
	enemyData[kind] = d
	t.Cleanup(func() { delete(enemyData, kind) })
}

// (e) Angriffsrate je Gegner: 2/s greift doppelt so oft an wie 1/s, ohne Eintrag 1/s.
func TestGegnerAngriffsrate(t *testing.T) {
	w := quietWorld(t)
	base := enemyData["skeleton"]
	fast, none := base, base
	fast.AttacksPerSecond, none.AttacksPerSecond = 2, 0
	withEnemy(t, "testFast", fast)
	withEnemy(t, "testNone", none)
	counts, cooldowns, ids := map[string]int{}, map[string]float64{}, map[string]int{}
	enemies := map[string]*Enemy{}
	for i, kind := range []string{"skeleton", "testFast", "testNone"} {
		x := w.HubX - 60 + float64(i)*15
		enemies[kind] = spawnEnemy(w, kind, x)
		ids[kind] = warriorAt(w, x+1).ID
	}
	stepEnemies(w, dt)
	for kind, e := range enemies {
		cooldowns[kind] = e.Cooldown
	}
	stepFor(w, 10)
	for kind, id := range ids {
		counts[kind] = hitsOn(w, id)
	}
	nSlow, nFast := counts["skeleton"], counts["testFast"]
	if base.AttacksPerSecond != 1 || cooldowns["skeleton"] != 1 || cooldowns["testFast"] != 0.5 ||
		cooldowns["testNone"] != 1 || nFast < 2*nSlow-2 || nFast > 2*nSlow || counts["testNone"] != nSlow {
		t.Fatalf("Abklingzeit %v, Angriffe in 10 s: %v", cooldowns, counts)
	}
	for kind, d := range enemyData {
		if !slices.Contains([]string{"testFast", "testNone"}, kind) && d.AttacksPerSecond <= 0 {
			t.Fatalf("%s ohne attacksPerSecond in data/enemies.json", kind)
		}
	}
}

// (f) Daten: Alle Traits sind bekannt, pack und stealsWood sind gestrichen, ein unbekanntes Trait scheitert beim Laden.
func TestGegnerTraitsInDaten(t *testing.T) {
	for kind, d := range enemyData {
		for _, tr := range d.Traits {
			if !slices.Contains(knownTraits, tr) || tr == "pack" || tr == "stealsWood" {
				t.Fatalf("%s: Trait %q", kind, tr)
			}
		}
	}
	fsys := fstest.MapFS{}
	for _, name := range []string{"buildings.json", "troops.json", "economy.json", "hub.json", "monarch.json", "waves.json"} {
		raw, err := fs.ReadFile(data.Files, name)
		if err != nil {
			t.Fatal(err)
		}
		fsys[name] = &fstest.MapFile{Data: raw}
	}
	fsys["enemies.json"] = &fstest.MapFile{Data: []byte(`{"wolf": {"tier": "standard", "traits": ["pack"]}}`)}
	before := len(enemyData)
	if err := UseData(fsys); err == nil || len(enemyData) != before {
		t.Fatalf("unbekanntes Trait geladen: %v, %d Gegnerarten", err, len(enemyData))
	}
}

// (h) Determinismus: derselbe Seed zweimal ergibt dieselben Gegner, mit Schwarm, Flächenschlag, Phasen und Kiting.
func TestGegnerTraitsDeterministisch(t *testing.T) {
	scenario := func() []byte {
		w := newWorld(2, "k1-traits", Options{})
		AddPlayer(w)
		AddPlayer(w)
		startWave(w)
		for _, kind := range []string{"caveTroll", "mineGhost", "goblinArcher"} {
			spawnEnemy(w, kind, w.HubX-12)
		}
		run(w, 40)
		raw, err := json.Marshal(struct {
			E []*Enemy
			P []*Player
		}{w.Enemies, w.Players})
		if err != nil {
			t.Fatal(err)
		}
		return raw
	}
	if a, b := scenario(), scenario(); string(a) != string(b) {
		t.Fatal("zwei Läufe mit demselben Seed weichen ab")
	}
}
