package sim

import (
	"bytes"
	"testing"
)

// Portale mit HP, Verteidiger-Wellen und sichere Stufe (B-927, docs/rules/kampf.md § 2), mit 2 Spielern über CreateIsland.

func portalIsland(t *testing.T) (*Island, *World) {
	t.Helper()
	isl, err := CreateIsland("portal", []int{0}, 1, CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	w.Troops, w.Enemies, w.SpawnQueue, w.Camps, w.Aggression = []*Troop{}, []*Enemy{}, []QueuedSpawn{}, []*Camp{}, nil
	if len(w.Players) != 2 || len(w.Portals) != 2 {
		t.Fatalf("%d Spieler, %d Portale", len(w.Players), len(w.Portals))
	}
	return isl, w
}

// attacker stellt einen Krieger im Angriff vor das Portal i.
func attacker(w *World, i int) *Troop {
	hp := troops["warrior"].HP
	t := &Troop{ID: w.newID(), Kind: "warrior", X: w.Portals[i], HP: hp, MaxHP: hp, AnchorX: w.HubX, attacking: true}
	w.Troops = append(w.Troops, t)
	return t
}

// siege rechnet seconds Sekunden und hält t dabei vor seinem Portal (den Marsch zum Portal setzt erst der Angriff, KS4).
func siege(w *World, t *Troop, x, seconds float64) {
	for tm := 0.0; tm < seconds; tm += dt {
		t.X = x
		Step(w, nil, dt)
	}
}

func TestPortalHPAusDenDaten(t *testing.T) {
	_, w := portalIsland(t)
	if len(w.PortalHP) != 2 || w.PortalHP[0] != w.Biome.Portals.HP || w.Biome.Portals.HP != 500 || w.Biome.Portals.Defense.Size <= 0 {
		t.Errorf("Portal-HP %v, Daten %+v", w.PortalHP, w.Biome.Portals)
	}
}

func TestNurTruppenImAngriffBeschaedigenPortale(t *testing.T) {
	_, w := portalIsland(t)
	idle := attacker(w, 0)
	idle.attacking = false
	for _, p := range w.Players { // Monarchen schlagen nicht auf Portale
		p.X = w.Portals[0]
	}
	spawnEnemy(w, "goblin", w.Portals[0]+1) // Gegner ebenso wenig
	siege(w, idle, w.Portals[0], 3)
	if w.PortalHP[0] != w.PortalMaxHP {
		t.Fatalf("Portal ohne Angreifer beschädigt: %v", w.PortalHP[0])
	}
	idle.attacking = true
	siege(w, idle, w.Portals[0], 3)
	if w.PortalHP[0] >= w.PortalMaxHP || w.PortalHP[1] != w.PortalMaxHP {
		t.Errorf("Angriff: HP %v", w.PortalHP)
	}
}

func TestAngegriffenesPortalSpawntVerteidiger(t *testing.T) {
	_, w := portalIsland(t)
	run(w, 25)
	if len(w.SpawnQueue) != 0 || len(w.Enemies) != 0 {
		t.Fatalf("Verteidiger ohne Angriff: %d + %d", len(w.SpawnQueue), len(w.Enemies))
	}
	a := attacker(w, 0)
	a.HP, a.MaxHP = 1e9, 1e9
	siege(w, a, w.Portals[0], 1)
	size := w.Biome.Portals.Defense.Size
	if got := len(w.SpawnQueue) + len(w.Enemies); got < size {
		t.Fatalf("%d Verteidiger, erwartet %d", got, size)
	}
	for _, s := range w.SpawnQueue {
		if s.X != w.Portals[0] {
			t.Errorf("Verteidiger am falschen Portal: %v", s.X)
		}
	}
	waves := 0 // der Krieger erschlägt die Verteidiger: gezählt wird das Ereignis
	for tm := 0.0; tm < w.Biome.Portals.Defense.IntervalSeconds+1; tm += dt {
		siege(w, a, w.Portals[0], dt)
		for _, e := range w.Events {
			if e["type"] == "portalDefense" {
				waves++
			}
		}
	}
	if waves != 1 {
		t.Errorf("%d Wellen im Takt, erwartet 1", waves)
	}
}

func TestLetztesPortalMachtStufeSicher(t *testing.T) {
	isl, w := portalIsland(t)
	pool := isl.SkillPool
	a := attacker(w, 0)
	w.PortalHP[0] = 1
	run(w, 1)
	if len(w.Portals) != 1 || len(w.PortalsDown) != 1 || w.Safe() {
		t.Fatalf("nach dem ersten Portal: %v / %v", w.Portals, w.PortalsDown)
	}
	if isl.SkillPool != pool {
		t.Error("Skill-Punkt schon beim ersten Portal")
	}
	a.X, w.PortalHP[0] = w.Portals[0], 1
	run(w, 1)
	if !w.Safe() || isl.SkillPool != pool+w.values().Monarch.SkillPointSources.Miniboss {
		t.Fatalf("sicher %v, Pool %d → %d", w.Safe(), pool, isl.SkillPool)
	}
	w.Cycle.Phase = "night"
	startWave(w)
	run(w, 20)
	if len(w.SpawnQueue) != 0 || len(w.Enemies) != 0 {
		t.Errorf("Welle in sicherer Stufe: %d + %d", len(w.SpawnQueue), len(w.Enemies))
	}
}

func TestSichereStufeBleibtNachDemLaden(t *testing.T) {
	isl, w := portalIsland(t)
	attacker(w, 0)
	w.PortalHP[0] = 1
	run(w, 1)
	save := isl.ToSave("2026-11-15T10:00:00Z")
	if len(save.Stages[0].PortalsDown) != 1 {
		t.Fatalf("Stand: %v", save.Stages[0].PortalsDown)
	}
	parsed, err := ParseIslandSave(saveJSON(t, save))
	if err != nil {
		t.Fatal(err)
	}
	loaded, err := FromIslandSave(parsed, islandSpeed)
	if err != nil {
		t.Fatal(err)
	}
	lw := loaded.Stages[0]
	if len(lw.Portals) != 1 || len(lw.PortalsDown) != 1 || lw.Portals[0] == lw.PortalsDown[0] {
		t.Fatalf("geladen: %v / %v", lw.Portals, lw.PortalsDown)
	}
	if !bytes.Equal(saveJSON(t, loaded.ToSave(save.SavedAt)), saveJSON(t, save)) {
		t.Error("Stand ändert sich beim Neuladen")
	}
	lw.Troops = []*Troop{}
	attacker(lw, 0).attacking = true
	lw.PortalHP[0] = 1
	run(lw, 1)
	save = loaded.ToSave(save.SavedAt)
	parsed, _ = ParseIslandSave(saveJSON(t, save))
	again, err := FromIslandSave(parsed, islandSpeed)
	if err != nil {
		t.Fatal(err)
	}
	aw := again.Stages[0]
	aw.Cycle.Phase = "night"
	startWave(aw)
	run(aw, 20)
	if !aw.Safe() || len(aw.SpawnQueue) != 0 || len(aw.Enemies) != 0 {
		t.Errorf("sicher %v, Welle %d + %d", aw.Safe(), len(aw.SpawnQueue), len(aw.Enemies))
	}
}

func TestDiebOhnePortalLaesstBeuteFallen(t *testing.T) {
	_, w := portalIsland(t)
	w.Portals, w.PortalHP = nil, nil
	p := w.Players[0]
	p.X, p.Gold = w.HubX+3, 50
	e := spawnEnemy(w, "greed", p.X+0.2)
	e.Cooldown = 0
	coins := len(w.Coins)
	run(w, 2)
	if e.CarriedGold != 0 || e.Fleeing || len(w.Coins) <= coins {
		t.Errorf("Beute %d, flieht %v, Münzen %d → %d", e.CarriedGold, e.Fleeing, coins, len(w.Coins))
	}
}

func TestPortalSimulationIstDeterministisch(t *testing.T) {
	var out [2][]byte
	for i := range out {
		isl, w := portalIsland(t)
		attacker(w, 0).HP = 1e9
		run(w, 60)
		out[i] = saveJSON(t, isl.ToSave("x"))
	}
	if !bytes.Equal(out[0], out[1]) {
		t.Error("zwei Läufe weichen ab")
	}
}

func TestLetztesPortalOhneDoppelPunktBeiMiniboss(t *testing.T) {
	isl, w := portalIsland(t)
	b := miniBoss(w.Biome.Depth)
	if b == nil {
		t.Skip("kein Miniboss in dieser Stufe")
	}
	mini := w.values().Monarch.SkillPointSources.Miniboss
	pool := isl.SkillPool
	secureStage(w) // Miniboss lebt noch oder ist nie erschienen: der Punkt kommt einmal
	if isl.SkillPool != pool+mini || !slicesContains(isl.DefeatedBosses, b.ID) {
		t.Fatalf("Pool %d → %d, besiegt %v", pool, isl.SkillPool, isl.DefeatedBosses)
	}
	secureStage(w)
	e := spawnBoss(w, b, w.HubX+100)
	e.HP = 0
	removeDeadEnemies(w)
	if isl.SkillPool != pool+mini {
		t.Errorf("Doppelter Punkt: Pool %d → %d", pool, isl.SkillPool)
	}
}

func slicesContains(s []string, v string) bool {
	for _, x := range s {
		if x == v {
			return true
		}
	}
	return false
}
