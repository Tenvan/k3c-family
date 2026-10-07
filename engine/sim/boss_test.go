package sim

import (
	"fmt"
	"math"
	"slices"
	"strings"
	"testing"
	"testing/fstest"

	"k3c/data"
)

// Bosse (K2.1a, B-130): Minibosse von Wald, Höhle und Mine, Auslöser, Fähigkeit, Belohnung, Skalierung, Merker.

// bossIsland: Insel mit Wald, Höhle und Mine und `players` Spielern im Wald; ohne Truppen und Gegner, damit nur
// Boss und Beschwörungen kämpfen.
func bossIsland(t *testing.T, players int) *Island {
	t.Helper()
	isl, err := CreateIsland("boss", []int{0, 1, 2}, 1)
	if err != nil {
		t.Fatal(err)
	}
	for range players {
		AddIslandPlayer(isl, 0)
	}
	for _, w := range isl.Stages {
		w.Troops, w.Enemies, w.SpawnQueue = []*Troop{}, []*Enemy{}, []QueuedSpawn{}
	}
	return isl
}

func bossesIn(w *World) []*Enemy {
	return slices.DeleteFunc(slices.Clone(w.Enemies), func(e *Enemy) bool { return !e.Boss })
}

func countKind(w *World, kind string) int {
	n := 0
	for _, e := range w.Enemies {
		if e.Kind == kind {
			n++
		}
	}
	return n
}

func countEvents(w *World, typ string) int {
	n := 0
	for _, ev := range w.Events {
		if ev["type"] == typ {
			n++
		}
	}
	return n
}

// waveUntil startet Wellen, bis die Stufe bei Welle n ist.
func waveUntil(w *World, n int) {
	for w.Wave < n {
		startWave(w)
	}
}

// (a) Wald ab Welle 5, Höhle und Mine ab Welle 3, über das erste Portal; kein zweiter, solange einer lebt.
func TestBossMinibossKommtMitSeinerWelle(t *testing.T) {
	isl := bossIsland(t, 1)
	for i, c := range []struct {
		wave int
		id   string
	}{{5, "goblinLeader"}, {3, "trollKing"}, {3, "ratQueen"}} {
		w := isl.Stages[i]
		waveUntil(w, c.wave-1)
		if len(bossesIn(w)) != 0 {
			t.Fatalf("%s schon vor Welle %d", c.id, c.wave)
		}
		startWave(w)
		b := bossesIn(w)
		if len(b) != 1 || b[0].Kind != c.id || b[0].X != w.Portals[0] {
			t.Fatalf("Welle %d: Bosse %v, erwartet %s am ersten Portal %.1f", c.wave, b, c.id, w.Portals[0])
		}
		startWave(w)
		if len(bossesIn(w)) != 1 {
			t.Fatalf("%s: zweiter Boss, obwohl einer lebt", c.id)
		}
	}
}

// (b) Goblin-Anführer ruft 2 Goblins je 10 s an seinen Ort.
func TestBossGoblinAnfuehrerRuftGoblins(t *testing.T) {
	isl := bossIsland(t, 1)
	w := isl.Stages[0]
	boss := spawnBoss(w, miniBoss(0), w.Portals[0])
	run(w, 9.9)
	if n := countKind(w, "goblin"); n != 0 {
		t.Fatalf("vor 10 s %d Goblins", n)
	}
	run(w, 0.2)
	if n := countKind(w, "goblin"); n != 2 {
		t.Fatalf("nach 10 s %d Goblins, erwartet 2", n)
	}
	run(w, 10)
	if n := countKind(w, "goblin"); n != 4 {
		t.Fatalf("nach 20 s %d Goblins, erwartet 4", n)
	}
	for _, e := range w.Enemies {
		if e.Kind == "goblin" && e.HomeX != boss.HomeX {
			t.Fatalf("Goblin flieht nach %.1f statt zum Portal des Bosses %.1f", e.HomeX, boss.HomeX)
		}
	}
}

