package sim

import (
	"fmt"
	"strings"
	"testing"
)

// Endboss je Insel und Bau in der tiefsten Stufe (B-981, WE5.2): jede Tiefe als tiefste Stufe, Wahl über `endBoss`,
// Fehlerfälle, zwei Spieler.

// lairIsland baut eine Insel der Test-Welt mit den Tiefen depths (Felder wie bei testIsland) und `players` Spielern.
func lairIsland(t *testing.T, depths []int, fields string, players int) (*Island, IslandDef) {
	t.Helper()
	raw := fmt.Sprintf(`,"depths":%v%s`, strings.ReplaceAll(fmt.Sprint(depths), " ", ","), fields)
	world, err := parseTestWorld(raw)
	if err != nil {
		t.Fatalf("Welt mit Tiefen %v: %v", depths, err)
	}
	def := world.Islands[0]
	isl, err := CreateIslandIn(world, "lair", depths, 1, def.CreateOptions())
	if err != nil {
		t.Fatal(err)
	}
	for range players {
		AddIslandPlayer(isl, len(depths)-1)
	}
	return isl, def
}

// AC-01: Eine Insel mit tiefster Stufe 0, 1, 2 oder 3 lädt und hat ihren Endboss; den Bau trägt nur die tiefste Stufe.
func TestInselTiefsteStufeTraegtBauUndEndboss(t *testing.T) {
	for deepest := range 4 {
		depths := make([]int, deepest+1)
		for i := range depths {
			depths[i] = i
		}
		isl, def := lairIsland(t, depths, "", 2)
		want := endBoss(deepest)
		if b := def.endBossOf(); want == nil || b != want || b.Depth != deepest {
			t.Fatalf("tiefste Stufe %d: Endboss %+v, erwartet %+v", deepest, b, want)
		}
		for i, w := range isl.Stages {
			_, hasLair := lairX(w)
			if hasLair != (i == deepest) || (w.lairBoss != nil) != hasLair {
				t.Errorf("tiefste Stufe %d, Stufe %d: Bau %v, Endboss %v", deepest, i, hasLair, w.lairBoss)
			}
		}
		if w := isl.Stages[deepest]; w.lairBoss != want || !AtLairNear(w) {
			t.Errorf("tiefste Stufe %d: lairBoss %v", deepest, w.lairBoss)
		}
	}
}

// AtLairNear: Ein Spieler steht am Bau, die Aktionsliste nennt ihn (AtLair).
func AtLairNear(w *World) bool {
	x, _ := lairX(w)
	w.Players[0].X = x
	return AtLair(w, w.Players[0])
}

// AC-01: `endBoss` wählt eine andere ID; der gewählte Endboss erscheint im Bau der tiefsten Stufe, ausgelöst von einem
// von zwei Spielern.
func TestInselEndbossWahl(t *testing.T) {
	isl, def := lairIsland(t, []int{0, 3}, `,"endBoss":"`+wardenID+`"`, 2)
	if b := def.endBossOf(); b == nil || b.ID != wardenID {
		t.Fatalf("Endboss %v, erwartet %s", b, wardenID)
	}
	w := isl.Stages[1]
	w.Troops, w.Enemies, w.SpawnQueue, w.Aggression = nil, nil, nil, nil
	x, ok := lairX(w)
	if !ok || w.Biome.Depth != 3 {
		t.Fatalf("Bau in Tiefe %d: %v", w.Biome.Depth, ok)
	}
	w.Players[0].X, w.Players[1].X = x-500, x-10
	events := endSteps(w, 5, true)
	if count(events, "bossSpawned") != 1 || warden(w) == nil || warden(w).X != x {
		t.Fatalf("Endboss %v, %d× bossSpawned", warden(w), count(events, "bossSpawned"))
	}
}

// AC-03: unbekannte ID und Miniboss in `endBoss` lehnen die Welt mit Meldung ab (zu kurzes Biom: level.CheckLairFit,
// lair_test.go; die kleinste Inselgröße von 16 Chunks trägt den Bau in jedem Biom).
func TestInselEndbossFehler(t *testing.T) {
	for field, want := range map[string]string{
		`,"endBoss":"x"`:            `Insel a: endBoss "x" unbekannt (erlaubt: forestChieftain, caveColossus, pitLord, smeltTitan, crystalHeartWarden)`,
		`,"endBoss":"trollKing"`:    `Insel a: endBoss "trollKing" ist ein Miniboss (erlaubt: `,
		`,"endBoss":"CaveColossus"`: `unbekannt`,
	} {
		if _, err := parseTestWorld(field); err == nil || !strings.Contains(err.Error(), want) {
			t.Errorf("%s: Fehler %v, erwartet %q", field, err, want)
		}
	}
}
