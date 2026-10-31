package sim

import (
	"encoding/json"
	"fmt"
	"reflect"
	"strings"
	"sync"
	"testing"
)

// Wertesatz je Raum (BAL8.2, B-681, docs/rules/welten.md § 1/§ 6): Jede Insel rechnet mit dem Wertesatz ihrer Welt;
// Welten mit verschiedenen Werten laufen nebeneinander, ohne sich zu beeinflussen; ohne Overrides gilt die Basis byte-gleich.

// teuerWorld erbt von der Standard-Welt und übersteuert die Mauer (Gold 9) und das Tempo der Bauern (5 statt 3).
func teuerWorld(t *testing.T) WorldDef {
	t.Helper()
	w := StandardWorld()
	w.ID, w.Name, w.Extends = "teuer", "Teuer", StandardWorldID
	w.Overrides = map[string]json.RawMessage{
		"buildings": json.RawMessage(`{"wall": {"cost": {"gold": 9}}}`),
		"troops":    json.RawMessage(`{"peasant": {"speed": 5}}`),
	}
	return need(ResolveWorld(w, standardLookup))(t)
}

func standardLookup(id string) (WorldDef, error) {
	if id == StandardWorldID {
		return StandardWorld(), nil
	}
	return WorldDef{}, fmt.Errorf("Welt %q unbekannt", id)
}

func need[T any](v T, err error) func(*testing.T) T {
	return func(t *testing.T) T {
		t.Helper()
		if err != nil {
			t.Fatal(err)
		}
		return v
	}
}

// worldRun legt die erste Insel der Welt an, tickt sie 40 Sekunden (schneller Zyklus, mit Nacht) und liefert die Stufen
// als JSON (Fingerabdruck des Zustands).
func worldRun(t *testing.T, world WorldDef) []byte {
	t.Helper()
	isl := need(CreateIslandIn(world, "werte-je-raum", world.Islands[0].Depths, islandSpeed, CreateOptions{}))(t)
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 0) // 2 Spieler wie 1
	for range 40 * 60 {
		StepIsland(isl, nil, 1.0/60)
	}
	return need(json.Marshal(isl.Stages))(t)
}

func wallCost(t *testing.T, world WorldDef) int {
	t.Helper()
	isl := need(CreateIslandIn(world, "mauer", world.Islands[0].Depths, 1, CreateOptions{}))(t)
	s := siteOfKind(isl.Stages[0], "wall")
	if s == nil {
		t.Fatal("keine Mauer in Stufe 0")
	}
	return siteCost(isl.Stages[0], s).Gold
}

// B-681/AC-01: Zwei Welten mit verschiedenen Kosten rechnen nebeneinander (Goroutinen) mit ihren eigenen Werten; jede
// liefert nebenläufig denselben Zustand wie allein, die Basis bleibt unverändert.
func TestWertesatzJeWeltParallelUnabhaengig(t *testing.T) {
	std, teuer := StandardWorld(), teuerWorld(t)
	baseWall, baseSpeed := buildings["wall"].Cost.Gold, troops["peasant"].Speed
	if got := wallCost(t, std); got != baseWall {
		t.Fatalf("Standard-Welt: Mauer %d Gold, erwartet %d (Basis)", got, baseWall)
	}
	if got := wallCost(t, teuer); got != 9 {
		t.Fatalf("Welt teuer: Mauer %d Gold, erwartet 9 (Override)", got)
	}
	soloStd, soloTeuer := worldRun(t, std), worldRun(t, teuer)
	if string(soloStd) == string(soloTeuer) {
		t.Fatal("Bauern-Tempo der Welt teuer wirkt nicht: gleicher Zustand wie die Standard-Welt")
	}
	var wg sync.WaitGroup
	got := make([][]byte, 4)
	for i, w := range []WorldDef{std, teuer, std, teuer} {
		wg.Go(func() { got[i] = worldRun(t, w) })
	}
	wg.Wait()
	for i, want := range [][]byte{soloStd, soloTeuer, soloStd, soloTeuer} {
		if string(got[i]) != string(want) {
			t.Errorf("Lauf %d nebenläufig anders als allein", i)
		}
	}
	if buildings["wall"].Cost.Gold != baseWall || troops["peasant"].Speed != baseSpeed {
		t.Fatalf("Basis verändert: Mauer %d, Bauer %v", buildings["wall"].Cost.Gold, troops["peasant"].Speed)
	}
}