// (b) Ratten-Königin ruft Rattenschwärme: 2 Schwärme je swarmSize Ratten.
func TestBossRattenKoeniginRuftSchwaerme(t *testing.T) {
	isl := bossIsland(t, 1)
	w := isl.Stages[2]
	boss := spawnBoss(w, miniBoss(2), w.Portals[0])
	boss.AoeIn = 0
	bossAbility(w, boss)
	if n, want := countKind(w, "ratSwarm"), 2*enemyData["ratSwarm"].SwarmSize; n != want {
		t.Fatalf("%d Ratten, erwartet %d", n, want)
	}
	if boss.AoeIn != 10 {
		t.Fatalf("nächste Beschwörung in %.1f s, erwartet 10", boss.AoeIn)
	}
}

// (b) Troll-König trifft alle Truppen im Radius 3 um sich, nicht weiter weg.
func TestBossTrollKoenigFlaechenschlag(t *testing.T) {
	isl := bossIsland(t, 1)
	w := isl.Stages[1]
	x := w.HubX + 20
	boss := spawnBoss(w, miniBoss(1), x)
	var near []*Troop
	for _, dx := range []float64{-2.9, 0, 3} {
		near = append(near, &Troop{ID: w.newID(), Kind: "warrior", X: x + dx, HP: 1000, MaxHP: 1000})
	}
	far := &Troop{ID: w.newID(), Kind: "warrior", X: x + 3.5, HP: 1000, MaxHP: 1000}
	w.Troops = append(append(w.Troops, near...), far)
	boss.AoeIn = 0
	bossAbility(w, boss)
	for _, tr := range near {
		if tr.HP != 1000-boss.Damage {
			t.Fatalf("Truppe bei %+.1f: HP %.0f, erwartet %.0f", tr.X-x, tr.HP, 1000-boss.Damage)
		}
	}
	if far.HP != 1000 {
		t.Fatal("Truppe außerhalb des Radius getroffen")
	}
	if boss.AoeIn != 4 {
		t.Fatalf("nächster Schlag in %.1f s, erwartet 4", boss.AoeIn)
	}
}

// (c) Belohnung: 100 Gold als Münzen, 50 Material in den Insel-Vorrat, 1 Skill-Punkt für beide Spieler.
func TestBossBelohnungFuerAlle(t *testing.T) {
	isl := bossIsland(t, 2)
	w := isl.Stages[0]
	boss := spawnBoss(w, miniBoss(0), w.HubX+15)
	coins, wood, pool := len(w.Coins), isl.Stock.Wood, isl.SkillPool
	boss.HP = 0
	removeDeadEnemies(w)
	if got := len(w.Coins) - coins; got != 100 {
		t.Fatalf("%d Münzen, erwartet 100", got)
	}
	if got := isl.Stock.Wood - wood; got != 50 {
		t.Fatalf("%d Holz, erwartet 50", got)
	}
	if isl.SkillPool != pool+1 {
		t.Fatalf("Skill-Pool %d, erwartet %d", isl.SkillPool, pool+1)
	}
	for _, p := range isl.Players() {
		if AvailablePoints(w, p) != pool+1 {
			t.Fatalf("Spieler %d hat %d freie Punkte, erwartet %d", p.Index, AvailablePoints(w, p), pool+1)
		}
	}
	for _, s := range isl.Stages {
		if s.SkillPoints != isl.SkillPool {
			t.Fatal("Stufe spiegelt den Pool nicht")
		}
	}
}

// (c) Über dem Lager-Maximum verfällt der Rest des Materials.
func TestBossMaterialBisZumLagerMaximum(t *testing.T) {
	isl := bossIsland(t, 1)
	w := isl.Stages[0]
	limit, ok := capacity(w)
	if !ok {
		t.Fatal("Insel ohne Lager-Maximum")
	}
	isl.Stock.Wood = limit - 10
	boss := spawnBoss(w, miniBoss(0), w.HubX+15)
	boss.HP = 0
	removeDeadEnemies(w)
	if isl.Stock.Wood != limit {
		t.Fatalf("Holz %d, erwartet %d", isl.Stock.Wood, limit)
	}
}

