package sim

import (
	"bytes"
	"slices"
	"strings"
	"testing"
)

// Spielstand Version 5 (K2.3b, B-130/AC-04, B-103/AC-04): besiegte Bosse, aktuelle Insel, Münzzähler, Skill-Basis.

func reload(t *testing.T, isl *Island) *Island {
	t.Helper()
	return loadIsland(t, saveJSON(t, isl.ToSave("2026-10-07T22:00:00Z")))
}

// (a) Round-Trip: alles Neue bleibt erhalten, Speichern, Laden, Speichern ist gleich.
func TestSpielstandBossRundlauf(t *testing.T) {
	def := withTestIslands(t)
	isl := nightIsland(t)
	isl.Number, isl.DefeatedBosses, isl.GoldCollected, isl.Won = 1, []string{"goblinLeader", "trollKing"}, 37, true
	p := isl.Stages[0].Players[0]
	p.Skills, p.SkillBase = []string{"taunt"}, 1
	first := saveJSON(t, isl.ToSave("2026-10-07T22:00:00Z"))
	got := loadIsland(t, first)
	if got.Number != 1 || got.Scaling != def.DepthScaling || !slices.Equal(got.DefeatedBosses, isl.DefeatedBosses) ||
		got.GoldCollected != 37 || !got.Won || got.EndbossDefeated {
		t.Fatalf("Insel %d, Tabelle %+v, Bosse %v, Münzen %d, Sieg %v, Endboss %v",
			got.Number, got.Scaling, got.DefeatedBosses, got.GoldCollected, got.Won, got.EndbossDefeated)
	}
	if q := got.Stages[0].Players[0]; q.SkillBase != 1 || AvailablePoints(got.Stages[0], q) != 0 {
		t.Errorf("Skill-Basis %d, verfügbar %d", q.SkillBase, AvailablePoints(got.Stages[0], q))
	}
	if second := saveJSON(t, got.ToSave("2026-10-07T22:00:00Z")); !bytes.Equal(first, second) {
		t.Errorf("Speichern, Laden, Speichern ergibt etwas anderes:\n%s\n%s", first, second)
	}
}

// (b) Nach dem Laden kommt ein besiegter Miniboss in seiner Welle nicht, ein nicht besiegter schon.
func TestSpielstandBossMinibossKehrtNichtZurueck(t *testing.T) {
	isl := nightIsland(t)
	isl.DefeatedBosses = []string{miniBoss(0).ID}
	got := reload(t, isl)
	for i, w := range got.Stages {
		w.Wave = miniBoss(w.Biome.Depth).Wave + 2
		triggerBoss(w)
		if spawned := count(w.Events, "bossSpawned"); spawned != i {
			t.Errorf("Tiefe %d: %d× bossSpawned, erwartet %d", w.Biome.Depth, spawned, i)
		}
	}
}

// (b) Ein besiegter Endboss bleibt besiegt (aus der Liste abgeleitet), sein Bau bleibt leer.
func TestSpielstandBossEndbossBleibtBesiegt(t *testing.T) {
	isl := mustIsland(t, "familie", []int{0, 4})
	p := AddIslandPlayer(isl, 1)
	isl.DefeatedBosses = []string{endBoss(4).ID}
	got := reload(t, isl)
	if !got.EndbossDefeated {
		t.Fatal("Endboss nach dem Laden nicht besiegt")
	}
	w := got.Stages[1]
	x, ok := lairX(w)
	if !ok {
		t.Fatal("Kristallhöhle ohne Bau")
	}
	w.Players[0].X = x
	stepEndboss(w, dt)
	if got.endboss != nil || slices.ContainsFunc(w.Enemies, func(e *Enemy) bool { return e.Boss }) {
		t.Errorf("besiegter Endboss erscheint wieder (Spieler %d am Bau)", p.Index)
	}
}

// (c) Ein Stufenverlust und erneutes Speichern ändern die Liste nicht.
func TestSpielstandBossStufenverlust(t *testing.T) {
	isl := nightIsland(t)
	isl.DefeatedBosses = []string{"goblinLeader"}
	w := isl.Stages[0]
	w.Castle.HP = 0
	castleFallen(w)
	got := reload(t, reload(t, isl))
	if !slices.Equal(got.DefeatedBosses, []string{"goblinLeader"}) {
		t.Errorf("Bosse nach Burgfall und zweimal Laden: %v", got.DefeatedBosses)
	}
}

// (d) Versionen vor 5 laden mit leerer Bossliste, Insel 0 und Zählern 0; die Fixtures ab v5 mit ihrem Boss.
func TestSpielstandBossAlteVersionen(t *testing.T) {
	for v := 1; v < 5; v++ {
		got := loadIsland(t, readFixture(t, v))
		if got.Number != 0 || len(got.DefeatedBosses) != 0 || got.GoldCollected != 0 || got.EndbossDefeated || got.Won {
			t.Errorf("v%d: Insel %d, Bosse %v, Münzen %d", v, got.Number, got.DefeatedBosses, got.GoldCollected)
		}
	}
	for v := 5; v <= IslandSaveVersion; v++ {
		got := loadIsland(t, readFixture(t, v))
		if !slices.Equal(got.DefeatedBosses, []string{"goblinLeader"}) || got.GoldCollected != 37 || got.EndbossDefeated {
			t.Errorf("v%d: Bosse %v, Münzen %d, Endboss %v", v, got.DefeatedBosses, got.GoldCollected, got.EndbossDefeated)
		}
	}
}

// (f) Unbekannter Boss, doppelter Boss, Insel außerhalb der Daten, negative Zähler: Fehler mit Meldung.
func TestSpielstandBossUngueltig(t *testing.T) {
	cases := map[string]func(s *IslandSave){
		"unbekannter Boss": func(s *IslandSave) { s.DefeatedBosses = []string{"drache"} },
		"doppelter Boss":   func(s *IslandSave) { s.DefeatedBosses = []string{"goblinLeader", "goblinLeader"} },
		"Insel zu groß":    func(s *IslandSave) { s.Island = len(islandDefs) },
		"Insel negativ":    func(s *IslandSave) { s.Island = -1 },
		"Münzen negativ":   func(s *IslandSave) { s.GoldCollected = -1 },
		"Skill-Basis":      func(s *IslandSave) { s.Players[0].SkillBase = 1 },
	}
	for name, change := range cases {
		s, err := ParseIslandSave(readFixture(t, IslandSaveVersion))
		if err != nil {
			t.Fatal(err)
		}
		change(&s)
		if _, err := FromIslandSave(s, islandSpeed); err == nil || !strings.HasPrefix(err.Error(), "spielstand:") {
			t.Errorf("%s: Fehler %v", name, err)
		}
	}
}