// B-681/AC-02: Ohne Overrides in der Kette (auch mit `extends`) gilt der Basis-Wertesatz; der Zustand ist byte-gleich zur
// Insel aus CreateIsland. Die Golden-Tests (golden_test.go) laufen unverändert über denselben Weg.
func TestWertesatzOhneOverridesByteGleich(t *testing.T) {
	kind := StandardWorld()
	kind.ID, kind.Extends = "kind", StandardWorldID
	kind = need(ResolveWorld(kind, standardLookup))(t)
	if !reflect.DeepEqual(*kind.values, *defaultValues()) {
		t.Fatal("Welt ohne Overrides: Wertesatz weicht von der Basis ab")
	}
	isl := need(CreateIsland("werte-je-raum", StandardWorld().Islands[0].Depths, islandSpeed, CreateOptions{}))(t)
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 0)
	for range 40 * 60 {
		StepIsland(isl, nil, 1.0/60)
	}
	base := need(json.Marshal(isl.Stages))(t)
	for _, w := range []WorldDef{StandardWorld(), kind} {
		if got := worldRun(t, w); string(got) != string(base) {
			t.Errorf("Welt %s: Zustand nicht byte-gleich zur Basis", w.ID)
		}
	}
}

// Ein ungültiger Override lehnt die Welt beim Auflösen ab (welten.md § 1.2), die Meldung nennt die Welt-Datei.
func TestResolveWorldUngueltigerOverride(t *testing.T) {
	w := StandardWorld()
	w.ID = "kaputt"
	w.Overrides = map[string]json.RawMessage{"buildings": json.RawMessage(`{"wall": {"cost": {"gold": "viel"}}}`)}
	_, err := ResolveWorld(w, standardLookup)
	if err == nil || !strings.Contains(err.Error(), "data/worlds/kaputt.json") {
		t.Fatalf("Fehler %v, erwartet Meldung mit data/worlds/kaputt.json", err)
	}
	if _, err := CreateIslandIn(w, "kaputt", w.Islands[0].Depths, 1, CreateOptions{}); err == nil {
		t.Fatal("Insel einer Welt mit ungültigem Override angelegt")
	}
}

// Eine Zahl kommt nie aus zwei Quellen: HP und MaxHP, Truhen- und Morgengold folgen demselben Wertesatz (Review BAL8.2).
func TestWertesatzEineQuelleJeWert(t *testing.T) {
	w := StandardWorld()
	w.ID, w.Name, w.Extends = "stark", "Stark", StandardWorldID
	w.Overrides = map[string]json.RawMessage{
		"troops":  json.RawMessage(`{"peasant": {"hp": 77}, "vagrant": {"hp": 66}}`),
		"monarch": json.RawMessage(`{"base": {"hp": 55}}`),
		"economy": json.RawMessage(`{"chestGold": [50, 60], "dawnGoldPerDepth": 7}`),
	}
	world := need(ResolveWorld(w, standardLookup))(t)
	isl := need(CreateIslandIn(world, "eine-quelle", world.Islands[0].Depths, 1, CreateOptions{}))(t)
	p := AddIslandPlayer(isl, 0)
	if p.HP != 55 || p.MaxHP != 55 {
		t.Fatalf("Monarch HP=%v MaxHP=%v, erwartet 55/55", p.HP, p.MaxHP)
	}
	st := isl.Stages[0]
	if got := st.values().Economy.ChestGold; got != [2]int{50, 60} {
		t.Fatalf("chestGold = %v", got)
	}
	if got, want := dawnGold(st), st.values().Economy.DawnGoldPerPlayer+7*st.Biome.Depth; got != want {
		t.Fatalf("dawnGold = %d, erwartet %d", got, want)
	}
	v := spawnVagrant(st, st.HubX, st.HubX)
	if v.HP != 66 || v.MaxHP != 66 {
		t.Fatalf("Landstreicher HP=%v MaxHP=%v, erwartet 66/66", v.HP, v.MaxHP)
	}
}