// (d) HP × (1 + 0,5 je Zusatzspieler der Insel) × enemyHp des Grads, nicht × waveSize; Tiefenfaktor greift.
func TestBossSkalierung(t *testing.T) {
	hpOf := func(players, depth int, grade string) float64 {
		isl := bossIsland(t, players)
		isl.Options.Grade = grade
		w := isl.Stages[depth]
		return spawnBoss(w, miniBoss(depth), w.Portals[0]).MaxHP
	}
	one, two := hpOf(1, 0, "normal"), hpOf(2, 0, "normal")
	if two != 1.5*one {
		t.Fatalf("2 Spieler: HP %.0f, erwartet 1,5 × %.0f", two, one)
	}
	if want := enemyData["goblin"].HP * 8; one != want {
		t.Fatalf("Wald: HP %.0f, erwartet %.0f", one, want)
	}
	g := difficulty["hard"]
	if g.WaveSize == g.EnemyHP {
		t.Fatal("Testannahme: waveSize und enemyHp von hard verschieden")
	}
	if hard := hpOf(1, 0, "hard"); hard != math.Round(one*g.EnemyHP) {
		t.Fatalf("hard: HP %.0f, erwartet %.0f (nur enemyHp)", hard, math.Round(one*g.EnemyHP))
	}
	want := math.Round(enemyData["skeleton"].HP * waves.DepthScaling.HP * 8)
	if cave := hpOf(1, 1, "normal"); cave != want {
		t.Fatalf("Höhle: HP %.0f, erwartet %.0f (Tiefenfaktor)", cave, want)
	}
}

// (e) Ein Boss flieht weder bei Tagesanbruch noch bei halber HP; seine Beschwörungen schon.
func TestBossFliehtNie(t *testing.T) {
	isl := bossIsland(t, 1)
	w := isl.Stages[0]
	boss := spawnBoss(w, miniBoss(0), w.HubX+30)
	spawnEnemy(w, "goblin", w.HubX+30)
	boss.HP = boss.MaxHP * 0.4
	sendEnemiesHome(w)
	run(w, 0.5)
	if boss.Fleeing || !slices.Contains(w.Enemies, boss) {
		t.Fatal("Boss flieht")
	}
	for _, e := range w.Enemies {
		if !e.Boss && !e.Fleeing {
			t.Fatal("Goblin flieht bei Tagesanbruch nicht")
		}
	}
}

// (f) Ein besiegter Boss kommt nie wieder, auch nicht nach einem Burgfall; ein lebender kommt nach dem Burgfall
// mit der nächsten Welle erneut.
func TestBossBesiegtKommtNieWieder(t *testing.T) {
	isl := bossIsland(t, 1)
	cave, mine := isl.Stages[1], isl.Stages[2]
	waveUntil(cave, 3)
	waveUntil(mine, 3)
	castleFallen(mine)
	startWave(mine)
	if len(bossesIn(mine)) != 1 {
		t.Fatal("unbesiegter Boss kommt nach dem Burgfall nicht wieder")
	}
	bossesIn(cave)[0].HP = 0
	removeDeadEnemies(cave)
	if !slices.Equal(isl.DefeatedBosses, []string{"trollKing"}) {
		t.Fatalf("Merker %v", isl.DefeatedBosses)
	}
	startWave(cave)
	castleFallen(cave)
	startWave(cave)
	startWave(cave)
	if len(bossesIn(cave)) != 0 {
		t.Fatal("besiegter Boss kehrt zurück")
	}
}

// (g) bossSpawned und bossDefeated je genau einmal (kein `kill` für den Boss).
func TestBossEreignisseGenauEinmal(t *testing.T) {
	isl := bossIsland(t, 2)
	w := isl.Stages[1]
	waveUntil(w, 5)
	bossesIn(w)[0].HP = 0
	removeDeadEnemies(w)
	startWave(w)
	s, d, k := countEvents(w, "bossSpawned"), countEvents(w, "bossDefeated"), countEvents(w, "kill")
	if s != 1 || d != 1 || k != 0 {
		t.Fatalf("bossSpawned %d, bossDefeated %d, kill %d; erwartet 1, 1, 0", s, d, k)
	}
	for _, ev := range w.Events {
		if ev["type"] == "bossDefeated" && ev["boss"] != "trollKing" {
			t.Fatalf("Ereignis %v", ev)
		}
	}
}

// (h) Gleicher Seed, gleicher Verlauf (zwei Spieler, Boss kämpft, beschwört und fällt).
func TestBossDeterministisch(t *testing.T) {
	trace := func() string {
		isl := bossIsland(t, 2)
		w := isl.Stages[0]
		waveUntil(w, 5)
		for range 900 {
			StepIsland(isl, nil, dt)
		}
		out := fmt.Sprint(len(w.Enemies))
		for _, e := range w.Enemies {
			out += fmt.Sprintf(" %s:%.3f:%.0f", e.Kind, e.X, e.HP)
		}
		if b := bossesIn(w); len(b) == 1 {
			b[0].HP = 0
		}
		StepIsland(isl, nil, dt)
		for _, c := range w.Coins {
			out += fmt.Sprintf(" c%.4f", c.X)
		}
		return out + fmt.Sprint(isl.DefeatedBosses, isl.SkillPool)
	}
	if a, b := trace(), trace(); a != b {
		t.Fatalf("Verlauf weicht ab:\n%s\n%s", a, b)
	}
}

// (i) Jeder Boss hat einen gültigen Bezug und Werte; Wald, Höhle und Mine haben einen Miniboss; Fehler beim Laden.
func TestBossDaten(t *testing.T) {
	for _, d := range []int{0, 1, 2} {
		if miniBoss(d) == nil {
			t.Fatalf("Stufe %d ohne Miniboss", d)
		}
	}
	for _, b := range bosses {
		if err := checkBoss(b); err != nil {
			t.Fatalf("%s: %v", b.ID, err)
		}
		if b.HPFactor != 8 || b.DamageFactor != 2 || b.Reward.Gold != 100 || b.Reward.Material != 50 {
			t.Fatalf("%s: Werte weichen von bosse.md § 1 ab: %+v", b.ID, b)
		}
	}
	raw, err := data.Files.ReadFile("bosses.json")
	if err != nil {
		t.Fatal(err)
	}
	var got []BossData
	if err := loadBosses(fstest.MapFS{"bosses.json": {Data: raw}}, &got); err != nil || len(got) != len(bosses) {
		t.Fatalf("eingebettete Daten: %v, %d Bosse", err, len(got))
	}
	for _, bad := range []string{`"base": "goblin"`, `"wave": 5`, `"depth": 0`, `"gold": 100`, `"enemy": "goblin"`} {
		broken := strings.Replace(string(raw), bad, strings.NewReplacer("goblin", "nope", "5", "0", "0", "9", "100", "-1").Replace(bad), 1)
		if broken == string(raw) {
			t.Fatalf("Testdaten: %s nicht gefunden", bad)
		}
		if err := loadBosses(fstest.MapFS{"bosses.json": {Data: []byte(broken)}}, &got); err == nil {
			t.Fatalf("%s: ungültige Daten geladen", bad)
		}
	}
}

// Ohne Insel (Golden-Läufe, Kampagne) gibt es keine Bosse.
func TestBossNurAufInseln(t *testing.T) {
	w := quietWorld(t)
	waveUntil(w, 6)
	if len(bossesIn(w)) != 0 {
		t.Fatal("Boss ohne Insel")
	}
}
